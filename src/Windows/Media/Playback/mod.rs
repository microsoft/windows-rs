#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutoLoadedDisplayPropertyKind(pub i32);
impl AutoLoadedDisplayPropertyKind {
    pub const None: AutoLoadedDisplayPropertyKind = AutoLoadedDisplayPropertyKind(0i32);
    pub const MusicOrVideo: AutoLoadedDisplayPropertyKind = AutoLoadedDisplayPropertyKind(1i32);
    pub const Music: AutoLoadedDisplayPropertyKind = AutoLoadedDisplayPropertyKind(2i32);
    pub const Video: AutoLoadedDisplayPropertyKind = AutoLoadedDisplayPropertyKind(3i32);
}
impl ::core::convert::From<i32> for AutoLoadedDisplayPropertyKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutoLoadedDisplayPropertyKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutoLoadedDisplayPropertyKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.AutoLoadedDisplayPropertyKind;i4)");
}
impl ::windows::core::DefaultType for AutoLoadedDisplayPropertyKind {
    type DefaultType = Self;
}
pub struct BackgroundMediaPlayer {}
impl BackgroundMediaPlayer {
    #[cfg(feature = "deprecated")]
    pub fn Current() -> ::windows::core::Result<MediaPlayer> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlayer>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceivedFromBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceivedFromBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceivedFromForeground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceivedFromForeground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SendMessageToBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SendMessageToForeground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn IsMediaPlaying() -> ::windows::core::Result<bool> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn Shutdown() -> ::windows::core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn IBackgroundMediaPlayerStatics<R, F: FnOnce(&IBackgroundMediaPlayerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundMediaPlayer, IBackgroundMediaPlayerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BackgroundMediaPlayer {
    const NAME: &'static str = "Windows.Media.Playback.BackgroundMediaPlayer";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CurrentMediaPlaybackItemChangedEventArgs(pub ::windows::core::IInspectable);
impl CurrentMediaPlaybackItemChangedEventArgs {
    pub fn NewItem(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn OldItem(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn Reason(&self) -> ::windows::core::Result<MediaPlaybackItemChangedReason> {
        let this = &::windows::core::Interface::cast::<ICurrentMediaPlaybackItemChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: MediaPlaybackItemChangedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackItemChangedReason>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CurrentMediaPlaybackItemChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.CurrentMediaPlaybackItemChangedEventArgs;{1743a892-5c43-4a15-967a-572d2d0f26c6})");
}
unsafe impl ::windows::core::Interface for CurrentMediaPlaybackItemChangedEventArgs {
    type Vtable = ICurrentMediaPlaybackItemChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1743a892_5c43_4a15_967a_572d2d0f26c6);
}
impl ::windows::core::RuntimeName for CurrentMediaPlaybackItemChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.CurrentMediaPlaybackItemChangedEventArgs";
}
impl ::core::convert::From<CurrentMediaPlaybackItemChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CurrentMediaPlaybackItemChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CurrentMediaPlaybackItemChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CurrentMediaPlaybackItemChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CurrentMediaPlaybackItemChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CurrentMediaPlaybackItemChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CurrentMediaPlaybackItemChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CurrentMediaPlaybackItemChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CurrentMediaPlaybackItemChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CurrentMediaPlaybackItemChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CurrentMediaPlaybackItemChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CurrentMediaPlaybackItemChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CurrentMediaPlaybackItemChangedEventArgs {}
unsafe impl ::core::marker::Sync for CurrentMediaPlaybackItemChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FailedMediaStreamKind(pub i32);
impl FailedMediaStreamKind {
    pub const Unknown: FailedMediaStreamKind = FailedMediaStreamKind(0i32);
    pub const Audio: FailedMediaStreamKind = FailedMediaStreamKind(1i32);
    pub const Video: FailedMediaStreamKind = FailedMediaStreamKind(2i32);
}
impl ::core::convert::From<i32> for FailedMediaStreamKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FailedMediaStreamKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FailedMediaStreamKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.FailedMediaStreamKind;i4)");
}
impl ::windows::core::DefaultType for FailedMediaStreamKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBackgroundMediaPlayerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBackgroundMediaPlayerStatics {
    type Vtable = IBackgroundMediaPlayerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x856ddbc1_55f7_471f_a0f2_68ac4c904592);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundMediaPlayerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICurrentMediaPlaybackItemChangedEventArgs {
    type Vtable = ICurrentMediaPlaybackItemChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1743a892_5c43_4a15_967a_572d2d0f26c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICurrentMediaPlaybackItemChangedEventArgs2 {
    type Vtable = ICurrentMediaPlaybackItemChangedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d80a51e_996e_40a9_be48_e66ec90b2b7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaPlaybackItemChangedReason) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaBreak(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaBreak {
    type Vtable = IMediaBreak_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x714be270_0def_4ebc_a489_6b34930e1558);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreak_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaBreakInsertionMethod) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaBreakEndedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaBreakEndedEventArgs {
    type Vtable = IMediaBreakEndedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32b93276_1c5d_4fee_8732_236dc3a88580);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakEndedEventArgs_abi(
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
pub struct IMediaBreakFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaBreakFactory {
    type Vtable = IMediaBreakFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4516e002_18e0_4079_8b5f_d33495c15d2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, insertionmethod: MediaBreakInsertionMethod, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, insertionmethod: MediaBreakInsertionMethod, presentationposition: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaBreakManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaBreakManager {
    type Vtable = IMediaBreakManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa854ddb1_feb4_4d9b_9d97_0fdbe58e5e39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaBreakSchedule(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaBreakSchedule {
    type Vtable = IMediaBreakSchedule_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa19a5813_98b6_41d8_83da_f971d22b7bba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakSchedule_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediabreak: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediabreak: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaBreakSeekedOverEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaBreakSeekedOverEventArgs {
    type Vtable = IMediaBreakSeekedOverEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5aa6746_0606_4492_b9d3_c3c8fde0a4ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakSeekedOverEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaBreakSkippedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaBreakSkippedEventArgs {
    type Vtable = IMediaBreakSkippedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ee94c05_2f54_4a3e_a3ab_24c3b270b4a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakSkippedEventArgs_abi(
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
pub struct IMediaBreakStartedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaBreakStartedEventArgs {
    type Vtable = IMediaBreakStartedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa87efe71_dfd4_454a_956e_0a4a648395f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakStartedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMediaEnginePlaybackSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaEnginePlaybackSource {
    type Vtable = IMediaEnginePlaybackSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c1d0ba7_3856_48b9_8dc6_244bf107bf8c);
}
impl IMediaEnginePlaybackSource {
    #[cfg(feature = "deprecated")]
    pub fn CurrentItem(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetPlaybackSource<'a, Param0: ::windows::core::IntoParam<'a, IMediaPlaybackSource>>(&self, source: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), source.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IMediaEnginePlaybackSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5c1d0ba7-3856-48b9-8dc6-244bf107bf8c}");
}
impl ::core::convert::From<IMediaEnginePlaybackSource> for ::windows::core::IUnknown {
    fn from(value: IMediaEnginePlaybackSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IMediaEnginePlaybackSource> for ::windows::core::IUnknown {
    fn from(value: &IMediaEnginePlaybackSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaEnginePlaybackSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMediaEnginePlaybackSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IMediaEnginePlaybackSource> for ::windows::core::IInspectable {
    fn from(value: IMediaEnginePlaybackSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaEnginePlaybackSource> for ::windows::core::IInspectable {
    fn from(value: &IMediaEnginePlaybackSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMediaEnginePlaybackSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IMediaEnginePlaybackSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEnginePlaybackSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaItemDisplayProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaItemDisplayProperties {
    type Vtable = IMediaItemDisplayProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e3c1b48_7097_4384_a217_c1291dfa8c16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaItemDisplayProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::MediaPlaybackType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::MediaPlaybackType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManager {
    type Vtable = IMediaPlaybackCommandManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5acee5a6_5cb6_4a5a_8521_cc86b1c1ed37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d6f4f23_5230_4411_a0e9_bad94c2a045c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerCommandBehavior(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerCommandBehavior {
    type Vtable = IMediaPlaybackCommandManagerCommandBehavior_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x786c1e78_ce78_4a10_afd6_843fcbb90c2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerCommandBehavior_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaCommandEnablingRule) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: MediaCommandEnablingRule) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerFastForwardReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f064d9_b491_4d0a_bc21_3098bd1332e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerNextReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerNextReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerNextReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1504433_a2b0_45d4_b9de_5f42ac14a839);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerNextReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPauseReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerPauseReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPauseReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ceccd1c_c25c_4221_b16c_c3c98ce012d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPauseReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPlayReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerPlayReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPlayReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9af0004e_578b_4c56_a006_16159d888a48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPlayReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPositionReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerPositionReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPositionReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5591a754_d627_4bdd_a90d_86a015b24902);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPositionReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPreviousReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerPreviousReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPreviousReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x525e3081_4632_4f76_99b1_d771623f6287);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPreviousReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerRateReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerRateReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRateReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18ea3939_4a16_4169_8b05_3eb9f5ff78eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerRateReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerRewindReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerRewindReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRewindReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f085947_a3c0_425d_aaef_97ba7898b141);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerRewindReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerShuffleReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerShuffleReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerShuffleReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50a05cef_63ee_4a96_b7b5_fee08b9ff90c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerShuffleReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackItem(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackItem {
    type Vtable = IMediaPlaybackItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x047097d2_e4af_48ab_b283_6929e674ece2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackItem2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackItem2 {
    type Vtable = IMediaPlaybackItem2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd859d171_d7ef_4b81_ac1f_f40493cbb091);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItem2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackItem3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackItem3 {
    type Vtable = IMediaPlaybackItem3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d328220_b80a_4d09_9ff8_f87094a1c831);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItem3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AutoLoadedDisplayPropertyKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AutoLoadedDisplayPropertyKind) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackItemError(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackItemError {
    type Vtable = IMediaPlaybackItemError_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69fbef2b_dcd6_4df9_a450_dbf4c6f1c2c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemError_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaPlaybackItemErrorCode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackItemFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackItemFactory {
    type Vtable = IMediaPlaybackItemFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7133fce1_1769_4ff9_a7c1_38d2c4d42360);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackItemFactory2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackItemFactory2 {
    type Vtable = IMediaPlaybackItemFactory2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd77cdf3a_b947_4972_b35d_adfb931a71e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr, starttime: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr, starttime: super::super::Foundation::TimeSpan, durationlimit: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackItemFailedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackItemFailedEventArgs {
    type Vtable = IMediaPlaybackItemFailedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7703134a_e9a7_47c3_862c_c656d30683d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemFailedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackItemOpenedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackItemOpenedEventArgs {
    type Vtable = IMediaPlaybackItemOpenedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbd9bd82_3037_4fbe_ae8f_39fc39edf4ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemOpenedEventArgs_abi(
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
pub struct IMediaPlaybackItemStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackItemStatics {
    type Vtable = IMediaPlaybackItemStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b1be7f4_4345_403c_8a67_f5de91df4c86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackList(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackList {
    type Vtable = IMediaPlaybackList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f77ee9c_dc42_4e26_a98d_7850df8ec925);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itemindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackList2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackList2 {
    type Vtable = IMediaPlaybackList2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e09b478_600a_4274_a14b_0b6723d0f48b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackList2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackList3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackList3 {
    type Vtable = IMediaPlaybackList3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd24bba9_bc47_4463_aa90_c18b7e5ffde1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackList3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackSession(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackSession {
    type Vtable = IMediaPlaybackSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc32b683d_0407_41ba_8946_8b345a5a5435);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaPlaybackState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackSession2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackSession2 {
    type Vtable = IMediaPlaybackSession2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8ba7c79_1fc8_4097_ad70_c0fa18cc0050);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSession2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rate1: f64, rate2: f64, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackSession3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackSession3 {
    type Vtable = IMediaPlaybackSession3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ba2b41a_a3e2_405f_b77b_a4812c238b66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSession3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackSessionBufferingStartedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackSessionBufferingStartedEventArgs {
    type Vtable = IMediaPlaybackSessionBufferingStartedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd6aafed_74e2_43b5_b115_76236c33791a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSessionBufferingStartedEventArgs_abi(
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
pub struct IMediaPlaybackSessionOutputDegradationPolicyState(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackSessionOutputDegradationPolicyState {
    type Vtable = IMediaPlaybackSessionOutputDegradationPolicyState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x558e727d_f633_49f9_965a_abaa1db709be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSessionOutputDegradationPolicyState_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaPlaybackSessionVideoConstrictionReason) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMediaPlaybackSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackSource {
    type Vtable = IMediaPlaybackSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef9dc2bc_9317_4696_b051_2bad643177b5);
}
impl IMediaPlaybackSource {}
unsafe impl ::windows::core::RuntimeType for IMediaPlaybackSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ef9dc2bc-9317-4696-b051-2bad643177b5}");
}
impl ::core::convert::From<IMediaPlaybackSource> for ::windows::core::IUnknown {
    fn from(value: IMediaPlaybackSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IMediaPlaybackSource> for ::windows::core::IUnknown {
    fn from(value: &IMediaPlaybackSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaPlaybackSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMediaPlaybackSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IMediaPlaybackSource> for ::windows::core::IInspectable {
    fn from(value: IMediaPlaybackSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaPlaybackSource> for ::windows::core::IInspectable {
    fn from(value: &IMediaPlaybackSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMediaPlaybackSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IMediaPlaybackSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackSphericalVideoProjection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackSphericalVideoProjection {
    type Vtable = IMediaPlaybackSphericalVideoProjection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd405b37c_6f0e_4661_b8ee_d487ba9752d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSphericalVideoProjection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SphericalVideoProjectionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: SphericalVideoProjectionMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlaybackTimedMetadataTrackList(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlaybackTimedMetadataTrackList {
    type Vtable = IMediaPlaybackTimedMetadataTrackList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b41319_bbfb_46a3_9372_9c9c744b9438);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackTimedMetadataTrackList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, value: TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayer {
    type Vtable = IMediaPlayer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x381a83cb_6fff_499b_8d64_2885dfc1249e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaPlayerState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayer2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayer2 {
    type Vtable = IMediaPlayer2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c841218_2123_4fc5_9082_2f883f77bdf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaPlayerAudioCategory) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: MediaPlayerAudioCategory) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaPlayerAudioDeviceType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: MediaPlayerAudioDeviceType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayer3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayer3 {
    type Vtable = IMediaPlayer3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee0660da_031b_4feb_bd9b_92e0a0a8d299);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StereoscopicVideoRenderMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: StereoscopicVideoRenderMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Casting")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Casting"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayer4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayer4 {
    type Vtable = IMediaPlayer4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80035db0_7448_4770_afcf_2a57450914c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayer5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayer5 {
    type Vtable = IMediaPlayer5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfe537fd_f86a_4446_bf4d_c8e792b7b4b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, targetrectangle: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))] usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destinationlefteye: ::windows::core::RawPtr, destinationrighteye: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayer6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayer6 {
    type Vtable = IMediaPlayer6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0caa086_ae65_414c_b010_8bc55f00e692);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer6_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, targetrectangle: super::super::Foundation::Rect, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayer7(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayer7 {
    type Vtable = IMediaPlayer7_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d1dc478_4500_4531_b3f4_777a71491f7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer7_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Audio")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Audio"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayerDataReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayerDataReceivedEventArgs {
    type Vtable = IMediaPlayerDataReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc75a9405_c801_412a_835b_83fc0e622a8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerDataReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayerEffects(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayerEffects {
    type Vtable = IMediaPlayerEffects_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85a1deda_cab6_4cc0_8be3_6035f4de2591);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerEffects_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectoptional: bool, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayerEffects2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayerEffects2 {
    type Vtable = IMediaPlayerEffects2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa419a79_1bbe_46c5_ae1f_8ee69fb3c2c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerEffects2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectoptional: bool, effectconfiguration: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayerFailedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayerFailedEventArgs {
    type Vtable = IMediaPlayerFailedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2744e9b9_a7e3_4f16_bac4_7914ebc08301);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerFailedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaPlayerError) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayerRateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayerRateChangedEventArgs {
    type Vtable = IMediaPlayerRateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40600d58_3b61_4bb2_989f_fc65608b6cab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerRateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayerSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayerSource {
    type Vtable = IMediaPlayerSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd4f8897_1423_4c3e_82c5_0fb1af94f715);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Protection")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))] usize,
    #[cfg(feature = "Media_Protection")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayerSource2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayerSource2 {
    type Vtable = IMediaPlayerSource2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82449b9f_7322_4c0b_b03b_3e69a48260c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerSource2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaPlayerSurface(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaPlayerSurface {
    type Vtable = IMediaPlayerSurface_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ed653bc_b736_49c3_830b_764a3845313a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerSurface_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlaybackMediaMarker(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlaybackMediaMarker {
    type Vtable = IPlaybackMediaMarker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4d22f5c_3c1c_4444_b6b9_778b0422d41a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackMediaMarker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlaybackMediaMarkerFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlaybackMediaMarkerFactory {
    type Vtable = IPlaybackMediaMarkerFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c530a78_e0ae_4e1a_a8c8_e23f982a937b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackMediaMarkerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan, mediamarkettype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlaybackMediaMarkerReachedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlaybackMediaMarkerReachedEventArgs {
    type Vtable = IPlaybackMediaMarkerReachedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x578cd1b9_90e2_4e60_abc4_8740b01f6196);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackMediaMarkerReachedEventArgs_abi(
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
pub struct IPlaybackMediaMarkerSequence(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlaybackMediaMarkerSequence {
    type Vtable = IPlaybackMediaMarkerSequence_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2810cee_638b_46cf_8817_1d111fe9d8c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackMediaMarkerSequence_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITimedMetadataPresentationModeChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITimedMetadataPresentationModeChangedEventArgs {
    type Vtable = ITimedMetadataPresentationModeChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1636099_65df_45ae_8cef_dc0b53fdc2bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataPresentationModeChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaBreak(pub ::windows::core::IInspectable);
impl MediaBreak {
    pub fn PlaybackList(&self) -> ::windows::core::Result<MediaPlaybackList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackList>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PresentationPosition(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    pub fn InsertionMethod(&self) -> ::windows::core::Result<MediaBreakInsertionMethod> {
        let this = self;
        unsafe {
            let mut result__: MediaBreakInsertionMethod = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaBreakInsertionMethod>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn CanStart(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanStart(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Create(insertionmethod: MediaBreakInsertionMethod) -> ::windows::core::Result<MediaBreak> {
        Self::IMediaBreakFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), insertionmethod, &mut result__).from_abi::<MediaBreak>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateWithPresentationPosition<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(insertionmethod: MediaBreakInsertionMethod, presentationposition: Param1) -> ::windows::core::Result<MediaBreak> {
        Self::IMediaBreakFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), insertionmethod, presentationposition.into_param().abi(), &mut result__).from_abi::<MediaBreak>(result__)
        })
    }
    pub fn IMediaBreakFactory<R, F: FnOnce(&IMediaBreakFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaBreak, IMediaBreakFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreak {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreak;{714be270-0def-4ebc-a489-6b34930e1558})");
}
unsafe impl ::windows::core::Interface for MediaBreak {
    type Vtable = IMediaBreak_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x714be270_0def_4ebc_a489_6b34930e1558);
}
impl ::windows::core::RuntimeName for MediaBreak {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreak";
}
impl ::core::convert::From<MediaBreak> for ::windows::core::IUnknown {
    fn from(value: MediaBreak) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaBreak> for ::windows::core::IUnknown {
    fn from(value: &MediaBreak) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaBreak> for ::windows::core::IInspectable {
    fn from(value: MediaBreak) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaBreak> for ::windows::core::IInspectable {
    fn from(value: &MediaBreak) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaBreak {}
unsafe impl ::core::marker::Sync for MediaBreak {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaBreakEndedEventArgs(pub ::windows::core::IInspectable);
impl MediaBreakEndedEventArgs {
    pub fn MediaBreak(&self) -> ::windows::core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaBreak>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreakEndedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakEndedEventArgs;{32b93276-1c5d-4fee-8732-236dc3a88580})");
}
unsafe impl ::windows::core::Interface for MediaBreakEndedEventArgs {
    type Vtable = IMediaBreakEndedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32b93276_1c5d_4fee_8732_236dc3a88580);
}
impl ::windows::core::RuntimeName for MediaBreakEndedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakEndedEventArgs";
}
impl ::core::convert::From<MediaBreakEndedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaBreakEndedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaBreakEndedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaBreakEndedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaBreakEndedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaBreakEndedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaBreakEndedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaBreakEndedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaBreakEndedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaBreakEndedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaBreakEndedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaBreakEndedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaBreakEndedEventArgs {}
unsafe impl ::core::marker::Sync for MediaBreakEndedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaBreakInsertionMethod(pub i32);
impl MediaBreakInsertionMethod {
    pub const Interrupt: MediaBreakInsertionMethod = MediaBreakInsertionMethod(0i32);
    pub const Replace: MediaBreakInsertionMethod = MediaBreakInsertionMethod(1i32);
}
impl ::core::convert::From<i32> for MediaBreakInsertionMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaBreakInsertionMethod {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaBreakInsertionMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaBreakInsertionMethod;i4)");
}
impl ::windows::core::DefaultType for MediaBreakInsertionMethod {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaBreakManager(pub ::windows::core::IInspectable);
impl MediaBreakManager {
    #[cfg(feature = "Foundation")]
    pub fn BreaksSeekedOver<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSeekedOverEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBreaksSeekedOver<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BreakStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakStartedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBreakStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BreakEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakEndedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBreakEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BreakSkipped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSkippedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBreakSkipped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn CurrentBreak(&self) -> ::windows::core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaBreak>(result__)
        }
    }
    pub fn PlaybackSession(&self) -> ::windows::core::Result<MediaPlaybackSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackSession>(result__)
        }
    }
    pub fn PlayBreak<'a, Param0: ::windows::core::IntoParam<'a, MediaBreak>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SkipCurrentBreak(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreakManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakManager;{a854ddb1-feb4-4d9b-9d97-0fdbe58e5e39})");
}
unsafe impl ::windows::core::Interface for MediaBreakManager {
    type Vtable = IMediaBreakManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa854ddb1_feb4_4d9b_9d97_0fdbe58e5e39);
}
impl ::windows::core::RuntimeName for MediaBreakManager {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakManager";
}
impl ::core::convert::From<MediaBreakManager> for ::windows::core::IUnknown {
    fn from(value: MediaBreakManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaBreakManager> for ::windows::core::IUnknown {
    fn from(value: &MediaBreakManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaBreakManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaBreakManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaBreakManager> for ::windows::core::IInspectable {
    fn from(value: MediaBreakManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaBreakManager> for ::windows::core::IInspectable {
    fn from(value: &MediaBreakManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaBreakManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaBreakManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaBreakManager {}
unsafe impl ::core::marker::Sync for MediaBreakManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaBreakSchedule(pub ::windows::core::IInspectable);
impl MediaBreakSchedule {
    #[cfg(feature = "Foundation")]
    pub fn ScheduleChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaBreakSchedule, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveScheduleChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn InsertMidrollBreak<'a, Param0: ::windows::core::IntoParam<'a, MediaBreak>>(&self, mediabreak: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mediabreak.into_param().abi()).ok() }
    }
    pub fn RemoveMidrollBreak<'a, Param0: ::windows::core::IntoParam<'a, MediaBreak>>(&self, mediabreak: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), mediabreak.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MidrollBreaks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaBreak>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaBreak>>(result__)
        }
    }
    pub fn SetPrerollBreak<'a, Param0: ::windows::core::IntoParam<'a, MediaBreak>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PrerollBreak(&self) -> ::windows::core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaBreak>(result__)
        }
    }
    pub fn SetPostrollBreak<'a, Param0: ::windows::core::IntoParam<'a, MediaBreak>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PostrollBreak(&self) -> ::windows::core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaBreak>(result__)
        }
    }
    pub fn PlaybackItem(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreakSchedule {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakSchedule;{a19a5813-98b6-41d8-83da-f971d22b7bba})");
}
unsafe impl ::windows::core::Interface for MediaBreakSchedule {
    type Vtable = IMediaBreakSchedule_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa19a5813_98b6_41d8_83da_f971d22b7bba);
}
impl ::windows::core::RuntimeName for MediaBreakSchedule {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakSchedule";
}
impl ::core::convert::From<MediaBreakSchedule> for ::windows::core::IUnknown {
    fn from(value: MediaBreakSchedule) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaBreakSchedule> for ::windows::core::IUnknown {
    fn from(value: &MediaBreakSchedule) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaBreakSchedule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaBreakSchedule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaBreakSchedule> for ::windows::core::IInspectable {
    fn from(value: MediaBreakSchedule) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaBreakSchedule> for ::windows::core::IInspectable {
    fn from(value: &MediaBreakSchedule) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaBreakSchedule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaBreakSchedule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaBreakSchedule {}
unsafe impl ::core::marker::Sync for MediaBreakSchedule {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaBreakSeekedOverEventArgs(pub ::windows::core::IInspectable);
impl MediaBreakSeekedOverEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SeekedOverBreaks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaBreak>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaBreak>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn OldPosition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn NewPosition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreakSeekedOverEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakSeekedOverEventArgs;{e5aa6746-0606-4492-b9d3-c3c8fde0a4ea})");
}
unsafe impl ::windows::core::Interface for MediaBreakSeekedOverEventArgs {
    type Vtable = IMediaBreakSeekedOverEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5aa6746_0606_4492_b9d3_c3c8fde0a4ea);
}
impl ::windows::core::RuntimeName for MediaBreakSeekedOverEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakSeekedOverEventArgs";
}
impl ::core::convert::From<MediaBreakSeekedOverEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaBreakSeekedOverEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaBreakSeekedOverEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaBreakSeekedOverEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaBreakSeekedOverEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaBreakSeekedOverEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaBreakSeekedOverEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaBreakSeekedOverEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaBreakSeekedOverEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaBreakSeekedOverEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaBreakSeekedOverEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaBreakSeekedOverEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaBreakSeekedOverEventArgs {}
unsafe impl ::core::marker::Sync for MediaBreakSeekedOverEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaBreakSkippedEventArgs(pub ::windows::core::IInspectable);
impl MediaBreakSkippedEventArgs {
    pub fn MediaBreak(&self) -> ::windows::core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaBreak>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreakSkippedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakSkippedEventArgs;{6ee94c05-2f54-4a3e-a3ab-24c3b270b4a3})");
}
unsafe impl ::windows::core::Interface for MediaBreakSkippedEventArgs {
    type Vtable = IMediaBreakSkippedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ee94c05_2f54_4a3e_a3ab_24c3b270b4a3);
}
impl ::windows::core::RuntimeName for MediaBreakSkippedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakSkippedEventArgs";
}
impl ::core::convert::From<MediaBreakSkippedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaBreakSkippedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaBreakSkippedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaBreakSkippedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaBreakSkippedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaBreakSkippedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaBreakSkippedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaBreakSkippedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaBreakSkippedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaBreakSkippedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaBreakSkippedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaBreakSkippedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaBreakSkippedEventArgs {}
unsafe impl ::core::marker::Sync for MediaBreakSkippedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaBreakStartedEventArgs(pub ::windows::core::IInspectable);
impl MediaBreakStartedEventArgs {
    pub fn MediaBreak(&self) -> ::windows::core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaBreak>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreakStartedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakStartedEventArgs;{a87efe71-dfd4-454a-956e-0a4a648395f8})");
}
unsafe impl ::windows::core::Interface for MediaBreakStartedEventArgs {
    type Vtable = IMediaBreakStartedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa87efe71_dfd4_454a_956e_0a4a648395f8);
}
impl ::windows::core::RuntimeName for MediaBreakStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakStartedEventArgs";
}
impl ::core::convert::From<MediaBreakStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaBreakStartedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaBreakStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaBreakStartedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaBreakStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaBreakStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaBreakStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaBreakStartedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaBreakStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaBreakStartedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaBreakStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaBreakStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaBreakStartedEventArgs {}
unsafe impl ::core::marker::Sync for MediaBreakStartedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCommandEnablingRule(pub i32);
impl MediaCommandEnablingRule {
    pub const Auto: MediaCommandEnablingRule = MediaCommandEnablingRule(0i32);
    pub const Always: MediaCommandEnablingRule = MediaCommandEnablingRule(1i32);
    pub const Never: MediaCommandEnablingRule = MediaCommandEnablingRule(2i32);
}
impl ::core::convert::From<i32> for MediaCommandEnablingRule {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaCommandEnablingRule {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaCommandEnablingRule {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaCommandEnablingRule;i4)");
}
impl ::windows::core::DefaultType for MediaCommandEnablingRule {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaItemDisplayProperties(pub ::windows::core::IInspectable);
impl MediaItemDisplayProperties {
    pub fn Type(&self) -> ::windows::core::Result<super::MediaPlaybackType> {
        let this = self;
        unsafe {
            let mut result__: super::MediaPlaybackType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaPlaybackType>(result__)
        }
    }
    pub fn SetType(&self, value: super::MediaPlaybackType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MusicProperties(&self) -> ::windows::core::Result<super::MusicDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MusicDisplayProperties>(result__)
        }
    }
    pub fn VideoProperties(&self) -> ::windows::core::Result<super::VideoDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoDisplayProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::RandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::RandomAccessStreamReference>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ClearAll(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaItemDisplayProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaItemDisplayProperties;{1e3c1b48-7097-4384-a217-c1291dfa8c16})");
}
unsafe impl ::windows::core::Interface for MediaItemDisplayProperties {
    type Vtable = IMediaItemDisplayProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e3c1b48_7097_4384_a217_c1291dfa8c16);
}
impl ::windows::core::RuntimeName for MediaItemDisplayProperties {
    const NAME: &'static str = "Windows.Media.Playback.MediaItemDisplayProperties";
}
impl ::core::convert::From<MediaItemDisplayProperties> for ::windows::core::IUnknown {
    fn from(value: MediaItemDisplayProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaItemDisplayProperties> for ::windows::core::IUnknown {
    fn from(value: &MediaItemDisplayProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaItemDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaItemDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaItemDisplayProperties> for ::windows::core::IInspectable {
    fn from(value: MediaItemDisplayProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaItemDisplayProperties> for ::windows::core::IInspectable {
    fn from(value: &MediaItemDisplayProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaItemDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaItemDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaItemDisplayProperties {}
unsafe impl ::core::marker::Sync for MediaItemDisplayProperties {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackAudioTrackList(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl MediaPlaybackAudioTrackList {
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<super::Core::AudioTrack> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<super::Core::AudioTrack>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, super::Core::AudioTrack>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetMany(&self, startindex: u32, items: &mut [<super::Core::AudioTrack as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::Core::AudioTrack>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::Core::AudioTrack>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::Core::AudioTrack>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn SelectedIndexChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<super::Core::ISingleSelectMediaTrackList, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn RemoveSelectedIndexChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SelectedIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for MediaPlaybackAudioTrackList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackAudioTrackList;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Media.Core.AudioTrack;{03e1fafc-c931-491a-b46b-c10ee8c256b7})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for MediaPlaybackAudioTrackList {
    type Vtable = super::super::Foundation::Collections::IVectorView_abi<super::Core::AudioTrack>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::Foundation::Collections::IVectorView<super::Core::AudioTrack> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for MediaPlaybackAudioTrackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackAudioTrackList";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<MediaPlaybackAudioTrackList> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackAudioTrackList) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&MediaPlaybackAudioTrackList> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackAudioTrackList) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<MediaPlaybackAudioTrackList> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackAudioTrackList) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&MediaPlaybackAudioTrackList> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackAudioTrackList) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<MediaPlaybackAudioTrackList> for super::super::Foundation::Collections::IVectorView<super::Core::AudioTrack> {
    fn from(value: MediaPlaybackAudioTrackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&MediaPlaybackAudioTrackList> for super::super::Foundation::Collections::IVectorView<super::Core::AudioTrack> {
    fn from(value: &MediaPlaybackAudioTrackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::Core::AudioTrack>> for MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<super::Core::AudioTrack>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::Core::AudioTrack>> for &MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<super::Core::AudioTrack>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<MediaPlaybackAudioTrackList> for super::super::Foundation::Collections::IIterable<super::Core::AudioTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackAudioTrackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&MediaPlaybackAudioTrackList> for super::super::Foundation::Collections::IIterable<super::Core::AudioTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackAudioTrackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::Core::AudioTrack>> for MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::Core::AudioTrack>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::Core::AudioTrack>> for &MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::Core::AudioTrack>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::Core::AudioTrack>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<MediaPlaybackAudioTrackList> for super::Core::ISingleSelectMediaTrackList {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackAudioTrackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackAudioTrackList> for super::Core::ISingleSelectMediaTrackList {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackAudioTrackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl<'a> ::windows::core::IntoParam<'a, super::Core::ISingleSelectMediaTrackList> for MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::Core::ISingleSelectMediaTrackList> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl<'a> ::windows::core::IntoParam<'a, super::Core::ISingleSelectMediaTrackList> for &MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::Core::ISingleSelectMediaTrackList> {
        ::core::convert::TryInto::<super::Core::ISingleSelectMediaTrackList>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for MediaPlaybackAudioTrackList {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for MediaPlaybackAudioTrackList {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for MediaPlaybackAudioTrackList {
    type Item = super::Core::AudioTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &MediaPlaybackAudioTrackList {
    type Item = super::Core::AudioTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackCommandManager(pub ::windows::core::IInspectable);
impl MediaPlaybackCommandManager {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MediaPlayer(&self) -> ::windows::core::Result<MediaPlayer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlayer>(result__)
        }
    }
    pub fn PlayBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn PauseBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn NextBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn PreviousBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn FastForwardBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn RewindBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn ShuffleBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn AutoRepeatModeBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn PositionBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn RateBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PlayReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPlayReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePlayReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PauseReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPauseReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePauseReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn NextReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerNextReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveNextReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PreviousReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPreviousReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePreviousReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn FastForwardReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerFastForwardReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveFastForwardReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RewindReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRewindReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRewindReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ShuffleReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerShuffleReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveShuffleReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AutoRepeatModeReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAutoRepeatModeReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PositionReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPositionReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RateReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRateReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRateReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManager;{5acee5a6-5cb6-4a5a-8521-cc86b1c1ed37})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManager {
    type Vtable = IMediaPlaybackCommandManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5acee5a6_5cb6_4a5a_8521_cc86b1c1ed37);
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManager {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManager";
}
impl ::core::convert::From<MediaPlaybackCommandManager> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackCommandManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManager> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackCommandManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackCommandManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackCommandManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackCommandManager> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackCommandManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManager> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackCommandManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackCommandManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackCommandManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManager {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AutoRepeatMode(&self) -> ::windows::core::Result<super::MediaPlaybackAutoRepeatMode> {
        let this = self;
        unsafe {
            let mut result__: super::MediaPlaybackAutoRepeatMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaPlaybackAutoRepeatMode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs;{3d6f4f23-5230-4411-a0e9-bad94c2a045c})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d6f4f23_5230_4411_a0e9_bad94c2a045c);
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackCommandManagerCommandBehavior(pub ::windows::core::IInspectable);
impl MediaPlaybackCommandManagerCommandBehavior {
    pub fn CommandManager(&self) -> ::windows::core::Result<MediaPlaybackCommandManager> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackCommandManager>(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn EnablingRule(&self) -> ::windows::core::Result<MediaCommandEnablingRule> {
        let this = self;
        unsafe {
            let mut result__: MediaCommandEnablingRule = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCommandEnablingRule>(result__)
        }
    }
    pub fn SetEnablingRule(&self, value: MediaCommandEnablingRule) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManagerCommandBehavior, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerCommandBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerCommandBehavior;{786c1e78-ce78-4a10-afd6-843fcbb90c2e})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerCommandBehavior {
    type Vtable = IMediaPlaybackCommandManagerCommandBehavior_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x786c1e78_ce78_4a10_afd6_843fcbb90c2e);
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerCommandBehavior {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerCommandBehavior";
}
impl ::core::convert::From<MediaPlaybackCommandManagerCommandBehavior> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerCommandBehavior) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerCommandBehavior> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerCommandBehavior) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackCommandManagerCommandBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackCommandManagerCommandBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerCommandBehavior> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerCommandBehavior) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerCommandBehavior> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerCommandBehavior) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackCommandManagerCommandBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackCommandManagerCommandBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerCommandBehavior {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerCommandBehavior {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackCommandManagerFastForwardReceivedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerFastForwardReceivedEventArgs;{30f064d9-b491-4d0a-bc21-3098bd1332e9})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f064d9_b491_4d0a_bc21_3098bd1332e9);
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerFastForwardReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerFastForwardReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerFastForwardReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerFastForwardReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerFastForwardReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerFastForwardReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerFastForwardReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerFastForwardReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerFastForwardReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackCommandManagerNextReceivedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlaybackCommandManagerNextReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerNextReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerNextReceivedEventArgs;{e1504433-a2b0-45d4-b9de-5f42ac14a839})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerNextReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerNextReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1504433_a2b0_45d4_b9de_5f42ac14a839);
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerNextReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerNextReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerNextReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerNextReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerNextReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerNextReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerNextReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerNextReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerNextReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerNextReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerNextReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerNextReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackCommandManagerPauseReceivedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlaybackCommandManagerPauseReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerPauseReceivedEventArgs;{5ceccd1c-c25c-4221-b16c-c3c98ce012d6})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPauseReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ceccd1c_c25c_4221_b16c_c3c98ce012d6);
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPauseReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerPauseReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerPauseReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPauseReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerPauseReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerPauseReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerPauseReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPauseReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerPauseReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerPauseReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerPauseReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackCommandManagerPlayReceivedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlaybackCommandManagerPlayReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerPlayReceivedEventArgs;{9af0004e-578b-4c56-a006-16159d888a48})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPlayReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9af0004e_578b_4c56_a006_16159d888a48);
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPlayReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerPlayReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerPlayReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPlayReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerPlayReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerPlayReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerPlayReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPlayReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerPlayReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerPlayReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerPlayReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackCommandManagerPositionReceivedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlaybackCommandManagerPositionReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerPositionReceivedEventArgs;{5591a754-d627-4bdd-a90d-86a015b24902})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPositionReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5591a754_d627_4bdd_a90d_86a015b24902);
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPositionReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerPositionReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerPositionReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPositionReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerPositionReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerPositionReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerPositionReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPositionReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerPositionReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerPositionReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerPositionReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackCommandManagerPreviousReceivedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerPreviousReceivedEventArgs;{525e3081-4632-4f76-99b1-d771623f6287})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPreviousReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x525e3081_4632_4f76_99b1_d771623f6287);
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPreviousReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerPreviousReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerPreviousReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPreviousReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerPreviousReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerPreviousReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerPreviousReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPreviousReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerPreviousReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerPreviousReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerPreviousReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackCommandManagerRateReceivedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlaybackCommandManagerRateReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PlaybackRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerRateReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerRateReceivedEventArgs;{18ea3939-4a16-4169-8b05-3eb9f5ff78eb})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerRateReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRateReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18ea3939_4a16_4169_8b05_3eb9f5ff78eb);
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerRateReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerRateReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerRateReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerRateReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerRateReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerRateReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerRateReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerRateReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerRateReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerRateReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerRateReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerRateReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackCommandManagerRewindReceivedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlaybackCommandManagerRewindReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerRewindReceivedEventArgs;{9f085947-a3c0-425d-aaef-97ba7898b141})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRewindReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f085947_a3c0_425d_aaef_97ba7898b141);
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerRewindReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerRewindReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerRewindReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerRewindReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerRewindReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerRewindReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerRewindReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerRewindReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerRewindReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerRewindReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerRewindReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackCommandManagerShuffleReceivedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsShuffleRequested(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerShuffleReceivedEventArgs;{50a05cef-63ee-4a96-b7b5-fee08b9ff90c})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerShuffleReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50a05cef_63ee_4a96_b7b5_fee08b9ff90c);
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerShuffleReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerShuffleReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerShuffleReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerShuffleReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerShuffleReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerShuffleReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerShuffleReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerShuffleReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerShuffleReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerShuffleReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerShuffleReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackItem(pub ::windows::core::IInspectable);
impl MediaPlaybackItem {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn AudioTracksChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioTracksChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn VideoTracksChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoTracksChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn TimedMetadataTracksChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveTimedMetadataTracksChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn Source(&self) -> ::windows::core::Result<super::Core::MediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::MediaSource>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AudioTracks(&self) -> ::windows::core::Result<MediaPlaybackAudioTrackList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackAudioTrackList>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn VideoTracks(&self) -> ::windows::core::Result<MediaPlaybackVideoTrackList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackVideoTrackList>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TimedMetadataTracks(&self) -> ::windows::core::Result<MediaPlaybackTimedMetadataTrackList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackTimedMetadataTrackList>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Core::MediaSource>>(source: Param0) -> ::windows::core::Result<MediaPlaybackItem> {
        Self::IMediaPlaybackItemFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), source.into_param().abi(), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        })
    }
    #[cfg(feature = "Media_Core")]
    pub fn FindFromMediaSource<'a, Param0: ::windows::core::IntoParam<'a, super::Core::MediaSource>>(source: Param0) -> ::windows::core::Result<MediaPlaybackItem> {
        Self::IMediaPlaybackItemStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), source.into_param().abi(), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        })
    }
    pub fn BreakSchedule(&self) -> ::windows::core::Result<MediaBreakSchedule> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaBreakSchedule>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DurationLimit(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    pub fn CanSkip(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanSkip(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GetDisplayProperties(&self) -> ::windows::core::Result<MediaItemDisplayProperties> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaItemDisplayProperties>(result__)
        }
    }
    pub fn ApplyDisplayProperties<'a, Param0: ::windows::core::IntoParam<'a, MediaItemDisplayProperties>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn CreateWithStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::Core::MediaSource>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(source: Param0, starttime: Param1) -> ::windows::core::Result<MediaPlaybackItem> {
        Self::IMediaPlaybackItemFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), source.into_param().abi(), starttime.into_param().abi(), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn CreateWithStartTimeAndDurationLimit<'a, Param0: ::windows::core::IntoParam<'a, super::Core::MediaSource>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(source: Param0, starttime: Param1, durationlimit: Param2) -> ::windows::core::Result<MediaPlaybackItem> {
        Self::IMediaPlaybackItemFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), source.into_param().abi(), starttime.into_param().abi(), durationlimit.into_param().abi(), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        })
    }
    pub fn IsDisabledInPlaybackList(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDisabledInPlaybackList(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TotalDownloadProgress(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn AutoLoadedDisplayProperties(&self) -> ::windows::core::Result<AutoLoadedDisplayPropertyKind> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe {
            let mut result__: AutoLoadedDisplayPropertyKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutoLoadedDisplayPropertyKind>(result__)
        }
    }
    pub fn SetAutoLoadedDisplayProperties(&self, value: AutoLoadedDisplayPropertyKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IMediaPlaybackItemFactory<R, F: FnOnce(&IMediaPlaybackItemFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaPlaybackItem, IMediaPlaybackItemFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaPlaybackItemStatics<R, F: FnOnce(&IMediaPlaybackItemStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaPlaybackItem, IMediaPlaybackItemStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaPlaybackItemFactory2<R, F: FnOnce(&IMediaPlaybackItemFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaPlaybackItem, IMediaPlaybackItemFactory2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackItem;{047097d2-e4af-48ab-b283-6929e674ece2})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackItem {
    type Vtable = IMediaPlaybackItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x047097d2_e4af_48ab_b283_6929e674ece2);
}
impl ::windows::core::RuntimeName for MediaPlaybackItem {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItem";
}
impl ::core::convert::From<MediaPlaybackItem> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackItem) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackItem> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackItem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackItem> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackItem> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MediaPlaybackItem> for IMediaPlaybackSource {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackItem) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaPlaybackItem> for IMediaPlaybackSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackItem) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaPlaybackSource> for MediaPlaybackItem {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaPlaybackSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaPlaybackSource> for &MediaPlaybackItem {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaPlaybackSource> {
        ::core::convert::TryInto::<IMediaPlaybackSource>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackItem {}
unsafe impl ::core::marker::Sync for MediaPlaybackItem {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaPlaybackItemChangedReason(pub i32);
impl MediaPlaybackItemChangedReason {
    pub const InitialItem: MediaPlaybackItemChangedReason = MediaPlaybackItemChangedReason(0i32);
    pub const EndOfStream: MediaPlaybackItemChangedReason = MediaPlaybackItemChangedReason(1i32);
    pub const Error: MediaPlaybackItemChangedReason = MediaPlaybackItemChangedReason(2i32);
    pub const AppRequested: MediaPlaybackItemChangedReason = MediaPlaybackItemChangedReason(3i32);
}
impl ::core::convert::From<i32> for MediaPlaybackItemChangedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaPlaybackItemChangedReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackItemChangedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackItemChangedReason;i4)");
}
impl ::windows::core::DefaultType for MediaPlaybackItemChangedReason {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackItemError(pub ::windows::core::IInspectable);
impl MediaPlaybackItemError {
    pub fn ErrorCode(&self) -> ::windows::core::Result<MediaPlaybackItemErrorCode> {
        let this = self;
        unsafe {
            let mut result__: MediaPlaybackItemErrorCode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackItemErrorCode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackItemError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackItemError;{69fbef2b-dcd6-4df9-a450-dbf4c6f1c2c2})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackItemError {
    type Vtable = IMediaPlaybackItemError_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69fbef2b_dcd6_4df9_a450_dbf4c6f1c2c2);
}
impl ::windows::core::RuntimeName for MediaPlaybackItemError {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItemError";
}
impl ::core::convert::From<MediaPlaybackItemError> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackItemError) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackItemError> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackItemError) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackItemError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackItemError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackItemError> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackItemError) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackItemError> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackItemError) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackItemError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackItemError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackItemError {}
unsafe impl ::core::marker::Sync for MediaPlaybackItemError {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaPlaybackItemErrorCode(pub i32);
impl MediaPlaybackItemErrorCode {
    pub const None: MediaPlaybackItemErrorCode = MediaPlaybackItemErrorCode(0i32);
    pub const Aborted: MediaPlaybackItemErrorCode = MediaPlaybackItemErrorCode(1i32);
    pub const NetworkError: MediaPlaybackItemErrorCode = MediaPlaybackItemErrorCode(2i32);
    pub const DecodeError: MediaPlaybackItemErrorCode = MediaPlaybackItemErrorCode(3i32);
    pub const SourceNotSupportedError: MediaPlaybackItemErrorCode = MediaPlaybackItemErrorCode(4i32);
    pub const EncryptionError: MediaPlaybackItemErrorCode = MediaPlaybackItemErrorCode(5i32);
}
impl ::core::convert::From<i32> for MediaPlaybackItemErrorCode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaPlaybackItemErrorCode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackItemErrorCode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackItemErrorCode;i4)");
}
impl ::windows::core::DefaultType for MediaPlaybackItemErrorCode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackItemFailedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlaybackItemFailedEventArgs {
    pub fn Item(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn Error(&self) -> ::windows::core::Result<MediaPlaybackItemError> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackItemError>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackItemFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackItemFailedEventArgs;{7703134a-e9a7-47c3-862c-c656d30683d4})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackItemFailedEventArgs {
    type Vtable = IMediaPlaybackItemFailedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7703134a_e9a7_47c3_862c_c656d30683d4);
}
impl ::windows::core::RuntimeName for MediaPlaybackItemFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItemFailedEventArgs";
}
impl ::core::convert::From<MediaPlaybackItemFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackItemFailedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackItemFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackItemFailedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackItemFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackItemFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackItemFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackItemFailedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackItemFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackItemFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackItemFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackItemFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackItemFailedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackItemFailedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackItemOpenedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlaybackItemOpenedEventArgs {
    pub fn Item(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackItemOpenedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackItemOpenedEventArgs;{cbd9bd82-3037-4fbe-ae8f-39fc39edf4ef})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackItemOpenedEventArgs {
    type Vtable = IMediaPlaybackItemOpenedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbd9bd82_3037_4fbe_ae8f_39fc39edf4ef);
}
impl ::windows::core::RuntimeName for MediaPlaybackItemOpenedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItemOpenedEventArgs";
}
impl ::core::convert::From<MediaPlaybackItemOpenedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackItemOpenedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackItemOpenedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackItemOpenedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackItemOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackItemOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackItemOpenedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackItemOpenedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackItemOpenedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackItemOpenedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackItemOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackItemOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackItemOpenedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackItemOpenedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackList(pub ::windows::core::IInspectable);
impl MediaPlaybackList {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaPlaybackList, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn ItemFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemFailedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CurrentItemChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackList, CurrentMediaPlaybackItemChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentItemChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ItemOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemOpenedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IObservableVector<MediaPlaybackItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IObservableVector<MediaPlaybackItem>>(result__)
        }
    }
    pub fn AutoRepeatEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoRepeatEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ShuffleEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetShuffleEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CurrentItem(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn CurrentItemIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MoveNext(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn MovePrevious(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn MoveTo(&self, itemindex: u32) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), itemindex, &mut result__).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MaxPrefetchTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetMaxPrefetchTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn StartingItem(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn SetStartingItem<'a, Param0: ::windows::core::IntoParam<'a, MediaPlaybackItem>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShuffledItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaPlaybackItem>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaPlaybackItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetShuffledItems<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<MediaPlaybackItem>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn MaxPlayedItemsToKeepOpen(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetMaxPlayedItemsToKeepOpen<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackList;{7f77ee9c-dc42-4e26-a98d-7850df8ec925})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackList {
    type Vtable = IMediaPlaybackList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f77ee9c_dc42_4e26_a98d_7850df8ec925);
}
impl ::windows::core::RuntimeName for MediaPlaybackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackList";
}
impl ::core::convert::From<MediaPlaybackList> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackList) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackList> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackList) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackList> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackList> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<MediaPlaybackList> for IMediaPlaybackSource {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaPlaybackList> for IMediaPlaybackSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaPlaybackSource> for MediaPlaybackList {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaPlaybackSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaPlaybackSource> for &MediaPlaybackList {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaPlaybackSource> {
        ::core::convert::TryInto::<IMediaPlaybackSource>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackList {}
unsafe impl ::core::marker::Sync for MediaPlaybackList {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackSession(pub ::windows::core::IInspectable);
impl MediaPlaybackSession {
    #[cfg(feature = "Foundation")]
    pub fn PlaybackStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PlaybackRateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackRateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SeekCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSeekCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BufferingStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBufferingStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BufferingEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBufferingEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BufferingProgressChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBufferingProgressChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DownloadProgressChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDownloadProgressChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn NaturalDurationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveNaturalDurationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PositionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn NaturalVideoSizeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveNaturalVideoSizeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn MediaPlayer(&self) -> ::windows::core::Result<MediaPlayer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlayer>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn NaturalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PlaybackState(&self) -> ::windows::core::Result<MediaPlaybackState> {
        let this = self;
        unsafe {
            let mut result__: MediaPlaybackState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackState>(result__)
        }
    }
    pub fn CanSeek(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanPause(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsProtected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn PlaybackRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetPlaybackRate(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn BufferingProgress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn DownloadProgress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn NaturalVideoHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn NaturalVideoWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn NormalizedSourceRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetNormalizedSourceRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn StereoscopicVideoPackingMode(&self) -> ::windows::core::Result<super::MediaProperties::StereoscopicVideoPackingMode> {
        let this = self;
        unsafe {
            let mut result__: super::MediaProperties::StereoscopicVideoPackingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::StereoscopicVideoPackingMode>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetStereoscopicVideoPackingMode(&self, value: super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BufferedRangesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBufferedRangesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PlayedRangesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePlayedRangesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SeekableRangesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSeekableRangesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SupportedPlaybackRatesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSupportedPlaybackRatesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn SphericalVideoProjection(&self) -> ::windows::core::Result<MediaPlaybackSphericalVideoProjection> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackSphericalVideoProjection>(result__)
        }
    }
    pub fn IsMirroring(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsMirroring(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetBufferedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetPlayedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSeekableRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>>(result__)
        }
    }
    pub fn IsSupportedPlaybackRateRange(&self, rate1: f64, rate2: f64) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), rate1, rate2, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PlaybackRotation(&self) -> ::windows::core::Result<super::MediaProperties::MediaRotation> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession3>(self)?;
        unsafe {
            let mut result__: super::MediaProperties::MediaRotation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaRotation>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetPlaybackRotation(&self, value: super::MediaProperties::MediaRotation) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GetOutputDegradationPolicyState(&self) -> ::windows::core::Result<MediaPlaybackSessionOutputDegradationPolicyState> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackSessionOutputDegradationPolicyState>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackSession;{c32b683d-0407-41ba-8946-8b345a5a5435})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackSession {
    type Vtable = IMediaPlaybackSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc32b683d_0407_41ba_8946_8b345a5a5435);
}
impl ::windows::core::RuntimeName for MediaPlaybackSession {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSession";
}
impl ::core::convert::From<MediaPlaybackSession> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackSession> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackSession> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackSession> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackSession {}
unsafe impl ::core::marker::Sync for MediaPlaybackSession {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackSessionBufferingStartedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlaybackSessionBufferingStartedEventArgs {
    pub fn IsPlaybackInterruption(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackSessionBufferingStartedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackSessionBufferingStartedEventArgs;{cd6aafed-74e2-43b5-b115-76236c33791a})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackSessionBufferingStartedEventArgs {
    type Vtable = IMediaPlaybackSessionBufferingStartedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd6aafed_74e2_43b5_b115_76236c33791a);
}
impl ::windows::core::RuntimeName for MediaPlaybackSessionBufferingStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSessionBufferingStartedEventArgs";
}
impl ::core::convert::From<MediaPlaybackSessionBufferingStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackSessionBufferingStartedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackSessionBufferingStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackSessionBufferingStartedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackSessionBufferingStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackSessionBufferingStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackSessionBufferingStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackSessionBufferingStartedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackSessionBufferingStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackSessionBufferingStartedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackSessionBufferingStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackSessionBufferingStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackSessionBufferingStartedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackSessionBufferingStartedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackSessionOutputDegradationPolicyState(pub ::windows::core::IInspectable);
impl MediaPlaybackSessionOutputDegradationPolicyState {
    pub fn VideoConstrictionReason(&self) -> ::windows::core::Result<MediaPlaybackSessionVideoConstrictionReason> {
        let this = self;
        unsafe {
            let mut result__: MediaPlaybackSessionVideoConstrictionReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackSessionVideoConstrictionReason>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackSessionOutputDegradationPolicyState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackSessionOutputDegradationPolicyState;{558e727d-f633-49f9-965a-abaa1db709be})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackSessionOutputDegradationPolicyState {
    type Vtable = IMediaPlaybackSessionOutputDegradationPolicyState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x558e727d_f633_49f9_965a_abaa1db709be);
}
impl ::windows::core::RuntimeName for MediaPlaybackSessionOutputDegradationPolicyState {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSessionOutputDegradationPolicyState";
}
impl ::core::convert::From<MediaPlaybackSessionOutputDegradationPolicyState> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackSessionOutputDegradationPolicyState) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackSessionOutputDegradationPolicyState> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackSessionOutputDegradationPolicyState) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackSessionOutputDegradationPolicyState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackSessionOutputDegradationPolicyState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackSessionOutputDegradationPolicyState> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackSessionOutputDegradationPolicyState) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackSessionOutputDegradationPolicyState> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackSessionOutputDegradationPolicyState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackSessionOutputDegradationPolicyState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackSessionOutputDegradationPolicyState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackSessionOutputDegradationPolicyState {}
unsafe impl ::core::marker::Sync for MediaPlaybackSessionOutputDegradationPolicyState {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaPlaybackSessionVideoConstrictionReason(pub i32);
impl MediaPlaybackSessionVideoConstrictionReason {
    pub const None: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(0i32);
    pub const VirtualMachine: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(1i32);
    pub const UnsupportedDisplayAdapter: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(2i32);
    pub const UnsignedDriver: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(3i32);
    pub const FrameServerEnabled: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(4i32);
    pub const OutputProtectionFailed: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(5i32);
    pub const Unknown: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(6i32);
}
impl ::core::convert::From<i32> for MediaPlaybackSessionVideoConstrictionReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaPlaybackSessionVideoConstrictionReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackSessionVideoConstrictionReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackSessionVideoConstrictionReason;i4)");
}
impl ::windows::core::DefaultType for MediaPlaybackSessionVideoConstrictionReason {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackSphericalVideoProjection(pub ::windows::core::IInspectable);
impl MediaPlaybackSphericalVideoProjection {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FrameFormat(&self) -> ::windows::core::Result<super::MediaProperties::SphericalVideoFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: super::MediaProperties::SphericalVideoFrameFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::SphericalVideoFrameFormat>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetFrameFormat(&self, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HorizontalFieldOfViewInDegrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetHorizontalFieldOfViewInDegrees(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ViewOrientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetViewOrientation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ProjectionMode(&self) -> ::windows::core::Result<SphericalVideoProjectionMode> {
        let this = self;
        unsafe {
            let mut result__: SphericalVideoProjectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SphericalVideoProjectionMode>(result__)
        }
    }
    pub fn SetProjectionMode(&self, value: SphericalVideoProjectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackSphericalVideoProjection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackSphericalVideoProjection;{d405b37c-6f0e-4661-b8ee-d487ba9752d5})");
}
unsafe impl ::windows::core::Interface for MediaPlaybackSphericalVideoProjection {
    type Vtable = IMediaPlaybackSphericalVideoProjection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd405b37c_6f0e_4661_b8ee_d487ba9752d5);
}
impl ::windows::core::RuntimeName for MediaPlaybackSphericalVideoProjection {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSphericalVideoProjection";
}
impl ::core::convert::From<MediaPlaybackSphericalVideoProjection> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackSphericalVideoProjection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlaybackSphericalVideoProjection> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackSphericalVideoProjection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackSphericalVideoProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackSphericalVideoProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlaybackSphericalVideoProjection> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackSphericalVideoProjection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlaybackSphericalVideoProjection> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackSphericalVideoProjection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackSphericalVideoProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackSphericalVideoProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackSphericalVideoProjection {}
unsafe impl ::core::marker::Sync for MediaPlaybackSphericalVideoProjection {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaPlaybackState(pub i32);
impl MediaPlaybackState {
    pub const None: MediaPlaybackState = MediaPlaybackState(0i32);
    pub const Opening: MediaPlaybackState = MediaPlaybackState(1i32);
    pub const Buffering: MediaPlaybackState = MediaPlaybackState(2i32);
    pub const Playing: MediaPlaybackState = MediaPlaybackState(3i32);
    pub const Paused: MediaPlaybackState = MediaPlaybackState(4i32);
}
impl ::core::convert::From<i32> for MediaPlaybackState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaPlaybackState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackState;i4)");
}
impl ::windows::core::DefaultType for MediaPlaybackState {
    type DefaultType = Self;
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackTimedMetadataTrackList(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl MediaPlaybackTimedMetadataTrackList {
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<super::Core::TimedMetadataTrack> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<super::Core::TimedMetadataTrack>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, super::Core::TimedMetadataTrack>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetMany(&self, startindex: u32, items: &mut [<super::Core::TimedMetadataTrack as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::Core::TimedMetadataTrack>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataTrack>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::Core::TimedMetadataTrack>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn PresentationModeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlaybackTimedMetadataTrackList, TimedMetadataPresentationModeChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePresentationModeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetPresentationMode(&self, index: u32) -> ::windows::core::Result<TimedMetadataTrackPresentationMode> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe {
            let mut result__: TimedMetadataTrackPresentationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<TimedMetadataTrackPresentationMode>(result__)
        }
    }
    pub fn SetPresentationMode(&self, index: u32, value: TimedMetadataTrackPresentationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), index, value).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for MediaPlaybackTimedMetadataTrackList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackTimedMetadataTrackList;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Media.Core.TimedMetadataTrack;{9e6aed9e-f67a-49a9-b330-cf03b0e9cf07})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for MediaPlaybackTimedMetadataTrackList {
    type Vtable = super::super::Foundation::Collections::IVectorView_abi<super::Core::TimedMetadataTrack>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for MediaPlaybackTimedMetadataTrackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackTimedMetadataTrackList";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<MediaPlaybackTimedMetadataTrackList> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackTimedMetadataTrackList) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&MediaPlaybackTimedMetadataTrackList> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackTimedMetadataTrackList) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<MediaPlaybackTimedMetadataTrackList> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackTimedMetadataTrackList) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&MediaPlaybackTimedMetadataTrackList> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackTimedMetadataTrackList) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<MediaPlaybackTimedMetadataTrackList> for super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack> {
    fn from(value: MediaPlaybackTimedMetadataTrackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&MediaPlaybackTimedMetadataTrackList> for super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack> {
    fn from(value: &MediaPlaybackTimedMetadataTrackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>> for MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>> for &MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<MediaPlaybackTimedMetadataTrackList> for super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackTimedMetadataTrackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&MediaPlaybackTimedMetadataTrackList> for super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackTimedMetadataTrackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataTrack>> for MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataTrack>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataTrack>> for &MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataTrack>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataTrack>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for MediaPlaybackTimedMetadataTrackList {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for MediaPlaybackTimedMetadataTrackList {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for MediaPlaybackTimedMetadataTrackList {
    type Item = super::Core::TimedMetadataTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &MediaPlaybackTimedMetadataTrackList {
    type Item = super::Core::TimedMetadataTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlaybackVideoTrackList(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl MediaPlaybackVideoTrackList {
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<super::Core::VideoTrack> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<super::Core::VideoTrack>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, super::Core::VideoTrack>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetMany(&self, startindex: u32, items: &mut [<super::Core::VideoTrack as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::Core::VideoTrack>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::Core::VideoTrack>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::Core::VideoTrack>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn SelectedIndexChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<super::Core::ISingleSelectMediaTrackList, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn RemoveSelectedIndexChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SelectedIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for MediaPlaybackVideoTrackList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackVideoTrackList;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Media.Core.VideoTrack;{03e1fafc-c931-491a-b46b-c10ee8c256b7})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for MediaPlaybackVideoTrackList {
    type Vtable = super::super::Foundation::Collections::IVectorView_abi<super::Core::VideoTrack>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::Foundation::Collections::IVectorView<super::Core::VideoTrack> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for MediaPlaybackVideoTrackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackVideoTrackList";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<MediaPlaybackVideoTrackList> for ::windows::core::IUnknown {
    fn from(value: MediaPlaybackVideoTrackList) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&MediaPlaybackVideoTrackList> for ::windows::core::IUnknown {
    fn from(value: &MediaPlaybackVideoTrackList) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<MediaPlaybackVideoTrackList> for ::windows::core::IInspectable {
    fn from(value: MediaPlaybackVideoTrackList) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&MediaPlaybackVideoTrackList> for ::windows::core::IInspectable {
    fn from(value: &MediaPlaybackVideoTrackList) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<MediaPlaybackVideoTrackList> for super::super::Foundation::Collections::IVectorView<super::Core::VideoTrack> {
    fn from(value: MediaPlaybackVideoTrackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&MediaPlaybackVideoTrackList> for super::super::Foundation::Collections::IVectorView<super::Core::VideoTrack> {
    fn from(value: &MediaPlaybackVideoTrackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::Core::VideoTrack>> for MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<super::Core::VideoTrack>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::Core::VideoTrack>> for &MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<super::Core::VideoTrack>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<MediaPlaybackVideoTrackList> for super::super::Foundation::Collections::IIterable<super::Core::VideoTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackVideoTrackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&MediaPlaybackVideoTrackList> for super::super::Foundation::Collections::IIterable<super::Core::VideoTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackVideoTrackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::Core::VideoTrack>> for MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::Core::VideoTrack>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::Core::VideoTrack>> for &MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::Core::VideoTrack>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::Core::VideoTrack>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<MediaPlaybackVideoTrackList> for super::Core::ISingleSelectMediaTrackList {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackVideoTrackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackVideoTrackList> for super::Core::ISingleSelectMediaTrackList {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackVideoTrackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl<'a> ::windows::core::IntoParam<'a, super::Core::ISingleSelectMediaTrackList> for MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::Core::ISingleSelectMediaTrackList> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl<'a> ::windows::core::IntoParam<'a, super::Core::ISingleSelectMediaTrackList> for &MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, super::Core::ISingleSelectMediaTrackList> {
        ::core::convert::TryInto::<super::Core::ISingleSelectMediaTrackList>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for MediaPlaybackVideoTrackList {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for MediaPlaybackVideoTrackList {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for MediaPlaybackVideoTrackList {
    type Item = super::Core::VideoTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &MediaPlaybackVideoTrackList {
    type Item = super::Core::VideoTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlayer(pub ::windows::core::IInspectable);
impl MediaPlayer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaPlayer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AutoPlay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoPlay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn NaturalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn BufferingProgress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CurrentState(&self) -> ::windows::core::Result<MediaPlayerState> {
        let this = self;
        unsafe {
            let mut result__: MediaPlayerState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlayerState>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CanSeek(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CanPause(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsLoopingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsLoopingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsProtected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsMuted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsMuted(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn PlaybackRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetPlaybackRate(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Volume(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetVolume(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn PlaybackMediaMarkers(&self) -> ::windows::core::Result<PlaybackMediaMarkerSequence> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlaybackMediaMarkerSequence>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MediaOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn MediaEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn MediaFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerFailedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn CurrentStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackMediaMarkerReached<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, PlaybackMediaMarkerReachedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackMediaMarkerReached<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn MediaPlayerRateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerRateChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaPlayerRateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn VolumeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVolumeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SeekCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSeekCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn BufferingStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBufferingStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn BufferingEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBufferingEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Play(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SetUriSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SystemMediaTransportControls(&self) -> ::windows::core::Result<super::SystemMediaTransportControls> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SystemMediaTransportControls>(result__)
        }
    }
    pub fn AudioCategory(&self) -> ::windows::core::Result<MediaPlayerAudioCategory> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe {
            let mut result__: MediaPlayerAudioCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlayerAudioCategory>(result__)
        }
    }
    pub fn SetAudioCategory(&self, value: MediaPlayerAudioCategory) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AudioDeviceType(&self) -> ::windows::core::Result<MediaPlayerAudioDeviceType> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe {
            let mut result__: MediaPlayerAudioDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlayerAudioDeviceType>(result__)
        }
    }
    pub fn SetAudioDeviceType(&self, value: MediaPlayerAudioDeviceType) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_Protection")]
    pub fn ProtectionManager(&self) -> ::windows::core::Result<super::Protection::MediaProtectionManager> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Protection::MediaProtectionManager>(result__)
        }
    }
    #[cfg(feature = "Media_Protection")]
    pub fn SetProtectionManager<'a, Param0: ::windows::core::IntoParam<'a, super::Protection::MediaProtectionManager>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    pub fn SetFileSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), file.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStreamSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, stream: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), stream.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media_Core")]
    pub fn SetMediaSource<'a, Param0: ::windows::core::IntoParam<'a, super::Core::IMediaSource>>(&self, source: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), source.into_param().abi()).ok() }
    }
    pub fn Source(&self) -> ::windows::core::Result<IMediaPlaybackSource> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMediaPlaybackSource>(result__)
        }
    }
    pub fn SetSource<'a, Param0: ::windows::core::IntoParam<'a, IMediaPlaybackSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddAudioEffect<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, effectoptional: bool, configuration: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerEffects>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), effectoptional, configuration.into_param().abi()).ok() }
    }
    pub fn RemoveAllEffects(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerEffects>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsMutedChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsMutedChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SourceChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn AudioBalance(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetAudioBalance(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RealTimePlayback(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRealTimePlayback(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn StereoscopicVideoRenderMode(&self) -> ::windows::core::Result<StereoscopicVideoRenderMode> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__: StereoscopicVideoRenderMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StereoscopicVideoRenderMode>(result__)
        }
    }
    pub fn SetStereoscopicVideoRenderMode(&self, value: StereoscopicVideoRenderMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn BreakManager(&self) -> ::windows::core::Result<MediaBreakManager> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaBreakManager>(result__)
        }
    }
    pub fn CommandManager(&self) -> ::windows::core::Result<MediaPlaybackCommandManager> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackCommandManager>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn AudioDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn SetAudioDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TimelineController(&self) -> ::windows::core::Result<super::MediaTimelineController> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaTimelineController>(result__)
        }
    }
    pub fn SetTimelineController<'a, Param0: ::windows::core::IntoParam<'a, super::MediaTimelineController>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TimelineControllerPositionOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTimelineControllerPositionOffset<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PlaybackSession(&self) -> ::windows::core::Result<MediaPlaybackSession> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackSession>(result__)
        }
    }
    pub fn StepForwardOneFrame(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn StepBackwardOneFrame(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Media_Casting")]
    pub fn GetAsCastingSource(&self) -> ::windows::core::Result<super::Casting::CastingSource> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Casting::CastingSource>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSurfaceSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, size: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), size.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn GetSurface<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Composition::Compositor>>(&self, compositor: Param0) -> ::windows::core::Result<MediaPlayerSurface> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<MediaPlayerSurface>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddVideoEffect<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, effectoptional: bool, effectconfiguration: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerEffects2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), effectoptional, effectconfiguration.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn VideoFrameAvailable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoFrameAvailable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn IsVideoFrameServerEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsVideoFrameServerEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CopyFrameToVideoSurface<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CopyFrameToVideoSurfaceWithTargetRectangle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, destination: Param0, targetrectangle: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), destination.into_param().abi(), targetrectangle.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CopyFrameToStereoscopicVideoSurfaces<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>>(&self, destinationlefteye: Param0, destinationrighteye: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), destinationlefteye.into_param().abi(), destinationrighteye.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SubtitleFrameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSubtitleFrameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn RenderSubtitlesToSurface<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>>(&self, destination: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), destination.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn RenderSubtitlesToSurfaceWithTargetRectangle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, destination: Param0, targetrectangle: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), destination.into_param().abi(), targetrectangle.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Media_Audio")]
    pub fn AudioStateMonitor(&self) -> ::windows::core::Result<super::Audio::AudioStateMonitor> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer7>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Audio::AudioStateMonitor>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlayer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayer;{381a83cb-6fff-499b-8d64-2885dfc1249e})");
}
unsafe impl ::windows::core::Interface for MediaPlayer {
    type Vtable = IMediaPlayer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x381a83cb_6fff_499b_8d64_2885dfc1249e);
}
impl ::windows::core::RuntimeName for MediaPlayer {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayer";
}
impl ::core::convert::From<MediaPlayer> for ::windows::core::IUnknown {
    fn from(value: MediaPlayer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlayer> for ::windows::core::IUnknown {
    fn from(value: &MediaPlayer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlayer> for ::windows::core::IInspectable {
    fn from(value: MediaPlayer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlayer> for ::windows::core::IInspectable {
    fn from(value: &MediaPlayer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaPlayer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlayer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaPlayer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlayer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MediaPlayer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MediaPlayer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaPlayer {}
unsafe impl ::core::marker::Sync for MediaPlayer {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaPlayerAudioCategory(pub i32);
impl MediaPlayerAudioCategory {
    pub const Other: MediaPlayerAudioCategory = MediaPlayerAudioCategory(0i32);
    pub const Communications: MediaPlayerAudioCategory = MediaPlayerAudioCategory(3i32);
    pub const Alerts: MediaPlayerAudioCategory = MediaPlayerAudioCategory(4i32);
    pub const SoundEffects: MediaPlayerAudioCategory = MediaPlayerAudioCategory(5i32);
    pub const GameEffects: MediaPlayerAudioCategory = MediaPlayerAudioCategory(6i32);
    pub const GameMedia: MediaPlayerAudioCategory = MediaPlayerAudioCategory(7i32);
    pub const GameChat: MediaPlayerAudioCategory = MediaPlayerAudioCategory(8i32);
    pub const Speech: MediaPlayerAudioCategory = MediaPlayerAudioCategory(9i32);
    pub const Movie: MediaPlayerAudioCategory = MediaPlayerAudioCategory(10i32);
    pub const Media: MediaPlayerAudioCategory = MediaPlayerAudioCategory(11i32);
}
impl ::core::convert::From<i32> for MediaPlayerAudioCategory {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaPlayerAudioCategory {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerAudioCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerAudioCategory;i4)");
}
impl ::windows::core::DefaultType for MediaPlayerAudioCategory {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaPlayerAudioDeviceType(pub i32);
impl MediaPlayerAudioDeviceType {
    pub const Console: MediaPlayerAudioDeviceType = MediaPlayerAudioDeviceType(0i32);
    pub const Multimedia: MediaPlayerAudioDeviceType = MediaPlayerAudioDeviceType(1i32);
    pub const Communications: MediaPlayerAudioDeviceType = MediaPlayerAudioDeviceType(2i32);
}
impl ::core::convert::From<i32> for MediaPlayerAudioDeviceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaPlayerAudioDeviceType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerAudioDeviceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerAudioDeviceType;i4)");
}
impl ::windows::core::DefaultType for MediaPlayerAudioDeviceType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlayerDataReceivedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlayerDataReceivedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayerDataReceivedEventArgs;{c75a9405-c801-412a-835b-83fc0e622a8e})");
}
unsafe impl ::windows::core::Interface for MediaPlayerDataReceivedEventArgs {
    type Vtable = IMediaPlayerDataReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc75a9405_c801_412a_835b_83fc0e622a8e);
}
impl ::windows::core::RuntimeName for MediaPlayerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerDataReceivedEventArgs";
}
impl ::core::convert::From<MediaPlayerDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlayerDataReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlayerDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlayerDataReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlayerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlayerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlayerDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlayerDataReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlayerDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlayerDataReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlayerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlayerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlayerDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlayerDataReceivedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaPlayerError(pub i32);
impl MediaPlayerError {
    pub const Unknown: MediaPlayerError = MediaPlayerError(0i32);
    pub const Aborted: MediaPlayerError = MediaPlayerError(1i32);
    pub const NetworkError: MediaPlayerError = MediaPlayerError(2i32);
    pub const DecodingError: MediaPlayerError = MediaPlayerError(3i32);
    pub const SourceNotSupported: MediaPlayerError = MediaPlayerError(4i32);
}
impl ::core::convert::From<i32> for MediaPlayerError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaPlayerError {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerError;i4)");
}
impl ::windows::core::DefaultType for MediaPlayerError {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlayerFailedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlayerFailedEventArgs {
    pub fn Error(&self) -> ::windows::core::Result<MediaPlayerError> {
        let this = self;
        unsafe {
            let mut result__: MediaPlayerError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlayerError>(result__)
        }
    }
    pub fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn ErrorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayerFailedEventArgs;{2744e9b9-a7e3-4f16-bac4-7914ebc08301})");
}
unsafe impl ::windows::core::Interface for MediaPlayerFailedEventArgs {
    type Vtable = IMediaPlayerFailedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2744e9b9_a7e3_4f16_bac4_7914ebc08301);
}
impl ::windows::core::RuntimeName for MediaPlayerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerFailedEventArgs";
}
impl ::core::convert::From<MediaPlayerFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlayerFailedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlayerFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlayerFailedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlayerFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlayerFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlayerFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlayerFailedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlayerFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlayerFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlayerFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlayerFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlayerFailedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlayerFailedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlayerRateChangedEventArgs(pub ::windows::core::IInspectable);
impl MediaPlayerRateChangedEventArgs {
    pub fn NewRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerRateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayerRateChangedEventArgs;{40600d58-3b61-4bb2-989f-fc65608b6cab})");
}
unsafe impl ::windows::core::Interface for MediaPlayerRateChangedEventArgs {
    type Vtable = IMediaPlayerRateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40600d58_3b61_4bb2_989f_fc65608b6cab);
}
impl ::windows::core::RuntimeName for MediaPlayerRateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerRateChangedEventArgs";
}
impl ::core::convert::From<MediaPlayerRateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPlayerRateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlayerRateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPlayerRateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlayerRateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlayerRateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlayerRateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPlayerRateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlayerRateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPlayerRateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlayerRateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlayerRateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaPlayerRateChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlayerRateChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaPlayerState(pub i32);
impl MediaPlayerState {
    pub const Closed: MediaPlayerState = MediaPlayerState(0i32);
    pub const Opening: MediaPlayerState = MediaPlayerState(1i32);
    pub const Buffering: MediaPlayerState = MediaPlayerState(2i32);
    pub const Playing: MediaPlayerState = MediaPlayerState(3i32);
    pub const Paused: MediaPlayerState = MediaPlayerState(4i32);
    pub const Stopped: MediaPlayerState = MediaPlayerState(5i32);
}
impl ::core::convert::From<i32> for MediaPlayerState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaPlayerState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerState;i4)");
}
impl ::windows::core::DefaultType for MediaPlayerState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaPlayerSurface(pub ::windows::core::IInspectable);
impl MediaPlayerSurface {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn CompositionSurface(&self) -> ::windows::core::Result<super::super::UI::Composition::ICompositionSurface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Composition::ICompositionSurface>(result__)
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::super::UI::Composition::Compositor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Composition::Compositor>(result__)
        }
    }
    pub fn MediaPlayer(&self) -> ::windows::core::Result<MediaPlayer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlayer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerSurface {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayerSurface;{0ed653bc-b736-49c3-830b-764a3845313a})");
}
unsafe impl ::windows::core::Interface for MediaPlayerSurface {
    type Vtable = IMediaPlayerSurface_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ed653bc_b736_49c3_830b_764a3845313a);
}
impl ::windows::core::RuntimeName for MediaPlayerSurface {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerSurface";
}
impl ::core::convert::From<MediaPlayerSurface> for ::windows::core::IUnknown {
    fn from(value: MediaPlayerSurface) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaPlayerSurface> for ::windows::core::IUnknown {
    fn from(value: &MediaPlayerSurface) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPlayerSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPlayerSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaPlayerSurface> for ::windows::core::IInspectable {
    fn from(value: MediaPlayerSurface) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaPlayerSurface> for ::windows::core::IInspectable {
    fn from(value: &MediaPlayerSurface) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPlayerSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPlayerSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaPlayerSurface> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlayerSurface) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaPlayerSurface> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlayerSurface) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MediaPlayerSurface {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MediaPlayerSurface {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaPlayerSurface {}
unsafe impl ::core::marker::Sync for MediaPlayerSurface {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlaybackMediaMarker(pub ::windows::core::IInspectable);
impl PlaybackMediaMarker {
    #[cfg(feature = "Foundation")]
    pub fn Time(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn MediaMarkerType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateFromTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(value: Param0) -> ::windows::core::Result<PlaybackMediaMarker> {
        Self::IPlaybackMediaMarkerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<PlaybackMediaMarker>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0, mediamarkettype: Param1, text: Param2) -> ::windows::core::Result<PlaybackMediaMarker> {
        Self::IPlaybackMediaMarkerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi(), mediamarkettype.into_param().abi(), text.into_param().abi(), &mut result__).from_abi::<PlaybackMediaMarker>(result__)
        })
    }
    pub fn IPlaybackMediaMarkerFactory<R, F: FnOnce(&IPlaybackMediaMarkerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlaybackMediaMarker, IPlaybackMediaMarkerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PlaybackMediaMarker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.PlaybackMediaMarker;{c4d22f5c-3c1c-4444-b6b9-778b0422d41a})");
}
unsafe impl ::windows::core::Interface for PlaybackMediaMarker {
    type Vtable = IPlaybackMediaMarker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4d22f5c_3c1c_4444_b6b9_778b0422d41a);
}
impl ::windows::core::RuntimeName for PlaybackMediaMarker {
    const NAME: &'static str = "Windows.Media.Playback.PlaybackMediaMarker";
}
impl ::core::convert::From<PlaybackMediaMarker> for ::windows::core::IUnknown {
    fn from(value: PlaybackMediaMarker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlaybackMediaMarker> for ::windows::core::IUnknown {
    fn from(value: &PlaybackMediaMarker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlaybackMediaMarker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlaybackMediaMarker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlaybackMediaMarker> for ::windows::core::IInspectable {
    fn from(value: PlaybackMediaMarker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlaybackMediaMarker> for ::windows::core::IInspectable {
    fn from(value: &PlaybackMediaMarker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlaybackMediaMarker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlaybackMediaMarker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PlaybackMediaMarker {}
unsafe impl ::core::marker::Sync for PlaybackMediaMarker {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlaybackMediaMarkerReachedEventArgs(pub ::windows::core::IInspectable);
impl PlaybackMediaMarkerReachedEventArgs {
    pub fn PlaybackMediaMarker(&self) -> ::windows::core::Result<PlaybackMediaMarker> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlaybackMediaMarker>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlaybackMediaMarkerReachedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.PlaybackMediaMarkerReachedEventArgs;{578cd1b9-90e2-4e60-abc4-8740b01f6196})");
}
unsafe impl ::windows::core::Interface for PlaybackMediaMarkerReachedEventArgs {
    type Vtable = IPlaybackMediaMarkerReachedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x578cd1b9_90e2_4e60_abc4_8740b01f6196);
}
impl ::windows::core::RuntimeName for PlaybackMediaMarkerReachedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.PlaybackMediaMarkerReachedEventArgs";
}
impl ::core::convert::From<PlaybackMediaMarkerReachedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PlaybackMediaMarkerReachedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlaybackMediaMarkerReachedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PlaybackMediaMarkerReachedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlaybackMediaMarkerReachedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlaybackMediaMarkerReachedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlaybackMediaMarkerReachedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PlaybackMediaMarkerReachedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlaybackMediaMarkerReachedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PlaybackMediaMarkerReachedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlaybackMediaMarkerReachedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlaybackMediaMarkerReachedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PlaybackMediaMarkerReachedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackMediaMarkerReachedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlaybackMediaMarkerSequence(pub ::windows::core::IInspectable);
impl PlaybackMediaMarkerSequence {
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, PlaybackMediaMarker>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<PlaybackMediaMarker>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<PlaybackMediaMarker>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<PlaybackMediaMarker>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlaybackMediaMarkerSequence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.PlaybackMediaMarkerSequence;{f2810cee-638b-46cf-8817-1d111fe9d8c4})");
}
unsafe impl ::windows::core::Interface for PlaybackMediaMarkerSequence {
    type Vtable = IPlaybackMediaMarkerSequence_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2810cee_638b_46cf_8817_1d111fe9d8c4);
}
impl ::windows::core::RuntimeName for PlaybackMediaMarkerSequence {
    const NAME: &'static str = "Windows.Media.Playback.PlaybackMediaMarkerSequence";
}
impl ::core::convert::From<PlaybackMediaMarkerSequence> for ::windows::core::IUnknown {
    fn from(value: PlaybackMediaMarkerSequence) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlaybackMediaMarkerSequence> for ::windows::core::IUnknown {
    fn from(value: &PlaybackMediaMarkerSequence) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlaybackMediaMarkerSequence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlaybackMediaMarkerSequence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlaybackMediaMarkerSequence> for ::windows::core::IInspectable {
    fn from(value: PlaybackMediaMarkerSequence) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlaybackMediaMarkerSequence> for ::windows::core::IInspectable {
    fn from(value: &PlaybackMediaMarkerSequence) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlaybackMediaMarkerSequence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlaybackMediaMarkerSequence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PlaybackMediaMarkerSequence> for super::super::Foundation::Collections::IIterable<PlaybackMediaMarker> {
    type Error = ::windows::core::Error;
    fn try_from(value: PlaybackMediaMarkerSequence) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlaybackMediaMarkerSequence> for super::super::Foundation::Collections::IIterable<PlaybackMediaMarker> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlaybackMediaMarkerSequence) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<PlaybackMediaMarker>> for PlaybackMediaMarkerSequence {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<PlaybackMediaMarker>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<PlaybackMediaMarker>> for &PlaybackMediaMarkerSequence {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<PlaybackMediaMarker>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<PlaybackMediaMarker>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PlaybackMediaMarkerSequence {}
unsafe impl ::core::marker::Sync for PlaybackMediaMarkerSequence {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for PlaybackMediaMarkerSequence {
    type Item = PlaybackMediaMarker;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &PlaybackMediaMarkerSequence {
    type Item = PlaybackMediaMarker;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SphericalVideoProjectionMode(pub i32);
impl SphericalVideoProjectionMode {
    pub const Spherical: SphericalVideoProjectionMode = SphericalVideoProjectionMode(0i32);
    pub const Flat: SphericalVideoProjectionMode = SphericalVideoProjectionMode(1i32);
}
impl ::core::convert::From<i32> for SphericalVideoProjectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SphericalVideoProjectionMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SphericalVideoProjectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.SphericalVideoProjectionMode;i4)");
}
impl ::windows::core::DefaultType for SphericalVideoProjectionMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StereoscopicVideoRenderMode(pub i32);
impl StereoscopicVideoRenderMode {
    pub const Mono: StereoscopicVideoRenderMode = StereoscopicVideoRenderMode(0i32);
    pub const Stereo: StereoscopicVideoRenderMode = StereoscopicVideoRenderMode(1i32);
}
impl ::core::convert::From<i32> for StereoscopicVideoRenderMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StereoscopicVideoRenderMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StereoscopicVideoRenderMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.StereoscopicVideoRenderMode;i4)");
}
impl ::windows::core::DefaultType for StereoscopicVideoRenderMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TimedMetadataPresentationModeChangedEventArgs(pub ::windows::core::IInspectable);
impl TimedMetadataPresentationModeChangedEventArgs {
    #[cfg(feature = "Media_Core")]
    pub fn Track(&self) -> ::windows::core::Result<super::Core::TimedMetadataTrack> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::TimedMetadataTrack>(result__)
        }
    }
    pub fn OldPresentationMode(&self) -> ::windows::core::Result<TimedMetadataTrackPresentationMode> {
        let this = self;
        unsafe {
            let mut result__: TimedMetadataTrackPresentationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedMetadataTrackPresentationMode>(result__)
        }
    }
    pub fn NewPresentationMode(&self) -> ::windows::core::Result<TimedMetadataTrackPresentationMode> {
        let this = self;
        unsafe {
            let mut result__: TimedMetadataTrackPresentationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedMetadataTrackPresentationMode>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TimedMetadataPresentationModeChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.TimedMetadataPresentationModeChangedEventArgs;{d1636099-65df-45ae-8cef-dc0b53fdc2bb})");
}
unsafe impl ::windows::core::Interface for TimedMetadataPresentationModeChangedEventArgs {
    type Vtable = ITimedMetadataPresentationModeChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1636099_65df_45ae_8cef_dc0b53fdc2bb);
}
impl ::windows::core::RuntimeName for TimedMetadataPresentationModeChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.TimedMetadataPresentationModeChangedEventArgs";
}
impl ::core::convert::From<TimedMetadataPresentationModeChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TimedMetadataPresentationModeChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TimedMetadataPresentationModeChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TimedMetadataPresentationModeChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedMetadataPresentationModeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TimedMetadataPresentationModeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TimedMetadataPresentationModeChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TimedMetadataPresentationModeChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TimedMetadataPresentationModeChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TimedMetadataPresentationModeChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedMetadataPresentationModeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TimedMetadataPresentationModeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TimedMetadataPresentationModeChangedEventArgs {}
unsafe impl ::core::marker::Sync for TimedMetadataPresentationModeChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TimedMetadataTrackPresentationMode(pub i32);
impl TimedMetadataTrackPresentationMode {
    pub const Disabled: TimedMetadataTrackPresentationMode = TimedMetadataTrackPresentationMode(0i32);
    pub const Hidden: TimedMetadataTrackPresentationMode = TimedMetadataTrackPresentationMode(1i32);
    pub const ApplicationPresented: TimedMetadataTrackPresentationMode = TimedMetadataTrackPresentationMode(2i32);
    pub const PlatformPresented: TimedMetadataTrackPresentationMode = TimedMetadataTrackPresentationMode(3i32);
}
impl ::core::convert::From<i32> for TimedMetadataTrackPresentationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TimedMetadataTrackPresentationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TimedMetadataTrackPresentationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.TimedMetadataTrackPresentationMode;i4)");
}
impl ::windows::core::DefaultType for TimedMetadataTrackPresentationMode {
    type DefaultType = Self;
}
