windows_core::imp::define_winrt_interface!(IOverlayActivatedEventArgs, IOverlayActivatedEventArgs_Vtbl, 0x277092d8_617c_50ba_9e57_6dc25078beb0, b"Windows.ApplicationModel.Overlay.IOverlayActivatedEventArgs");
#[repr(C)]
#[doc(hidden)]
pub struct IOverlayActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Command: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IOverlayApp, IOverlayApp_Vtbl, 0x0410c228_3813_54fc_b402_88520526d1f5, b"Windows.ApplicationModel.Overlay.IOverlayApp");
#[repr(C)]
#[doc(hidden)]
pub struct IOverlayApp_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VisualState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OverlayVisualState) -> windows_core::HRESULT,
    pub VisualStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveVisualStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub RequestInteractiveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExitInteractiveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPersistentModeEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetUnreadNotificationCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetHasActiveNotification: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Activated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveActivated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub VisualTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveVisualTransition: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Disconnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveDisconnected: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IOverlayAppOptions, IOverlayAppOptions_Vtbl, 0x9e0dd89a_0f2a_5cfd_805d_dbb13164c146, b"Windows.ApplicationModel.Overlay.IOverlayAppOptions");
#[repr(C)]
#[doc(hidden)]
pub struct IOverlayAppOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CustomTransitionAnimations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCustomTransitionAnimations: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IOverlayAppStatics, IOverlayAppStatics_Vtbl, 0x2c112604_c435_53d0_9be6_6431e39eb26c, b"Windows.ApplicationModel.Overlay.IOverlayAppStatics");
#[repr(C)]
#[doc(hidden)]
pub struct IOverlayAppStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IOverlayCommand, IOverlayCommand_Vtbl, 0xdde596f8_6093_5029_8cbc_06b046d328dc, b"Windows.ApplicationModel.Overlay.IOverlayCommand");
#[repr(C)]
#[doc(hidden)]
pub struct IOverlayCommand_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OverlayCommandSource) -> windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IOverlayCommandFactory, IOverlayCommandFactory_Vtbl, 0x10210bb2_4755_520d_802b_7490c260f802, b"Windows.ApplicationModel.Overlay.IOverlayCommandFactory");
#[repr(C)]
#[doc(hidden)]
pub struct IOverlayCommandFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, OverlayCommandSource, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IOverlayDisconnectedEventArgs, IOverlayDisconnectedEventArgs_Vtbl, 0x9573e2be_6692_513e_9ed4_71642a182d42, b"Windows.ApplicationModel.Overlay.IOverlayDisconnectedEventArgs");
#[repr(C)]
#[doc(hidden)]
pub struct IOverlayDisconnectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(IOverlayVisualTransitionEventArgs, IOverlayVisualTransitionEventArgs_Vtbl, 0x0cc0e9dd_b088_5f81_a02b_c925259a464d, b"Windows.ApplicationModel.Overlay.IOverlayVisualTransitionEventArgs");
#[repr(C)]
#[doc(hidden)]
pub struct IOverlayVisualTransitionEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NewState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OverlayVisualState) -> windows_core::HRESULT,
    pub Command: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransitionAnimation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OverlayTransitionAnimation) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlayActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OverlayActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl OverlayActivatedEventArgs {
    pub fn Command(&self) -> windows_core::Result<OverlayCommand> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Command)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OverlayActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOverlayActivatedEventArgs>();
}
unsafe impl windows_core::Interface for OverlayActivatedEventArgs {
    type Vtable = <IOverlayActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOverlayActivatedEventArgs as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for OverlayActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Overlay.OverlayActivatedEventArgs";
}
unsafe impl Send for OverlayActivatedEventArgs {}
unsafe impl Sync for OverlayActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlayApp(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OverlayApp, windows_core::IUnknown, windows_core::IInspectable);
impl OverlayApp {
    pub fn StartAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DisconnectAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisconnectAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn VisualState(&self) -> windows_core::Result<OverlayVisualState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VisualState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn VisualStateChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) + Send + Sync + 'static,
    {
        let handler = <super::super::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).VisualStateChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveVisualStateChanged))
        }
    }
    pub fn RequestInteractiveAsync<P0>(&self, command: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<OverlayCommand>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestInteractiveAsync)(windows_core::Interface::as_raw(self), command.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ExitInteractiveAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExitInteractiveAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetPersistentModeEnabled(&self, enabled: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPersistentModeEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
    pub fn SetUnreadNotificationCount(&self, count: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUnreadNotificationCount)(windows_core::Interface::as_raw(self), count).ok() }
    }
    pub fn SetHasActiveNotification(&self, hasactive: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHasActiveNotification)(windows_core::Interface::as_raw(self), hasactive).ok() }
    }
    pub fn Activated<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<OverlayActivatedEventArgs>) + Send + Sync + 'static,
    {
        let handler = <super::super::Foundation::EventHandler<OverlayActivatedEventArgs>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Activated)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveActivated))
        }
    }
    pub fn VisualTransition<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<OverlayVisualTransitionEventArgs>) + Send + Sync + 'static,
    {
        let handler = <super::super::Foundation::EventHandler<OverlayVisualTransitionEventArgs>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).VisualTransition)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveVisualTransition))
        }
    }
    pub fn Disconnected<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<OverlayDisconnectedEventArgs>) + Send + Sync + 'static,
    {
        let handler = <super::super::Foundation::EventHandler<OverlayDisconnectedEventArgs>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Disconnected)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveDisconnected))
        }
    }
    pub fn Create(hwnd: i64) -> windows_core::Result<Self> {
        Self::IOverlayAppStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), hwnd, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn Create2<P1>(hwnd: i64, options: P1) -> windows_core::Result<Self>
    where
        P1: windows_core::Param<OverlayAppOptions>,
    {
        Self::IOverlayAppStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), hwnd, options.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IOverlayAppStatics<R, F: FnOnce(&IOverlayAppStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OverlayApp, IOverlayAppStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for OverlayApp {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOverlayApp>();
}
unsafe impl windows_core::Interface for OverlayApp {
    type Vtable = <IOverlayApp as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOverlayApp as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for OverlayApp {
    const NAME: &'static str = "Windows.ApplicationModel.Overlay.OverlayApp";
}
unsafe impl Send for OverlayApp {}
unsafe impl Sync for OverlayApp {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlayAppOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OverlayAppOptions, windows_core::IUnknown, windows_core::IInspectable);
impl OverlayAppOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OverlayAppOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CustomTransitionAnimations(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomTransitionAnimations)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetCustomTransitionAnimations(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCustomTransitionAnimations)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for OverlayAppOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOverlayAppOptions>();
}
unsafe impl windows_core::Interface for OverlayAppOptions {
    type Vtable = <IOverlayAppOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOverlayAppOptions as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for OverlayAppOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Overlay.OverlayAppOptions";
}
unsafe impl Send for OverlayAppOptions {}
unsafe impl Sync for OverlayAppOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlayCommand(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OverlayCommand, windows_core::IUnknown, windows_core::IInspectable);
impl OverlayCommand {
    pub fn Source(&self) -> windows_core::Result<OverlayCommandSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Source)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P1>(source: OverlayCommandSource, uri: P1) -> windows_core::Result<Self>
    where
        P1: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IOverlayCommandFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), source, uri.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IOverlayCommandFactory<R, F: FnOnce(&IOverlayCommandFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OverlayCommand, IOverlayCommandFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for OverlayCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOverlayCommand>();
}
unsafe impl windows_core::Interface for OverlayCommand {
    type Vtable = <IOverlayCommand as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOverlayCommand as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for OverlayCommand {
    const NAME: &'static str = "Windows.ApplicationModel.Overlay.OverlayCommand";
}
unsafe impl Send for OverlayCommand {}
unsafe impl Sync for OverlayCommand {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OverlayCommandSource(pub i32);
impl OverlayCommandSource {
    pub const Keyboard: Self = Self(0);
    pub const Gamepad: Self = Self(1);
    pub const Programmatic: Self = Self(2);
    pub const Pointer: Self = Self(3);
}
impl windows_core::imp::TypeKind for OverlayCommandSource {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for OverlayCommandSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Overlay.OverlayCommandSource;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Overlay.OverlayCommandSource");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlayDisconnectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OverlayDisconnectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl OverlayDisconnectedEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeferral)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OverlayDisconnectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOverlayDisconnectedEventArgs>();
}
unsafe impl windows_core::Interface for OverlayDisconnectedEventArgs {
    type Vtable = <IOverlayDisconnectedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOverlayDisconnectedEventArgs as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for OverlayDisconnectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Overlay.OverlayDisconnectedEventArgs";
}
unsafe impl Send for OverlayDisconnectedEventArgs {}
unsafe impl Sync for OverlayDisconnectedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OverlayTransitionAnimation(pub i32);
impl OverlayTransitionAnimation {
    pub const Disabled: Self = Self(0);
    pub const PlatformAuto: Self = Self(1);
    pub const Fade: Self = Self(2);
    pub const SlideLeft: Self = Self(3);
    pub const SlideRight: Self = Self(4);
}
impl windows_core::imp::TypeKind for OverlayTransitionAnimation {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for OverlayTransitionAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Overlay.OverlayTransitionAnimation;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Overlay.OverlayTransitionAnimation");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OverlayVisualState(pub i32);
impl OverlayVisualState {
    pub const Hidden: Self = Self(0);
    pub const Persistent: Self = Self(1);
    pub const Interactive: Self = Self(2);
}
impl windows_core::imp::TypeKind for OverlayVisualState {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for OverlayVisualState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Overlay.OverlayVisualState;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Overlay.OverlayVisualState");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlayVisualTransitionEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OverlayVisualTransitionEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl OverlayVisualTransitionEventArgs {
    pub fn NewState(&self) -> windows_core::Result<OverlayVisualState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Command(&self) -> windows_core::Result<OverlayCommand> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Command)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn TransitionAnimation(&self) -> windows_core::Result<OverlayTransitionAnimation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransitionAnimation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeferral)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OverlayVisualTransitionEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOverlayVisualTransitionEventArgs>();
}
unsafe impl windows_core::Interface for OverlayVisualTransitionEventArgs {
    type Vtable = <IOverlayVisualTransitionEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOverlayVisualTransitionEventArgs as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for OverlayVisualTransitionEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Overlay.OverlayVisualTransitionEventArgs";
}
unsafe impl Send for OverlayVisualTransitionEventArgs {}
unsafe impl Sync for OverlayVisualTransitionEventArgs {}
