pub struct GameBar;
impl GameBar {
    pub fn VisibilityChanged<F>(handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) + Send + 'static,
    {
        let handler = <super::super::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        Self::IGameBarStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).VisibilityChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveVisibilityChanged))
        })
    }
    pub fn IsInputRedirectedChanged<F>(handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) + Send + 'static,
    {
        let handler = <super::super::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        Self::IGameBarStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).IsInputRedirectedChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveIsInputRedirectedChanged))
        })
    }
    pub fn Visible() -> windows_core::Result<bool> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsInputRedirected() -> windows_core::Result<bool> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInputRedirected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn IGameBarStatics<R, F: FnOnce(&IGameBarStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameBar, IGameBarStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GameBar {
    const NAME: &'static str = "Windows.Gaming.UI.GameBar";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameChatMessageOrigin(pub i32);
impl GameChatMessageOrigin {
    pub const Voice: Self = Self(0);
    pub const Text: Self = Self(1);
}
impl windows_core::TypeKind for GameChatMessageOrigin {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GameChatMessageOrigin {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.UI.GameChatMessageOrigin;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Gaming.UI.GameChatMessageOrigin");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameChatMessageReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameChatMessageReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GameChatMessageReceivedEventArgs {
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn AppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SenderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Origin(&self) -> windows_core::Result<GameChatMessageOrigin> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Origin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GameChatMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameChatMessageReceivedEventArgs>();
}
unsafe impl windows_core::Interface for GameChatMessageReceivedEventArgs {
    type Vtable = <IGameChatMessageReceivedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGameChatMessageReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameChatMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatMessageReceivedEventArgs";
}
unsafe impl Send for GameChatMessageReceivedEventArgs {}
unsafe impl Sync for GameChatMessageReceivedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameChatOverlay(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameChatOverlay, windows_core::IUnknown, windows_core::IInspectable);
impl GameChatOverlay {
    pub fn DesiredPosition(&self) -> windows_core::Result<GameChatOverlayPosition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredPosition(&self, value: GameChatOverlayPosition) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredPosition)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn AddMessage(&self, sender: &windows_core::HSTRING, message: &windows_core::HSTRING, origin: GameChatMessageOrigin) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddMessage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sender), core::mem::transmute_copy(message), origin).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<Self> {
        Self::IGameChatOverlayStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IGameChatOverlayStatics<R, F: FnOnce(&IGameChatOverlayStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameChatOverlay, IGameChatOverlayStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GameChatOverlay {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameChatOverlay>();
}
unsafe impl windows_core::Interface for GameChatOverlay {
    type Vtable = <IGameChatOverlay as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGameChatOverlay as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameChatOverlay {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatOverlay";
}
unsafe impl Send for GameChatOverlay {}
unsafe impl Sync for GameChatOverlay {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameChatOverlayMessageSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameChatOverlayMessageSource, windows_core::IUnknown, windows_core::IInspectable);
impl GameChatOverlayMessageSource {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameChatOverlayMessageSource, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MessageReceived<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<GameChatMessageReceivedEventArgs>) + Send + 'static,
    {
        let handler = <super::super::Foundation::TypedEventHandler<Self, GameChatMessageReceivedEventArgs>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).MessageReceived)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveMessageReceived))
        }
    }
    pub fn SetDelayBeforeClosingAfterMessageReceived(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDelayBeforeClosingAfterMessageReceived)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for GameChatOverlayMessageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameChatOverlayMessageSource>();
}
unsafe impl windows_core::Interface for GameChatOverlayMessageSource {
    type Vtable = <IGameChatOverlayMessageSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGameChatOverlayMessageSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameChatOverlayMessageSource {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatOverlayMessageSource";
}
unsafe impl Send for GameChatOverlayMessageSource {}
unsafe impl Sync for GameChatOverlayMessageSource {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameChatOverlayPosition(pub i32);
impl GameChatOverlayPosition {
    pub const BottomCenter: Self = Self(0);
    pub const BottomLeft: Self = Self(1);
    pub const BottomRight: Self = Self(2);
    pub const MiddleRight: Self = Self(3);
    pub const MiddleLeft: Self = Self(4);
    pub const TopCenter: Self = Self(5);
    pub const TopLeft: Self = Self(6);
    pub const TopRight: Self = Self(7);
}
impl windows_core::TypeKind for GameChatOverlayPosition {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GameChatOverlayPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.UI.GameChatOverlayPosition;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Gaming.UI.GameChatOverlayPosition");
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameUIProviderActivatedEventArgs(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::interface_hierarchy!(GameUIProviderActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(GameUIProviderActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs);
#[cfg(feature = "ApplicationModel_Activation")]
impl GameUIProviderActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GameUIArgs(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GameUIArgs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReportCompleted<P0>(&self, results: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReportCompleted)(windows_core::Interface::as_raw(self), results.param().abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for GameUIProviderActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameUIProviderActivatedEventArgs>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl windows_core::Interface for GameUIProviderActivatedEventArgs {
    type Vtable = <IGameUIProviderActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGameUIProviderActivatedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeName for GameUIProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.Gaming.UI.GameUIProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Send for GameUIProviderActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl Sync for GameUIProviderActivatedEventArgs {}
windows_core::imp::define_interface!(IGameBarStatics, IGameBarStatics_Vtbl, 0x1db9a292_cc78_4173_be45_b61e67283ea7);
impl windows_core::RuntimeType for IGameBarStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Gaming.UI.IGameBarStatics");
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveVisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub IsInputRedirectedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveIsInputRedirectedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInputRedirected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameChatMessageReceivedEventArgs, IGameChatMessageReceivedEventArgs_Vtbl, 0xa28201f1_3fb9_4e42_a403_7afce2023b1e);
impl windows_core::RuntimeType for IGameChatMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Gaming.UI.IGameChatMessageReceivedEventArgs");
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatMessageReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SenderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Origin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameChatMessageOrigin) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameChatOverlay, IGameChatOverlay_Vtbl, 0xfbc64865_f6fc_4a48_ae07_03ac6ed43704);
impl windows_core::RuntimeType for IGameChatOverlay {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Gaming.UI.IGameChatOverlay");
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatOverlay_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesiredPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameChatOverlayPosition) -> windows_core::HRESULT,
    pub SetDesiredPosition: unsafe extern "system" fn(*mut core::ffi::c_void, GameChatOverlayPosition) -> windows_core::HRESULT,
    pub AddMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, GameChatMessageOrigin) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameChatOverlayMessageSource, IGameChatOverlayMessageSource_Vtbl, 0x1e177397_59fb_4f4f_8e9a_80acf817743c);
impl windows_core::RuntimeType for IGameChatOverlayMessageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Gaming.UI.IGameChatOverlayMessageSource");
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatOverlayMessageSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveMessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SetDelayBeforeClosingAfterMessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, windows_time::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameChatOverlayStatics, IGameChatOverlayStatics_Vtbl, 0x89acf614_7867_49f7_9687_25d9dbf444d1);
impl windows_core::RuntimeType for IGameChatOverlayStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Gaming.UI.IGameChatOverlayStatics");
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameChatOverlayStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::define_interface!(IGameUIProviderActivatedEventArgs, IGameUIProviderActivatedEventArgs_Vtbl, 0xa7b3203e_caf7_4ded_bbd2_47de43bb6dd5);
#[cfg(feature = "ApplicationModel_Activation")]
impl windows_core::RuntimeType for IGameUIProviderActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Gaming.UI.IGameUIProviderActivatedEventArgs");
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
#[doc(hidden)]
pub struct IGameUIProviderActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GameUIArgs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GameUIArgs: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompleted: usize,
}
