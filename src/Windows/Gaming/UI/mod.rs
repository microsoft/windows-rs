#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Gaming_UI`*"]
pub struct GameBar {}
impl GameBar {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_UI`, `Foundation`*"]
    pub fn VisibilityChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_UI`, `Foundation`*"]
    pub fn RemoveVisibilityChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IGameBarStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_UI`, `Foundation`*"]
    pub fn IsInputRedirectedChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_UI`, `Foundation`*"]
    pub fn RemoveIsInputRedirectedChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IGameBarStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Gaming_UI`*"]
    pub fn Visible() -> ::windows::runtime::Result<bool> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Gaming_UI`*"]
    pub fn IsInputRedirected() -> ::windows::runtime::Result<bool> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IGameBarStatics<R, F: FnOnce(&IGameBarStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GameBar, IGameBarStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for GameBar {
    const NAME: &'static str = "Windows.Gaming.UI.GameBar";
}
#[doc = "*Required features: `Gaming_UI`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameChatMessageOrigin(pub i32);
impl GameChatMessageOrigin {
    pub const Voice: GameChatMessageOrigin = GameChatMessageOrigin(0i32);
    pub const Text: GameChatMessageOrigin = GameChatMessageOrigin(1i32);
}
impl ::std::convert::From<i32> for GameChatMessageOrigin {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GameChatMessageOrigin {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GameChatMessageOrigin {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.UI.GameChatMessageOrigin;i4)");
}
#[doc = "*Required features: `Gaming_UI`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct GameChatMessageReceivedEventArgs(::windows::runtime::IInspectable);
impl GameChatMessageReceivedEventArgs {
    #[doc = "*Required features: `Gaming_UI`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_UI`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_UI`*"]
    pub fn SenderName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_UI`*"]
    pub fn Message(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_UI`*"]
    pub fn Origin(&self) -> ::windows::runtime::Result<GameChatMessageOrigin> {
        let this = self;
        unsafe {
            let mut result__: GameChatMessageOrigin = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameChatMessageOrigin>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameChatMessageReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.UI.GameChatMessageReceivedEventArgs;{a28201f1-3fb9-4e42-a403-7afce2023b1e})");
}
unsafe impl ::windows::runtime::Interface for GameChatMessageReceivedEventArgs {
    type Vtable = IGameChatMessageReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2726429169, 16313, 20034, [164, 3, 122, 252, 226, 2, 59, 30]);
}
impl ::windows::runtime::RuntimeName for GameChatMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatMessageReceivedEventArgs";
}
unsafe impl ::std::marker::Send for GameChatMessageReceivedEventArgs {}
unsafe impl ::std::marker::Sync for GameChatMessageReceivedEventArgs {}
#[doc = "*Required features: `Gaming_UI`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct GameChatOverlay(::windows::runtime::IInspectable);
impl GameChatOverlay {
    #[doc = "*Required features: `Gaming_UI`*"]
    pub fn DesiredPosition(&self) -> ::windows::runtime::Result<GameChatOverlayPosition> {
        let this = self;
        unsafe {
            let mut result__: GameChatOverlayPosition = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameChatOverlayPosition>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_UI`*"]
    pub fn SetDesiredPosition(&self, value: GameChatOverlayPosition) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Gaming_UI`*"]
    pub fn AddMessage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, sender: Param0, message: Param1, origin: GameChatMessageOrigin) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), sender.into_param().abi(), message.into_param().abi(), origin).ok() }
    }
    #[doc = "*Required features: `Gaming_UI`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<GameChatOverlay> {
        Self::IGameChatOverlayStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameChatOverlay>(result__)
        })
    }
    pub fn IGameChatOverlayStatics<R, F: FnOnce(&IGameChatOverlayStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GameChatOverlay, IGameChatOverlayStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameChatOverlay {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.UI.GameChatOverlay;{fbc64865-f6fc-4a48-ae07-03ac6ed43704})");
}
unsafe impl ::windows::runtime::Interface for GameChatOverlay {
    type Vtable = IGameChatOverlay_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4224075877, 63228, 19016, [174, 7, 3, 172, 110, 212, 55, 4]);
}
impl ::windows::runtime::RuntimeName for GameChatOverlay {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatOverlay";
}
unsafe impl ::std::marker::Send for GameChatOverlay {}
unsafe impl ::std::marker::Sync for GameChatOverlay {}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct GameChatOverlayContract(pub u8);
#[doc = "*Required features: `Gaming_UI`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct GameChatOverlayMessageSource(::windows::runtime::IInspectable);
impl GameChatOverlayMessageSource {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GameChatOverlayMessageSource, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_UI`, `Foundation`*"]
    pub fn MessageReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_UI`, `Foundation`*"]
    pub fn RemoveMessageReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_UI`, `Foundation`*"]
    pub fn SetDelayBeforeClosingAfterMessageReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameChatOverlayMessageSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.UI.GameChatOverlayMessageSource;{1e177397-59fb-4f4f-8e9a-80acf817743c})");
}
unsafe impl ::windows::runtime::Interface for GameChatOverlayMessageSource {
    type Vtable = IGameChatOverlayMessageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(504853399, 23035, 20303, [142, 154, 128, 172, 248, 23, 116, 60]);
}
impl ::windows::runtime::RuntimeName for GameChatOverlayMessageSource {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatOverlayMessageSource";
}
unsafe impl ::std::marker::Send for GameChatOverlayMessageSource {}
unsafe impl ::std::marker::Sync for GameChatOverlayMessageSource {}
#[doc = "*Required features: `Gaming_UI`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameChatOverlayPosition(pub i32);
impl GameChatOverlayPosition {
    pub const BottomCenter: GameChatOverlayPosition = GameChatOverlayPosition(0i32);
    pub const BottomLeft: GameChatOverlayPosition = GameChatOverlayPosition(1i32);
    pub const BottomRight: GameChatOverlayPosition = GameChatOverlayPosition(2i32);
    pub const MiddleRight: GameChatOverlayPosition = GameChatOverlayPosition(3i32);
    pub const MiddleLeft: GameChatOverlayPosition = GameChatOverlayPosition(4i32);
    pub const TopCenter: GameChatOverlayPosition = GameChatOverlayPosition(5i32);
    pub const TopLeft: GameChatOverlayPosition = GameChatOverlayPosition(6i32);
    pub const TopRight: GameChatOverlayPosition = GameChatOverlayPosition(7i32);
}
impl ::std::convert::From<i32> for GameChatOverlayPosition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GameChatOverlayPosition {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GameChatOverlayPosition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.UI.GameChatOverlayPosition;i4)");
}
#[doc = "*Required features: `Gaming_UI`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct GameUIProviderActivatedEventArgs(::windows::runtime::IInspectable);
impl GameUIProviderActivatedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Gaming_UI`, `Foundation_Collections`*"]
    pub fn GameUIArgs(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Gaming_UI`, `Foundation_Collections`*"]
    pub fn ReportCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, results: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), results.into_param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    #[doc = "*Required features: `Gaming_UI`, `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    #[doc = "*Required features: `Gaming_UI`, `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ApplicationExecutionState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    #[doc = "*Required features: `Gaming_UI`, `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameUIProviderActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.UI.GameUIProviderActivatedEventArgs;{a7b3203e-caf7-4ded-bbd2-47de43bb6dd5})");
}
unsafe impl ::windows::runtime::Interface for GameUIProviderActivatedEventArgs {
    type Vtable = IGameUIProviderActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2813534270, 51959, 19949, [187, 210, 71, 222, 67, 187, 109, 213]);
}
impl ::windows::runtime::RuntimeName for GameUIProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.Gaming.UI.GameUIProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<GameUIProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: GameUIProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&GameUIProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &GameUIProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> for GameUIProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> for &GameUIProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::std::convert::TryInto::<super::super::ApplicationModel::Activation::IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for GameUIProviderActivatedEventArgs {}
unsafe impl ::std::marker::Sync for GameUIProviderActivatedEventArgs {}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct GamingUIProviderContract(pub u8);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameBarStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameBarStatics {
    type Vtable = IGameBarStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(498705042, 52344, 16755, [190, 69, 182, 30, 103, 40, 62, 167]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameChatMessageReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameChatMessageReceivedEventArgs {
    type Vtable = IGameChatMessageReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2726429169, 16313, 20034, [164, 3, 122, 252, 226, 2, 59, 30]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatMessageReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GameChatMessageOrigin) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameChatOverlay(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameChatOverlay {
    type Vtable = IGameChatOverlay_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4224075877, 63228, 19016, [174, 7, 3, 172, 110, 212, 55, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatOverlay_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GameChatOverlayPosition) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: GameChatOverlayPosition) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, message: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, origin: GameChatMessageOrigin) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameChatOverlayMessageSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameChatOverlayMessageSource {
    type Vtable = IGameChatOverlayMessageSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(504853399, 23035, 20303, [142, 154, 128, 172, 248, 23, 116, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatOverlayMessageSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameChatOverlayStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameChatOverlayStatics {
    type Vtable = IGameChatOverlayStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2309813780, 30823, 18935, [150, 135, 37, 217, 219, 244, 68, 209]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatOverlayStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGameUIProviderActivatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameUIProviderActivatedEventArgs {
    type Vtable = IGameUIProviderActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2813534270, 51959, 19949, [187, 210, 71, 222, 67, 187, 109, 213]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameUIProviderActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, results: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
