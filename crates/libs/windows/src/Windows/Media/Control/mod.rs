#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Media_Control'*"]
#[repr(transparent)]
pub struct CurrentSessionChangedEventArgs(::windows::core::IUnknown);
impl CurrentSessionChangedEventArgs {}
impl ::core::clone::Clone for CurrentSessionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CurrentSessionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrentSessionChangedEventArgs {}
impl ::core::fmt::Debug for CurrentSessionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrentSessionChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CurrentSessionChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.CurrentSessionChangedEventArgs;{6969cb39-0bfa-5fe0-8d73-09cc5e5408e1})");
}
unsafe impl ::windows::core::Interface for CurrentSessionChangedEventArgs {
    type Vtable = ICurrentSessionChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6969cb39_0bfa_5fe0_8d73_09cc5e5408e1);
}
impl ::windows::core::RuntimeName for CurrentSessionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.CurrentSessionChangedEventArgs";
}
impl ::core::convert::From<CurrentSessionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CurrentSessionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrentSessionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CurrentSessionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CurrentSessionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CurrentSessionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CurrentSessionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CurrentSessionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrentSessionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CurrentSessionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CurrentSessionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CurrentSessionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CurrentSessionChangedEventArgs {}
unsafe impl ::core::marker::Sync for CurrentSessionChangedEventArgs {}
#[doc = "*Required features: 'Media_Control'*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSession(::windows::core::IUnknown);
impl GlobalSystemMediaTransportControlsSession {
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn SourceAppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetMediaPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionMediaProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionMediaProperties>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn GetTimelineProperties(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSessionTimelineProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GlobalSystemMediaTransportControlsSessionTimelineProperties>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn GetPlaybackInfo(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSessionPlaybackInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GlobalSystemMediaTransportControlsSessionPlaybackInfo>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryPlayAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryPauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryStopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryRecordAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryFastForwardAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryRewindAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySkipNextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySkipPreviousAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangeChannelUpAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangeChannelDownAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryTogglePlayPauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangeAutoRepeatModeAsync(&self, requestedautorepeatmode: super::MediaPlaybackAutoRepeatMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), requestedautorepeatmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangePlaybackRateAsync(&self, requestedplaybackrate: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), requestedplaybackrate, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangeShuffleActiveAsync(&self, requestedshufflestate: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), requestedshufflestate, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangePlaybackPositionAsync(&self, requestedplaybackposition: i64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), requestedplaybackposition, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TimelinePropertiesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, TimelinePropertiesChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTimelinePropertiesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackInfoChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, PlaybackInfoChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackInfoChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MediaPropertiesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, MediaPropertiesChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaPropertiesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSession {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSession;{7148c835-9b14-5ae2-ab85-dc9b1c14e1a8})");
}
unsafe impl ::windows::core::Interface for GlobalSystemMediaTransportControlsSession {
    type Vtable = IGlobalSystemMediaTransportControlsSessionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7148c835_9b14_5ae2_ab85_dc9b1c14e1a8);
}
impl ::windows::core::RuntimeName for GlobalSystemMediaTransportControlsSession {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSession";
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSession> for ::windows::core::IUnknown {
    fn from(value: GlobalSystemMediaTransportControlsSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSession> for ::windows::core::IUnknown {
    fn from(value: &GlobalSystemMediaTransportControlsSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GlobalSystemMediaTransportControlsSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GlobalSystemMediaTransportControlsSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSession> for ::windows::core::IInspectable {
    fn from(value: GlobalSystemMediaTransportControlsSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSession> for ::windows::core::IInspectable {
    fn from(value: &GlobalSystemMediaTransportControlsSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GlobalSystemMediaTransportControlsSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GlobalSystemMediaTransportControlsSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSession {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSession {}
#[doc = "*Required features: 'Media_Control'*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionManager(::windows::core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionManager {
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn GetCurrentSession(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GlobalSystemMediaTransportControlsSession>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSessions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<GlobalSystemMediaTransportControlsSession>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<GlobalSystemMediaTransportControlsSession>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CurrentSessionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, CurrentSessionChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentSessionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SessionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, SessionsChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSessionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionManager>> {
        Self::IGlobalSystemMediaTransportControlsSessionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionManager>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGlobalSystemMediaTransportControlsSessionManagerStatics<R, F: FnOnce(&IGlobalSystemMediaTransportControlsSessionManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GlobalSystemMediaTransportControlsSessionManager, IGlobalSystemMediaTransportControlsSessionManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionManager {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSessionManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager;{cace8eac-e86e-504a-ab31-5ff8ff1bce49})");
}
unsafe impl ::windows::core::Interface for GlobalSystemMediaTransportControlsSessionManager {
    type Vtable = IGlobalSystemMediaTransportControlsSessionManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcace8eac_e86e_504a_ab31_5ff8ff1bce49);
}
impl ::windows::core::RuntimeName for GlobalSystemMediaTransportControlsSessionManager {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager";
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionManager> for ::windows::core::IUnknown {
    fn from(value: GlobalSystemMediaTransportControlsSessionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionManager> for ::windows::core::IUnknown {
    fn from(value: &GlobalSystemMediaTransportControlsSessionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GlobalSystemMediaTransportControlsSessionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GlobalSystemMediaTransportControlsSessionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionManager> for ::windows::core::IInspectable {
    fn from(value: GlobalSystemMediaTransportControlsSessionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionManager> for ::windows::core::IInspectable {
    fn from(value: &GlobalSystemMediaTransportControlsSessionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GlobalSystemMediaTransportControlsSessionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GlobalSystemMediaTransportControlsSessionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionManager {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionManager {}
#[doc = "*Required features: 'Media_Control'*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionMediaProperties(::windows::core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionMediaProperties {
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn AlbumArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn AlbumTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn TrackNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genres(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn AlbumTrackCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackType(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::MediaPlaybackType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::MediaPlaybackType>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionMediaProperties {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionMediaProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties;{68856cf6-adb4-54b2-ac16-05837907acb6})");
}
unsafe impl ::windows::core::Interface for GlobalSystemMediaTransportControlsSessionMediaProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionMediaPropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68856cf6_adb4_54b2_ac16_05837907acb6);
}
impl ::windows::core::RuntimeName for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties";
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionMediaProperties> for ::windows::core::IUnknown {
    fn from(value: GlobalSystemMediaTransportControlsSessionMediaProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionMediaProperties> for ::windows::core::IUnknown {
    fn from(value: &GlobalSystemMediaTransportControlsSessionMediaProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionMediaProperties> for ::windows::core::IInspectable {
    fn from(value: GlobalSystemMediaTransportControlsSessionMediaProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionMediaProperties> for ::windows::core::IInspectable {
    fn from(value: &GlobalSystemMediaTransportControlsSessionMediaProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionMediaProperties {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionMediaProperties {}
#[doc = "*Required features: 'Media_Control'*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackControls(::windows::core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionPlaybackControls {
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsPlayEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsPauseEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsStopEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsRecordEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsFastForwardEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsRewindEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsNextEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsPreviousEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsChannelUpEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsChannelDownEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsPlayPauseToggleEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsShuffleEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsRepeatEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsPlaybackRateEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn IsPlaybackPositionEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionPlaybackControls").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackControls;{6501a3e6-bc7a-503a-bb1b-68f158f3fb03})");
}
unsafe impl ::windows::core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackControlsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6501a3e6_bc7a_503a_bb1b_68f158f3fb03);
}
impl ::windows::core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackControls";
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionPlaybackControls> for ::windows::core::IUnknown {
    fn from(value: GlobalSystemMediaTransportControlsSessionPlaybackControls) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionPlaybackControls> for ::windows::core::IUnknown {
    fn from(value: &GlobalSystemMediaTransportControlsSessionPlaybackControls) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionPlaybackControls> for ::windows::core::IInspectable {
    fn from(value: GlobalSystemMediaTransportControlsSessionPlaybackControls) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionPlaybackControls> for ::windows::core::IInspectable {
    fn from(value: &GlobalSystemMediaTransportControlsSessionPlaybackControls) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
#[doc = "*Required features: 'Media_Control'*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackInfo(::windows::core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn Controls(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSessionPlaybackControls> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GlobalSystemMediaTransportControlsSessionPlaybackControls>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control'*"]
    pub fn PlaybackStatus(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSessionPlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__: GlobalSystemMediaTransportControlsSessionPlaybackStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GlobalSystemMediaTransportControlsSessionPlaybackStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackType(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::MediaPlaybackType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::MediaPlaybackType>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AutoRepeatMode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::MediaPlaybackAutoRepeatMode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::MediaPlaybackAutoRepeatMode>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackRate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn IsShuffleActive(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionPlaybackInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackInfo;{94b4b6cf-e8ba-51ad-87a7-c10ade106127})");
}
unsafe impl ::windows::core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94b4b6cf_e8ba_51ad_87a7_c10ade106127);
}
impl ::windows::core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackInfo";
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionPlaybackInfo> for ::windows::core::IUnknown {
    fn from(value: GlobalSystemMediaTransportControlsSessionPlaybackInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionPlaybackInfo> for ::windows::core::IUnknown {
    fn from(value: &GlobalSystemMediaTransportControlsSessionPlaybackInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionPlaybackInfo> for ::windows::core::IInspectable {
    fn from(value: GlobalSystemMediaTransportControlsSessionPlaybackInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionPlaybackInfo> for ::windows::core::IInspectable {
    fn from(value: &GlobalSystemMediaTransportControlsSessionPlaybackInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
#[doc = "*Required features: 'Media_Control'*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackStatus(pub i32);
impl GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
    pub const Changing: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Playing: Self = Self(4i32);
    pub const Paused: Self = Self(5i32);
}
impl ::core::marker::Copy for GlobalSystemMediaTransportControlsSessionPlaybackStatus {}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionPlaybackStatus {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionPlaybackStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackStatus;i4)");
}
impl ::windows::core::DefaultType for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Control'*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionTimelineProperties(::windows::core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionTimelineProperties {
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MinSeekTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSeekTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Control', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn LastUpdatedTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionTimelineProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionTimelineProperties;{ede34136-6f25-588d-8ecf-ea5b6735aaa5})");
}
unsafe impl ::windows::core::Interface for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionTimelinePropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xede34136_6f25_588d_8ecf_ea5b6735aaa5);
}
impl ::windows::core::RuntimeName for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionTimelineProperties";
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionTimelineProperties> for ::windows::core::IUnknown {
    fn from(value: GlobalSystemMediaTransportControlsSessionTimelineProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionTimelineProperties> for ::windows::core::IUnknown {
    fn from(value: &GlobalSystemMediaTransportControlsSessionTimelineProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionTimelineProperties> for ::windows::core::IInspectable {
    fn from(value: GlobalSystemMediaTransportControlsSessionTimelineProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionTimelineProperties> for ::windows::core::IInspectable {
    fn from(value: &GlobalSystemMediaTransportControlsSessionTimelineProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentSessionChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICurrentSessionChangedEventArgs {
    type Vtable = ICurrentSessionChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6969cb39_0bfa_5fe0_8d73_09cc5e5408e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentSessionChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSession {
    type Vtable = IGlobalSystemMediaTransportControlsSessionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7148c835_9b14_5ae2_ab85_dc9b1c14e1a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedautorepeatmode: super::MediaPlaybackAutoRepeatMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedplaybackrate: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedshufflestate: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedplaybackposition: i64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSessionManager {
    type Vtable = IGlobalSystemMediaTransportControlsSessionManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcace8eac_e86e_504a_ab31_5ff8ff1bce49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSessionManagerStatics {
    type Vtable = IGlobalSystemMediaTransportControlsSessionManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2050c4ee_11a0_57de_aed7_c97c70338245);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSessionMediaProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionMediaPropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68856cf6_adb4_54b2_ac16_05837907acb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionMediaPropertiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSessionPlaybackControls {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackControlsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6501a3e6_bc7a_503a_bb1b_68f158f3fb03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControlsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSessionPlaybackInfo {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94b4b6cf_e8ba_51ad_87a7_c10ade106127);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GlobalSystemMediaTransportControlsSessionPlaybackStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSessionTimelineProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionTimelinePropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xede34136_6f25_588d_8ecf_ea5b6735aaa5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionTimelinePropertiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPropertiesChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaPropertiesChangedEventArgs {
    type Vtable = IMediaPropertiesChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d3741cb_adf0_5cef_91ba_cfabcdd77678);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPropertiesChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackInfoChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaybackInfoChangedEventArgs {
    type Vtable = IPlaybackInfoChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x786756c2_bc0d_50a5_8807_054291fef139);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackInfoChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISessionsChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISessionsChangedEventArgs {
    type Vtable = ISessionsChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbf0cd32_42c4_5a58_b317_f34bbfbd26e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISessionsChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimelinePropertiesChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimelinePropertiesChangedEventArgs {
    type Vtable = ITimelinePropertiesChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29033a2f_c923_5a77_bcaf_055ff415ad32);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelinePropertiesChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Media_Control'*"]
#[repr(transparent)]
pub struct MediaPropertiesChangedEventArgs(::windows::core::IUnknown);
impl MediaPropertiesChangedEventArgs {}
impl ::core::clone::Clone for MediaPropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPropertiesChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPropertiesChangedEventArgs {}
impl ::core::fmt::Debug for MediaPropertiesChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPropertiesChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPropertiesChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.MediaPropertiesChangedEventArgs;{7d3741cb-adf0-5cef-91ba-cfabcdd77678})");
}
unsafe impl ::windows::core::Interface for MediaPropertiesChangedEventArgs {
    type Vtable = IMediaPropertiesChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d3741cb_adf0_5cef_91ba_cfabcdd77678);
}
impl ::windows::core::RuntimeName for MediaPropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.MediaPropertiesChangedEventArgs";
}
impl ::core::convert::From<MediaPropertiesChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaPropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPropertiesChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaPropertiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPropertiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaPropertiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPropertiesChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaPropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPropertiesChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaPropertiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPropertiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaPropertiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPropertiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPropertiesChangedEventArgs {}
#[doc = "*Required features: 'Media_Control'*"]
#[repr(transparent)]
pub struct PlaybackInfoChangedEventArgs(::windows::core::IUnknown);
impl PlaybackInfoChangedEventArgs {}
impl ::core::clone::Clone for PlaybackInfoChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaybackInfoChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackInfoChangedEventArgs {}
impl ::core::fmt::Debug for PlaybackInfoChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackInfoChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlaybackInfoChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.PlaybackInfoChangedEventArgs;{786756c2-bc0d-50a5-8807-054291fef139})");
}
unsafe impl ::windows::core::Interface for PlaybackInfoChangedEventArgs {
    type Vtable = IPlaybackInfoChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x786756c2_bc0d_50a5_8807_054291fef139);
}
impl ::windows::core::RuntimeName for PlaybackInfoChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.PlaybackInfoChangedEventArgs";
}
impl ::core::convert::From<PlaybackInfoChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PlaybackInfoChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackInfoChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PlaybackInfoChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlaybackInfoChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlaybackInfoChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaybackInfoChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PlaybackInfoChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackInfoChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PlaybackInfoChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlaybackInfoChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlaybackInfoChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PlaybackInfoChangedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackInfoChangedEventArgs {}
#[doc = "*Required features: 'Media_Control'*"]
#[repr(transparent)]
pub struct SessionsChangedEventArgs(::windows::core::IUnknown);
impl SessionsChangedEventArgs {}
impl ::core::clone::Clone for SessionsChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SessionsChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SessionsChangedEventArgs {}
impl ::core::fmt::Debug for SessionsChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SessionsChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SessionsChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.SessionsChangedEventArgs;{bbf0cd32-42c4-5a58-b317-f34bbfbd26e0})");
}
unsafe impl ::windows::core::Interface for SessionsChangedEventArgs {
    type Vtable = ISessionsChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbf0cd32_42c4_5a58_b317_f34bbfbd26e0);
}
impl ::windows::core::RuntimeName for SessionsChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.SessionsChangedEventArgs";
}
impl ::core::convert::From<SessionsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SessionsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SessionsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SessionsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SessionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SessionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SessionsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SessionsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SessionsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SessionsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SessionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SessionsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SessionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for SessionsChangedEventArgs {}
#[doc = "*Required features: 'Media_Control'*"]
#[repr(transparent)]
pub struct TimelinePropertiesChangedEventArgs(::windows::core::IUnknown);
impl TimelinePropertiesChangedEventArgs {}
impl ::core::clone::Clone for TimelinePropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimelinePropertiesChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimelinePropertiesChangedEventArgs {}
impl ::core::fmt::Debug for TimelinePropertiesChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimelinePropertiesChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimelinePropertiesChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.TimelinePropertiesChangedEventArgs;{29033a2f-c923-5a77-bcaf-055ff415ad32})");
}
unsafe impl ::windows::core::Interface for TimelinePropertiesChangedEventArgs {
    type Vtable = ITimelinePropertiesChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29033a2f_c923_5a77_bcaf_055ff415ad32);
}
impl ::windows::core::RuntimeName for TimelinePropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.TimelinePropertiesChangedEventArgs";
}
impl ::core::convert::From<TimelinePropertiesChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TimelinePropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimelinePropertiesChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TimelinePropertiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimelinePropertiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimelinePropertiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimelinePropertiesChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TimelinePropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimelinePropertiesChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TimelinePropertiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimelinePropertiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimelinePropertiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimelinePropertiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for TimelinePropertiesChangedEventArgs {}
