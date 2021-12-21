#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Gaming_UI'*"]
pub struct GameBar {}
impl GameBar {
    #[doc = "*Required features: 'Gaming_UI', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn VisibilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'Gaming_UI', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisibilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IGameBarStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'Gaming_UI', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn IsInputRedirectedChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'Gaming_UI', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsInputRedirectedChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IGameBarStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'Gaming_UI'*"]
    pub fn Visible() -> ::windows::core::Result<bool> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Gaming_UI'*"]
    pub fn IsInputRedirected() -> ::windows::core::Result<bool> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameBarStatics<R, F: FnOnce(&IGameBarStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameBar, IGameBarStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GameBar {
    const NAME: &'static str = "Windows.Gaming.UI.GameBar";
}
#[doc = "*Required features: 'Gaming_UI'*"]
#[repr(transparent)]
pub struct GameChatMessageOrigin(pub i32);
impl GameChatMessageOrigin {
    pub const Voice: Self = Self(0i32);
    pub const Text: Self = Self(1i32);
}
impl ::core::marker::Copy for GameChatMessageOrigin {}
impl ::core::clone::Clone for GameChatMessageOrigin {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GameChatMessageOrigin {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GameChatMessageOrigin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameChatMessageOrigin {}
impl ::core::fmt::Debug for GameChatMessageOrigin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameChatMessageOrigin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameChatMessageOrigin {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.UI.GameChatMessageOrigin;i4)");
}
impl ::windows::core::DefaultType for GameChatMessageOrigin {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Gaming_UI'*"]
#[repr(transparent)]
pub struct GameChatMessageReceivedEventArgs(::windows::core::IUnknown);
impl GameChatMessageReceivedEventArgs {
    #[doc = "*Required features: 'Gaming_UI'*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_UI'*"]
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_UI'*"]
    pub fn SenderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_UI'*"]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_UI'*"]
    pub fn Origin(&self) -> ::windows::core::Result<GameChatMessageOrigin> {
        let this = self;
        unsafe {
            let mut result__: GameChatMessageOrigin = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameChatMessageOrigin>(result__)
        }
    }
}
impl ::core::clone::Clone for GameChatMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameChatMessageReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameChatMessageReceivedEventArgs {}
impl ::core::fmt::Debug for GameChatMessageReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameChatMessageReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameChatMessageReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.UI.GameChatMessageReceivedEventArgs;{a28201f1-3fb9-4e42-a403-7afce2023b1e})");
}
unsafe impl ::windows::core::Interface for GameChatMessageReceivedEventArgs {
    type Vtable = IGameChatMessageReceivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa28201f1_3fb9_4e42_a403_7afce2023b1e);
}
impl ::windows::core::RuntimeName for GameChatMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatMessageReceivedEventArgs";
}
impl ::core::convert::From<GameChatMessageReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GameChatMessageReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameChatMessageReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GameChatMessageReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameChatMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GameChatMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameChatMessageReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GameChatMessageReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameChatMessageReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GameChatMessageReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameChatMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GameChatMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameChatMessageReceivedEventArgs {}
unsafe impl ::core::marker::Sync for GameChatMessageReceivedEventArgs {}
#[doc = "*Required features: 'Gaming_UI'*"]
#[repr(transparent)]
pub struct GameChatOverlay(::windows::core::IUnknown);
impl GameChatOverlay {
    #[doc = "*Required features: 'Gaming_UI'*"]
    pub fn DesiredPosition(&self) -> ::windows::core::Result<GameChatOverlayPosition> {
        let this = self;
        unsafe {
            let mut result__: GameChatOverlayPosition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameChatOverlayPosition>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_UI'*"]
    pub fn SetDesiredPosition(&self, value: GameChatOverlayPosition) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Gaming_UI'*"]
    pub fn AddMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, sender: Param0, message: Param1, origin: GameChatMessageOrigin) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), sender.into_param().abi(), message.into_param().abi(), origin).ok() }
    }
    #[doc = "*Required features: 'Gaming_UI'*"]
    pub fn GetDefault() -> ::windows::core::Result<GameChatOverlay> {
        Self::IGameChatOverlayStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameChatOverlay>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameChatOverlayStatics<R, F: FnOnce(&IGameChatOverlayStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameChatOverlay, IGameChatOverlayStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GameChatOverlay {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameChatOverlay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameChatOverlay {}
impl ::core::fmt::Debug for GameChatOverlay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameChatOverlay").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameChatOverlay {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.UI.GameChatOverlay;{fbc64865-f6fc-4a48-ae07-03ac6ed43704})");
}
unsafe impl ::windows::core::Interface for GameChatOverlay {
    type Vtable = IGameChatOverlayVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc64865_f6fc_4a48_ae07_03ac6ed43704);
}
impl ::windows::core::RuntimeName for GameChatOverlay {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatOverlay";
}
impl ::core::convert::From<GameChatOverlay> for ::windows::core::IUnknown {
    fn from(value: GameChatOverlay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameChatOverlay> for ::windows::core::IUnknown {
    fn from(value: &GameChatOverlay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameChatOverlay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GameChatOverlay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameChatOverlay> for ::windows::core::IInspectable {
    fn from(value: GameChatOverlay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameChatOverlay> for ::windows::core::IInspectable {
    fn from(value: &GameChatOverlay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameChatOverlay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GameChatOverlay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameChatOverlay {}
unsafe impl ::core::marker::Sync for GameChatOverlay {}
#[doc = "*Required features: 'Gaming_UI'*"]
#[repr(transparent)]
pub struct GameChatOverlayMessageSource(::windows::core::IUnknown);
impl GameChatOverlayMessageSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameChatOverlayMessageSource, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Gaming_UI', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_UI', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Gaming_UI', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDelayBeforeClosingAfterMessageReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for GameChatOverlayMessageSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameChatOverlayMessageSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameChatOverlayMessageSource {}
impl ::core::fmt::Debug for GameChatOverlayMessageSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameChatOverlayMessageSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameChatOverlayMessageSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.UI.GameChatOverlayMessageSource;{1e177397-59fb-4f4f-8e9a-80acf817743c})");
}
unsafe impl ::windows::core::Interface for GameChatOverlayMessageSource {
    type Vtable = IGameChatOverlayMessageSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e177397_59fb_4f4f_8e9a_80acf817743c);
}
impl ::windows::core::RuntimeName for GameChatOverlayMessageSource {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatOverlayMessageSource";
}
impl ::core::convert::From<GameChatOverlayMessageSource> for ::windows::core::IUnknown {
    fn from(value: GameChatOverlayMessageSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameChatOverlayMessageSource> for ::windows::core::IUnknown {
    fn from(value: &GameChatOverlayMessageSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameChatOverlayMessageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GameChatOverlayMessageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameChatOverlayMessageSource> for ::windows::core::IInspectable {
    fn from(value: GameChatOverlayMessageSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameChatOverlayMessageSource> for ::windows::core::IInspectable {
    fn from(value: &GameChatOverlayMessageSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameChatOverlayMessageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GameChatOverlayMessageSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameChatOverlayMessageSource {}
unsafe impl ::core::marker::Sync for GameChatOverlayMessageSource {}
#[doc = "*Required features: 'Gaming_UI'*"]
#[repr(transparent)]
pub struct GameChatOverlayPosition(pub i32);
impl GameChatOverlayPosition {
    pub const BottomCenter: Self = Self(0i32);
    pub const BottomLeft: Self = Self(1i32);
    pub const BottomRight: Self = Self(2i32);
    pub const MiddleRight: Self = Self(3i32);
    pub const MiddleLeft: Self = Self(4i32);
    pub const TopCenter: Self = Self(5i32);
    pub const TopLeft: Self = Self(6i32);
    pub const TopRight: Self = Self(7i32);
}
impl ::core::marker::Copy for GameChatOverlayPosition {}
impl ::core::clone::Clone for GameChatOverlayPosition {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GameChatOverlayPosition {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GameChatOverlayPosition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameChatOverlayPosition {}
impl ::core::fmt::Debug for GameChatOverlayPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameChatOverlayPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameChatOverlayPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.UI.GameChatOverlayPosition;i4)");
}
impl ::windows::core::DefaultType for GameChatOverlayPosition {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Gaming_UI'*"]
#[repr(transparent)]
pub struct GameUIProviderActivatedEventArgs(::windows::core::IUnknown);
impl GameUIProviderActivatedEventArgs {
    #[doc = "*Required features: 'Gaming_UI', 'ApplicationModel_Activation'*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_UI', 'ApplicationModel_Activation'*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_UI', 'ApplicationModel_Activation'*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_UI', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GameUIArgs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: 'Gaming_UI', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReportCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, results: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), results.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for GameUIProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameUIProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameUIProviderActivatedEventArgs {}
impl ::core::fmt::Debug for GameUIProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameUIProviderActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameUIProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.UI.GameUIProviderActivatedEventArgs;{a7b3203e-caf7-4ded-bbd2-47de43bb6dd5})");
}
unsafe impl ::windows::core::Interface for GameUIProviderActivatedEventArgs {
    type Vtable = IGameUIProviderActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7b3203e_caf7_4ded_bbd2_47de43bb6dd5);
}
impl ::windows::core::RuntimeName for GameUIProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.Gaming.UI.GameUIProviderActivatedEventArgs";
}
impl ::core::convert::From<GameUIProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GameUIProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameUIProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GameUIProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameUIProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GameUIProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameUIProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GameUIProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameUIProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GameUIProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameUIProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GameUIProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<GameUIProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: GameUIProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&GameUIProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &GameUIProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> for GameUIProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> for &GameUIProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<super::super::ApplicationModel::Activation::IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GameUIProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for GameUIProviderActivatedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameBarStatics {
    type Vtable = IGameBarStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1db9a292_cc78_4173_be45_b61e67283ea7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameChatMessageReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameChatMessageReceivedEventArgs {
    type Vtable = IGameChatMessageReceivedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa28201f1_3fb9_4e42_a403_7afce2023b1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatMessageReceivedEventArgsVtbl(
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
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameChatMessageOrigin) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameChatOverlay(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameChatOverlay {
    type Vtable = IGameChatOverlayVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc64865_f6fc_4a48_ae07_03ac6ed43704);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatOverlayVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameChatOverlayPosition) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GameChatOverlayPosition) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, origin: GameChatMessageOrigin) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameChatOverlayMessageSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameChatOverlayMessageSource {
    type Vtable = IGameChatOverlayMessageSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e177397_59fb_4f4f_8e9a_80acf817743c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatOverlayMessageSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameChatOverlayStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameChatOverlayStatics {
    type Vtable = IGameChatOverlayStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89acf614_7867_49f7_9687_25d9dbf444d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatOverlayStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameUIProviderActivatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameUIProviderActivatedEventArgs {
    type Vtable = IGameUIProviderActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7b3203e_caf7_4ded_bbd2_47de43bb6dd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameUIProviderActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, results: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
