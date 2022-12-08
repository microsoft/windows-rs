#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameBarStatics {
    type Vtable = IGameBarStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameBarStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1db9a292_cc78_4173_be45_b61e67283ea7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub VisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisibilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVisibilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub IsInputRedirectedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsInputRedirectedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsInputRedirectedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsInputRedirectedChanged: usize,
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsInputRedirected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameChatMessageReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameChatMessageReceivedEventArgs {
    type Vtable = IGameChatMessageReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameChatMessageReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa28201f1_3fb9_4e42_a403_7afce2023b1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatMessageReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AppDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SenderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Origin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameChatMessageOrigin) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameChatOverlay(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameChatOverlay {
    type Vtable = IGameChatOverlay_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameChatOverlay {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc64865_f6fc_4a48_ae07_03ac6ed43704);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatOverlay_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DesiredPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameChatOverlayPosition) -> ::windows::core::HRESULT,
    pub SetDesiredPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GameChatOverlayPosition) -> ::windows::core::HRESULT,
    pub AddMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, origin: GameChatMessageOrigin) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameChatOverlayMessageSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameChatOverlayMessageSource {
    type Vtable = IGameChatOverlayMessageSource_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameChatOverlayMessageSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e177397_59fb_4f4f_8e9a_80acf817743c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatOverlayMessageSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub SetDelayBeforeClosingAfterMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDelayBeforeClosingAfterMessageReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameChatOverlayStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameChatOverlayStatics {
    type Vtable = IGameChatOverlayStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameChatOverlayStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89acf614_7867_49f7_9687_25d9dbf444d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatOverlayStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameUIProviderActivatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameUIProviderActivatedEventArgs {
    type Vtable = IGameUIProviderActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameUIProviderActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7b3203e_caf7_4ded_bbd2_47de43bb6dd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameUIProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GameUIArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GameUIArgs: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, results: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompleted: usize,
}
#[doc = "*Required features: `\"Gaming_UI\"`*"]
pub struct GameBar;
impl GameBar {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VisibilityChanged(handler: &super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VisibilityChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisibilityChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IGameBarStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveVisibilityChanged)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsInputRedirectedChanged(handler: &super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsInputRedirectedChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsInputRedirectedChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IGameBarStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveIsInputRedirectedChanged)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    pub fn Visible() -> ::windows::core::Result<bool> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Visible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsInputRedirected() -> ::windows::core::Result<bool> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsInputRedirected)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameBarStatics<R, F: FnOnce(&IGameBarStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GameBar, IGameBarStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GameBar {
    const NAME: &'static str = "Windows.Gaming.UI.GameBar";
}
#[doc = "*Required features: `\"Gaming_UI\"`*"]
#[repr(transparent)]
pub struct GameChatMessageReceivedEventArgs(::windows::core::IUnknown);
impl GameChatMessageReceivedEventArgs {
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SenderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SenderName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Message)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Origin(&self) -> ::windows::core::Result<GameChatMessageOrigin> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Origin)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameChatMessageReceivedEventArgs {
    type Vtable = IGameChatMessageReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for GameChatMessageReceivedEventArgs {
    const IID: ::windows::core::GUID = <IGameChatMessageReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameChatMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatMessageReceivedEventArgs";
}
::windows::core::interface_hierarchy!(GameChatMessageReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameChatMessageReceivedEventArgs {}
unsafe impl ::core::marker::Sync for GameChatMessageReceivedEventArgs {}
#[doc = "*Required features: `\"Gaming_UI\"`*"]
#[repr(transparent)]
pub struct GameChatOverlay(::windows::core::IUnknown);
impl GameChatOverlay {
    pub fn DesiredPosition(&self) -> ::windows::core::Result<GameChatOverlayPosition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredPosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDesiredPosition(&self, value: GameChatOverlayPosition) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDesiredPosition)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AddMessage(&self, sender: &::windows::core::HSTRING, message: &::windows::core::HSTRING, origin: GameChatMessageOrigin) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender), ::core::mem::transmute_copy(message), origin).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<GameChatOverlay> {
        Self::IGameChatOverlayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameChatOverlayStatics<R, F: FnOnce(&IGameChatOverlayStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GameChatOverlay, IGameChatOverlayStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameChatOverlay {
    type Vtable = IGameChatOverlay_Vtbl;
}
unsafe impl ::windows::core::Interface for GameChatOverlay {
    const IID: ::windows::core::GUID = <IGameChatOverlay as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameChatOverlay {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatOverlay";
}
::windows::core::interface_hierarchy!(GameChatOverlay, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameChatOverlay {}
unsafe impl ::core::marker::Sync for GameChatOverlay {}
#[doc = "*Required features: `\"Gaming_UI\"`*"]
#[repr(transparent)]
pub struct GameChatOverlayMessageSource(::windows::core::IUnknown);
impl GameChatOverlayMessageSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GameChatOverlayMessageSource, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceived(&self, handler: &super::super::Foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MessageReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMessageReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDelayBeforeClosingAfterMessageReceived(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDelayBeforeClosingAfterMessageReceived)(::windows::core::Vtable::as_raw(this), value).ok() }
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameChatOverlayMessageSource {
    type Vtable = IGameChatOverlayMessageSource_Vtbl;
}
unsafe impl ::windows::core::Interface for GameChatOverlayMessageSource {
    const IID: ::windows::core::GUID = <IGameChatOverlayMessageSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameChatOverlayMessageSource {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatOverlayMessageSource";
}
::windows::core::interface_hierarchy!(GameChatOverlayMessageSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameChatOverlayMessageSource {}
unsafe impl ::core::marker::Sync for GameChatOverlayMessageSource {}
#[doc = "*Required features: `\"Gaming_UI\"`*"]
#[repr(transparent)]
pub struct GameUIProviderActivatedEventArgs(::windows::core::IUnknown);
impl GameUIProviderActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousExecutionState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SplashScreen)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GameUIArgs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GameUIArgs)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReportCompleted(&self, results: &super::super::Foundation::Collections::ValueSet) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ReportCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(results)).ok() }
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameUIProviderActivatedEventArgs {
    type Vtable = IGameUIProviderActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for GameUIProviderActivatedEventArgs {
    const IID: ::windows::core::GUID = <IGameUIProviderActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameUIProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.Gaming.UI.GameUIProviderActivatedEventArgs";
}
::windows::core::interface_hierarchy!(GameUIProviderActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&GameUIProviderActivatedEventArgs> for ::windows::core::InParam<super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &GameUIProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for GameUIProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for GameUIProviderActivatedEventArgs {}
#[doc = "*Required features: `\"Gaming_UI\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for GameChatMessageOrigin {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GameChatMessageOrigin {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameChatMessageOrigin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameChatMessageOrigin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameChatMessageOrigin {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.UI.GameChatMessageOrigin;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_UI\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for GameChatOverlayPosition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GameChatOverlayPosition {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameChatOverlayPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameChatOverlayPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameChatOverlayPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.UI.GameChatOverlayPosition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
