windows_core::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_core::HRESULT);
windows_core::link!("user32.dll" "system" fn GetDpiForWindow(hwnd : HWND) -> u32);
windows_core::link!("user32.dll" "system" fn GetMonitorInfoW(hmonitor : HMONITOR, lpmi : *mut MONITORINFO) -> windows_core::BOOL);
windows_core::link!("microsoft.windowsappruntime.bootstrap.dll" "system" fn MddBootstrapInitialize2(majorminorversion : u32, versiontag : *const u16, minversion : PACKAGE_VERSION, options : MddBootstrapInitializeOptions) -> windows_core::HRESULT);
windows_core::link!("microsoft.windowsappruntime.bootstrap.dll" "system" fn MddBootstrapShutdown());
windows_core::link!("user32.dll" "system" fn MonitorFromWindow(hwnd : HWND, dwflags : MONITOR_FROM_FLAGS) -> HMONITOR);
windows_core::link!("user32.dll" "system" fn PostMessageW(hwnd : HWND, msg : u32, wparam : WPARAM, lparam : LPARAM) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn SetProcessDpiAwarenessContext(value : DPI_AWARENESS_CONTEXT) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn SetWindowPos(hwnd : HWND, hwndinsertafter : HWND, x : i32, y : i32, cx : i32, cy : i32, uflags : SET_WINDOW_POS_FLAGS) -> windows_core::BOOL);
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBar, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(
    AppBar,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl AppBar {}
impl windows_core::RuntimeType for AppBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppBar>();
}
unsafe impl windows_core::Interface for AppBar {
    type Vtable = <IAppBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppBar {
    type Target = IAppBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppBar {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.AppBar";
}
unsafe impl Send for AppBar {}
unsafe impl Sync for AppBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppBarButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppBarButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    AppBarButton,
    ICommandBarElement,
    Button,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl AppBarButton {
    pub fn new() -> windows_core::Result<AppBarButton> {
        Self::IAppBarButtonFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<AppBarButton>
    where
        T: windows_core::Compose,
    {
        Self::IAppBarButtonFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IAppBarButtonFactory<R, F: FnOnce(&IAppBarButtonFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppBarButton, IAppBarButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppBarButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppBarButton>();
}
unsafe impl windows_core::Interface for AppBarButton {
    type Vtable = <IAppBarButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppBarButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppBarButton {
    type Target = IAppBarButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppBarButton {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.AppBarButton";
}
unsafe impl Send for AppBarButton {}
unsafe impl Sync for AppBarButton {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppBarSeparator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppBarSeparator,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    AppBarSeparator,
    ICommandBarElement,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl AppBarSeparator {
    pub fn new() -> windows_core::Result<AppBarSeparator> {
        Self::IAppBarSeparatorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<AppBarSeparator>
    where
        T: windows_core::Compose,
    {
        Self::IAppBarSeparatorFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IAppBarSeparatorFactory<
        R,
        F: FnOnce(&IAppBarSeparatorFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppBarSeparator, IAppBarSeparatorFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppBarSeparator {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppBarSeparator>();
}
unsafe impl windows_core::Interface for AppBarSeparator {
    type Vtable = <IAppBarSeparator as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppBarSeparator as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppBarSeparator {
    type Target = IAppBarSeparator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppBarSeparator {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.AppBarSeparator";
}
unsafe impl Send for AppBarSeparator {}
unsafe impl Sync for AppBarSeparator {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppBarToggleButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppBarToggleButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    AppBarToggleButton,
    ICommandBarElement,
    ToggleButton,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl AppBarToggleButton {
    pub fn new() -> windows_core::Result<AppBarToggleButton> {
        Self::IAppBarToggleButtonFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<AppBarToggleButton>
    where
        T: windows_core::Compose,
    {
        Self::IAppBarToggleButtonFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IAppBarToggleButtonFactory<
        R,
        F: FnOnce(&IAppBarToggleButtonFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            AppBarToggleButton,
            IAppBarToggleButtonFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppBarToggleButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppBarToggleButton>();
}
unsafe impl windows_core::Interface for AppBarToggleButton {
    type Vtable = <IAppBarToggleButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppBarToggleButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppBarToggleButton {
    type Target = IAppBarToggleButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppBarToggleButton {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.AppBarToggleButton";
}
unsafe impl Send for AppBarToggleButton {}
unsafe impl Sync for AppBarToggleButton {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppWindow(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppWindow,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl AppWindow {}
impl windows_core::RuntimeType for AppWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppWindow>();
}
unsafe impl windows_core::Interface for AppWindow {
    type Vtable = <IAppWindow as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppWindow as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppWindow {
    type Target = IAppWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppWindow {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindow";
}
unsafe impl Send for AppWindow {}
unsafe impl Sync for AppWindow {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppWindowPresenter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppWindowPresenter,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl AppWindowPresenter {}
impl windows_core::RuntimeType for AppWindowPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowPresenter>();
}
unsafe impl windows_core::Interface for AppWindowPresenter {
    type Vtable = <IAppWindowPresenter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppWindowPresenter as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppWindowPresenter {
    type Target = IAppWindowPresenter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppWindowPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowPresenter";
}
unsafe impl Send for AppWindowPresenter {}
unsafe impl Sync for AppWindowPresenter {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AppWindowPresenterKind(pub i32);
impl AppWindowPresenterKind {
    pub const Default: Self = Self(0i32);
    pub const CompactOverlay: Self = Self(1i32);
    pub const FullScreen: Self = Self(2i32);
    pub const Overlapped: Self = Self(3i32);
}
impl windows_core::TypeKind for AppWindowPresenterKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AppWindowPresenterKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.AppWindowPresenterKind;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppWindowTitleBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppWindowTitleBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl AppWindowTitleBar {}
impl windows_core::RuntimeType for AppWindowTitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowTitleBar>();
}
unsafe impl windows_core::Interface for AppWindowTitleBar {
    type Vtable = <IAppWindowTitleBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppWindowTitleBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppWindowTitleBar {
    type Target = IAppWindowTitleBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppWindowTitleBar {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowTitleBar";
}
unsafe impl Send for AppWindowTitleBar {}
unsafe impl Sync for AppWindowTitleBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Application(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Application,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Application {
    pub fn new() -> windows_core::Result<Application> {
        Self::IApplicationFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Application>
    where
        T: windows_core::Compose,
    {
        Self::IApplicationFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    pub fn get_Current() -> windows_core::Result<Application> {
        Self::IApplicationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_Current)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Start<P0>(callback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ApplicationInitializationCallback>,
    {
        Self::IApplicationStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).Start)(
                windows_core::Interface::as_raw(this),
                callback.param().abi(),
            )
            .ok()
        })
    }
    fn IApplicationFactory<R, F: FnOnce(&IApplicationFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Application, IApplicationFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IApplicationStatics<R, F: FnOnce(&IApplicationStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Application, IApplicationStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Application {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IApplication>();
}
unsafe impl windows_core::Interface for Application {
    type Vtable = <IApplication as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IApplication as windows_core::Interface>::IID;
}
impl core::ops::Deref for Application {
    type Target = IApplication;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Application {
    const NAME: &'static str = "Microsoft.UI.Xaml.Application";
}
unsafe impl Send for Application {}
unsafe impl Sync for Application {}
windows_core::imp::define_interface!(
    ApplicationInitializationCallback,
    ApplicationInitializationCallback_Vtbl,
    0xd8eef1c9_1234_56f1_9963_45dd9c80a661
);
impl windows_core::RuntimeType for ApplicationInitializationCallback {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ApplicationInitializationCallback {
    pub fn new<F: Fn(windows_core::Ref<ApplicationInitializationCallbackParams>) + 'static>(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<ApplicationInitializationCallback, F>::new(
            &ApplicationInitializationCallbackBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ApplicationInitializationCallback_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        p: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct ApplicationInitializationCallbackBox<
    F: Fn(windows_core::Ref<ApplicationInitializationCallbackParams>) + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<ApplicationInitializationCallbackParams>) + 'static>
    ApplicationInitializationCallbackBox<F>
{
    const VTABLE: ApplicationInitializationCallback_Vtbl = ApplicationInitializationCallback_Vtbl {
        base__:
            windows_core::IUnknown_Vtbl {
                QueryInterface: windows_core::imp::DelegateBox::<
                    ApplicationInitializationCallback,
                    F,
                >::QueryInterface,
                AddRef:
                    windows_core::imp::DelegateBox::<ApplicationInitializationCallback, F>::AddRef,
                Release:
                    windows_core::imp::DelegateBox::<ApplicationInitializationCallback, F>::Release,
            },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        p: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<ApplicationInitializationCallback, F>);
            (this.invoke)(core::mem::transmute_copy(&p));
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationInitializationCallbackParams(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ApplicationInitializationCallbackParams,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ApplicationInitializationCallbackParams {}
impl windows_core::RuntimeType for ApplicationInitializationCallbackParams {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        IApplicationInitializationCallbackParams,
    >();
}
unsafe impl windows_core::Interface for ApplicationInitializationCallbackParams {
    type Vtable = <IApplicationInitializationCallbackParams as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IApplicationInitializationCallbackParams as windows_core::Interface>::IID;
}
impl core::ops::Deref for ApplicationInitializationCallbackParams {
    type Target = IApplicationInitializationCallbackParams;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ApplicationInitializationCallbackParams {
    const NAME: &'static str = "Microsoft.UI.Xaml.ApplicationInitializationCallbackParams";
}
unsafe impl Send for ApplicationInitializationCallbackParams {}
unsafe impl Sync for ApplicationInitializationCallbackParams {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoSuggestBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AutoSuggestBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    AutoSuggestBox,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl AutoSuggestBox {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            AutoSuggestBox,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AutoSuggestBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAutoSuggestBox>();
}
unsafe impl windows_core::Interface for AutoSuggestBox {
    type Vtable = <IAutoSuggestBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAutoSuggestBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for AutoSuggestBox {
    type Target = IAutoSuggestBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AutoSuggestBox {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.AutoSuggestBox";
}
unsafe impl Send for AutoSuggestBox {}
unsafe impl Sync for AutoSuggestBox {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoSuggestBoxQuerySubmittedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AutoSuggestBoxQuerySubmittedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(AutoSuggestBoxQuerySubmittedEventArgs, DependencyObject);
impl AutoSuggestBoxQuerySubmittedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            AutoSuggestBoxQuerySubmittedEventArgs,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AutoSuggestBoxQuerySubmittedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAutoSuggestBoxQuerySubmittedEventArgs>();
}
unsafe impl windows_core::Interface for AutoSuggestBoxQuerySubmittedEventArgs {
    type Vtable = <IAutoSuggestBoxQuerySubmittedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IAutoSuggestBoxQuerySubmittedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for AutoSuggestBoxQuerySubmittedEventArgs {
    type Target = IAutoSuggestBoxQuerySubmittedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AutoSuggestBoxQuerySubmittedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.AutoSuggestBoxQuerySubmittedEventArgs";
}
unsafe impl Send for AutoSuggestBoxQuerySubmittedEventArgs {}
unsafe impl Sync for AutoSuggestBoxQuerySubmittedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoSuggestBoxSuggestionChosenEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AutoSuggestBoxSuggestionChosenEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(AutoSuggestBoxSuggestionChosenEventArgs, DependencyObject);
impl AutoSuggestBoxSuggestionChosenEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            AutoSuggestBoxSuggestionChosenEventArgs,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AutoSuggestBoxSuggestionChosenEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        IAutoSuggestBoxSuggestionChosenEventArgs,
    >();
}
unsafe impl windows_core::Interface for AutoSuggestBoxSuggestionChosenEventArgs {
    type Vtable = <IAutoSuggestBoxSuggestionChosenEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IAutoSuggestBoxSuggestionChosenEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for AutoSuggestBoxSuggestionChosenEventArgs {
    type Target = IAutoSuggestBoxSuggestionChosenEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AutoSuggestBoxSuggestionChosenEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.AutoSuggestBoxSuggestionChosenEventArgs";
}
unsafe impl Send for AutoSuggestBoxSuggestionChosenEventArgs {}
unsafe impl Sync for AutoSuggestBoxSuggestionChosenEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoSuggestBoxTextChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AutoSuggestBoxTextChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(AutoSuggestBoxTextChangedEventArgs, DependencyObject);
impl AutoSuggestBoxTextChangedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            AutoSuggestBoxTextChangedEventArgs,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AutoSuggestBoxTextChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAutoSuggestBoxTextChangedEventArgs>();
}
unsafe impl windows_core::Interface for AutoSuggestBoxTextChangedEventArgs {
    type Vtable = <IAutoSuggestBoxTextChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IAutoSuggestBoxTextChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for AutoSuggestBoxTextChangedEventArgs {
    type Target = IAutoSuggestBoxTextChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AutoSuggestBoxTextChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.AutoSuggestBoxTextChangedEventArgs";
}
unsafe impl Send for AutoSuggestBoxTextChangedEventArgs {}
unsafe impl Sync for AutoSuggestBoxTextChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AutoSuggestionBoxTextChangeReason(pub i32);
impl AutoSuggestionBoxTextChangeReason {
    pub const UserInput: Self = Self(0i32);
    pub const ProgrammaticChange: Self = Self(1i32);
    pub const SuggestionChosen: Self = Self(2i32);
}
impl windows_core::TypeKind for AutoSuggestionBoxTextChangeReason {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AutoSuggestionBoxTextChangeReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.AutoSuggestionBoxTextChangeReason;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AutomationHeadingLevel(pub i32);
impl AutomationHeadingLevel {
    pub const None: Self = Self(0i32);
    pub const Level1: Self = Self(1i32);
    pub const Level2: Self = Self(2i32);
    pub const Level3: Self = Self(3i32);
    pub const Level4: Self = Self(4i32);
    pub const Level5: Self = Self(5i32);
    pub const Level6: Self = Self(6i32);
    pub const Level7: Self = Self(7i32);
    pub const Level8: Self = Self(8i32);
    pub const Level9: Self = Self(9i32);
}
impl windows_core::TypeKind for AutomationHeadingLevel {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AutomationHeadingLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.Peers.AutomationHeadingLevel;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AutomationLiveSetting(pub i32);
impl AutomationLiveSetting {
    pub const Off: Self = Self(0i32);
    pub const Polite: Self = Self(1i32);
    pub const Assertive: Self = Self(2i32);
}
impl windows_core::TypeKind for AutomationLiveSetting {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AutomationLiveSetting {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.Peers.AutomationLiveSetting;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutomationPeer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AutomationPeer,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(AutomationPeer, DependencyObject);
impl AutomationPeer {}
impl windows_core::RuntimeType for AutomationPeer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAutomationPeer>();
}
unsafe impl windows_core::Interface for AutomationPeer {
    type Vtable = <IAutomationPeer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAutomationPeer as windows_core::Interface>::IID;
}
impl core::ops::Deref for AutomationPeer {
    type Target = IAutomationPeer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AutomationPeer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.Peers.AutomationPeer";
}
unsafe impl Send for AutomationPeer {}
unsafe impl Sync for AutomationPeer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutomationProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AutomationProperties,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl AutomationProperties {
    pub fn SetAutomationId<P0>(element: P0, value: &str) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAutomationId)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        })
    }
    pub fn SetHelpText<P0>(element: P0, value: &str) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetHelpText)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        })
    }
    pub fn SetName<P0>(element: P0, value: &str) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetName)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        })
    }
    pub fn SetLiveSetting<P0>(element: P0, value: AutomationLiveSetting) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetLiveSetting)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetHeadingLevel<P0>(
        element: P0,
        value: AutomationHeadingLevel,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetHeadingLevel)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    fn IAutomationPropertiesStatics<
        R,
        F: FnOnce(&IAutomationPropertiesStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            AutomationProperties,
            IAutomationPropertiesStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AutomationProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAutomationProperties>();
}
unsafe impl windows_core::Interface for AutomationProperties {
    type Vtable = <IAutomationProperties as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAutomationProperties as windows_core::Interface>::IID;
}
impl core::ops::Deref for AutomationProperties {
    type Target = IAutomationProperties;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AutomationProperties {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationProperties";
}
unsafe impl Send for AutomationProperties {}
unsafe impl Sync for AutomationProperties {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapImage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BitmapImage,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(BitmapImage, BitmapSource, ImageSource, DependencyObject);
impl BitmapImage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            BitmapImage,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BitmapImage {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBitmapImage>();
}
unsafe impl windows_core::Interface for BitmapImage {
    type Vtable = <IBitmapImage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapImage as windows_core::Interface>::IID;
}
impl core::ops::Deref for BitmapImage {
    type Target = IBitmapImage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BitmapImage {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.BitmapImage";
}
unsafe impl Send for BitmapImage {}
unsafe impl Sync for BitmapImage {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BitmapSource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(BitmapSource, ImageSource, DependencyObject);
impl BitmapSource {}
impl windows_core::RuntimeType for BitmapSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBitmapSource>();
}
unsafe impl windows_core::Interface for BitmapSource {
    type Vtable = <IBitmapSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapSource as windows_core::Interface>::IID;
}
impl core::ops::Deref for BitmapSource {
    type Target = IBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BitmapSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.BitmapSource";
}
unsafe impl Send for BitmapSource {}
unsafe impl Sync for BitmapSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Block, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Block, TextElement, DependencyObject);
impl Block {}
impl windows_core::RuntimeType for Block {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBlock>();
}
unsafe impl windows_core::Interface for Block {
    type Vtable = <IBlock as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBlock as windows_core::Interface>::IID;
}
impl core::ops::Deref for Block {
    type Target = IBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Block {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Block";
}
unsafe impl Send for Block {}
unsafe impl Sync for Block {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BlockCollection,
    windows_core::IUnknown,
    windows_core::IInspectable,
    windows_collections::IVector<Block>
);
impl BlockCollection {}
impl windows_core::RuntimeType for BlockCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVector<Block>>();
}
unsafe impl windows_core::Interface for BlockCollection {
    type Vtable = <windows_collections::IVector<Block> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <windows_collections::IVector<Block> as windows_core::Interface>::IID;
}
impl core::ops::Deref for BlockCollection {
    type Target = windows_collections::IVector<Block>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BlockCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.BlockCollection";
}
unsafe impl Send for BlockCollection {}
unsafe impl Sync for BlockCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Border(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Border, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Border, FrameworkElement, UIElement, DependencyObject);
impl Border {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Border, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Border {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBorder>();
}
unsafe impl windows_core::Interface for Border {
    type Vtable = <IBorder as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBorder as windows_core::Interface>::IID;
}
impl core::ops::Deref for Border {
    type Target = IBorder;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Border {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Border";
}
unsafe impl Send for Border {}
unsafe impl Sync for Border {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BreadcrumbBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BreadcrumbBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    BreadcrumbBar,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl BreadcrumbBar {
    pub fn new() -> windows_core::Result<BreadcrumbBar> {
        Self::IBreadcrumbBarFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<BreadcrumbBar>
    where
        T: windows_core::Compose,
    {
        Self::IBreadcrumbBarFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IBreadcrumbBarFactory<R, F: FnOnce(&IBreadcrumbBarFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BreadcrumbBar, IBreadcrumbBarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BreadcrumbBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBreadcrumbBar>();
}
unsafe impl windows_core::Interface for BreadcrumbBar {
    type Vtable = <IBreadcrumbBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBreadcrumbBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for BreadcrumbBar {
    type Target = IBreadcrumbBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BreadcrumbBar {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.BreadcrumbBar";
}
unsafe impl Send for BreadcrumbBar {}
unsafe impl Sync for BreadcrumbBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BreadcrumbBarItemClickedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BreadcrumbBarItemClickedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl BreadcrumbBarItemClickedEventArgs {}
impl windows_core::RuntimeType for BreadcrumbBarItemClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBreadcrumbBarItemClickedEventArgs>();
}
unsafe impl windows_core::Interface for BreadcrumbBarItemClickedEventArgs {
    type Vtable = <IBreadcrumbBarItemClickedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IBreadcrumbBarItemClickedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for BreadcrumbBarItemClickedEventArgs {
    type Target = IBreadcrumbBarItemClickedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BreadcrumbBarItemClickedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.BreadcrumbBarItemClickedEventArgs";
}
unsafe impl Send for BreadcrumbBarItemClickedEventArgs {}
unsafe impl Sync for BreadcrumbBarItemClickedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Brush(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Brush, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Brush, DependencyObject);
impl Brush {}
impl windows_core::RuntimeType for Brush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBrush>();
}
unsafe impl windows_core::Interface for Brush {
    type Vtable = <IBrush as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBrush as windows_core::Interface>::IID;
}
impl core::ops::Deref for Brush {
    type Target = IBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Brush {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Brush";
}
unsafe impl Send for Brush {}
unsafe impl Sync for Brush {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Button(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Button, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(
    Button,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Button {
    pub fn new() -> windows_core::Result<Button> {
        Self::IButtonFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Button>
    where
        T: windows_core::Compose,
    {
        Self::IButtonFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IButtonFactory<R, F: FnOnce(&IButtonFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Button, IButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Button {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IButton>();
}
unsafe impl windows_core::Interface for Button {
    type Vtable = <IButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for Button {
    type Target = IButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Button {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Button";
}
unsafe impl Send for Button {}
unsafe impl Sync for Button {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ButtonBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ButtonBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ButtonBase {}
impl windows_core::RuntimeType for ButtonBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IButtonBase>();
}
unsafe impl windows_core::Interface for ButtonBase {
    type Vtable = <IButtonBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IButtonBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for ButtonBase {
    type Target = IButtonBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ButtonBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ButtonBase";
}
unsafe impl Send for ButtonBase {}
unsafe impl Sync for ButtonBase {}
pub type COINIT = i32;
pub const COINIT_APARTMENTTHREADED: COINIT = 2i32;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarDatePicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CalendarDatePicker,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CalendarDatePicker,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl CalendarDatePicker {
    pub fn new() -> windows_core::Result<CalendarDatePicker> {
        Self::ICalendarDatePickerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<CalendarDatePicker>
    where
        T: windows_core::Compose,
    {
        Self::ICalendarDatePickerFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ICalendarDatePickerFactory<
        R,
        F: FnOnce(&ICalendarDatePickerFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            CalendarDatePicker,
            ICalendarDatePickerFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CalendarDatePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICalendarDatePicker>();
}
unsafe impl windows_core::Interface for CalendarDatePicker {
    type Vtable = <ICalendarDatePicker as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICalendarDatePicker as windows_core::Interface>::IID;
}
impl core::ops::Deref for CalendarDatePicker {
    type Target = ICalendarDatePicker;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CalendarDatePicker {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.CalendarDatePicker";
}
unsafe impl Send for CalendarDatePicker {}
unsafe impl Sync for CalendarDatePicker {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarDatePickerDateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CalendarDatePickerDateChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl CalendarDatePickerDateChangedEventArgs {}
impl windows_core::RuntimeType for CalendarDatePickerDateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICalendarDatePickerDateChangedEventArgs>(
        );
}
unsafe impl windows_core::Interface for CalendarDatePickerDateChangedEventArgs {
    type Vtable = <ICalendarDatePickerDateChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ICalendarDatePickerDateChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for CalendarDatePickerDateChangedEventArgs {
    type Target = ICalendarDatePickerDateChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CalendarDatePickerDateChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.CalendarDatePickerDateChangedEventArgs";
}
unsafe impl Send for CalendarDatePickerDateChangedEventArgs {}
unsafe impl Sync for CalendarDatePickerDateChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CalendarView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CalendarView,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl CalendarView {
    pub fn new() -> windows_core::Result<CalendarView> {
        Self::ICalendarViewFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<CalendarView>
    where
        T: windows_core::Compose,
    {
        Self::ICalendarViewFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ICalendarViewFactory<R, F: FnOnce(&ICalendarViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CalendarView, ICalendarViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CalendarView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICalendarView>();
}
unsafe impl windows_core::Interface for CalendarView {
    type Vtable = <ICalendarView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICalendarView as windows_core::Interface>::IID;
}
impl core::ops::Deref for CalendarView {
    type Target = ICalendarView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CalendarView {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.CalendarView";
}
unsafe impl Send for CalendarView {}
unsafe impl Sync for CalendarView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarViewSelectedDatesChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CalendarViewSelectedDatesChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl CalendarViewSelectedDatesChangedEventArgs {}
impl windows_core::RuntimeType for CalendarViewSelectedDatesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        ICalendarViewSelectedDatesChangedEventArgs,
    >();
}
unsafe impl windows_core::Interface for CalendarViewSelectedDatesChangedEventArgs {
    type Vtable = <ICalendarViewSelectedDatesChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ICalendarViewSelectedDatesChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for CalendarViewSelectedDatesChangedEventArgs {
    type Target = ICalendarViewSelectedDatesChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CalendarViewSelectedDatesChangedEventArgs {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.CalendarViewSelectedDatesChangedEventArgs";
}
unsafe impl Send for CalendarViewSelectedDatesChangedEventArgs {}
unsafe impl Sync for CalendarViewSelectedDatesChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Canvas(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Canvas, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(
    Canvas,
    Panel,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Canvas {
    pub fn new() -> windows_core::Result<Canvas> {
        Self::ICanvasFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Canvas>
    where
        T: windows_core::Compose,
    {
        Self::ICanvasFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    pub fn GetLeft<P0>(element: P0) -> windows_core::Result<f64>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::ICanvasStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetLeft)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn SetLeft<P0>(element: P0, length: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::ICanvasStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetLeft)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                length,
            )
            .ok()
        })
    }
    pub fn GetTop<P0>(element: P0) -> windows_core::Result<f64>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::ICanvasStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTop)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn SetTop<P0>(element: P0, length: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::ICanvasStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetTop)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                length,
            )
            .ok()
        })
    }
    pub fn SetZIndex<P0>(element: P0, value: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::ICanvasStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetZIndex)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    fn ICanvasFactory<R, F: FnOnce(&ICanvasFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Canvas, ICanvasFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ICanvasStatics<R, F: FnOnce(&ICanvasStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Canvas, ICanvasStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Canvas {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICanvas>();
}
unsafe impl windows_core::Interface for Canvas {
    type Vtable = <ICanvas as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICanvas as windows_core::Interface>::IID;
}
impl core::ops::Deref for Canvas {
    type Target = ICanvas;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Canvas {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Canvas";
}
unsafe impl Send for Canvas {}
unsafe impl Sync for Canvas {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CheckBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CheckBox,
    ToggleButton,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl CheckBox {
    pub fn new() -> windows_core::Result<CheckBox> {
        Self::ICheckBoxFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<CheckBox>
    where
        T: windows_core::Compose,
    {
        Self::ICheckBoxFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ICheckBoxFactory<R, F: FnOnce(&ICheckBoxFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CheckBox, ICheckBoxFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CheckBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICheckBox>();
}
unsafe impl windows_core::Interface for CheckBox {
    type Vtable = <ICheckBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICheckBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for CheckBox {
    type Target = ICheckBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CheckBox {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.CheckBox";
}
unsafe impl Send for CheckBox {}
unsafe impl Sync for CheckBox {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub A: u8,
    pub R: u8,
    pub G: u8,
    pub B: u8,
}
impl windows_core::TypeKind for Color {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Color {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ColorChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ColorChangedEventArgs {}
impl windows_core::RuntimeType for ColorChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IColorChangedEventArgs>();
}
unsafe impl windows_core::Interface for ColorChangedEventArgs {
    type Vtable = <IColorChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IColorChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ColorChangedEventArgs {
    type Target = IColorChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ColorChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ColorChangedEventArgs";
}
unsafe impl Send for ColorChangedEventArgs {}
unsafe impl Sync for ColorChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorPicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ColorPicker,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ColorPicker,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ColorPicker {
    pub fn new() -> windows_core::Result<ColorPicker> {
        Self::IColorPickerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<ColorPicker>
    where
        T: windows_core::Compose,
    {
        Self::IColorPickerFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IColorPickerFactory<R, F: FnOnce(&IColorPickerFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ColorPicker, IColorPickerFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ColorPicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IColorPicker>();
}
unsafe impl windows_core::Interface for ColorPicker {
    type Vtable = <IColorPicker as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IColorPicker as windows_core::Interface>::IID;
}
impl core::ops::Deref for ColorPicker {
    type Target = IColorPicker;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ColorPicker {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ColorPicker";
}
unsafe impl Send for ColorPicker {}
unsafe impl Sync for ColorPicker {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ColumnDefinition,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ColumnDefinition, DependencyObject);
impl ColumnDefinition {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            ColumnDefinition,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ColumnDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IColumnDefinition>();
}
unsafe impl windows_core::Interface for ColumnDefinition {
    type Vtable = <IColumnDefinition as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IColumnDefinition as windows_core::Interface>::IID;
}
impl core::ops::Deref for ColumnDefinition {
    type Target = IColumnDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ColumnDefinition {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ColumnDefinition";
}
unsafe impl Send for ColumnDefinition {}
unsafe impl Sync for ColumnDefinition {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnDefinitionCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ColumnDefinitionCollection,
    windows_core::IUnknown,
    windows_core::IInspectable,
    windows_collections::IVector<ColumnDefinition>
);
impl ColumnDefinitionCollection {}
impl windows_core::RuntimeType for ColumnDefinitionCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        windows_collections::IVector<ColumnDefinition>,
    >();
}
unsafe impl windows_core::Interface for ColumnDefinitionCollection {
    type Vtable =
        <windows_collections::IVector<ColumnDefinition> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <windows_collections::IVector<ColumnDefinition> as windows_core::Interface>::IID;
}
impl core::ops::Deref for ColumnDefinitionCollection {
    type Target = windows_collections::IVector<ColumnDefinition>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ColumnDefinitionCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ColumnDefinitionCollection";
}
unsafe impl Send for ColumnDefinitionCollection {}
unsafe impl Sync for ColumnDefinitionCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComboBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ComboBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ComboBox,
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ComboBox {
    pub fn new() -> windows_core::Result<ComboBox> {
        Self::IComboBoxFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<ComboBox>
    where
        T: windows_core::Compose,
    {
        Self::IComboBoxFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IComboBoxFactory<R, F: FnOnce(&IComboBoxFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ComboBox, IComboBoxFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ComboBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IComboBox>();
}
unsafe impl windows_core::Interface for ComboBox {
    type Vtable = <IComboBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IComboBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for ComboBox {
    type Target = IComboBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ComboBox {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ComboBox";
}
unsafe impl Send for ComboBox {}
unsafe impl Sync for ComboBox {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CommandBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CommandBar,
    AppBar,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl CommandBar {
    pub fn new() -> windows_core::Result<CommandBar> {
        Self::ICommandBarFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<CommandBar>
    where
        T: windows_core::Compose,
    {
        Self::ICommandBarFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ICommandBarFactory<R, F: FnOnce(&ICommandBarFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CommandBar, ICommandBarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CommandBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICommandBar>();
}
unsafe impl windows_core::Interface for CommandBar {
    type Vtable = <ICommandBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICommandBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for CommandBar {
    type Target = ICommandBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CommandBar {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.CommandBar";
}
unsafe impl Send for CommandBar {}
unsafe impl Sync for CommandBar {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CommandBarDefaultLabelPosition(pub i32);
impl CommandBarDefaultLabelPosition {
    pub const Bottom: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
    pub const Collapsed: Self = Self(2i32);
}
impl windows_core::TypeKind for CommandBarDefaultLabelPosition {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CommandBarDefaultLabelPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.CommandBarDefaultLabelPosition;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandBarFlyout(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CommandBarFlyout,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CommandBarFlyout, FlyoutBase, DependencyObject);
impl CommandBarFlyout {
    pub fn new() -> windows_core::Result<CommandBarFlyout> {
        Self::ICommandBarFlyoutFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<CommandBarFlyout>
    where
        T: windows_core::Compose,
    {
        Self::ICommandBarFlyoutFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ICommandBarFlyoutFactory<
        R,
        F: FnOnce(&ICommandBarFlyoutFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CommandBarFlyout, ICommandBarFlyoutFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CommandBarFlyout {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICommandBarFlyout>();
}
unsafe impl windows_core::Interface for CommandBarFlyout {
    type Vtable = <ICommandBarFlyout as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICommandBarFlyout as windows_core::Interface>::IID;
}
impl core::ops::Deref for CommandBarFlyout {
    type Target = ICommandBarFlyout;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CommandBarFlyout {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.CommandBarFlyout";
}
unsafe impl Send for CommandBarFlyout {}
unsafe impl Sync for CommandBarFlyout {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionAnimation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionAnimation,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CompositionAnimation,
    ICompositionAnimationBase,
    CompositionObject
);
impl CompositionAnimation {}
impl windows_core::RuntimeType for CompositionAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionAnimation>();
}
unsafe impl windows_core::Interface for CompositionAnimation {
    type Vtable = <ICompositionAnimation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionAnimation as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionAnimation {
    type Target = ICompositionAnimation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionAnimation {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionAnimation";
}
unsafe impl Send for CompositionAnimation {}
unsafe impl Sync for CompositionAnimation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionEasingFunction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionEasingFunction,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CompositionEasingFunction, CompositionObject);
impl CompositionEasingFunction {}
impl windows_core::RuntimeType for CompositionEasingFunction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionEasingFunction>();
}
unsafe impl windows_core::Interface for CompositionEasingFunction {
    type Vtable = <ICompositionEasingFunction as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionEasingFunction as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionEasingFunction {
    type Target = ICompositionEasingFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionEasingFunction {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionEasingFunction";
}
unsafe impl Send for CompositionEasingFunction {}
unsafe impl Sync for CompositionEasingFunction {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionObject,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl CompositionObject {}
impl windows_core::RuntimeType for CompositionObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionObject>();
}
unsafe impl windows_core::Interface for CompositionObject {
    type Vtable = <ICompositionObject as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionObject as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionObject {
    type Target = ICompositionObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionObject {
    const NAME: &'static str = "Microsoft.UI.Composition.CompositionObject";
}
unsafe impl Send for CompositionObject {}
unsafe impl Sync for CompositionObject {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionTarget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionTarget,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl CompositionTarget {
    pub fn add_Rendering<F>(handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<windows_core::IInspectable>,
            ) + 'static,
    {
        let handler: EventHandler<windows_core::IInspectable> = {
            let com =
                windows_core::imp::DelegateBox::<EventHandler<windows_core::IInspectable>, F>::new(
                    &EventHandlerBox::<windows_core::IInspectable, F>::VTABLE,
                    handler,
                );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        Self::ICompositionTargetStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).add_Rendering)(
                windows_core::Interface::as_raw(this),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                this.clone(),
                token__,
                windows_core::Interface::vtable(this).remove_Rendering,
            ))
        })
    }
    fn ICompositionTargetStatics<
        R,
        F: FnOnce(&ICompositionTargetStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            CompositionTarget,
            ICompositionTargetStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CompositionTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionTarget>();
}
unsafe impl windows_core::Interface for CompositionTarget {
    type Vtable = <ICompositionTarget as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionTarget as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompositionTarget {
    type Target = ICompositionTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompositionTarget {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.CompositionTarget";
}
unsafe impl Send for CompositionTarget {}
unsafe impl Sync for CompositionTarget {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Compositor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Compositor,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Compositor {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            Compositor,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Compositor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositor>();
}
unsafe impl windows_core::Interface for Compositor {
    type Vtable = <ICompositor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositor as windows_core::Interface>::IID;
}
impl core::ops::Deref for Compositor {
    type Target = ICompositor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Compositor {
    const NAME: &'static str = "Microsoft.UI.Composition.Compositor";
}
unsafe impl Send for Compositor {}
unsafe impl Sync for Compositor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ContentControl,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ContentControl {}
impl windows_core::RuntimeType for ContentControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IContentControl>();
}
unsafe impl windows_core::Interface for ContentControl {
    type Vtable = <IContentControl as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContentControl as windows_core::Interface>::IID;
}
impl core::ops::Deref for ContentControl {
    type Target = IContentControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ContentControl {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ContentControl";
}
unsafe impl Send for ContentControl {}
unsafe impl Sync for ContentControl {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentDialog(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ContentDialog,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ContentDialog,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ContentDialog {
    pub fn new() -> windows_core::Result<ContentDialog> {
        Self::IContentDialogFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<ContentDialog>
    where
        T: windows_core::Compose,
    {
        Self::IContentDialogFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IContentDialogFactory<R, F: FnOnce(&IContentDialogFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContentDialog, IContentDialogFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ContentDialog {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IContentDialog>();
}
unsafe impl windows_core::Interface for ContentDialog {
    type Vtable = <IContentDialog as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContentDialog as windows_core::Interface>::IID;
}
impl core::ops::Deref for ContentDialog {
    type Target = IContentDialog;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ContentDialog {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ContentDialog";
}
unsafe impl Send for ContentDialog {}
unsafe impl Sync for ContentDialog {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentDialogClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ContentDialogClosedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ContentDialogClosedEventArgs {}
impl windows_core::RuntimeType for ContentDialogClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IContentDialogClosedEventArgs>();
}
unsafe impl windows_core::Interface for ContentDialogClosedEventArgs {
    type Vtable = <IContentDialogClosedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContentDialogClosedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ContentDialogClosedEventArgs {
    type Target = IContentDialogClosedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ContentDialogClosedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ContentDialogClosedEventArgs";
}
unsafe impl Send for ContentDialogClosedEventArgs {}
unsafe impl Sync for ContentDialogClosedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ContentDialogPlacement(pub i32);
impl ContentDialogPlacement {
    pub const Popup: Self = Self(0i32);
    pub const InPlace: Self = Self(1i32);
}
impl windows_core::TypeKind for ContentDialogPlacement {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ContentDialogPlacement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.ContentDialogPlacement;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ContentDialogResult(pub i32);
impl ContentDialogResult {
    pub const None: Self = Self(0i32);
    pub const Primary: Self = Self(1i32);
    pub const Secondary: Self = Self(2i32);
}
impl windows_core::TypeKind for ContentDialogResult {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ContentDialogResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.ContentDialogResult;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Control(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Control,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(Control, FrameworkElement, UIElement, DependencyObject);
impl Control {
    pub fn new() -> windows_core::Result<Control> {
        Self::IControlFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Control>
    where
        T: windows_core::Compose,
    {
        Self::IControlFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    pub fn get_FontSizeProperty() -> windows_core::Result<DependencyProperty> {
        Self::IControlStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_FontSizeProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_FontFamilyProperty() -> windows_core::Result<DependencyProperty> {
        Self::IControlStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_FontFamilyProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_FontWeightProperty() -> windows_core::Result<DependencyProperty> {
        Self::IControlStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_FontWeightProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_ForegroundProperty() -> windows_core::Result<DependencyProperty> {
        Self::IControlStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_ForegroundProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_IsEnabledProperty() -> windows_core::Result<DependencyProperty> {
        Self::IControlStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_IsEnabledProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_PaddingProperty() -> windows_core::Result<DependencyProperty> {
        Self::IControlStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_PaddingProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_BackgroundProperty() -> windows_core::Result<DependencyProperty> {
        Self::IControlStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_BackgroundProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IControlFactory<R, F: FnOnce(&IControlFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Control, IControlFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IControlStatics<R, F: FnOnce(&IControlStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Control, IControlStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Control {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IControl>();
}
unsafe impl windows_core::Interface for Control {
    type Vtable = <IControl as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IControl as windows_core::Interface>::IID;
}
impl core::ops::Deref for Control {
    type Target = IControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Control {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Control";
}
unsafe impl Send for Control {}
unsafe impl Sync for Control {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CornerRadius {
    pub TopLeft: f64,
    pub TopRight: f64,
    pub BottomRight: f64,
    pub BottomLeft: f64,
}
impl windows_core::TypeKind for CornerRadius {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CornerRadius {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.CornerRadius;f8;f8;f8;f8)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CubicBezierEasingFunction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CubicBezierEasingFunction,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    CubicBezierEasingFunction,
    CompositionEasingFunction,
    CompositionObject
);
impl CubicBezierEasingFunction {}
impl windows_core::RuntimeType for CubicBezierEasingFunction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICubicBezierEasingFunction>();
}
unsafe impl windows_core::Interface for CubicBezierEasingFunction {
    type Vtable = <ICubicBezierEasingFunction as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICubicBezierEasingFunction as windows_core::Interface>::IID;
}
impl core::ops::Deref for CubicBezierEasingFunction {
    type Target = ICubicBezierEasingFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CubicBezierEasingFunction {
    const NAME: &'static str = "Microsoft.UI.Composition.CubicBezierEasingFunction";
}
unsafe impl Send for CubicBezierEasingFunction {}
unsafe impl Sync for CubicBezierEasingFunction {}
pub type DPI_AWARENESS_CONTEXT = *mut core::ffi::c_void;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = -4i32 as _;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataTemplate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DataTemplate,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(DataTemplate, FrameworkTemplate, DependencyObject);
impl DataTemplate {
    pub fn new() -> windows_core::Result<DataTemplate> {
        Self::IDataTemplateFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<DataTemplate>
    where
        T: windows_core::Compose,
    {
        Self::IDataTemplateFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IDataTemplateFactory<R, F: FnOnce(&IDataTemplateFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataTemplate, IDataTemplateFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DataTemplate {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDataTemplate>();
}
unsafe impl windows_core::Interface for DataTemplate {
    type Vtable = <IDataTemplate as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDataTemplate as windows_core::Interface>::IID;
}
impl core::ops::Deref for DataTemplate {
    type Target = IDataTemplate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DataTemplate {
    const NAME: &'static str = "Microsoft.UI.Xaml.DataTemplate";
}
unsafe impl Send for DataTemplate {}
unsafe impl Sync for DataTemplate {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatePicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DatePicker,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    DatePicker,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl DatePicker {
    pub fn new() -> windows_core::Result<DatePicker> {
        Self::IDatePickerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<DatePicker>
    where
        T: windows_core::Compose,
    {
        Self::IDatePickerFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IDatePickerFactory<R, F: FnOnce(&IDatePickerFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DatePicker, IDatePickerFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DatePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDatePicker>();
}
unsafe impl windows_core::Interface for DatePicker {
    type Vtable = <IDatePicker as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDatePicker as windows_core::Interface>::IID;
}
impl core::ops::Deref for DatePicker {
    type Target = IDatePicker;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DatePicker {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.DatePicker";
}
unsafe impl Send for DatePicker {}
unsafe impl Sync for DatePicker {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatePickerSelectedValueChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DatePickerSelectedValueChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DatePickerSelectedValueChangedEventArgs {}
impl windows_core::RuntimeType for DatePickerSelectedValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        IDatePickerSelectedValueChangedEventArgs,
    >();
}
unsafe impl windows_core::Interface for DatePickerSelectedValueChangedEventArgs {
    type Vtable = <IDatePickerSelectedValueChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IDatePickerSelectedValueChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for DatePickerSelectedValueChangedEventArgs {
    type Target = IDatePickerSelectedValueChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DatePickerSelectedValueChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.DatePickerSelectedValueChangedEventArgs";
}
unsafe impl Send for DatePickerSelectedValueChangedEventArgs {}
unsafe impl Sync for DatePickerSelectedValueChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DependencyObject,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DependencyObject {}
impl windows_core::RuntimeType for DependencyObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDependencyObject>();
}
unsafe impl windows_core::Interface for DependencyObject {
    type Vtable = <IDependencyObject as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDependencyObject as windows_core::Interface>::IID;
}
impl core::ops::Deref for DependencyObject {
    type Target = IDependencyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DependencyObject {
    const NAME: &'static str = "Microsoft.UI.Xaml.DependencyObject";
}
unsafe impl Send for DependencyObject {}
unsafe impl Sync for DependencyObject {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyProperty(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DependencyProperty,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DependencyProperty {}
impl windows_core::RuntimeType for DependencyProperty {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDependencyProperty>();
}
unsafe impl windows_core::Interface for DependencyProperty {
    type Vtable = <IDependencyProperty as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDependencyProperty as windows_core::Interface>::IID;
}
impl core::ops::Deref for DependencyProperty {
    type Target = IDependencyProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DependencyProperty {
    const NAME: &'static str = "Microsoft.UI.Xaml.DependencyProperty";
}
unsafe impl Send for DependencyProperty {}
unsafe impl Sync for DependencyProperty {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DesktopAcrylicBackdrop(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DesktopAcrylicBackdrop,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(DesktopAcrylicBackdrop, SystemBackdrop, DependencyObject);
impl DesktopAcrylicBackdrop {
    pub fn new() -> windows_core::Result<DesktopAcrylicBackdrop> {
        Self::IDesktopAcrylicBackdropFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<DesktopAcrylicBackdrop>
    where
        T: windows_core::Compose,
    {
        Self::IDesktopAcrylicBackdropFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IDesktopAcrylicBackdropFactory<
        R,
        F: FnOnce(&IDesktopAcrylicBackdropFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            DesktopAcrylicBackdrop,
            IDesktopAcrylicBackdropFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DesktopAcrylicBackdrop {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDesktopAcrylicBackdrop>();
}
unsafe impl windows_core::Interface for DesktopAcrylicBackdrop {
    type Vtable = <IDesktopAcrylicBackdrop as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDesktopAcrylicBackdrop as windows_core::Interface>::IID;
}
impl core::ops::Deref for DesktopAcrylicBackdrop {
    type Target = IDesktopAcrylicBackdrop;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DesktopAcrylicBackdrop {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.DesktopAcrylicBackdrop";
}
unsafe impl Send for DesktopAcrylicBackdrop {}
unsafe impl Sync for DesktopAcrylicBackdrop {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatcherQueue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DispatcherQueue,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DispatcherQueue {
    pub fn GetForCurrentThread() -> windows_core::Result<DispatcherQueue> {
        Self::IDispatcherQueueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentThread)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IDispatcherQueueStatics<
        R,
        F: FnOnce(&IDispatcherQueueStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DispatcherQueue, IDispatcherQueueStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DispatcherQueue {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDispatcherQueue>();
}
unsafe impl windows_core::Interface for DispatcherQueue {
    type Vtable = <IDispatcherQueue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDispatcherQueue as windows_core::Interface>::IID;
}
impl core::ops::Deref for DispatcherQueue {
    type Target = IDispatcherQueue;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DispatcherQueue {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueue";
}
unsafe impl Send for DispatcherQueue {}
unsafe impl Sync for DispatcherQueue {}
windows_core::imp::define_interface!(
    DispatcherQueueHandler,
    DispatcherQueueHandler_Vtbl,
    0x2e0872a9_4e29_5f14_b688_fb96d5f9d5f8
);
impl windows_core::RuntimeType for DispatcherQueueHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl DispatcherQueueHandler {
    pub fn new<F: Fn() + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<DispatcherQueueHandler, F>::new(
            &DispatcherQueueHandlerBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DispatcherQueueHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct DispatcherQueueHandlerBox<F: Fn() + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn() + 'static> DispatcherQueueHandlerBox<F> {
    const VTABLE: DispatcherQueueHandler_Vtbl = DispatcherQueueHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<DispatcherQueueHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<DispatcherQueueHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<DispatcherQueueHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<DispatcherQueueHandler, F>);
            (this.invoke)();
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DispatcherQueuePriority(pub i32);
impl DispatcherQueuePriority {
    pub const Low: Self = Self(-10i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(10i32);
}
impl windows_core::TypeKind for DispatcherQueuePriority {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DispatcherQueuePriority {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Dispatching.DispatcherQueuePriority;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatcherQueueTimer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DispatcherQueueTimer,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DispatcherQueueTimer {}
impl windows_core::RuntimeType for DispatcherQueueTimer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDispatcherQueueTimer>();
}
unsafe impl windows_core::Interface for DispatcherQueueTimer {
    type Vtable = <IDispatcherQueueTimer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDispatcherQueueTimer as windows_core::Interface>::IID;
}
impl core::ops::Deref for DispatcherQueueTimer {
    type Target = IDispatcherQueueTimer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DispatcherQueueTimer {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueueTimer";
}
unsafe impl Send for DispatcherQueueTimer {}
unsafe impl Sync for DispatcherQueueTimer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DropDownButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DropDownButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    DropDownButton,
    Button,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl DropDownButton {
    pub fn new() -> windows_core::Result<DropDownButton> {
        Self::IDropDownButtonFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<DropDownButton>
    where
        T: windows_core::Compose,
    {
        Self::IDropDownButtonFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IDropDownButtonFactory<R, F: FnOnce(&IDropDownButtonFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DropDownButton, IDropDownButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DropDownButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDropDownButton>();
}
unsafe impl windows_core::Interface for DropDownButton {
    type Vtable = <IDropDownButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDropDownButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for DropDownButton {
    type Target = IDropDownButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DropDownButton {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.DropDownButton";
}
unsafe impl Send for DropDownButton {}
unsafe impl Sync for DropDownButton {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElementCompositionPreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ElementCompositionPreview,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ElementCompositionPreview {
    pub fn GetElementVisual<P0>(element: P0) -> windows_core::Result<Visual>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetElementVisual)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IElementCompositionPreviewStatics<
        R,
        F: FnOnce(&IElementCompositionPreviewStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            ElementCompositionPreview,
            IElementCompositionPreviewStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ElementCompositionPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IElementCompositionPreview>();
}
unsafe impl windows_core::Interface for ElementCompositionPreview {
    type Vtable = <IElementCompositionPreview as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IElementCompositionPreview as windows_core::Interface>::IID;
}
impl core::ops::Deref for ElementCompositionPreview {
    type Target = IElementCompositionPreview;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ElementCompositionPreview {
    const NAME: &'static str = "Microsoft.UI.Xaml.Hosting.ElementCompositionPreview";
}
unsafe impl Send for ElementCompositionPreview {}
unsafe impl Sync for ElementCompositionPreview {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ElementTheme(pub i32);
impl ElementTheme {
    pub const Default: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Dark: Self = Self(2i32);
}
impl windows_core::TypeKind for ElementTheme {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ElementTheme {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.ElementTheme;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ellipse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Ellipse,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    Ellipse,
    Shape,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Ellipse {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            Ellipse,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Ellipse {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IEllipse>();
}
unsafe impl windows_core::Interface for Ellipse {
    type Vtable = <IEllipse as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IEllipse as windows_core::Interface>::IID;
}
impl core::ops::Deref for Ellipse {
    type Target = IEllipse;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Ellipse {
    const NAME: &'static str = "Microsoft.UI.Xaml.Shapes.Ellipse";
}
unsafe impl Send for Ellipse {}
unsafe impl Sync for Ellipse {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventHandler<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for EventHandler<T> {
    type Vtable = EventHandler_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for EventHandler<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({9de1c535-6ae1-11e0-84e1-18a905bcc53f}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
#[repr(C)]
#[doc(hidden)]
pub struct EventHandler_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        args: windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
struct EventHandlerBox<
    T,
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<T>) + 'static,
>(core::marker::PhantomData<(T, fn() -> F)>)
where
    T: windows_core::RuntimeType + 'static;
impl<
    T: windows_core::RuntimeType + 'static,
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<T>) + 'static,
> EventHandlerBox<T, F>
{
    const VTABLE: EventHandler_Vtbl<T> = EventHandler_Vtbl::<T> {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<EventHandler<T>, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<EventHandler<T>, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<EventHandler<T>, F>::Release,
        },
        Invoke: Self::Invoke,
        T: core::marker::PhantomData::<T>,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        args: windows_core::AbiType<T>,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<EventHandler<T>, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&args),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expander(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Expander,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    Expander,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Expander {
    pub fn new() -> windows_core::Result<Expander> {
        Self::IExpanderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Expander>
    where
        T: windows_core::Compose,
    {
        Self::IExpanderFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IExpanderFactory<R, F: FnOnce(&IExpanderFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Expander, IExpanderFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Expander {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IExpander>();
}
unsafe impl windows_core::Interface for Expander {
    type Vtable = <IExpander as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IExpander as windows_core::Interface>::IID;
}
impl core::ops::Deref for Expander {
    type Target = IExpander;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Expander {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Expander";
}
unsafe impl Send for Expander {}
unsafe impl Sync for Expander {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpanderCollapsedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ExpanderCollapsedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ExpanderCollapsedEventArgs {}
impl windows_core::RuntimeType for ExpanderCollapsedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IExpanderCollapsedEventArgs>();
}
unsafe impl windows_core::Interface for ExpanderCollapsedEventArgs {
    type Vtable = <IExpanderCollapsedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IExpanderCollapsedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ExpanderCollapsedEventArgs {
    type Target = IExpanderCollapsedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ExpanderCollapsedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ExpanderCollapsedEventArgs";
}
unsafe impl Send for ExpanderCollapsedEventArgs {}
unsafe impl Sync for ExpanderCollapsedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpanderExpandingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ExpanderExpandingEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ExpanderExpandingEventArgs {}
impl windows_core::RuntimeType for ExpanderExpandingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IExpanderExpandingEventArgs>();
}
unsafe impl windows_core::Interface for ExpanderExpandingEventArgs {
    type Vtable = <IExpanderExpandingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IExpanderExpandingEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ExpanderExpandingEventArgs {
    type Target = IExpanderExpandingEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ExpanderExpandingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ExpanderExpandingEventArgs";
}
unsafe impl Send for ExpanderExpandingEventArgs {}
unsafe impl Sync for ExpanderExpandingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlipView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FlipView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    FlipView,
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl FlipView {
    pub fn new() -> windows_core::Result<FlipView> {
        Self::IFlipViewFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<FlipView>
    where
        T: windows_core::Compose,
    {
        Self::IFlipViewFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IFlipViewFactory<R, F: FnOnce(&IFlipViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FlipView, IFlipViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FlipView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFlipView>();
}
unsafe impl windows_core::Interface for FlipView {
    type Vtable = <IFlipView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFlipView as windows_core::Interface>::IID;
}
impl core::ops::Deref for FlipView {
    type Target = IFlipView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FlipView {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.FlipView";
}
unsafe impl Send for FlipView {}
unsafe impl Sync for FlipView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Flyout(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Flyout, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Flyout, FlyoutBase, DependencyObject);
impl Flyout {
    pub fn new() -> windows_core::Result<Flyout> {
        Self::IFlyoutFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Flyout>
    where
        T: windows_core::Compose,
    {
        Self::IFlyoutFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IFlyoutFactory<R, F: FnOnce(&IFlyoutFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Flyout, IFlyoutFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Flyout {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFlyout>();
}
unsafe impl windows_core::Interface for Flyout {
    type Vtable = <IFlyout as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFlyout as windows_core::Interface>::IID;
}
impl core::ops::Deref for Flyout {
    type Target = IFlyout;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Flyout {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Flyout";
}
unsafe impl Send for Flyout {}
unsafe impl Sync for Flyout {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlyoutBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FlyoutBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(FlyoutBase, DependencyObject);
impl FlyoutBase {}
impl windows_core::RuntimeType for FlyoutBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFlyoutBase>();
}
unsafe impl windows_core::Interface for FlyoutBase {
    type Vtable = <IFlyoutBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFlyoutBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for FlyoutBase {
    type Target = IFlyoutBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FlyoutBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.FlyoutBase";
}
unsafe impl Send for FlyoutBase {}
unsafe impl Sync for FlyoutBase {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FlyoutPlacementMode(pub i32);
impl FlyoutPlacementMode {
    pub const Top: Self = Self(0i32);
    pub const Bottom: Self = Self(1i32);
    pub const Left: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const Full: Self = Self(4i32);
    pub const TopEdgeAlignedLeft: Self = Self(5i32);
    pub const TopEdgeAlignedRight: Self = Self(6i32);
    pub const BottomEdgeAlignedLeft: Self = Self(7i32);
    pub const BottomEdgeAlignedRight: Self = Self(8i32);
    pub const LeftEdgeAlignedTop: Self = Self(9i32);
    pub const LeftEdgeAlignedBottom: Self = Self(10i32);
    pub const RightEdgeAlignedTop: Self = Self(11i32);
    pub const RightEdgeAlignedBottom: Self = Self(12i32);
    pub const Auto: Self = Self(13i32);
}
impl windows_core::TypeKind for FlyoutPlacementMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FlyoutPlacementMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.Primitives.FlyoutPlacementMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontFamily(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FontFamily,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FontFamily {
    pub fn CreateInstanceWithName(familyname: &str) -> windows_core::Result<FontFamily> {
        Self::IFontFamilyFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&windows_core::HSTRING::from(familyname)),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstanceWithName_compose<T>(
        familyname: &str,
        compose: T,
    ) -> windows_core::Result<FontFamily>
    where
        T: windows_core::Compose,
    {
        Self::IFontFamilyFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&windows_core::HSTRING::from(familyname)),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IFontFamilyFactory<R, F: FnOnce(&IFontFamilyFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FontFamily, IFontFamilyFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FontFamily {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFontFamily>();
}
unsafe impl windows_core::Interface for FontFamily {
    type Vtable = <IFontFamily as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFontFamily as windows_core::Interface>::IID;
}
impl core::ops::Deref for FontFamily {
    type Target = IFontFamily;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FontFamily {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.FontFamily";
}
unsafe impl Send for FontFamily {}
unsafe impl Sync for FontFamily {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FontWeight {
    pub Weight: u16,
}
impl windows_core::TypeKind for FontWeight {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FontWeight {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Text.FontWeight;u2)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameworkElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FrameworkElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(FrameworkElement, UIElement, DependencyObject);
impl FrameworkElement {
    pub fn new() -> windows_core::Result<FrameworkElement> {
        Self::IFrameworkElementFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<FrameworkElement>
    where
        T: windows_core::Compose,
    {
        Self::IFrameworkElementFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    pub fn get_WidthProperty() -> windows_core::Result<DependencyProperty> {
        Self::IFrameworkElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_WidthProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_HeightProperty() -> windows_core::Result<DependencyProperty> {
        Self::IFrameworkElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_HeightProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_MinWidthProperty() -> windows_core::Result<DependencyProperty> {
        Self::IFrameworkElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_MinWidthProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_MaxWidthProperty() -> windows_core::Result<DependencyProperty> {
        Self::IFrameworkElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_MaxWidthProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_MinHeightProperty() -> windows_core::Result<DependencyProperty> {
        Self::IFrameworkElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_MinHeightProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_MaxHeightProperty() -> windows_core::Result<DependencyProperty> {
        Self::IFrameworkElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_MaxHeightProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_HorizontalAlignmentProperty() -> windows_core::Result<DependencyProperty> {
        Self::IFrameworkElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_HorizontalAlignmentProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_VerticalAlignmentProperty() -> windows_core::Result<DependencyProperty> {
        Self::IFrameworkElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_VerticalAlignmentProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_MarginProperty() -> windows_core::Result<DependencyProperty> {
        Self::IFrameworkElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_MarginProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IFrameworkElementFactory<
        R,
        F: FnOnce(&IFrameworkElementFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FrameworkElement, IFrameworkElementFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IFrameworkElementStatics<
        R,
        F: FnOnce(&IFrameworkElementStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FrameworkElement, IFrameworkElementStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FrameworkElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFrameworkElement>();
}
unsafe impl windows_core::Interface for FrameworkElement {
    type Vtable = <IFrameworkElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFrameworkElement as windows_core::Interface>::IID;
}
impl core::ops::Deref for FrameworkElement {
    type Target = IFrameworkElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FrameworkElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.FrameworkElement";
}
unsafe impl Send for FrameworkElement {}
unsafe impl Sync for FrameworkElement {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameworkElementAutomationPeer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FrameworkElementAutomationPeer,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    FrameworkElementAutomationPeer,
    AutomationPeer,
    DependencyObject
);
impl FrameworkElementAutomationPeer {
    pub fn CreateInstanceWithOwner<P0>(
        owner: P0,
    ) -> windows_core::Result<FrameworkElementAutomationPeer>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IFrameworkElementAutomationPeerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithOwner)(
                windows_core::Interface::as_raw(this),
                owner.param().abi(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstanceWithOwner_compose<P0, T>(
        owner: P0,
        compose: T,
    ) -> windows_core::Result<FrameworkElementAutomationPeer>
    where
        P0: windows_core::Param<FrameworkElement>,
        T: windows_core::Compose,
    {
        Self::IFrameworkElementAutomationPeerFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithOwner)(
                windows_core::Interface::as_raw(this),
                owner.param().abi(),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    pub fn FromElement<P0>(element: P0) -> windows_core::Result<AutomationPeer>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IFrameworkElementAutomationPeerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromElement)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreatePeerForElement<P0>(element: P0) -> windows_core::Result<AutomationPeer>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IFrameworkElementAutomationPeerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePeerForElement)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IFrameworkElementAutomationPeerFactory<
        R,
        F: FnOnce(&IFrameworkElementAutomationPeerFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            FrameworkElementAutomationPeer,
            IFrameworkElementAutomationPeerFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IFrameworkElementAutomationPeerStatics<
        R,
        F: FnOnce(&IFrameworkElementAutomationPeerStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            FrameworkElementAutomationPeer,
            IFrameworkElementAutomationPeerStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FrameworkElementAutomationPeer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFrameworkElementAutomationPeer>();
}
unsafe impl windows_core::Interface for FrameworkElementAutomationPeer {
    type Vtable = <IFrameworkElementAutomationPeer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IFrameworkElementAutomationPeer as windows_core::Interface>::IID;
}
impl core::ops::Deref for FrameworkElementAutomationPeer {
    type Target = IFrameworkElementAutomationPeer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FrameworkElementAutomationPeer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.Peers.FrameworkElementAutomationPeer";
}
unsafe impl Send for FrameworkElementAutomationPeer {}
unsafe impl Sync for FrameworkElementAutomationPeer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameworkTemplate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FrameworkTemplate,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(FrameworkTemplate, DependencyObject);
impl FrameworkTemplate {}
impl windows_core::RuntimeType for FrameworkTemplate {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFrameworkTemplate>();
}
unsafe impl windows_core::Interface for FrameworkTemplate {
    type Vtable = <IFrameworkTemplate as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFrameworkTemplate as windows_core::Interface>::IID;
}
impl core::ops::Deref for FrameworkTemplate {
    type Target = IFrameworkTemplate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FrameworkTemplate {
    const NAME: &'static str = "Microsoft.UI.Xaml.FrameworkTemplate";
}
unsafe impl Send for FrameworkTemplate {}
unsafe impl Sync for FrameworkTemplate {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Grid, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Grid, Panel, FrameworkElement, UIElement, DependencyObject);
impl Grid {
    pub fn new() -> windows_core::Result<Grid> {
        Self::IGridFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Grid>
    where
        T: windows_core::Compose,
    {
        Self::IGridFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    pub fn GetRow<P0>(element: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IGridStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRow)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn SetRow<P0>(element: P0, value: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IGridStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetRow)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetColumn<P0>(element: P0, value: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IGridStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetColumn)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetRowSpan<P0>(element: P0, value: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IGridStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetRowSpan)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetColumnSpan<P0>(element: P0, value: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IGridStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetColumnSpan)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    fn IGridFactory<R, F: FnOnce(&IGridFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Grid, IGridFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IGridStatics<R, F: FnOnce(&IGridStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Grid, IGridStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Grid {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IGrid>();
}
unsafe impl windows_core::Interface for Grid {
    type Vtable = <IGrid as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGrid as windows_core::Interface>::IID;
}
impl core::ops::Deref for Grid {
    type Target = IGrid;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Grid {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Grid";
}
unsafe impl Send for Grid {}
unsafe impl Sync for Grid {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GridLength {
    pub Value: f64,
    pub GridUnitType: GridUnitType,
}
impl windows_core::TypeKind for GridLength {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GridLength {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.GridLength;f8;enum(Microsoft.UI.Xaml.GridUnitType;i4))",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GridUnitType(pub i32);
impl GridUnitType {
    pub const Auto: Self = Self(0i32);
    pub const Pixel: Self = Self(1i32);
    pub const Star: Self = Self(2i32);
}
impl windows_core::TypeKind for GridUnitType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GridUnitType {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.GridUnitType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GridView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    GridView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    GridView,
    ListViewBase,
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl GridView {
    pub fn new() -> windows_core::Result<GridView> {
        Self::IGridViewFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<GridView>
    where
        T: windows_core::Compose,
    {
        Self::IGridViewFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IGridViewFactory<R, F: FnOnce(&IGridViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GridView, IGridViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GridView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IGridView>();
}
unsafe impl windows_core::Interface for GridView {
    type Vtable = <IGridView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGridView as windows_core::Interface>::IID;
}
impl core::ops::Deref for GridView {
    type Target = IGridView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for GridView {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.GridView";
}
unsafe impl Send for GridView {}
unsafe impl Sync for GridView {}
pub type HMONITOR = *mut core::ffi::c_void;
pub const HTCLIENT: u32 = 1u32;
pub type HWND = *mut core::ffi::c_void;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HorizontalAlignment(pub i32);
impl HorizontalAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Stretch: Self = Self(3i32);
}
impl windows_core::TypeKind for HorizontalAlignment {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HorizontalAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.HorizontalAlignment;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HyperlinkButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    HyperlinkButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    HyperlinkButton,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl HyperlinkButton {
    pub fn new() -> windows_core::Result<HyperlinkButton> {
        Self::IHyperlinkButtonFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<HyperlinkButton>
    where
        T: windows_core::Compose,
    {
        Self::IHyperlinkButtonFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IHyperlinkButtonFactory<
        R,
        F: FnOnce(&IHyperlinkButtonFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HyperlinkButton, IHyperlinkButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HyperlinkButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IHyperlinkButton>();
}
unsafe impl windows_core::Interface for HyperlinkButton {
    type Vtable = <IHyperlinkButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHyperlinkButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for HyperlinkButton {
    type Target = IHyperlinkButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for HyperlinkButton {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.HyperlinkButton";
}
unsafe impl Send for HyperlinkButton {}
unsafe impl Sync for HyperlinkButton {}
windows_core::imp::define_interface!(
    IAppBar,
    IAppBar_Vtbl,
    0x3d8c2927_5ac5_51bb_8bec_13ff4c1bd6c8
);
impl windows_core::RuntimeType for IAppBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsOpen: usize,
    put_IsOpen: usize,
    get_IsSticky: usize,
    put_IsSticky: usize,
    get_ClosedDisplayMode: usize,
    put_ClosedDisplayMode: usize,
    get_TemplateSettings: usize,
    get_LightDismissOverlayMode: usize,
    put_LightDismissOverlayMode: usize,
    add_Opening: usize,
    remove_Opening: usize,
    add_Opened: usize,
    remove_Opened: usize,
    add_Closing: usize,
    remove_Closing: usize,
    add_Closed: usize,
    remove_Closed: usize,
}
windows_core::imp::define_interface!(
    IAppBarButton,
    IAppBarButton_Vtbl,
    0x8ab0e278_b6ae_569e_8dcd_d293552fe4d5
);
impl windows_core::RuntimeType for IAppBarButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppBarButton {
    pub fn get_Label(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Label)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn put_Label(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Label)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Icon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IconElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Icon)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Label: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Label: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Icon: usize,
    pub put_Icon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_LabelPosition: usize,
    put_LabelPosition: usize,
    get_KeyboardAcceleratorTextOverride: usize,
    put_KeyboardAcceleratorTextOverride: usize,
    get_TemplateSettings: usize,
}
windows_core::imp::define_interface!(
    IAppBarButtonFactory,
    IAppBarButtonFactory_Vtbl,
    0x4168a40a_d11f_5aeb_974e_bb43a6e7f9b2
);
impl windows_core::RuntimeType for IAppBarButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppBarSeparator,
    IAppBarSeparator_Vtbl,
    0x57bb94a3_1e56_5ebe_8a57_3a243c491d67
);
impl windows_core::RuntimeType for IAppBarSeparator {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarSeparator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IAppBarSeparatorFactory,
    IAppBarSeparatorFactory_Vtbl,
    0x6497d326_fb55_5cf5_8cc4_c556b1a958fb
);
impl windows_core::RuntimeType for IAppBarSeparatorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarSeparatorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppBarToggleButton,
    IAppBarToggleButton_Vtbl,
    0x9687c0b1_c390_59be_acdc_4fc48f552823
);
impl windows_core::RuntimeType for IAppBarToggleButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppBarToggleButton {
    pub fn put_Label(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Label)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Icon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IconElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Icon)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarToggleButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Label: usize,
    pub put_Label: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Icon: usize,
    pub put_Icon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_LabelPosition: usize,
    put_LabelPosition: usize,
    get_KeyboardAcceleratorTextOverride: usize,
    put_KeyboardAcceleratorTextOverride: usize,
    get_TemplateSettings: usize,
}
windows_core::imp::define_interface!(
    IAppBarToggleButtonFactory,
    IAppBarToggleButtonFactory_Vtbl,
    0x07bfb2d6_23b9_57a2_9122_006294bfa92f
);
impl windows_core::RuntimeType for IAppBarToggleButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarToggleButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindow,
    IAppWindow_Vtbl,
    0xcfa788b3_643b_5c5e_ad4e_321d48a82acd
);
impl windows_core::RuntimeType for IAppWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindow {
    pub fn get_Presenter(&self) -> windows_core::Result<AppWindowPresenter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Presenter)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_Size(&self) -> windows_core::Result<SizeInt32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Size)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_TitleBar(&self) -> windows_core::Result<AppWindowTitleBar> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TitleBar)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPresenterByKind(
        &self,
        appwindowpresenterkind: AppWindowPresenterKind,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPresenterByKind)(
                windows_core::Interface::as_raw(self),
                appwindowpresenterkind,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindow_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Id: usize,
    get_IsShownInSwitchers: usize,
    put_IsShownInSwitchers: usize,
    get_IsVisible: usize,
    get_OwnerWindowId: usize,
    get_Position: usize,
    pub get_Presenter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_Size:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut SizeInt32) -> windows_core::HRESULT,
    get_Title: usize,
    put_Title: usize,
    pub get_TitleBar: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Destroy: usize,
    Hide: usize,
    Move: usize,
    MoveAndResize: usize,
    MoveAndResizeRelativeToDisplayArea: usize,
    Resize: usize,
    SetIcon: usize,
    SetIconWithIconId: usize,
    SetPresenter: usize,
    pub SetPresenterByKind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        AppWindowPresenterKind,
    ) -> windows_core::HRESULT,
    Show: usize,
    ShowWithActivation: usize,
    add_Changed: usize,
    remove_Changed: usize,
    add_Closing: usize,
    remove_Closing: usize,
    add_Destroying: usize,
    remove_Destroying: usize,
}
windows_core::imp::define_interface!(
    IAppWindow2,
    IAppWindow2_Vtbl,
    0x6cd41292_794c_5cac_8961_210d012c6ebc
);
impl windows_core::RuntimeType for IAppWindow2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindow2 {
    pub fn get_ClientSize(&self) -> windows_core::Result<SizeInt32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClientSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ResizeClient(&self, size: SizeInt32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ResizeClient)(
                windows_core::Interface::as_raw(self),
                size,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindow2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_ClientSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut SizeInt32) -> windows_core::HRESULT,
    MoveInZOrderAtBottom: usize,
    MoveInZOrderAtTop: usize,
    MoveInZOrderBelow: usize,
    pub ResizeClient:
        unsafe extern "system" fn(*mut core::ffi::c_void, SizeInt32) -> windows_core::HRESULT,
    ShowOnceWithRequestedStartupState: usize,
}
windows_core::imp::define_interface!(
    IAppWindowPresenter,
    IAppWindowPresenter_Vtbl,
    0xbc3042c2_c6c6_5632_8989_ff0ec6d3b40d
);
impl windows_core::RuntimeType for IAppWindowPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Kind: usize,
}
windows_core::imp::define_interface!(
    IAppWindowTitleBar,
    IAppWindowTitleBar_Vtbl,
    0x5574efa2_c91c_5700_a363_539c71a7aaf4
);
impl windows_core::RuntimeType for IAppWindowTitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_BackgroundColor: usize,
    put_BackgroundColor: usize,
    get_ButtonBackgroundColor: usize,
    put_ButtonBackgroundColor: usize,
    get_ButtonForegroundColor: usize,
    put_ButtonForegroundColor: usize,
    get_ButtonHoverBackgroundColor: usize,
    put_ButtonHoverBackgroundColor: usize,
    get_ButtonHoverForegroundColor: usize,
    put_ButtonHoverForegroundColor: usize,
    get_ButtonInactiveBackgroundColor: usize,
    put_ButtonInactiveBackgroundColor: usize,
    get_ButtonInactiveForegroundColor: usize,
    put_ButtonInactiveForegroundColor: usize,
    get_ButtonPressedBackgroundColor: usize,
    put_ButtonPressedBackgroundColor: usize,
    get_ButtonPressedForegroundColor: usize,
    put_ButtonPressedForegroundColor: usize,
    get_ExtendsContentIntoTitleBar: usize,
    put_ExtendsContentIntoTitleBar: usize,
    get_ForegroundColor: usize,
    put_ForegroundColor: usize,
    get_Height: usize,
    get_IconShowOptions: usize,
    put_IconShowOptions: usize,
    get_InactiveBackgroundColor: usize,
    put_InactiveBackgroundColor: usize,
    get_InactiveForegroundColor: usize,
    put_InactiveForegroundColor: usize,
    get_LeftInset: usize,
    get_RightInset: usize,
    ResetToDefault: usize,
    SetDragRectangles: usize,
}
windows_core::imp::define_interface!(
    IAppWindowTitleBar2,
    IAppWindowTitleBar2_Vtbl,
    0x86faed38_748a_5b4b_9ccf_3ba0496c9041
);
impl windows_core::RuntimeType for IAppWindowTitleBar2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindowTitleBar2 {
    pub fn put_PreferredHeightOption(
        &self,
        value: TitleBarHeightOption,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PreferredHeightOption)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBar2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_PreferredHeightOption: usize,
    pub put_PreferredHeightOption: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TitleBarHeightOption,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindowTitleBar3,
    IAppWindowTitleBar3_Vtbl,
    0x07146e74_0410_5597_aba7_1af276d2ae07
);
impl windows_core::RuntimeType for IAppWindowTitleBar3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindowTitleBar3 {
    pub fn put_PreferredTheme(&self, value: TitleBarTheme) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PreferredTheme)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBar3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_PreferredTheme: usize,
    pub put_PreferredTheme:
        unsafe extern "system" fn(*mut core::ffi::c_void, TitleBarTheme) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IApplication,
    IApplication_Vtbl,
    0x06a8f4e7_1146_55af_820d_ebd55643b021
);
impl windows_core::RuntimeType for IApplication {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IApplication {
    pub fn get_Resources(&self) -> windows_core::Result<ResourceDictionary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Resources)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplication_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Resources: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    put_Resources: usize,
    get_DebugSettings: usize,
    get_RequestedTheme: usize,
    put_RequestedTheme: usize,
    get_FocusVisualKind: usize,
    put_FocusVisualKind: usize,
    get_HighContrastAdjustment: usize,
    put_HighContrastAdjustment: usize,
    add_UnhandledException: usize,
    remove_UnhandledException: usize,
    Exit: usize,
}
windows_core::imp::define_interface!(
    IApplicationFactory,
    IApplicationFactory_Vtbl,
    0x9fd96657_5294_5a65_a1db_4fea143597da
);
impl windows_core::RuntimeType for IApplicationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IApplicationInitializationCallbackParams,
    IApplicationInitializationCallbackParams_Vtbl,
    0x1b1906ea_5b7b_5876_81ab_7c2281ac3d20
);
impl windows_core::RuntimeType for IApplicationInitializationCallbackParams {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationInitializationCallbackParams_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IApplicationOverrides,
    IApplicationOverrides_Vtbl,
    0xa33e81ef_c665_503b_8827_d27ef1720a06
);
impl windows_core::RuntimeType for IApplicationOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IApplicationOverrides");
}
impl IApplicationOverrides {
    pub fn OnLaunched<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LaunchActivatedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnLaunched)(
                windows_core::Interface::as_raw(self),
                args.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for IApplicationOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.IApplicationOverrides";
}
pub trait IApplicationOverrides_Impl: windows_core::IUnknownImpl {
    fn OnLaunched(
        &self,
        args: windows_core::Ref<LaunchActivatedEventArgs>,
    ) -> windows_core::Result<()>;
}
impl IApplicationOverrides_Vtbl {
    pub const fn new<Identity: IApplicationOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLaunched<
            Identity: IApplicationOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationOverrides_Impl::OnLaunched(this, core::mem::transmute_copy(&args))
                    .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IApplicationOverrides, OFFSET>(
            ),
            OnLaunched: OnLaunched::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApplicationOverrides as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnLaunched: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IApplicationStatics,
    IApplicationStatics_Vtbl,
    0x4e0d09f5_4358_512c_a987_503b52848e95
);
impl windows_core::RuntimeType for IApplicationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Current: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    LoadComponent: usize,
    LoadComponentWithResourceLocation: usize,
}
windows_core::imp::define_interface!(
    IAutoSuggestBox,
    IAutoSuggestBox_Vtbl,
    0x3eea809e_b2db_521d_97db_e0648fb5d798
);
impl windows_core::RuntimeType for IAutoSuggestBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAutoSuggestBox {
    pub fn get_Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn put_Text(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Text)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_PlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn add_SuggestionChosen<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<AutoSuggestBox>,
                windows_core::Ref<AutoSuggestBoxSuggestionChosenEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<AutoSuggestBox, AutoSuggestBoxSuggestionChosenEventArgs> = {
            let com =
                windows_core::imp::DelegateBox::<
                    TypedEventHandler<AutoSuggestBox, AutoSuggestBoxSuggestionChosenEventArgs>,
                    F,
                >::new(
                    &TypedEventHandlerBox::<
                        AutoSuggestBox,
                        AutoSuggestBoxSuggestionChosenEventArgs,
                        F,
                    >::VTABLE,
                    handler,
                );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SuggestionChosen)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SuggestionChosen,
            ))
        }
    }
    pub fn add_TextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<AutoSuggestBox>,
                windows_core::Ref<AutoSuggestBoxTextChangedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<AutoSuggestBox, AutoSuggestBoxTextChangedEventArgs> = {
            let com = windows_core::imp::DelegateBox::< TypedEventHandler < AutoSuggestBox , AutoSuggestBoxTextChangedEventArgs > , F >::new (& TypedEventHandlerBox::< AutoSuggestBox , AutoSuggestBoxTextChangedEventArgs , F >::VTABLE , handler) ;
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_TextChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_TextChanged,
            ))
        }
    }
    pub fn add_QuerySubmitted<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<AutoSuggestBox>,
                windows_core::Ref<AutoSuggestBoxQuerySubmittedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<AutoSuggestBox, AutoSuggestBoxQuerySubmittedEventArgs> = {
            let com = windows_core::imp::DelegateBox::< TypedEventHandler < AutoSuggestBox , AutoSuggestBoxQuerySubmittedEventArgs > , F >::new (& TypedEventHandlerBox::< AutoSuggestBox , AutoSuggestBoxQuerySubmittedEventArgs , F >::VTABLE , handler) ;
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_QuerySubmitted)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_QuerySubmitted,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoSuggestBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_MaxSuggestionListHeight: usize,
    put_MaxSuggestionListHeight: usize,
    get_IsSuggestionListOpen: usize,
    put_IsSuggestionListOpen: usize,
    get_TextMemberPath: usize,
    put_TextMemberPath: usize,
    pub get_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_UpdateTextOnSelect: usize,
    put_UpdateTextOnSelect: usize,
    get_PlaceholderText: usize,
    pub put_PlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_AutoMaximizeSuggestionArea: usize,
    put_AutoMaximizeSuggestionArea: usize,
    get_TextBoxStyle: usize,
    put_TextBoxStyle: usize,
    get_QueryIcon: usize,
    put_QueryIcon: usize,
    get_LightDismissOverlayMode: usize,
    put_LightDismissOverlayMode: usize,
    get_Description: usize,
    put_Description: usize,
    pub add_SuggestionChosen: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SuggestionChosen:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_TextChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_TextChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_QuerySubmitted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_QuerySubmitted:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAutoSuggestBoxQuerySubmittedEventArgs,
    IAutoSuggestBoxQuerySubmittedEventArgs_Vtbl,
    0x26da5de4_57a6_57bf_acc9_aac599c0b22b
);
impl windows_core::RuntimeType for IAutoSuggestBoxQuerySubmittedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAutoSuggestBoxQuerySubmittedEventArgs {
    pub fn get_QueryText(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_QueryText)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoSuggestBoxQuerySubmittedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_QueryText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ChosenSuggestion: usize,
}
windows_core::imp::define_interface!(
    IAutoSuggestBoxSuggestionChosenEventArgs,
    IAutoSuggestBoxSuggestionChosenEventArgs_Vtbl,
    0x7547c7e9_7429_5045_ad98_338a96b270b1
);
impl windows_core::RuntimeType for IAutoSuggestBoxSuggestionChosenEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAutoSuggestBoxSuggestionChosenEventArgs {
    pub fn get_SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoSuggestBoxSuggestionChosenEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_SelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAutoSuggestBoxTextChangedEventArgs,
    IAutoSuggestBoxTextChangedEventArgs_Vtbl,
    0xd7191d84_e886_547f_a3e2_12f0e05b20fa
);
impl windows_core::RuntimeType for IAutoSuggestBoxTextChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAutoSuggestBoxTextChangedEventArgs {
    pub fn get_Reason(&self) -> windows_core::Result<AutoSuggestionBoxTextChangeReason> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Reason)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoSuggestBoxTextChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Reason: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AutoSuggestionBoxTextChangeReason,
    ) -> windows_core::HRESULT,
    put_Reason: usize,
    CheckCurrent: usize,
}
windows_core::imp::define_interface!(
    IAutomationPeer,
    IAutomationPeer_Vtbl,
    0xe51d3e4e_34f0_568c_999f_6277e2afe6d7
);
impl windows_core::RuntimeType for IAutomationPeer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPeer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_EventsSource: usize,
    put_EventsSource: usize,
    GetPattern: usize,
    RaiseAutomationEvent: usize,
    RaisePropertyChangedEvent: usize,
    GetAcceleratorKey: usize,
    GetAccessKey: usize,
    GetAutomationControlType: usize,
    GetAutomationId: usize,
    GetBoundingRectangle: usize,
    GetChildren: usize,
    Navigate: usize,
    GetClassName: usize,
    GetClickablePoint: usize,
    GetHelpText: usize,
    GetItemStatus: usize,
    GetItemType: usize,
    GetLabeledBy: usize,
    GetLocalizedControlType: usize,
    GetName: usize,
    GetOrientation: usize,
    HasKeyboardFocus: usize,
    IsContentElement: usize,
    IsControlElement: usize,
    IsEnabled: usize,
    IsKeyboardFocusable: usize,
    IsOffscreen: usize,
    IsPassword: usize,
    IsRequiredForForm: usize,
    SetFocus: usize,
    GetParent: usize,
    InvalidatePeer: usize,
    GetPeerFromPoint: usize,
    GetElementFromPoint: usize,
    GetFocusedElement: usize,
    GetLiveSetting: usize,
    ShowContextMenu: usize,
    GetControlledPeers: usize,
    GetAnnotations: usize,
    SetParent: usize,
    RaiseTextEditTextChangedEvent: usize,
    GetPositionInSet: usize,
    GetSizeOfSet: usize,
    GetLevel: usize,
    RaiseStructureChangedEvent: usize,
    GetLandmarkType: usize,
    GetLocalizedLandmarkType: usize,
    IsPeripheral: usize,
    IsDataValidForForm: usize,
    GetFullDescription: usize,
    GetCulture: usize,
    RaiseNotificationEvent: usize,
    GetHeadingLevel: usize,
    IsDialog: usize,
}
windows_core::imp::define_interface!(
    IAutomationProperties,
    IAutomationProperties_Vtbl,
    0x525c6a71_dd8a_52a0_977b_db1b02f8e896
);
impl windows_core::RuntimeType for IAutomationProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IAutomationPropertiesStatics,
    IAutomationPropertiesStatics_Vtbl,
    0xb1e3e0f3_112f_5966_87dc_7862d4ad50e5
);
impl windows_core::RuntimeType for IAutomationPropertiesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_AcceleratorKeyProperty: usize,
    GetAcceleratorKey: usize,
    SetAcceleratorKey: usize,
    get_AccessKeyProperty: usize,
    GetAccessKey: usize,
    SetAccessKey: usize,
    get_AutomationIdProperty: usize,
    GetAutomationId: usize,
    pub SetAutomationId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HelpTextProperty: usize,
    GetHelpText: usize,
    pub SetHelpText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsRequiredForFormProperty: usize,
    GetIsRequiredForForm: usize,
    SetIsRequiredForForm: usize,
    get_ItemStatusProperty: usize,
    GetItemStatus: usize,
    SetItemStatus: usize,
    get_ItemTypeProperty: usize,
    GetItemType: usize,
    SetItemType: usize,
    get_LabeledByProperty: usize,
    GetLabeledBy: usize,
    SetLabeledBy: usize,
    get_NameProperty: usize,
    GetName: usize,
    pub SetName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_LiveSettingProperty: usize,
    GetLiveSetting: usize,
    pub SetLiveSetting: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        AutomationLiveSetting,
    ) -> windows_core::HRESULT,
    get_AccessibilityViewProperty: usize,
    GetAccessibilityView: usize,
    SetAccessibilityView: usize,
    get_ControlledPeersProperty: usize,
    GetControlledPeers: usize,
    get_PositionInSetProperty: usize,
    GetPositionInSet: usize,
    SetPositionInSet: usize,
    get_SizeOfSetProperty: usize,
    GetSizeOfSet: usize,
    SetSizeOfSet: usize,
    get_LevelProperty: usize,
    GetLevel: usize,
    SetLevel: usize,
    get_AnnotationsProperty: usize,
    GetAnnotations: usize,
    get_LandmarkTypeProperty: usize,
    GetLandmarkType: usize,
    SetLandmarkType: usize,
    get_LocalizedLandmarkTypeProperty: usize,
    GetLocalizedLandmarkType: usize,
    SetLocalizedLandmarkType: usize,
    get_IsPeripheralProperty: usize,
    GetIsPeripheral: usize,
    SetIsPeripheral: usize,
    get_IsDataValidForFormProperty: usize,
    GetIsDataValidForForm: usize,
    SetIsDataValidForForm: usize,
    get_FullDescriptionProperty: usize,
    GetFullDescription: usize,
    SetFullDescription: usize,
    get_LocalizedControlTypeProperty: usize,
    GetLocalizedControlType: usize,
    SetLocalizedControlType: usize,
    get_DescribedByProperty: usize,
    GetDescribedBy: usize,
    get_FlowsToProperty: usize,
    GetFlowsTo: usize,
    get_FlowsFromProperty: usize,
    GetFlowsFrom: usize,
    get_CultureProperty: usize,
    GetCulture: usize,
    SetCulture: usize,
    get_HeadingLevelProperty: usize,
    GetHeadingLevel: usize,
    pub SetHeadingLevel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        AutomationHeadingLevel,
    ) -> windows_core::HRESULT,
    get_IsDialogProperty: usize,
    GetIsDialog: usize,
    SetIsDialog: usize,
}
windows_core::imp::define_interface!(
    IBitmapImage,
    IBitmapImage_Vtbl,
    0x5cc29916_a411_5bc2_a3c5_a00d99a59da8
);
impl windows_core::RuntimeType for IBitmapImage {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IBitmapImage {
    pub fn put_UriSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Uri>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_UriSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_CreateOptions: usize,
    put_CreateOptions: usize,
    get_UriSource: usize,
    pub put_UriSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_DecodePixelWidth: usize,
    put_DecodePixelWidth: usize,
    get_DecodePixelHeight: usize,
    put_DecodePixelHeight: usize,
    get_DecodePixelType: usize,
    put_DecodePixelType: usize,
    get_IsAnimatedBitmap: usize,
    get_IsPlaying: usize,
    get_AutoPlay: usize,
    put_AutoPlay: usize,
    add_DownloadProgress: usize,
    remove_DownloadProgress: usize,
    add_ImageOpened: usize,
    remove_ImageOpened: usize,
    add_ImageFailed: usize,
    remove_ImageFailed: usize,
    Play: usize,
    Stop: usize,
}
windows_core::imp::define_interface!(
    IBitmapSource,
    IBitmapSource_Vtbl,
    0x8424269d_9b82_534f_8fea_af5b5ef96bf2
);
impl windows_core::RuntimeType for IBitmapSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_PixelWidth: usize,
    get_PixelHeight: usize,
    SetSource: usize,
    SetSourceAsync: usize,
}
windows_core::imp::define_interface!(IBlock, IBlock_Vtbl, 0x8149d507_672f_5fd5_a10a_351389ba9659);
impl windows_core::RuntimeType for IBlock {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlock_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_TextAlignment: usize,
    put_TextAlignment: usize,
    get_HorizontalTextAlignment: usize,
    put_HorizontalTextAlignment: usize,
    get_LineHeight: usize,
    put_LineHeight: usize,
    get_LineStackingStrategy: usize,
    put_LineStackingStrategy: usize,
    get_Margin: usize,
    put_Margin: usize,
}
windows_core::imp::define_interface!(
    IBorder,
    IBorder_Vtbl,
    0x1ca13b47_ff5c_5abc_a411_a177df9482a9
);
impl windows_core::RuntimeType for IBorder {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IBorder {
    pub fn put_BorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_BorderBrush)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_BorderThickness(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_BorderThickness)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Background<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Background)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_CornerRadius(&self, value: CornerRadius) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_CornerRadius)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Padding(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Padding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_Child(&self) -> windows_core::Result<UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Child)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_Child<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Child)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBorder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_BorderBrush: usize,
    pub put_BorderBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_BorderThickness: usize,
    pub put_BorderThickness:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    get_Background: usize,
    pub put_Background: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_BackgroundSizing: usize,
    put_BackgroundSizing: usize,
    get_CornerRadius: usize,
    pub put_CornerRadius:
        unsafe extern "system" fn(*mut core::ffi::c_void, CornerRadius) -> windows_core::HRESULT,
    get_Padding: usize,
    pub put_Padding:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    pub get_Child: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Child: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ChildTransitions: usize,
    put_ChildTransitions: usize,
    get_BackgroundTransition: usize,
    put_BackgroundTransition: usize,
}
windows_core::imp::define_interface!(
    IBreadcrumbBar,
    IBreadcrumbBar_Vtbl,
    0x2e47b7d6_5fbd_54c7_b0b1_ceff4a19c744
);
impl windows_core::RuntimeType for IBreadcrumbBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IBreadcrumbBar {
    pub fn put_ItemsSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_ItemsSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn add_ItemClicked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<BreadcrumbBar>,
                windows_core::Ref<BreadcrumbBarItemClickedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<BreadcrumbBar, BreadcrumbBarItemClickedEventArgs> = {
            let com = windows_core::imp::DelegateBox::< TypedEventHandler < BreadcrumbBar , BreadcrumbBarItemClickedEventArgs > , F >::new (& TypedEventHandlerBox::< BreadcrumbBar , BreadcrumbBarItemClickedEventArgs , F >::VTABLE , handler) ;
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ItemClicked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ItemClicked,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBreadcrumbBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_ItemsSource: usize,
    pub put_ItemsSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ItemTemplate: usize,
    put_ItemTemplate: usize,
    pub add_ItemClicked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ItemClicked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IBreadcrumbBarFactory,
    IBreadcrumbBarFactory_Vtbl,
    0xd5b6a6d9_3148_5cbc_a6ae_0f44cde41952
);
impl windows_core::RuntimeType for IBreadcrumbBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBreadcrumbBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IBreadcrumbBarItemClickedEventArgs,
    IBreadcrumbBarItemClickedEventArgs_Vtbl,
    0x1ceea503_365e_580d_bcd4_e9ad0248f6b5
);
impl windows_core::RuntimeType for IBreadcrumbBarItemClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IBreadcrumbBarItemClickedEventArgs {
    pub fn get_Index(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Index)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBreadcrumbBarItemClickedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Index:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    get_Item: usize,
}
windows_core::imp::define_interface!(IBrush, IBrush_Vtbl, 0x2de3cb83_1329_5679_88f8_c822bc5442cb);
impl windows_core::RuntimeType for IBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrush_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Opacity: usize,
    put_Opacity: usize,
    get_Transform: usize,
    put_Transform: usize,
    get_RelativeTransform: usize,
    put_RelativeTransform: usize,
}
windows_core::imp::define_interface!(
    IButton,
    IButton_Vtbl,
    0x216c183d_d07a_5aa5_b8a4_0300a2683e87
);
impl windows_core::RuntimeType for IButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IButton {
    pub fn get_Flyout(&self) -> windows_core::Result<FlyoutBase> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Flyout)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_Flyout<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FlyoutBase>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Flyout)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Flyout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Flyout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IButtonBase,
    IButtonBase_Vtbl,
    0x65714269_2473_5327_a652_0ea6bce7f403
);
impl windows_core::RuntimeType for IButtonBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IButtonBase {
    pub fn add_Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Click)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Click,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_ClickMode: usize,
    put_ClickMode: usize,
    get_IsPointerOver: usize,
    get_IsPressed: usize,
    get_Command: usize,
    put_Command: usize,
    get_CommandParameter: usize,
    put_CommandParameter: usize,
    pub add_Click: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Click:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IButtonFactory,
    IButtonFactory_Vtbl,
    0xfe393422_d91c_57b1_9a9c_2c7e3f41f77c
);
impl windows_core::RuntimeType for IButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICalendarDatePicker,
    ICalendarDatePicker_Vtbl,
    0xe3d9faa1_0fd0_5932_a6db_ff00c003ac21
);
impl windows_core::RuntimeType for ICalendarDatePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICalendarDatePicker {
    pub fn put_IsCalendarOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsCalendarOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_PlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsTodayHighlighted(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsTodayHighlighted)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_DateChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<CalendarDatePicker>,
                windows_core::Ref<CalendarDatePickerDateChangedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<CalendarDatePicker, CalendarDatePickerDateChangedEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<CalendarDatePicker, CalendarDatePickerDateChangedEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<
                    CalendarDatePicker,
                    CalendarDatePickerDateChangedEventArgs,
                    F,
                >::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_DateChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_DateChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarDatePicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Date: usize,
    put_Date: usize,
    get_IsCalendarOpen: usize,
    pub put_IsCalendarOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_DateFormat: usize,
    put_DateFormat: usize,
    get_PlaceholderText: usize,
    pub put_PlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_CalendarViewStyle: usize,
    put_CalendarViewStyle: usize,
    get_LightDismissOverlayMode: usize,
    put_LightDismissOverlayMode: usize,
    get_Description: usize,
    put_Description: usize,
    get_MinDate: usize,
    put_MinDate: usize,
    get_MaxDate: usize,
    put_MaxDate: usize,
    get_IsTodayHighlighted: usize,
    pub put_IsTodayHighlighted:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_DisplayMode: usize,
    put_DisplayMode: usize,
    get_FirstDayOfWeek: usize,
    put_FirstDayOfWeek: usize,
    get_DayOfWeekFormat: usize,
    put_DayOfWeekFormat: usize,
    get_CalendarIdentifier: usize,
    put_CalendarIdentifier: usize,
    get_IsOutOfScopeEnabled: usize,
    put_IsOutOfScopeEnabled: usize,
    get_IsGroupLabelVisible: usize,
    put_IsGroupLabelVisible: usize,
    add_CalendarViewDayItemChanging: usize,
    remove_CalendarViewDayItemChanging: usize,
    pub add_DateChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_DateChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_Opened: usize,
    remove_Opened: usize,
    add_Closed: usize,
    remove_Closed: usize,
    SetDisplayDate: usize,
    SetYearDecadeDisplayDimensions: usize,
}
windows_core::imp::define_interface!(
    ICalendarDatePickerDateChangedEventArgs,
    ICalendarDatePickerDateChangedEventArgs_Vtbl,
    0x57c212d7_c2f2_54a0_9d41_f263b1e5106e
);
impl windows_core::RuntimeType for ICalendarDatePickerDateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICalendarDatePickerDateChangedEventArgs {
    pub fn get_NewDate(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NewDate)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<windows_time::DateTime>| r__.Value())
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarDatePickerDateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_NewDate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_OldDate: usize,
}
windows_core::imp::define_interface!(
    ICalendarDatePickerFactory,
    ICalendarDatePickerFactory_Vtbl,
    0xb00c7818_955a_5524_b451_93868230892f
);
impl windows_core::RuntimeType for ICalendarDatePickerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarDatePickerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICalendarView,
    ICalendarView_Vtbl,
    0xe786081e_b680_56ab_bfbb_52d5b17c365e
);
impl windows_core::RuntimeType for ICalendarView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICalendarView {
    pub fn put_IsGroupLabelVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsGroupLabelVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsTodayHighlighted(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsTodayHighlighted)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_SelectedDatesChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<CalendarView>,
                windows_core::Ref<CalendarViewSelectedDatesChangedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<CalendarView, CalendarViewSelectedDatesChangedEventArgs> = {
            let com =
                windows_core::imp::DelegateBox::<
                    TypedEventHandler<CalendarView, CalendarViewSelectedDatesChangedEventArgs>,
                    F,
                >::new(
                    &TypedEventHandlerBox::<
                        CalendarView,
                        CalendarViewSelectedDatesChangedEventArgs,
                        F,
                    >::VTABLE,
                    handler,
                );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectedDatesChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectedDatesChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_CalendarIdentifier: usize,
    put_CalendarIdentifier: usize,
    get_DayOfWeekFormat: usize,
    put_DayOfWeekFormat: usize,
    get_IsGroupLabelVisible: usize,
    pub put_IsGroupLabelVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_DisplayMode: usize,
    put_DisplayMode: usize,
    get_FirstDayOfWeek: usize,
    put_FirstDayOfWeek: usize,
    get_IsOutOfScopeEnabled: usize,
    put_IsOutOfScopeEnabled: usize,
    get_IsTodayHighlighted: usize,
    pub put_IsTodayHighlighted:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_MaxDate: usize,
    put_MaxDate: usize,
    get_MinDate: usize,
    put_MinDate: usize,
    get_NumberOfWeeksInView: usize,
    put_NumberOfWeeksInView: usize,
    get_SelectedDates: usize,
    get_SelectionMode: usize,
    put_SelectionMode: usize,
    get_TemplateSettings: usize,
    get_FocusBorderBrush: usize,
    put_FocusBorderBrush: usize,
    get_SelectedHoverBorderBrush: usize,
    put_SelectedHoverBorderBrush: usize,
    get_SelectedPressedBorderBrush: usize,
    put_SelectedPressedBorderBrush: usize,
    get_SelectedDisabledBorderBrush: usize,
    put_SelectedDisabledBorderBrush: usize,
    get_SelectedBorderBrush: usize,
    put_SelectedBorderBrush: usize,
    get_HoverBorderBrush: usize,
    put_HoverBorderBrush: usize,
    get_PressedBorderBrush: usize,
    put_PressedBorderBrush: usize,
    get_TodaySelectedInnerBorderBrush: usize,
    put_TodaySelectedInnerBorderBrush: usize,
    get_BlackoutStrikethroughBrush: usize,
    put_BlackoutStrikethroughBrush: usize,
    get_CalendarItemBorderBrush: usize,
    put_CalendarItemBorderBrush: usize,
    get_BlackoutBackground: usize,
    put_BlackoutBackground: usize,
    get_OutOfScopeBackground: usize,
    put_OutOfScopeBackground: usize,
    get_CalendarItemBackground: usize,
    put_CalendarItemBackground: usize,
    get_CalendarItemHoverBackground: usize,
    put_CalendarItemHoverBackground: usize,
    get_CalendarItemPressedBackground: usize,
    put_CalendarItemPressedBackground: usize,
    get_CalendarItemDisabledBackground: usize,
    put_CalendarItemDisabledBackground: usize,
    get_TodayBackground: usize,
    put_TodayBackground: usize,
    get_TodayBlackoutBackground: usize,
    put_TodayBlackoutBackground: usize,
    get_TodayHoverBackground: usize,
    put_TodayHoverBackground: usize,
    get_TodayPressedBackground: usize,
    put_TodayPressedBackground: usize,
    get_TodayDisabledBackground: usize,
    put_TodayDisabledBackground: usize,
    get_PressedForeground: usize,
    put_PressedForeground: usize,
    get_TodayForeground: usize,
    put_TodayForeground: usize,
    get_BlackoutForeground: usize,
    put_BlackoutForeground: usize,
    get_TodayBlackoutForeground: usize,
    put_TodayBlackoutForeground: usize,
    get_SelectedForeground: usize,
    put_SelectedForeground: usize,
    get_SelectedHoverForeground: usize,
    put_SelectedHoverForeground: usize,
    get_SelectedPressedForeground: usize,
    put_SelectedPressedForeground: usize,
    get_SelectedDisabledForeground: usize,
    put_SelectedDisabledForeground: usize,
    get_OutOfScopeForeground: usize,
    put_OutOfScopeForeground: usize,
    get_OutOfScopeHoverForeground: usize,
    put_OutOfScopeHoverForeground: usize,
    get_OutOfScopePressedForeground: usize,
    put_OutOfScopePressedForeground: usize,
    get_CalendarItemForeground: usize,
    put_CalendarItemForeground: usize,
    get_DisabledForeground: usize,
    put_DisabledForeground: usize,
    get_DayItemFontFamily: usize,
    put_DayItemFontFamily: usize,
    get_DayItemFontSize: usize,
    put_DayItemFontSize: usize,
    get_DayItemFontStyle: usize,
    put_DayItemFontStyle: usize,
    get_DayItemFontWeight: usize,
    put_DayItemFontWeight: usize,
    get_TodayFontWeight: usize,
    put_TodayFontWeight: usize,
    get_FirstOfMonthLabelFontFamily: usize,
    put_FirstOfMonthLabelFontFamily: usize,
    get_FirstOfMonthLabelFontSize: usize,
    put_FirstOfMonthLabelFontSize: usize,
    get_FirstOfMonthLabelFontStyle: usize,
    put_FirstOfMonthLabelFontStyle: usize,
    get_FirstOfMonthLabelFontWeight: usize,
    put_FirstOfMonthLabelFontWeight: usize,
    get_MonthYearItemFontFamily: usize,
    put_MonthYearItemFontFamily: usize,
    get_MonthYearItemFontSize: usize,
    put_MonthYearItemFontSize: usize,
    get_MonthYearItemFontStyle: usize,
    put_MonthYearItemFontStyle: usize,
    get_MonthYearItemFontWeight: usize,
    put_MonthYearItemFontWeight: usize,
    get_FirstOfYearDecadeLabelFontFamily: usize,
    put_FirstOfYearDecadeLabelFontFamily: usize,
    get_FirstOfYearDecadeLabelFontSize: usize,
    put_FirstOfYearDecadeLabelFontSize: usize,
    get_FirstOfYearDecadeLabelFontStyle: usize,
    put_FirstOfYearDecadeLabelFontStyle: usize,
    get_FirstOfYearDecadeLabelFontWeight: usize,
    put_FirstOfYearDecadeLabelFontWeight: usize,
    get_DayItemMargin: usize,
    put_DayItemMargin: usize,
    get_MonthYearItemMargin: usize,
    put_MonthYearItemMargin: usize,
    get_FirstOfMonthLabelMargin: usize,
    put_FirstOfMonthLabelMargin: usize,
    get_FirstOfYearDecadeLabelMargin: usize,
    put_FirstOfYearDecadeLabelMargin: usize,
    get_HorizontalDayItemAlignment: usize,
    put_HorizontalDayItemAlignment: usize,
    get_VerticalDayItemAlignment: usize,
    put_VerticalDayItemAlignment: usize,
    get_HorizontalFirstOfMonthLabelAlignment: usize,
    put_HorizontalFirstOfMonthLabelAlignment: usize,
    get_VerticalFirstOfMonthLabelAlignment: usize,
    put_VerticalFirstOfMonthLabelAlignment: usize,
    get_CalendarItemBorderThickness: usize,
    put_CalendarItemBorderThickness: usize,
    get_CalendarViewDayItemStyle: usize,
    put_CalendarViewDayItemStyle: usize,
    get_CalendarItemCornerRadius: usize,
    put_CalendarItemCornerRadius: usize,
    add_CalendarViewDayItemChanging: usize,
    remove_CalendarViewDayItemChanging: usize,
    pub add_SelectedDatesChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectedDatesChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    SetDisplayDate: usize,
    SetYearDecadeDisplayDimensions: usize,
}
windows_core::imp::define_interface!(
    ICalendarViewFactory,
    ICalendarViewFactory_Vtbl,
    0x2f31d1eb_8229_517c_b3a6_5a5f28724e8f
);
impl windows_core::RuntimeType for ICalendarViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICalendarViewSelectedDatesChangedEventArgs,
    ICalendarViewSelectedDatesChangedEventArgs_Vtbl,
    0x675f688f_bc30_59f6_83c4_140bd053a09a
);
impl windows_core::RuntimeType for ICalendarViewSelectedDatesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICalendarViewSelectedDatesChangedEventArgs {
    pub fn get_AddedDates(
        &self,
    ) -> windows_core::Result<windows_collections::IVectorView<windows_time::DateTime>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AddedDates)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_RemovedDates(
        &self,
    ) -> windows_core::Result<windows_collections::IVectorView<windows_time::DateTime>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemovedDates)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarViewSelectedDatesChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_AddedDates: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_RemovedDates: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICanvas,
    ICanvas_Vtbl,
    0x457ba139_1146_51d2_807e_d9d65c927060
);
impl windows_core::RuntimeType for ICanvas {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvas_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICanvasFactory,
    ICanvasFactory_Vtbl,
    0x374c5050_3481_5557_9948_804c0b8eea89
);
impl windows_core::RuntimeType for ICanvasFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICanvasStatics,
    ICanvasStatics_Vtbl,
    0xc00d5e0f_77e3_5c59_8fcd_86761f0c6607
);
impl windows_core::RuntimeType for ICanvasStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_LeftProperty: usize,
    pub GetLeft: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut f64,
    ) -> windows_core::HRESULT,
    pub SetLeft: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        f64,
    ) -> windows_core::HRESULT,
    get_TopProperty: usize,
    pub GetTop: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut f64,
    ) -> windows_core::HRESULT,
    pub SetTop: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        f64,
    ) -> windows_core::HRESULT,
    get_ZIndexProperty: usize,
    GetZIndex: usize,
    pub SetZIndex: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICheckBox,
    ICheckBox_Vtbl,
    0xc5830000_4c9d_5fdd_9346_674c71cd80c5
);
impl windows_core::RuntimeType for ICheckBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICheckBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICheckBoxFactory,
    ICheckBoxFactory_Vtbl,
    0xf43ff58d_31d5_5835_af7b_375bc6a9bcf3
);
impl windows_core::RuntimeType for ICheckBoxFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICheckBoxFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IColorChangedEventArgs,
    IColorChangedEventArgs_Vtbl,
    0x148d57a2_b1cb_5f5d_b6b5_512805d71761
);
impl windows_core::RuntimeType for IColorChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IColorChangedEventArgs {
    pub fn get_NewColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NewColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_OldColor: usize,
    pub get_NewColor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IColorPicker,
    IColorPicker_Vtbl,
    0xae72b24b_f93f_5a19_8ce4_a18b73c3356d
);
impl windows_core::RuntimeType for IColorPicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IColorPicker {
    pub fn put_Color(&self, value: Color) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Color)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsAlphaEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsAlphaEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsColorSliderVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsColorSliderVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsColorChannelTextInputVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsColorChannelTextInputVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsHexInputVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsHexInputVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_ColorChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<ColorPicker>, windows_core::Ref<ColorChangedEventArgs>) + 'static,
    {
        let handler: TypedEventHandler<ColorPicker, ColorChangedEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<ColorPicker, ColorChangedEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<ColorPicker, ColorChangedEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ColorChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ColorChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorPicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Color: usize,
    pub put_Color:
        unsafe extern "system" fn(*mut core::ffi::c_void, Color) -> windows_core::HRESULT,
    get_PreviousColor: usize,
    put_PreviousColor: usize,
    get_IsAlphaEnabled: usize,
    pub put_IsAlphaEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsColorSpectrumVisible: usize,
    put_IsColorSpectrumVisible: usize,
    get_IsColorPreviewVisible: usize,
    put_IsColorPreviewVisible: usize,
    get_IsColorSliderVisible: usize,
    pub put_IsColorSliderVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsAlphaSliderVisible: usize,
    put_IsAlphaSliderVisible: usize,
    get_IsMoreButtonVisible: usize,
    put_IsMoreButtonVisible: usize,
    get_IsColorChannelTextInputVisible: usize,
    pub put_IsColorChannelTextInputVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsAlphaTextInputVisible: usize,
    put_IsAlphaTextInputVisible: usize,
    get_IsHexInputVisible: usize,
    pub put_IsHexInputVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_MinHue: usize,
    put_MinHue: usize,
    get_MaxHue: usize,
    put_MaxHue: usize,
    get_MinSaturation: usize,
    put_MinSaturation: usize,
    get_MaxSaturation: usize,
    put_MaxSaturation: usize,
    get_MinValue: usize,
    put_MinValue: usize,
    get_MaxValue: usize,
    put_MaxValue: usize,
    get_ColorSpectrumShape: usize,
    put_ColorSpectrumShape: usize,
    get_ColorSpectrumComponents: usize,
    put_ColorSpectrumComponents: usize,
    pub add_ColorChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ColorChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IColorPickerFactory,
    IColorPickerFactory_Vtbl,
    0x72c350e2_0a20_5b9b_ac54_633b97d7ffde
);
impl windows_core::RuntimeType for IColorPickerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorPickerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IColumnDefinition,
    IColumnDefinition_Vtbl,
    0x454cea14_87ec_5890_bb62_f1d82a94758e
);
impl windows_core::RuntimeType for IColumnDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IColumnDefinition {
    pub fn put_Width(&self, value: GridLength) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Width)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IColumnDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Width: usize,
    pub put_Width:
        unsafe extern "system" fn(*mut core::ffi::c_void, GridLength) -> windows_core::HRESULT,
    get_MaxWidth: usize,
    put_MaxWidth: usize,
    get_MinWidth: usize,
    put_MinWidth: usize,
    get_ActualWidth: usize,
}
windows_core::imp::define_interface!(
    IComboBox,
    IComboBox_Vtbl,
    0xc77da58b_4fd7_51e0_a431_f84658a83e9e
);
impl windows_core::RuntimeType for IComboBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IComboBox {
    pub fn put_IsEditable(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsEditable)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_PlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComboBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsDropDownOpen: usize,
    put_IsDropDownOpen: usize,
    get_IsEditable: usize,
    pub put_IsEditable:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsSelectionBoxHighlighted: usize,
    get_MaxDropDownHeight: usize,
    put_MaxDropDownHeight: usize,
    get_SelectionBoxItem: usize,
    get_SelectionBoxItemTemplate: usize,
    get_TemplateSettings: usize,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_PlaceholderText: usize,
    pub put_PlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_LightDismissOverlayMode: usize,
    put_LightDismissOverlayMode: usize,
    get_IsTextSearchEnabled: usize,
    put_IsTextSearchEnabled: usize,
    get_SelectionChangedTrigger: usize,
    put_SelectionChangedTrigger: usize,
    get_PlaceholderForeground: usize,
    put_PlaceholderForeground: usize,
    get_Text: usize,
    put_Text: usize,
    get_TextBoxStyle: usize,
    put_TextBoxStyle: usize,
    get_Description: usize,
    put_Description: usize,
    add_DropDownClosed: usize,
    remove_DropDownClosed: usize,
    add_DropDownOpened: usize,
    remove_DropDownOpened: usize,
    add_TextSubmitted: usize,
    remove_TextSubmitted: usize,
}
windows_core::imp::define_interface!(
    IComboBoxFactory,
    IComboBoxFactory_Vtbl,
    0x71c1014b_acdf_5c03_b5ed_02871caaeb6b
);
impl windows_core::RuntimeType for IComboBoxFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IComboBoxFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICommandBar,
    ICommandBar_Vtbl,
    0xb7ca8ee3_a07a_5f69_8ab8_be4e3e4cf0c8
);
impl windows_core::RuntimeType for ICommandBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICommandBar {
    pub fn get_PrimaryCommands(
        &self,
    ) -> windows_core::Result<windows_collections::IObservableVector<ICommandBarElement>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PrimaryCommands)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_SecondaryCommands(
        &self,
    ) -> windows_core::Result<windows_collections::IObservableVector<ICommandBarElement>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SecondaryCommands)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_DefaultLabelPosition(
        &self,
        value: CommandBarDefaultLabelPosition,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_DefaultLabelPosition)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_PrimaryCommands: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_SecondaryCommands: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_CommandBarOverflowPresenterStyle: usize,
    put_CommandBarOverflowPresenterStyle: usize,
    get_CommandBarTemplateSettings: usize,
    get_DefaultLabelPosition: usize,
    pub put_DefaultLabelPosition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        CommandBarDefaultLabelPosition,
    ) -> windows_core::HRESULT,
    get_OverflowButtonVisibility: usize,
    put_OverflowButtonVisibility: usize,
    get_IsDynamicOverflowEnabled: usize,
    put_IsDynamicOverflowEnabled: usize,
    add_DynamicOverflowItemsChanging: usize,
    remove_DynamicOverflowItemsChanging: usize,
}
windows_core::imp::define_interface!(
    ICommandBarElement,
    ICommandBarElement_Vtbl,
    0xf8eb20b4_373e_5327_9942_66a1ea21f5f9
);
impl windows_core::RuntimeType for ICommandBarElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    ICommandBarElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsCompact: usize,
    put_IsCompact: usize,
    get_IsInOverflow: usize,
    get_DynamicOverflowOrder: usize,
    put_DynamicOverflowOrder: usize,
}
windows_core::imp::define_interface!(
    ICommandBarFactory,
    ICommandBarFactory_Vtbl,
    0x8d4079c3_fa0a_5bb1_b45d_499c378761b4
);
impl windows_core::RuntimeType for ICommandBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICommandBarFlyout,
    ICommandBarFlyout_Vtbl,
    0x18e2cc40_09cb_5f20_a715_f0b2039c5e18
);
impl windows_core::RuntimeType for ICommandBarFlyout {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICommandBarFlyout {
    pub fn get_PrimaryCommands(
        &self,
    ) -> windows_core::Result<windows_collections::IObservableVector<ICommandBarElement>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PrimaryCommands)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_SecondaryCommands(
        &self,
    ) -> windows_core::Result<windows_collections::IObservableVector<ICommandBarElement>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SecondaryCommands)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarFlyout_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_PrimaryCommands: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_SecondaryCommands: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICommandBarFlyoutFactory,
    ICommandBarFlyoutFactory_Vtbl,
    0xa194dbe6_4311_5bd2_a8eb_b49c4733a33c
);
impl windows_core::RuntimeType for ICommandBarFlyoutFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarFlyoutFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionAnimation,
    ICompositionAnimation_Vtbl,
    0xa829ccc8_6fde_5b90_ad37_efd307e1b631
);
impl windows_core::RuntimeType for ICompositionAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionAnimation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ClearAllParameters: usize,
    ClearParameter: usize,
    SetColorParameter: usize,
    SetMatrix3x2Parameter: usize,
    SetMatrix4x4Parameter: usize,
    SetQuaternionParameter: usize,
    SetReferenceParameter: usize,
    SetScalarParameter: usize,
    SetVector2Parameter: usize,
    SetVector3Parameter: usize,
    SetVector4Parameter: usize,
}
windows_core::imp::define_interface!(
    ICompositionAnimation2,
    ICompositionAnimation2_Vtbl,
    0x0926eb58_8965_5c74_bdac_852ebb5e8542
);
impl windows_core::RuntimeType for ICompositionAnimation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionAnimation2 {
    pub fn put_Target(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Target)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionAnimation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    SetBooleanParameter: usize,
    get_Target: usize,
    pub put_Target: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionAnimationBase,
    ICompositionAnimationBase_Vtbl,
    0xa77c0e5a_f059_4e85_bcef_c068694cec78
);
impl windows_core::RuntimeType for ICompositionAnimationBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    ICompositionAnimationBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionAnimationBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICompositionEasingFunction,
    ICompositionEasingFunction_Vtbl,
    0x8e1ecd0d_57d8_5bc9_9bcd_e43d0dd733c4
);
impl windows_core::RuntimeType for ICompositionEasingFunction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionEasingFunction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICompositionObject,
    ICompositionObject_Vtbl,
    0x0e583d49_fb5e_5481_a426_d3c41e059a5a
);
impl windows_core::RuntimeType for ICompositionObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionObject {
    pub fn get_Compositor(&self) -> windows_core::Result<Compositor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Compositor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P1>(&self, propertyname: &str, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<CompositionAnimation>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).StartAnimation)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(propertyname)),
                animation.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Compositor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Properties: usize,
    pub StartAnimation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    StopAnimation: usize,
}
windows_core::imp::define_interface!(
    ICompositionObject2,
    ICompositionObject2_Vtbl,
    0xbcbbfebf_799c_51ce_9c82_b6e49e7e62e1
);
impl windows_core::RuntimeType for ICompositionObject2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositionObject2 {
    pub fn put_ImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ImplicitAnimationCollection>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_ImplicitAnimations)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionObject2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Comment: usize,
    put_Comment: usize,
    get_ImplicitAnimations: usize,
    pub put_ImplicitAnimations: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    StartAnimationGroup: usize,
    StopAnimationGroup: usize,
}
windows_core::imp::define_interface!(
    ICompositionTarget,
    ICompositionTarget_Vtbl,
    0x7d938324_e3ad_597c_93f6_520725410e68
);
impl windows_core::RuntimeType for ICompositionTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ICompositionTargetStatics,
    ICompositionTargetStatics_Vtbl,
    0x12a4be6f_6db1_5165_b622_d57ab782745b
);
impl windows_core::RuntimeType for ICompositionTargetStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTargetStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub add_Rendering: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Rendering:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_Rendered: usize,
    remove_Rendered: usize,
    add_SurfaceContentsLost: usize,
    remove_SurfaceContentsLost: usize,
    GetCompositorForCurrentThread: usize,
}
windows_core::imp::define_interface!(
    ICompositor,
    ICompositor_Vtbl,
    0x95213c13_c4cb_57de_b267_d21ab901ae38
);
impl windows_core::RuntimeType for ICompositor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositor {
    pub fn CreateCubicBezierEasingFunction(
        &self,
        controlpoint1: windows_numerics::Vector2,
        controlpoint2: windows_numerics::Vector2,
    ) -> windows_core::Result<CubicBezierEasingFunction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCubicBezierEasingFunction)(
                windows_core::Interface::as_raw(self),
                controlpoint1,
                controlpoint2,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateLinearEasingFunction(&self) -> windows_core::Result<LinearEasingFunction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLinearEasingFunction)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateScalarKeyFrameAnimation(&self) -> windows_core::Result<ScalarKeyFrameAnimation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateScalarKeyFrameAnimation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateVector3KeyFrameAnimation(&self) -> windows_core::Result<Vector3KeyFrameAnimation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVector3KeyFrameAnimation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CreateColorKeyFrameAnimation: usize,
    CreateColorBrush: usize,
    CreateColorBrushWithColor: usize,
    CreateContainerVisual: usize,
    pub CreateCubicBezierEasingFunction: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector2,
        windows_numerics::Vector2,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateEffectFactory: usize,
    CreateEffectFactoryWithProperties: usize,
    CreateExpressionAnimation: usize,
    CreateExpressionAnimationWithExpression: usize,
    CreateInsetClip: usize,
    CreateInsetClipWithInsets: usize,
    pub CreateLinearEasingFunction: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreatePropertySet: usize,
    CreateQuaternionKeyFrameAnimation: usize,
    pub CreateScalarKeyFrameAnimation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateScopedBatch: usize,
    CreateSpriteVisual: usize,
    CreateSurfaceBrush: usize,
    CreateSurfaceBrushWithSurface: usize,
    CreateVector2KeyFrameAnimation: usize,
    pub CreateVector3KeyFrameAnimation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateVector4KeyFrameAnimation: usize,
    GetCommitBatch: usize,
}
windows_core::imp::define_interface!(
    ICompositor2,
    ICompositor2_Vtbl,
    0xa9ffedad_3982_576d_a38a_c888ff605819
);
impl windows_core::RuntimeType for ICompositor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompositor2 {
    pub fn CreateImplicitAnimationCollection(
        &self,
    ) -> windows_core::Result<ImplicitAnimationCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateImplicitAnimationCollection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CreateAmbientLight: usize,
    CreateAnimationGroup: usize,
    CreateBackdropBrush: usize,
    CreateDistantLight: usize,
    CreateDropShadow: usize,
    pub CreateImplicitAnimationCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateLayerVisual: usize,
    CreateMaskBrush: usize,
    CreateNineGridBrush: usize,
    CreatePointLight: usize,
    CreateSpotLight: usize,
    CreateStepEasingFunction: usize,
    CreateStepEasingFunctionWithStepCount: usize,
}
windows_core::imp::define_interface!(
    IContentControl,
    IContentControl_Vtbl,
    0x07e81761_11b2_52ae_8f8b_4d53d2b5900a
);
impl windows_core::RuntimeType for IContentControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IContentControl {
    pub fn get_Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_Content<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Content)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ContentTemplate: usize,
    put_ContentTemplate: usize,
    get_ContentTemplateSelector: usize,
    put_ContentTemplateSelector: usize,
    get_ContentTransitions: usize,
    put_ContentTransitions: usize,
    get_ContentTemplateRoot: usize,
}
windows_core::imp::define_interface!(
    IContentDialog,
    IContentDialog_Vtbl,
    0xac2145a3_4a32_5305_a81d_47509515bfce
);
impl windows_core::RuntimeType for IContentDialog {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IContentDialog {
    pub fn put_Title<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Title)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_PrimaryButtonText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PrimaryButtonText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_SecondaryButtonText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SecondaryButtonText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_CloseButtonText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_CloseButtonText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_IsPrimaryButtonEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsPrimaryButtonEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsSecondaryButtonEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsSecondaryButtonEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_Closed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<ContentDialog>, windows_core::Ref<ContentDialogClosedEventArgs>)
            + 'static,
    {
        let handler: TypedEventHandler<ContentDialog, ContentDialogClosedEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<ContentDialog, ContentDialogClosedEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<ContentDialog, ContentDialogClosedEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Closed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Closed,
            ))
        }
    }
    pub fn Hide(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Hide)(windows_core::Interface::as_raw(self)).ok()
        }
    }
    pub fn ShowAsync(
        &self,
    ) -> windows_core::Result<windows_future::IAsyncOperation<ContentDialogResult>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowAsyncWithPlacement(
        &self,
        placement: ContentDialogPlacement,
    ) -> windows_core::Result<windows_future::IAsyncOperation<ContentDialogResult>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowAsyncWithPlacement)(
                windows_core::Interface::as_raw(self),
                placement,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentDialog_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Title: usize,
    pub put_Title: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TitleTemplate: usize,
    put_TitleTemplate: usize,
    get_FullSizeDesired: usize,
    put_FullSizeDesired: usize,
    get_PrimaryButtonText: usize,
    pub put_PrimaryButtonText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_SecondaryButtonText: usize,
    pub put_SecondaryButtonText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_CloseButtonText: usize,
    pub put_CloseButtonText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_PrimaryButtonCommand: usize,
    put_PrimaryButtonCommand: usize,
    get_SecondaryButtonCommand: usize,
    put_SecondaryButtonCommand: usize,
    get_CloseButtonCommand: usize,
    put_CloseButtonCommand: usize,
    get_PrimaryButtonCommandParameter: usize,
    put_PrimaryButtonCommandParameter: usize,
    get_SecondaryButtonCommandParameter: usize,
    put_SecondaryButtonCommandParameter: usize,
    get_CloseButtonCommandParameter: usize,
    put_CloseButtonCommandParameter: usize,
    get_IsPrimaryButtonEnabled: usize,
    pub put_IsPrimaryButtonEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsSecondaryButtonEnabled: usize,
    pub put_IsSecondaryButtonEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_PrimaryButtonStyle: usize,
    put_PrimaryButtonStyle: usize,
    get_SecondaryButtonStyle: usize,
    put_SecondaryButtonStyle: usize,
    get_CloseButtonStyle: usize,
    put_CloseButtonStyle: usize,
    get_DefaultButton: usize,
    put_DefaultButton: usize,
    add_Closing: usize,
    remove_Closing: usize,
    pub add_Closed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Closed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_Opened: usize,
    remove_Opened: usize,
    add_PrimaryButtonClick: usize,
    remove_PrimaryButtonClick: usize,
    add_SecondaryButtonClick: usize,
    remove_SecondaryButtonClick: usize,
    add_CloseButtonClick: usize,
    remove_CloseButtonClick: usize,
    pub Hide: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ShowAsyncWithPlacement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ContentDialogPlacement,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContentDialogClosedEventArgs,
    IContentDialogClosedEventArgs_Vtbl,
    0x9b84e681_1ab6_5485_88b2_d0d3c05b29f3
);
impl windows_core::RuntimeType for IContentDialogClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IContentDialogClosedEventArgs {
    pub fn get_Result(&self) -> windows_core::Result<ContentDialogResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Result)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentDialogClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Result: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ContentDialogResult,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContentDialogFactory,
    IContentDialogFactory_Vtbl,
    0xa05b3ad7_c60e_545a_9ee4_f098220ed816
);
impl windows_core::RuntimeType for IContentDialogFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentDialogFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControl,
    IControl_Vtbl,
    0x857d6e8a_d45a_5c69_a99c_bf6a5c54fb38
);
impl windows_core::RuntimeType for IControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IControl {
    pub fn put_FontSize(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_FontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FontFamily>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontFamily)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_FontWeight(&self, value: FontWeight) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontWeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Foreground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Foreground)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn get_IsEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_IsEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Padding(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Padding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Background<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Background)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsFocusEngagementEnabled: usize,
    put_IsFocusEngagementEnabled: usize,
    get_IsFocusEngaged: usize,
    put_IsFocusEngaged: usize,
    get_RequiresPointer: usize,
    put_RequiresPointer: usize,
    get_FontSize: usize,
    pub put_FontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_FontFamily: usize,
    pub put_FontFamily: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_FontWeight: usize,
    pub put_FontWeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, FontWeight) -> windows_core::HRESULT,
    get_FontStyle: usize,
    put_FontStyle: usize,
    get_FontStretch: usize,
    put_FontStretch: usize,
    get_CharacterSpacing: usize,
    put_CharacterSpacing: usize,
    get_Foreground: usize,
    pub put_Foreground: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsTextScaleFactorEnabled: usize,
    put_IsTextScaleFactorEnabled: usize,
    pub get_IsEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub put_IsEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_TabNavigation: usize,
    put_TabNavigation: usize,
    get_Template: usize,
    put_Template: usize,
    get_Padding: usize,
    pub put_Padding:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    get_HorizontalContentAlignment: usize,
    put_HorizontalContentAlignment: usize,
    get_VerticalContentAlignment: usize,
    put_VerticalContentAlignment: usize,
    get_Background: usize,
    pub put_Background: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_BackgroundSizing: usize,
    put_BackgroundSizing: usize,
    get_BorderThickness: usize,
    put_BorderThickness: usize,
    get_BorderBrush: usize,
    put_BorderBrush: usize,
    get_DefaultStyleResourceUri: usize,
    put_DefaultStyleResourceUri: usize,
    get_ElementSoundMode: usize,
    put_ElementSoundMode: usize,
    get_CornerRadius: usize,
    put_CornerRadius: usize,
    add_FocusEngaged: usize,
    remove_FocusEngaged: usize,
    add_FocusDisengaged: usize,
    remove_FocusDisengaged: usize,
    add_IsEnabledChanged: usize,
    remove_IsEnabledChanged: usize,
    RemoveFocusEngagement: usize,
    ApplyTemplate: usize,
}
windows_core::imp::define_interface!(
    IControlFactory,
    IControlFactory_Vtbl,
    0x25159233_9438_5534_aeb9_00eb059cf73f
);
impl windows_core::RuntimeType for IControlFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControlStatics,
    IControlStatics_Vtbl,
    0xc3ae388d_aa36_5e10_acac_98415f47bcc7
);
impl windows_core::RuntimeType for IControlStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsFocusEngagementEnabledProperty: usize,
    get_IsFocusEngagedProperty: usize,
    get_RequiresPointerProperty: usize,
    pub get_FontSizeProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_FontFamilyProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_FontWeightProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_FontStyleProperty: usize,
    get_FontStretchProperty: usize,
    get_CharacterSpacingProperty: usize,
    pub get_ForegroundProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsTextScaleFactorEnabledProperty: usize,
    pub get_IsEnabledProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TabNavigationProperty: usize,
    get_TemplateProperty: usize,
    pub get_PaddingProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HorizontalContentAlignmentProperty: usize,
    get_VerticalContentAlignmentProperty: usize,
    pub get_BackgroundProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_BackgroundSizingProperty: usize,
    get_BorderThicknessProperty: usize,
    get_BorderBrushProperty: usize,
    get_DefaultStyleKeyProperty: usize,
    get_DefaultStyleResourceUriProperty: usize,
    get_ElementSoundModeProperty: usize,
    get_CornerRadiusProperty: usize,
    get_IsTemplateFocusTargetProperty: usize,
    GetIsTemplateFocusTarget: usize,
    SetIsTemplateFocusTarget: usize,
    get_IsTemplateKeyTipTargetProperty: usize,
    GetIsTemplateKeyTipTarget: usize,
    SetIsTemplateKeyTipTarget: usize,
}
windows_core::imp::define_interface!(
    ICubicBezierEasingFunction,
    ICubicBezierEasingFunction_Vtbl,
    0x35e7fcde_f9ce_590a_8b88_64a82a6b4b48
);
impl windows_core::RuntimeType for ICubicBezierEasingFunction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICubicBezierEasingFunction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_ControlPoint1: usize,
    get_ControlPoint2: usize,
}
windows_core::imp::define_interface!(
    IDataTemplate,
    IDataTemplate_Vtbl,
    0x08fa70fa_ee75_5e92_a101_f52d0e1e9fab
);
impl windows_core::RuntimeType for IDataTemplate {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    LoadContent: usize,
}
windows_core::imp::define_interface!(
    IDataTemplateFactory,
    IDataTemplateFactory_Vtbl,
    0xd8e8249d_305b_5ca5_acf8_3e1beffd0219
);
impl windows_core::RuntimeType for IDataTemplateFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDatePicker,
    IDatePicker_Vtbl,
    0xca1dc351_3ae3_5247_8263_16bd516c6e72
);
impl windows_core::RuntimeType for IDatePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDatePicker {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_DayVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_DayVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_MonthVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MonthVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_YearVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_YearVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_SelectedDateChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<DatePicker>,
                windows_core::Ref<DatePickerSelectedValueChangedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<DatePicker, DatePickerSelectedValueChangedEventArgs> = {
            let com = windows_core::imp::DelegateBox::< TypedEventHandler < DatePicker , DatePickerSelectedValueChangedEventArgs > , F >::new (& TypedEventHandlerBox::< DatePicker , DatePickerSelectedValueChangedEventArgs , F >::VTABLE , handler) ;
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectedDateChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectedDateChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatePicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_CalendarIdentifier: usize,
    put_CalendarIdentifier: usize,
    get_Date: usize,
    put_Date: usize,
    get_DayVisible: usize,
    pub put_DayVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_MonthVisible: usize,
    pub put_MonthVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_YearVisible: usize,
    pub put_YearVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_DayFormat: usize,
    put_DayFormat: usize,
    get_MonthFormat: usize,
    put_MonthFormat: usize,
    get_YearFormat: usize,
    put_YearFormat: usize,
    get_MinYear: usize,
    put_MinYear: usize,
    get_MaxYear: usize,
    put_MaxYear: usize,
    get_Orientation: usize,
    put_Orientation: usize,
    get_LightDismissOverlayMode: usize,
    put_LightDismissOverlayMode: usize,
    get_SelectedDate: usize,
    put_SelectedDate: usize,
    add_DateChanged: usize,
    remove_DateChanged: usize,
    pub add_SelectedDateChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectedDateChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDatePickerFactory,
    IDatePickerFactory_Vtbl,
    0xa16bea02_b3e0_5fdc_b5bb_25f794dc483b
);
impl windows_core::RuntimeType for IDatePickerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatePickerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDatePickerSelectedValueChangedEventArgs,
    IDatePickerSelectedValueChangedEventArgs_Vtbl,
    0x305ed436_370f_5e82_acf2_f1413e8f9ec4
);
impl windows_core::RuntimeType for IDatePickerSelectedValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDatePickerSelectedValueChangedEventArgs {
    pub fn get_NewDate(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NewDate)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<windows_time::DateTime>| r__.Value())
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatePickerSelectedValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_OldDate: usize,
    pub get_NewDate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDependencyObject,
    IDependencyObject_Vtbl,
    0xe7beaee7_160e_50f7_8789_d63463f979fa
);
impl windows_core::RuntimeType for IDependencyObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDependencyObject {
    pub fn ClearValue<P0>(&self, dp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyProperty>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearValue)(
                windows_core::Interface::as_raw(self),
                dp.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    GetValue: usize,
    SetValue: usize,
    pub ClearValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ReadLocalValue: usize,
    GetAnimationBaseValue: usize,
    RegisterPropertyChangedCallback: usize,
    UnregisterPropertyChangedCallback: usize,
    get_Dispatcher: usize,
    get_DispatcherQueue: usize,
}
windows_core::imp::define_interface!(
    IDependencyProperty,
    IDependencyProperty_Vtbl,
    0x960eab49_9672_58a0_995b_3a42e5ea6278
);
impl windows_core::RuntimeType for IDependencyProperty {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyProperty_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    GetMetadata: usize,
}
windows_core::imp::define_interface!(
    IDesktopAcrylicBackdrop,
    IDesktopAcrylicBackdrop_Vtbl,
    0xbfd9915b_82a6_5df6_aff0_a4824ddc1143
);
impl windows_core::RuntimeType for IDesktopAcrylicBackdrop {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicBackdrop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IDesktopAcrylicBackdropFactory,
    IDesktopAcrylicBackdropFactory_Vtbl,
    0x00922e6d_ae51_564a_bce2_1973d5e463dd
);
impl windows_core::RuntimeType for IDesktopAcrylicBackdropFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicBackdropFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherQueue,
    IDispatcherQueue_Vtbl,
    0xf6ebf8fa_be1c_5bf6_a467_73da28738ae8
);
impl windows_core::RuntimeType for IDispatcherQueue {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDispatcherQueue {
    pub fn CreateTimer(&self) -> windows_core::Result<DispatcherQueueTimer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTimer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryEnqueue<P0>(&self, callback: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<DispatcherQueueHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryEnqueue)(
                windows_core::Interface::as_raw(self),
                callback.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn TryEnqueueWithPriority<P1>(
        &self,
        priority: DispatcherQueuePriority,
        callback: P1,
    ) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<DispatcherQueueHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryEnqueueWithPriority)(
                windows_core::Interface::as_raw(self),
                priority,
                callback.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateTimer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub TryEnqueue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub TryEnqueueWithPriority: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DispatcherQueuePriority,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    add_ShutdownStarting: usize,
    remove_ShutdownStarting: usize,
    add_ShutdownCompleted: usize,
    remove_ShutdownCompleted: usize,
}
windows_core::imp::define_interface!(
    IDispatcherQueueStatics,
    IDispatcherQueueStatics_Vtbl,
    0xcd3382ea_a455_5124_b63a_ca40d34ca23c
);
impl windows_core::RuntimeType for IDispatcherQueueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentThread: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherQueueTimer,
    IDispatcherQueueTimer_Vtbl,
    0xad4d63fd_88fe_541f_ac11_bf2dc1ed2ce5
);
impl windows_core::RuntimeType for IDispatcherQueueTimer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDispatcherQueueTimer {
    pub fn put_Interval(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Interval)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsRepeating(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsRepeating)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
        }
    }
    pub fn add_Tick<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<DispatcherQueueTimer>,
                windows_core::Ref<windows_core::IInspectable>,
            ) + 'static,
    {
        let handler: TypedEventHandler<DispatcherQueueTimer, windows_core::IInspectable> = {
            let com = windows_core::imp::DelegateBox::< TypedEventHandler < DispatcherQueueTimer , windows_core::IInspectable > , F >::new (& TypedEventHandlerBox::< DispatcherQueueTimer , windows_core::IInspectable , F >::VTABLE , handler) ;
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Tick)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Tick,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueTimer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Interval: usize,
    pub put_Interval: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    get_IsRunning: usize,
    get_IsRepeating: usize,
    pub put_IsRepeating:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub add_Tick: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Tick:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDropDownButton,
    IDropDownButton_Vtbl,
    0xc1e9fa91_4f95_5796_8a7b_3b7594a12c69
);
impl windows_core::RuntimeType for IDropDownButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropDownButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IDropDownButtonFactory,
    IDropDownButtonFactory_Vtbl,
    0x7cf3e13b_668d_57e7_b5d6_f5ca3dbc80bd
);
impl windows_core::RuntimeType for IDropDownButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropDownButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IElementCompositionPreview,
    IElementCompositionPreview_Vtbl,
    0xc8ad1ef4_a93f_5a25_85bd_7c498d9856d3
);
impl windows_core::RuntimeType for IElementCompositionPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IElementCompositionPreviewStatics,
    IElementCompositionPreviewStatics_Vtbl,
    0x84da5a6c_0cfa_532b_9b15_ccd986374342
);
impl windows_core::RuntimeType for IElementCompositionPreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetElementVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetElementChildVisual: usize,
    SetElementChildVisual: usize,
    GetScrollViewerManipulationPropertySet: usize,
    SetImplicitShowAnimation: usize,
    SetImplicitHideAnimation: usize,
    SetIsTranslationEnabled: usize,
    GetPointerPositionPropertySet: usize,
}
windows_core::imp::define_interface!(
    IEllipse,
    IEllipse_Vtbl,
    0x805c39aa_fa8a_5e0b_9847_4ab81b42a3df
);
impl windows_core::RuntimeType for IEllipse {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IEllipse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IExpander,
    IExpander_Vtbl,
    0xca633942_e584_55c2_b7ee_cffc73c8127a
);
impl windows_core::RuntimeType for IExpander {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IExpander {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsExpanded(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsExpanded)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_Expanding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Expander>, windows_core::Ref<ExpanderExpandingEventArgs>) + 'static,
    {
        let handler: TypedEventHandler<Expander, ExpanderExpandingEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<Expander, ExpanderExpandingEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<Expander, ExpanderExpandingEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Expanding)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Expanding,
            ))
        }
    }
    pub fn add_Collapsed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Expander>, windows_core::Ref<ExpanderCollapsedEventArgs>) + 'static,
    {
        let handler: TypedEventHandler<Expander, ExpanderCollapsedEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<Expander, ExpanderCollapsedEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<Expander, ExpanderCollapsedEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Collapsed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Collapsed,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpander_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_HeaderTemplateSelector: usize,
    put_HeaderTemplateSelector: usize,
    get_IsExpanded: usize,
    pub put_IsExpanded:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_ExpandDirection: usize,
    put_ExpandDirection: usize,
    pub add_Expanding: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Expanding:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_Collapsed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Collapsed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    get_TemplateSettings: usize,
}
windows_core::imp::define_interface!(
    IExpanderCollapsedEventArgs,
    IExpanderCollapsedEventArgs_Vtbl,
    0x968a6870_7426_535e_a526_279e6eedecd0
);
impl windows_core::RuntimeType for IExpanderCollapsedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpanderCollapsedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IExpanderExpandingEventArgs,
    IExpanderExpandingEventArgs_Vtbl,
    0x433f2e36_19e7_579c_b4ce_9ce5d510d001
);
impl windows_core::RuntimeType for IExpanderExpandingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpanderExpandingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IExpanderFactory,
    IExpanderFactory_Vtbl,
    0x51a5afc2_b16d_516e_83ae_5a10476b13af
);
impl windows_core::RuntimeType for IExpanderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpanderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFlipView,
    IFlipView_Vtbl,
    0x96c35e7f_cc48_5acc_b3b4_8ab4bdf1fe17
);
impl windows_core::RuntimeType for IFlipView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlipView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_UseTouchAnimationsForAllNavigation: usize,
    put_UseTouchAnimationsForAllNavigation: usize,
}
windows_core::imp::define_interface!(
    IFlipViewFactory,
    IFlipViewFactory_Vtbl,
    0xf3a89be6_81e3_53c4_9dc9_98a5a4e79b13
);
impl windows_core::RuntimeType for IFlipViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlipViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFlyout,
    IFlyout_Vtbl,
    0xd4a1eb7d_59b8_5df9_87c3_bd5e3856923f
);
impl windows_core::RuntimeType for IFlyout {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFlyout {
    pub fn put_Content<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Content)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyout_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Content: usize,
    pub put_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_FlyoutPresenterStyle: usize,
    put_FlyoutPresenterStyle: usize,
}
windows_core::imp::define_interface!(
    IFlyoutBase,
    IFlyoutBase_Vtbl,
    0xbb6603bf_744d_5c31_a87d_744394634d77
);
impl windows_core::RuntimeType for IFlyoutBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFlyoutBase {
    pub fn put_Placement(&self, value: FlyoutPlacementMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Placement)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Placement: usize,
    pub put_Placement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        FlyoutPlacementMode,
    ) -> windows_core::HRESULT,
    get_Target: usize,
    get_AllowFocusOnInteraction: usize,
    put_AllowFocusOnInteraction: usize,
    get_LightDismissOverlayMode: usize,
    put_LightDismissOverlayMode: usize,
    get_AllowFocusWhenDisabled: usize,
    put_AllowFocusWhenDisabled: usize,
    get_ShowMode: usize,
    put_ShowMode: usize,
    get_InputDevicePrefersPrimaryCommands: usize,
    get_AreOpenCloseAnimationsEnabled: usize,
    put_AreOpenCloseAnimationsEnabled: usize,
    get_ShouldConstrainToRootBounds: usize,
    put_ShouldConstrainToRootBounds: usize,
    get_IsConstrainedToRootBounds: usize,
    get_ElementSoundMode: usize,
    put_ElementSoundMode: usize,
    get_OverlayInputPassThroughElement: usize,
    put_OverlayInputPassThroughElement: usize,
    get_IsOpen: usize,
    get_XamlRoot: usize,
    put_XamlRoot: usize,
    add_Opened: usize,
    remove_Opened: usize,
    add_Closed: usize,
    remove_Closed: usize,
    add_Opening: usize,
    remove_Opening: usize,
    add_Closing: usize,
    remove_Closing: usize,
    ShowAt: usize,
    ShowAtWithOptions: usize,
    Hide: usize,
    TryInvokeKeyboardAccelerator: usize,
}
windows_core::imp::define_interface!(
    IFlyoutFactory,
    IFlyoutFactory_Vtbl,
    0xfd19002e_66b3_5656_b49c_b2aca11e9602
);
impl windows_core::RuntimeType for IFlyoutFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFontFamily,
    IFontFamily_Vtbl,
    0x18fa5bc1_7294_527c_bb02_b213e0b3a2a3
);
impl windows_core::RuntimeType for IFontFamily {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamily_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Source: usize,
}
windows_core::imp::define_interface!(
    IFontFamilyFactory,
    IFontFamilyFactory_Vtbl,
    0x61b88a77_d0f9_5e9e_8c28_eda01fede22e
);
impl windows_core::RuntimeType for IFontFamilyFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamilyFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElement,
    IFrameworkElement_Vtbl,
    0xfe08f13d_dc6a_5495_ad44_c2d8d21863b0
);
impl windows_core::RuntimeType for IFrameworkElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFrameworkElement {
    pub fn get_Resources(&self) -> windows_core::Result<ResourceDictionary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Resources)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Tag)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_Tag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Tag)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn get_ActualWidth(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ActualWidth)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_ActualHeight(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ActualHeight)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_Width(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Width)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Height(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Height)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_MinWidth(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MinWidth)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_MaxWidth(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MaxWidth)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_MinHeight(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MinHeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_MaxHeight(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MaxHeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_HorizontalAlignment(&self, value: HorizontalAlignment) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_HorizontalAlignment)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_VerticalAlignment(&self, value: VerticalAlignment) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_VerticalAlignment)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Margin(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Margin)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Style<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Style>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Style)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_RequestedTheme(&self, value: ElementTheme) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_RequestedTheme)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_ActualTheme(&self) -> windows_core::Result<ElementTheme> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ActualTheme)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn add_SizeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<SizeChangedEventArgs>,
            ) + 'static,
    {
        let handler: SizeChangedEventHandler = {
            let com = windows_core::imp::DelegateBox::<SizeChangedEventHandler, F>::new(
                &SizeChangedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SizeChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SizeChanged,
            ))
        }
    }
    pub fn add_ActualThemeChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<FrameworkElement>, windows_core::Ref<windows_core::IInspectable>)
            + 'static,
    {
        let handler: TypedEventHandler<FrameworkElement, windows_core::IInspectable> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<FrameworkElement, windows_core::IInspectable>,
                F,
            >::new(
                &TypedEventHandlerBox::<FrameworkElement, windows_core::IInspectable, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ActualThemeChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ActualThemeChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Triggers: usize,
    pub get_Resources: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    put_Resources: usize,
    pub get_Tag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Tag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Language: usize,
    put_Language: usize,
    pub get_ActualWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_ActualHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    get_Width: usize,
    pub put_Width: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Height: usize,
    pub put_Height: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_MinWidth: usize,
    pub put_MinWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_MaxWidth: usize,
    pub put_MaxWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_MinHeight: usize,
    pub put_MinHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_MaxHeight: usize,
    pub put_MaxHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_HorizontalAlignment: usize,
    pub put_HorizontalAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HorizontalAlignment,
    ) -> windows_core::HRESULT,
    get_VerticalAlignment: usize,
    pub put_VerticalAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VerticalAlignment,
    ) -> windows_core::HRESULT,
    get_Margin: usize,
    pub put_Margin:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    get_Name: usize,
    put_Name: usize,
    get_BaseUri: usize,
    get_DataContext: usize,
    put_DataContext: usize,
    get_AllowFocusOnInteraction: usize,
    put_AllowFocusOnInteraction: usize,
    get_FocusVisualMargin: usize,
    put_FocusVisualMargin: usize,
    get_FocusVisualSecondaryThickness: usize,
    put_FocusVisualSecondaryThickness: usize,
    get_FocusVisualPrimaryThickness: usize,
    put_FocusVisualPrimaryThickness: usize,
    get_FocusVisualSecondaryBrush: usize,
    put_FocusVisualSecondaryBrush: usize,
    get_FocusVisualPrimaryBrush: usize,
    put_FocusVisualPrimaryBrush: usize,
    get_AllowFocusWhenDisabled: usize,
    put_AllowFocusWhenDisabled: usize,
    get_Style: usize,
    pub put_Style: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Parent: usize,
    get_FlowDirection: usize,
    put_FlowDirection: usize,
    get_RequestedTheme: usize,
    pub put_RequestedTheme:
        unsafe extern "system" fn(*mut core::ffi::c_void, ElementTheme) -> windows_core::HRESULT,
    get_IsLoaded: usize,
    pub get_ActualTheme: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ElementTheme,
    ) -> windows_core::HRESULT,
    add_Loaded: usize,
    remove_Loaded: usize,
    add_Unloaded: usize,
    remove_Unloaded: usize,
    add_DataContextChanged: usize,
    remove_DataContextChanged: usize,
    pub add_SizeChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SizeChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_LayoutUpdated: usize,
    remove_LayoutUpdated: usize,
    add_Loading: usize,
    remove_Loading: usize,
    pub add_ActualThemeChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ActualThemeChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_EffectiveViewportChanged: usize,
    remove_EffectiveViewportChanged: usize,
    FindName: usize,
    SetBinding: usize,
    GetBindingExpression: usize,
}
windows_core::imp::define_interface!(
    IFrameworkElementAutomationPeer,
    IFrameworkElementAutomationPeer_Vtbl,
    0x7dab4f24_605c_51cb_87db_3eed1b9fb37b
);
impl windows_core::RuntimeType for IFrameworkElementAutomationPeer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementAutomationPeer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Owner: usize,
}
windows_core::imp::define_interface!(
    IFrameworkElementAutomationPeerFactory,
    IFrameworkElementAutomationPeerFactory_Vtbl,
    0x1682c3f8_238d_5c7e_a5a5_08cc3c10ac7c
);
impl windows_core::RuntimeType for IFrameworkElementAutomationPeerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementAutomationPeerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithOwner: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElementAutomationPeerStatics,
    IFrameworkElementAutomationPeerStatics_Vtbl,
    0x081f6fbe_6500_528a_a506_f5a4d41ddf6c
);
impl windows_core::RuntimeType for IFrameworkElementAutomationPeerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementAutomationPeerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreatePeerForElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElementFactory,
    IFrameworkElementFactory_Vtbl,
    0xbd3f2272_3efa_5f92_b759_90b1cc3e784c
);
impl windows_core::RuntimeType for IFrameworkElementFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElementStatics,
    IFrameworkElementStatics_Vtbl,
    0x894e2704_14e7_569a_b21e_afc7df7145a1
);
impl windows_core::RuntimeType for IFrameworkElementStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_TagProperty: usize,
    get_LanguageProperty: usize,
    get_ActualWidthProperty: usize,
    get_ActualHeightProperty: usize,
    pub get_WidthProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_HeightProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_MinWidthProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_MaxWidthProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_MinHeightProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_MaxHeightProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_HorizontalAlignmentProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_VerticalAlignmentProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_MarginProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_NameProperty: usize,
    get_DataContextProperty: usize,
    get_AllowFocusOnInteractionProperty: usize,
    get_FocusVisualMarginProperty: usize,
    get_FocusVisualSecondaryThicknessProperty: usize,
    get_FocusVisualPrimaryThicknessProperty: usize,
    get_FocusVisualSecondaryBrushProperty: usize,
    get_FocusVisualPrimaryBrushProperty: usize,
    get_AllowFocusWhenDisabledProperty: usize,
    get_StyleProperty: usize,
    get_FlowDirectionProperty: usize,
    get_RequestedThemeProperty: usize,
    get_ActualThemeProperty: usize,
    DeferTree: usize,
}
windows_core::imp::define_interface!(
    IFrameworkTemplate,
    IFrameworkTemplate_Vtbl,
    0x0084c7c2_de48_5b0b_8a5a_e4fb76b7f7d1
);
impl windows_core::RuntimeType for IFrameworkTemplate {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkTemplate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IGrid, IGrid_Vtbl, 0xc4496219_9014_58a1_b4ad_c5044913a5bb);
impl windows_core::RuntimeType for IGrid {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IGrid {
    pub fn get_RowDefinitions(&self) -> windows_core::Result<RowDefinitionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RowDefinitions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_ColumnDefinitions(&self) -> windows_core::Result<ColumnDefinitionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ColumnDefinitions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_RowSpacing(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_RowSpacing)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_ColumnSpacing(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_ColumnSpacing)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGrid_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_RowDefinitions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_ColumnDefinitions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_BackgroundSizing: usize,
    put_BackgroundSizing: usize,
    get_BorderBrush: usize,
    put_BorderBrush: usize,
    get_BorderThickness: usize,
    put_BorderThickness: usize,
    get_CornerRadius: usize,
    put_CornerRadius: usize,
    get_Padding: usize,
    put_Padding: usize,
    get_RowSpacing: usize,
    pub put_RowSpacing:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_ColumnSpacing: usize,
    pub put_ColumnSpacing:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IGridFactory,
    IGridFactory_Vtbl,
    0xb16bf561_fc6c_57c6_8ebc_0b06ce4513aa
);
impl windows_core::RuntimeType for IGridFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IGridStatics,
    IGridStatics_Vtbl,
    0xef9cf81d_a431_50f4_abf5_3023fe447704
);
impl windows_core::RuntimeType for IGridStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_BackgroundSizingProperty: usize,
    get_BorderBrushProperty: usize,
    get_BorderThicknessProperty: usize,
    get_CornerRadiusProperty: usize,
    get_PaddingProperty: usize,
    get_RowSpacingProperty: usize,
    get_ColumnSpacingProperty: usize,
    get_RowProperty: usize,
    pub GetRow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub SetRow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
    ) -> windows_core::HRESULT,
    get_ColumnProperty: usize,
    GetColumn: usize,
    pub SetColumn: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
    ) -> windows_core::HRESULT,
    get_RowSpanProperty: usize,
    GetRowSpan: usize,
    pub SetRowSpan: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
    ) -> windows_core::HRESULT,
    get_ColumnSpanProperty: usize,
    GetColumnSpan: usize,
    pub SetColumnSpan: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IGridView,
    IGridView_Vtbl,
    0xd495f2fa_594d_5170_b1e8_8629a179f9fb
);
impl windows_core::RuntimeType for IGridView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IGridViewFactory,
    IGridViewFactory_Vtbl,
    0x892947ea_6b86_5f17_a9b8_2121b2251271
);
impl windows_core::RuntimeType for IGridViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IHyperlinkButton,
    IHyperlinkButton_Vtbl,
    0x6dbee605_8df0_50cc_9a42_250eb138f0c6
);
impl windows_core::RuntimeType for IHyperlinkButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IHyperlinkButton {
    pub fn put_NavigateUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Uri>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_NavigateUri)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_NavigateUri: usize,
    pub put_NavigateUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IHyperlinkButtonFactory,
    IHyperlinkButtonFactory_Vtbl,
    0x01f775ea_c5ed_514a_a23d_89c494a8f09d
);
impl windows_core::RuntimeType for IHyperlinkButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IIconElement,
    IIconElement_Vtbl,
    0x18f69350_279e_50ea_8d23_138e717ed939
);
impl windows_core::RuntimeType for IIconElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIconElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Foreground: usize,
    put_Foreground: usize,
}
windows_core::imp::define_interface!(IImage, IImage_Vtbl, 0x220d3d8d_66de_53a1_a215_ba9c165565ab);
impl windows_core::RuntimeType for IImage {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IImage {
    pub fn put_Source<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ImageSource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Source)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_Stretch(&self, value: Stretch) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Stretch)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Source: usize,
    pub put_Source: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Stretch: usize,
    pub put_Stretch:
        unsafe extern "system" fn(*mut core::ffi::c_void, Stretch) -> windows_core::HRESULT,
    get_NineGrid: usize,
    put_NineGrid: usize,
    add_ImageFailed: usize,
    remove_ImageFailed: usize,
    add_ImageOpened: usize,
    remove_ImageOpened: usize,
    GetAsCastingSource: usize,
    GetAlphaMask: usize,
}
windows_core::imp::define_interface!(
    IImageSource,
    IImageSource_Vtbl,
    0x6c2038f6_d6d5_55e9_9b9e_082f12dbff60
);
impl windows_core::RuntimeType for IImageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IImplicitAnimationCollection,
    IImplicitAnimationCollection_Vtbl,
    0xc5c0689e_f5ae_5bed_829b_c522cda39717
);
impl windows_core::RuntimeType for IImplicitAnimationCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IImplicitAnimationCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInfoBadge,
    IInfoBadge_Vtbl,
    0x82104d7f_03d4_5ea4_872e_f9ecab758601
);
impl windows_core::RuntimeType for IInfoBadge {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInfoBadge {
    pub fn put_Value(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Value)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInfoBadge_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Value: usize,
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_IconSource: usize,
    put_IconSource: usize,
    get_TemplateSettings: usize,
}
windows_core::imp::define_interface!(
    IInfoBadgeFactory,
    IInfoBadgeFactory_Vtbl,
    0xfb498205_2de0_5986_8aec_2c46ac235087
);
impl windows_core::RuntimeType for IInfoBadgeFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInfoBadgeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInfoBar,
    IInfoBar_Vtbl,
    0xc1c3a438_dd79_5d22_9e42_5a3cdf8113a9
);
impl windows_core::RuntimeType for IInfoBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInfoBar {
    pub fn put_IsOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Title(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Title)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Message(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Message)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Severity(&self, value: InfoBarSeverity) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Severity)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsClosable(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsClosable)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_Closed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<InfoBar>, windows_core::Ref<InfoBarClosedEventArgs>) + 'static,
    {
        let handler: TypedEventHandler<InfoBar, InfoBarClosedEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<InfoBar, InfoBarClosedEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<InfoBar, InfoBarClosedEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Closed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Closed,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInfoBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsOpen: usize,
    pub put_IsOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_Title: usize,
    pub put_Title: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Message: usize,
    pub put_Message: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Severity: usize,
    pub put_Severity:
        unsafe extern "system" fn(*mut core::ffi::c_void, InfoBarSeverity) -> windows_core::HRESULT,
    get_IconSource: usize,
    put_IconSource: usize,
    get_IsIconVisible: usize,
    put_IsIconVisible: usize,
    get_IsClosable: usize,
    pub put_IsClosable:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_CloseButtonStyle: usize,
    put_CloseButtonStyle: usize,
    get_CloseButtonCommand: usize,
    put_CloseButtonCommand: usize,
    get_CloseButtonCommandParameter: usize,
    put_CloseButtonCommandParameter: usize,
    get_ActionButton: usize,
    put_ActionButton: usize,
    get_Content: usize,
    put_Content: usize,
    get_ContentTemplate: usize,
    put_ContentTemplate: usize,
    get_TemplateSettings: usize,
    add_CloseButtonClick: usize,
    remove_CloseButtonClick: usize,
    add_Closing: usize,
    remove_Closing: usize,
    pub add_Closed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Closed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInfoBarClosedEventArgs,
    IInfoBarClosedEventArgs_Vtbl,
    0x593af0b3_bded_53da_8f56_80ed3c64322c
);
impl windows_core::RuntimeType for IInfoBarClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInfoBarClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Reason: usize,
}
windows_core::imp::define_interface!(
    IInfoBarFactory,
    IInfoBarFactory_Vtbl,
    0x60618a60_9be7_5df5_be0d_933d34ddb44c
);
impl windows_core::RuntimeType for IInfoBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInfoBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInline,
    IInline_Vtbl,
    0x813d427a_8980_5a79_a8fa_f27919cfb24f
);
impl windows_core::RuntimeType for IInline {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInline_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IItemContainer,
    IItemContainer_Vtbl,
    0x6332a67f_7fd9_53c7_afd8_cfa1237cf6d1
);
impl windows_core::RuntimeType for IItemContainer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemContainer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Child: usize,
    put_Child: usize,
    get_IsSelected: usize,
    put_IsSelected: usize,
}
windows_core::imp::define_interface!(
    IItemsControl,
    IItemsControl_Vtbl,
    0xbf1ccb54_83e2_5b98_acbc_736f876c3d35
);
impl windows_core::RuntimeType for IItemsControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IItemsControl {
    pub fn put_ItemsSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_ItemsSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn get_Items(&self) -> windows_core::Result<ItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemsControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_ItemsSource: usize,
    pub put_ItemsSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ItemTemplate: usize,
    put_ItemTemplate: usize,
    get_ItemTemplateSelector: usize,
    put_ItemTemplateSelector: usize,
    get_ItemsPanel: usize,
    put_ItemsPanel: usize,
    get_DisplayMemberPath: usize,
    put_DisplayMemberPath: usize,
    get_ItemsPanelRoot: usize,
    get_ItemContainerStyle: usize,
    put_ItemContainerStyle: usize,
    get_ItemContainerStyleSelector: usize,
    put_ItemContainerStyleSelector: usize,
    get_ItemContainerGenerator: usize,
    get_ItemContainerTransitions: usize,
    put_ItemContainerTransitions: usize,
    get_GroupStyle: usize,
    get_GroupStyleSelector: usize,
    put_GroupStyleSelector: usize,
    get_IsGrouping: usize,
    GroupHeaderContainerFromItemContainer: usize,
}
windows_core::imp::define_interface!(
    IKeyFrameAnimation,
    IKeyFrameAnimation_Vtbl,
    0x5a8f57f0_f059_5b47_b308_c4c80fc71248
);
impl windows_core::RuntimeType for IKeyFrameAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IKeyFrameAnimation {
    pub fn put_Duration(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Duration)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn InsertExpressionKeyFrame(
        &self,
        normalizedprogresskey: f32,
        value: &str,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).InsertExpressionKeyFrame)(
                windows_core::Interface::as_raw(self),
                normalizedprogresskey,
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn InsertExpressionKeyFrameWithEasingFunction<P2>(
        &self,
        normalizedprogresskey: f32,
        value: &str,
        easingfunction: P2,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<CompositionEasingFunction>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InsertExpressionKeyFrameWithEasingFunction)(
                windows_core::Interface::as_raw(self),
                normalizedprogresskey,
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
                easingfunction.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyFrameAnimation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_DelayTime: usize,
    put_DelayTime: usize,
    get_Duration: usize,
    pub put_Duration: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    get_IterationBehavior: usize,
    put_IterationBehavior: usize,
    get_IterationCount: usize,
    put_IterationCount: usize,
    get_KeyFrameCount: usize,
    get_StopBehavior: usize,
    put_StopBehavior: usize,
    pub InsertExpressionKeyFrame: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub InsertExpressionKeyFrameWithEasingFunction:
        unsafe extern "system" fn(
            *mut core::ffi::c_void,
            f32,
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IKeyboardAccelerator,
    IKeyboardAccelerator_Vtbl,
    0x6f8bf1e2_4e91_5cf9_a6be_4770caf3d770
);
impl windows_core::RuntimeType for IKeyboardAccelerator {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IKeyboardAccelerator {
    pub fn put_Key(&self, value: VirtualKey) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Key)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Modifiers(&self, value: VirtualKeyModifiers) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Modifiers)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_Invoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<KeyboardAccelerator>,
                windows_core::Ref<KeyboardAcceleratorInvokedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs> = {
            let com =
                windows_core::imp::DelegateBox::<
                    TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs>,
                    F,
                >::new(
                    &TypedEventHandlerBox::<
                        KeyboardAccelerator,
                        KeyboardAcceleratorInvokedEventArgs,
                        F,
                    >::VTABLE,
                    handler,
                );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Invoked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Invoked,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAccelerator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Key: usize,
    pub put_Key:
        unsafe extern "system" fn(*mut core::ffi::c_void, VirtualKey) -> windows_core::HRESULT,
    get_Modifiers: usize,
    pub put_Modifiers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VirtualKeyModifiers,
    ) -> windows_core::HRESULT,
    get_IsEnabled: usize,
    put_IsEnabled: usize,
    get_ScopeOwner: usize,
    put_ScopeOwner: usize,
    pub add_Invoked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Invoked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IKeyboardAcceleratorFactory,
    IKeyboardAcceleratorFactory_Vtbl,
    0xca1d410a_af2a_51b9_a1de_6c0af9f3b598
);
impl windows_core::RuntimeType for IKeyboardAcceleratorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IKeyboardAcceleratorInvokedEventArgs,
    IKeyboardAcceleratorInvokedEventArgs_Vtbl,
    0x62c9fdb0_b574_527d_97eb_5c7f674441e0
);
impl windows_core::RuntimeType for IKeyboardAcceleratorInvokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IKeyboardAcceleratorInvokedEventArgs {
    pub fn put_Handled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Handled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Handled: usize,
    pub put_Handled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_Element: usize,
    get_KeyboardAccelerator: usize,
}
windows_core::imp::define_interface!(
    ILaunchActivatedEventArgs,
    ILaunchActivatedEventArgs_Vtbl,
    0xd505cea9_1bcb_5b29_a8be_944e00f06f78
);
impl windows_core::RuntimeType for ILaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Arguments: usize,
    get_UWPLaunchActivatedEventArgs: usize,
}
windows_core::imp::define_interface!(ILine, ILine_Vtbl, 0x507b3858_af7e_559b_a87e_4cc6a5d8ba96);
impl windows_core::RuntimeType for ILine {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ILine {
    pub fn put_X1(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_X1)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Y1(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Y1)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_X2(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_X2)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Y2(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Y2)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILine_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_X1: usize,
    pub put_X1: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Y1: usize,
    pub put_Y1: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_X2: usize,
    pub put_X2: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Y2: usize,
    pub put_Y2: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ILinearEasingFunction,
    ILinearEasingFunction_Vtbl,
    0x79bfeef6_70c7_50a6_bb3a_0e9636148695
);
impl windows_core::RuntimeType for ILinearEasingFunction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearEasingFunction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IListBox,
    IListBox_Vtbl,
    0xf3e7dedf_7a3c_59f1_9e05_ae5026b54293
);
impl windows_core::RuntimeType for IListBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IListBox {
    pub fn put_SelectionMode(&self, value: SelectionMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectionMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IListBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_SelectedItems: usize,
    get_SelectionMode: usize,
    pub put_SelectionMode:
        unsafe extern "system" fn(*mut core::ffi::c_void, SelectionMode) -> windows_core::HRESULT,
    get_SingleSelectionFollowsFocus: usize,
    put_SingleSelectionFollowsFocus: usize,
    ScrollIntoView: usize,
    SelectAll: usize,
}
windows_core::imp::define_interface!(
    IListBoxFactory,
    IListBoxFactory_Vtbl,
    0xd048e0bc_4692_5c60_b2e9_07c4433ce050
);
impl windows_core::RuntimeType for IListBoxFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IListBoxFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IListView,
    IListView_Vtbl,
    0xf6015db1_df63_52fd_a164_0df44715ee0a
);
impl windows_core::RuntimeType for IListView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IListView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IListViewBase,
    IListViewBase_Vtbl,
    0x775c57ac_abce_5beb_8e34_3b8158aedd80
);
impl windows_core::RuntimeType for IListViewBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IListViewBase {
    pub fn put_SelectionMode(&self, value: ListViewSelectionMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectionMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ScrollIntoView<P0>(&self, item: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ScrollIntoView)(
                windows_core::Interface::as_raw(self),
                item.param().abi(),
            )
            .ok()
        }
    }
    pub fn ScrollIntoViewWithAlignment<P0>(
        &self,
        item: P0,
        alignment: ScrollIntoViewAlignment,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ScrollIntoViewWithAlignment)(
                windows_core::Interface::as_raw(self),
                item.param().abi(),
                alignment,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_SelectedItems: usize,
    get_SelectionMode: usize,
    pub put_SelectionMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ListViewSelectionMode,
    ) -> windows_core::HRESULT,
    get_IsSwipeEnabled: usize,
    put_IsSwipeEnabled: usize,
    get_CanDragItems: usize,
    put_CanDragItems: usize,
    get_CanReorderItems: usize,
    put_CanReorderItems: usize,
    get_IsItemClickEnabled: usize,
    put_IsItemClickEnabled: usize,
    get_DataFetchSize: usize,
    put_DataFetchSize: usize,
    get_IncrementalLoadingThreshold: usize,
    put_IncrementalLoadingThreshold: usize,
    get_IncrementalLoadingTrigger: usize,
    put_IncrementalLoadingTrigger: usize,
    get_ShowsScrollingPlaceholders: usize,
    put_ShowsScrollingPlaceholders: usize,
    get_ReorderMode: usize,
    put_ReorderMode: usize,
    get_SelectedRanges: usize,
    get_IsMultiSelectCheckBoxEnabled: usize,
    put_IsMultiSelectCheckBoxEnabled: usize,
    get_SingleSelectionFollowsFocus: usize,
    put_SingleSelectionFollowsFocus: usize,
    add_ItemClick: usize,
    remove_ItemClick: usize,
    add_DragItemsStarting: usize,
    remove_DragItemsStarting: usize,
    add_DragItemsCompleted: usize,
    remove_DragItemsCompleted: usize,
    add_ContainerContentChanging: usize,
    remove_ContainerContentChanging: usize,
    add_ChoosingItemContainer: usize,
    remove_ChoosingItemContainer: usize,
    add_ChoosingGroupHeaderContainer: usize,
    remove_ChoosingGroupHeaderContainer: usize,
    pub ScrollIntoView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SelectAll: usize,
    LoadMoreItemsAsync: usize,
    pub ScrollIntoViewWithAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        ScrollIntoViewAlignment,
    ) -> windows_core::HRESULT,
    SetDesiredContainerUpdateDuration: usize,
    SelectRange: usize,
    DeselectRange: usize,
    IsDragSource: usize,
    TryStartConnectedAnimationAsync: usize,
    PrepareConnectedAnimation: usize,
    get_Header: usize,
    put_Header: usize,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_HeaderTransitions: usize,
    put_HeaderTransitions: usize,
    get_Footer: usize,
    put_Footer: usize,
    get_FooterTemplate: usize,
    put_FooterTemplate: usize,
    get_FooterTransitions: usize,
    put_FooterTransitions: usize,
}
windows_core::imp::define_interface!(
    IListViewFactory,
    IListViewFactory_Vtbl,
    0x03ebefb8_f64a_5bf9_9570_cb09eeea2335
);
impl windows_core::RuntimeType for IListViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IListViewItem,
    IListViewItem_Vtbl,
    0x05fe41c2_0451_5d38_9c55_5d10cfd08889
);
impl windows_core::RuntimeType for IListViewItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_TemplateSettings: usize,
}
windows_core::imp::define_interface!(
    IMenuBar,
    IMenuBar_Vtbl,
    0xba97f337_8f1e_5141_b53f_e77a8ba3ebbd
);
impl windows_core::RuntimeType for IMenuBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IMenuBar {
    pub fn get_Items(&self) -> windows_core::Result<windows_collections::IVector<MenuBarItem>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuBarFactory,
    IMenuBarFactory_Vtbl,
    0x76aa8759_04ee_5a4c_b98c_d03742d47cdb
);
impl windows_core::RuntimeType for IMenuBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuBarItem,
    IMenuBarItem_Vtbl,
    0xa7900980_51cc_531d_97c5_356b13573398
);
impl windows_core::RuntimeType for IMenuBarItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IMenuBarItem {
    pub fn put_Title(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Title)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn get_Items(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<MenuFlyoutItemBase>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuBarItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Title: usize,
    pub put_Title: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuBarItemFactory,
    IMenuBarItemFactory_Vtbl,
    0x87d02172_83cb_5459_940f_173f7501b300
);
impl windows_core::RuntimeType for IMenuBarItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuBarItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuFlyout,
    IMenuFlyout_Vtbl,
    0xf4c77c6c_1fa5_5d85_8559_5d02b7d4e5e7
);
impl windows_core::RuntimeType for IMenuFlyout {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IMenuFlyout {
    pub fn get_Items(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<MenuFlyoutItemBase>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyout_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_MenuFlyoutPresenterStyle: usize,
    put_MenuFlyoutPresenterStyle: usize,
    ShowAt: usize,
}
windows_core::imp::define_interface!(
    IMenuFlyoutFactory,
    IMenuFlyoutFactory_Vtbl,
    0xa3d225de_6b35_5442_b6c9_06fd24139a63
);
impl windows_core::RuntimeType for IMenuFlyoutFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyoutFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuFlyoutItem,
    IMenuFlyoutItem_Vtbl,
    0x4252df5a_44f9_5ee8_b1cc_53de9aaa4095
);
impl windows_core::RuntimeType for IMenuFlyoutItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IMenuFlyoutItem {
    pub fn get_Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn put_Text(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Text)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn add_Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Click)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Click,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyoutItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Command: usize,
    put_Command: usize,
    get_CommandParameter: usize,
    put_CommandParameter: usize,
    get_Icon: usize,
    put_Icon: usize,
    get_KeyboardAcceleratorTextOverride: usize,
    put_KeyboardAcceleratorTextOverride: usize,
    get_TemplateSettings: usize,
    pub add_Click: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Click:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuFlyoutItemBase,
    IMenuFlyoutItemBase_Vtbl,
    0x4bee2715_44a1_5f94_86e8_02ddbe3dc6b9
);
impl windows_core::RuntimeType for IMenuFlyoutItemBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyoutItemBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IMenuFlyoutItemFactory,
    IMenuFlyoutItemFactory_Vtbl,
    0x9c3c9a1f_89af_521a_81a5_8a01db7a79af
);
impl windows_core::RuntimeType for IMenuFlyoutItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyoutItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuFlyoutSeparator,
    IMenuFlyoutSeparator_Vtbl,
    0x3eaf5fd5_935e_5ed7_8d05_f6bafa936d25
);
impl windows_core::RuntimeType for IMenuFlyoutSeparator {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyoutSeparator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IMenuFlyoutSeparatorFactory,
    IMenuFlyoutSeparatorFactory_Vtbl,
    0x26156c9c_95ef_5e55_8342_773fc43baac3
);
impl windows_core::RuntimeType for IMenuFlyoutSeparatorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyoutSeparatorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuFlyoutSubItem,
    IMenuFlyoutSubItem_Vtbl,
    0x6b0688c1_47b0_53b5_b6f9_5ec5d6623b84
);
impl windows_core::RuntimeType for IMenuFlyoutSubItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IMenuFlyoutSubItem {
    pub fn get_Items(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<MenuFlyoutItemBase>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_Text(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Text)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyoutSubItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Text: usize,
    pub put_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Icon: usize,
    put_Icon: usize,
}
windows_core::imp::define_interface!(
    IMicaBackdrop,
    IMicaBackdrop_Vtbl,
    0xc156a404_3dac_593a_b1f3_7a33c289dc83
);
impl windows_core::RuntimeType for IMicaBackdrop {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IMicaBackdrop {
    pub fn put_Kind(&self, value: MicaKind) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Kind)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaBackdrop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Kind: usize,
    pub put_Kind:
        unsafe extern "system" fn(*mut core::ffi::c_void, MicaKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMicaBackdropFactory,
    IMicaBackdropFactory_Vtbl,
    0x774379ce_74bd_59d4_849d_d99c4184d838
);
impl windows_core::RuntimeType for IMicaBackdropFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaBackdropFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationView,
    INavigationView_Vtbl,
    0xe77a4b36_3dd1_53d9_9f97_65dccaa74a5c
);
impl windows_core::RuntimeType for INavigationView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INavigationView {
    pub fn put_IsPaneOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsPaneOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsSettingsVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsSettingsVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsPaneToggleButtonVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsPaneToggleButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_SelectedItem<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectedItem)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn get_MenuItems(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MenuItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_AutoSuggestBox(&self) -> windows_core::Result<AutoSuggestBox> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AutoSuggestBox)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_AutoSuggestBox<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AutoSuggestBox>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_AutoSuggestBox)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn add_SelectionChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<NavigationView>,
                windows_core::Ref<NavigationViewSelectionChangedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<NavigationView, NavigationViewSelectionChangedEventArgs> = {
            let com =
                windows_core::imp::DelegateBox::<
                    TypedEventHandler<NavigationView, NavigationViewSelectionChangedEventArgs>,
                    F,
                >::new(
                    &TypedEventHandlerBox::<
                        NavigationView,
                        NavigationViewSelectionChangedEventArgs,
                        F,
                    >::VTABLE,
                    handler,
                );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectionChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsPaneOpen: usize,
    pub put_IsPaneOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_CompactModeThresholdWidth: usize,
    put_CompactModeThresholdWidth: usize,
    get_ExpandedModeThresholdWidth: usize,
    put_ExpandedModeThresholdWidth: usize,
    get_FooterMenuItems: usize,
    get_FooterMenuItemsSource: usize,
    put_FooterMenuItemsSource: usize,
    get_PaneFooter: usize,
    put_PaneFooter: usize,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_DisplayMode: usize,
    get_IsSettingsVisible: usize,
    pub put_IsSettingsVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsPaneToggleButtonVisible: usize,
    pub put_IsPaneToggleButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_AlwaysShowHeader: usize,
    put_AlwaysShowHeader: usize,
    get_CompactPaneLength: usize,
    put_CompactPaneLength: usize,
    get_OpenPaneLength: usize,
    put_OpenPaneLength: usize,
    get_PaneToggleButtonStyle: usize,
    put_PaneToggleButtonStyle: usize,
    pub get_SelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_SelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_MenuItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_MenuItemsSource: usize,
    put_MenuItemsSource: usize,
    get_SettingsItem: usize,
    pub get_AutoSuggestBox: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_AutoSuggestBox: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_MenuItemTemplate: usize,
    put_MenuItemTemplate: usize,
    get_MenuItemTemplateSelector: usize,
    put_MenuItemTemplateSelector: usize,
    get_MenuItemContainerStyle: usize,
    put_MenuItemContainerStyle: usize,
    get_MenuItemContainerStyleSelector: usize,
    put_MenuItemContainerStyleSelector: usize,
    MenuItemFromContainer: usize,
    ContainerFromMenuItem: usize,
    pub add_SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_ItemInvoked: usize,
    remove_ItemInvoked: usize,
    add_DisplayModeChanged: usize,
    remove_DisplayModeChanged: usize,
    get_IsTitleBarAutoPaddingEnabled: usize,
    put_IsTitleBarAutoPaddingEnabled: usize,
}
windows_core::imp::define_interface!(
    INavigationView2,
    INavigationView2_Vtbl,
    0x05b428cf_014c_56dd_896a_a3e7089d73b5
);
impl windows_core::RuntimeType for INavigationView2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INavigationView2 {
    pub fn put_IsBackButtonVisible(
        &self,
        value: NavigationViewBackButtonVisible,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsBackButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsBackEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsBackEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_PaneTitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PaneTitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn add_BackRequested<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<NavigationView>,
                windows_core::Ref<NavigationViewBackRequestedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<NavigationView, NavigationViewBackRequestedEventArgs> = {
            let com = windows_core::imp::DelegateBox::< TypedEventHandler < NavigationView , NavigationViewBackRequestedEventArgs > , F >::new (& TypedEventHandlerBox::< NavigationView , NavigationViewBackRequestedEventArgs , F >::VTABLE , handler) ;
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_BackRequested)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_BackRequested,
            ))
        }
    }
    pub fn put_PaneDisplayMode(
        &self,
        value: NavigationViewPaneDisplayMode,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PaneDisplayMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationView2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsBackButtonVisible: usize,
    pub put_IsBackButtonVisible: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        NavigationViewBackButtonVisible,
    ) -> windows_core::HRESULT,
    get_IsBackEnabled: usize,
    pub put_IsBackEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_PaneTitle: usize,
    pub put_PaneTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub add_BackRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_BackRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_PaneClosed: usize,
    remove_PaneClosed: usize,
    add_PaneClosing: usize,
    remove_PaneClosing: usize,
    add_PaneOpened: usize,
    remove_PaneOpened: usize,
    add_PaneOpening: usize,
    remove_PaneOpening: usize,
    get_PaneDisplayMode: usize,
    pub put_PaneDisplayMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        NavigationViewPaneDisplayMode,
    ) -> windows_core::HRESULT,
    get_PaneHeader: usize,
    put_PaneHeader: usize,
    get_PaneCustomContent: usize,
    put_PaneCustomContent: usize,
    get_ContentOverlay: usize,
    put_ContentOverlay: usize,
    get_IsPaneVisible: usize,
    put_IsPaneVisible: usize,
    get_SelectionFollowsFocus: usize,
    put_SelectionFollowsFocus: usize,
    get_TemplateSettings: usize,
    get_ShoulderNavigationEnabled: usize,
    put_ShoulderNavigationEnabled: usize,
    get_OverflowLabelMode: usize,
    put_OverflowLabelMode: usize,
    add_Expanding: usize,
    remove_Expanding: usize,
    add_Collapsed: usize,
    remove_Collapsed: usize,
    Expand: usize,
    Collapse: usize,
}
windows_core::imp::define_interface!(
    INavigationViewBackRequestedEventArgs,
    INavigationViewBackRequestedEventArgs_Vtbl,
    0xae752207_bd1b_5afa_a872_e9bbaeea0ede
);
impl windows_core::RuntimeType for INavigationViewBackRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewBackRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    INavigationViewFactory,
    INavigationViewFactory_Vtbl,
    0xffea1ada_9232_5507_a320_ed2fadbe6127
);
impl windows_core::RuntimeType for INavigationViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewItem,
    INavigationViewItem_Vtbl,
    0x3ab3d503_a37c_5836_8adb_2882062e73a1
);
impl windows_core::RuntimeType for INavigationViewItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INavigationViewItem {
    pub fn put_Icon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IconElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Icon)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Icon: usize,
    pub put_Icon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_CompactPaneLength: usize,
}
windows_core::imp::define_interface!(
    INavigationViewItem2,
    INavigationViewItem2_Vtbl,
    0x2d5bd889_9dac_5675_b254_68226f077a61
);
impl windows_core::RuntimeType for INavigationViewItem2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INavigationViewItem2 {
    pub fn get_MenuItems(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MenuItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItem2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_SelectsOnInvoked: usize,
    put_SelectsOnInvoked: usize,
    get_IsExpanded: usize,
    put_IsExpanded: usize,
    get_HasUnrealizedChildren: usize,
    put_HasUnrealizedChildren: usize,
    get_IsChildSelected: usize,
    put_IsChildSelected: usize,
    pub get_MenuItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_MenuItemsSource: usize,
    put_MenuItemsSource: usize,
}
windows_core::imp::define_interface!(
    INavigationViewItemBase,
    INavigationViewItemBase_Vtbl,
    0x33586494_af48_513f_be4d_f645e8c89005
);
impl windows_core::RuntimeType for INavigationViewItemBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    INavigationViewItemFactory,
    INavigationViewItemFactory_Vtbl,
    0xde60a001_9385_5535_80e1_2b68f4bfde26
);
impl windows_core::RuntimeType for INavigationViewItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewItemHeader,
    INavigationViewItemHeader_Vtbl,
    0x432bc062_45bc_57ef_a2d3_11851a56a882
);
impl windows_core::RuntimeType for INavigationViewItemHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemHeader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    INavigationViewItemHeaderFactory,
    INavigationViewItemHeaderFactory_Vtbl,
    0x6a5447cd_2918_5fe3_899b_93d6961285e6
);
impl windows_core::RuntimeType for INavigationViewItemHeaderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemHeaderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewSelectionChangedEventArgs,
    INavigationViewSelectionChangedEventArgs_Vtbl,
    0x14a064a5_c79d_5f63_ac6e_1c313fe63566
);
impl windows_core::RuntimeType for INavigationViewSelectionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INavigationViewSelectionChangedEventArgs {
    pub fn get_SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewSelectionChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_SelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsSettingsSelected: usize,
}
windows_core::imp::define_interface!(
    INumberBox,
    INumberBox_Vtbl,
    0xc18eb0e9_29fb_525d_abbc_d6b2110f542e
);
impl windows_core::RuntimeType for INumberBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INumberBox {
    pub fn put_Minimum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Minimum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Maximum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Maximum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Value(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Value)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn add_ValueChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<NumberBox>, windows_core::Ref<NumberBoxValueChangedEventArgs>)
            + 'static,
    {
        let handler: TypedEventHandler<NumberBox, NumberBoxValueChangedEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<NumberBox, NumberBoxValueChangedEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<NumberBox, NumberBoxValueChangedEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ValueChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ValueChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Minimum: usize,
    pub put_Minimum:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Maximum: usize,
    pub put_Maximum:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Value: usize,
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_SmallChange: usize,
    put_SmallChange: usize,
    get_LargeChange: usize,
    put_LargeChange: usize,
    get_Text: usize,
    put_Text: usize,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_PlaceholderText: usize,
    put_PlaceholderText: usize,
    get_SelectionFlyout: usize,
    put_SelectionFlyout: usize,
    get_SelectionHighlightColor: usize,
    put_SelectionHighlightColor: usize,
    get_TextReadingOrder: usize,
    put_TextReadingOrder: usize,
    get_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    put_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    get_Description: usize,
    put_Description: usize,
    get_ValidationMode: usize,
    put_ValidationMode: usize,
    get_SpinButtonPlacementMode: usize,
    put_SpinButtonPlacementMode: usize,
    get_IsWrapEnabled: usize,
    put_IsWrapEnabled: usize,
    get_AcceptsExpression: usize,
    put_AcceptsExpression: usize,
    get_NumberFormatter: usize,
    put_NumberFormatter: usize,
    pub add_ValueChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ValueChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INumberBoxFactory,
    INumberBoxFactory_Vtbl,
    0x6b81f3cb_45a4_5d19_9bbb_a9fe4656ac4d
);
impl windows_core::RuntimeType for INumberBoxFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberBoxFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INumberBoxValueChangedEventArgs,
    INumberBoxValueChangedEventArgs_Vtbl,
    0xc66cf16e_7c8a_532e_9d23_058c1c98dd50
);
impl windows_core::RuntimeType for INumberBoxValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INumberBoxValueChangedEventArgs {
    pub fn get_NewValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NewValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberBoxValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_OldValue: usize,
    pub get_NewValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IOverlappedPresenter3,
    IOverlappedPresenter3_Vtbl,
    0x55d26138_4c38_57e7_a0c1_d467b774db8c
);
impl windows_core::RuntimeType for IOverlappedPresenter3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IOverlappedPresenter3 {
    pub fn put_PreferredMinimumHeight(&self, value: Option<i32>) -> windows_core::Result<()> {
        let value__ =
            value.map(<windows_reference::IReference<i32> as core::convert::From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).put_PreferredMinimumHeight)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn put_PreferredMinimumWidth(&self, value: Option<i32>) -> windows_core::Result<()> {
        let value__ =
            value.map(<windows_reference::IReference<i32> as core::convert::From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).put_PreferredMinimumWidth)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn put_PreferredMaximumWidth(&self, value: Option<i32>) -> windows_core::Result<()> {
        let value__ =
            value.map(<windows_reference::IReference<i32> as core::convert::From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).put_PreferredMaximumWidth)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn put_PreferredMaximumHeight(&self, value: Option<i32>) -> windows_core::Result<()> {
        let value__ =
            value.map(<windows_reference::IReference<i32> as core::convert::From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).put_PreferredMaximumHeight)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenter3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_PreferredMinimumHeight: usize,
    pub put_PreferredMinimumHeight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_PreferredMinimumWidth: usize,
    pub put_PreferredMinimumWidth: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_PreferredMaximumWidth: usize,
    pub put_PreferredMaximumWidth: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_PreferredMaximumHeight: usize,
    pub put_PreferredMaximumHeight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPanel, IPanel_Vtbl, 0x27a1b418_56f3_525e_b883_cefed905eed3);
impl windows_core::RuntimeType for IPanel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPanel {
    pub fn get_Children(&self) -> windows_core::Result<UIElementCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Children)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_Background<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Background)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPanel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Children: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Background: usize,
    pub put_Background: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsItemsHost: usize,
    get_ChildrenTransitions: usize,
    put_ChildrenTransitions: usize,
    get_BackgroundTransition: usize,
    put_BackgroundTransition: usize,
}
windows_core::imp::define_interface!(
    IParagraph,
    IParagraph_Vtbl,
    0x9ed64c77_329d_502f_a257_f58398edab51
);
impl windows_core::RuntimeType for IParagraph {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IParagraph {
    pub fn get_Inlines(&self) -> windows_core::Result<InlineCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Inlines)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraph_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Inlines: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TextIndent: usize,
    put_TextIndent: usize,
}
windows_core::imp::define_interface!(
    IPasswordBox,
    IPasswordBox_Vtbl,
    0x6d3ccff7_aaee_5adc_8298_33300fa119da
);
impl windows_core::RuntimeType for IPasswordBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPasswordBox {
    pub fn get_Password(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Password)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn put_Password(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Password)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_IsPasswordRevealButtonEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsPasswordRevealButtonEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_PlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_PasswordRevealMode(&self, value: PasswordRevealMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PasswordRevealMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_PasswordChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_PasswordChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_PasswordChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPasswordBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Password: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Password: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_PasswordChar: usize,
    put_PasswordChar: usize,
    get_IsPasswordRevealButtonEnabled: usize,
    pub put_IsPasswordRevealButtonEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_MaxLength: usize,
    put_MaxLength: usize,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_PlaceholderText: usize,
    pub put_PlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_SelectionHighlightColor: usize,
    put_SelectionHighlightColor: usize,
    get_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    put_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    get_PasswordRevealMode: usize,
    pub put_PasswordRevealMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PasswordRevealMode,
    ) -> windows_core::HRESULT,
    get_TextReadingOrder: usize,
    put_TextReadingOrder: usize,
    get_InputScope: usize,
    put_InputScope: usize,
    get_CanPasteClipboardContent: usize,
    get_SelectionFlyout: usize,
    put_SelectionFlyout: usize,
    get_Description: usize,
    put_Description: usize,
    pub add_PasswordChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_PasswordChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_ContextMenuOpening: usize,
    remove_ContextMenuOpening: usize,
    add_Paste: usize,
    remove_Paste: usize,
    add_PasswordChanging: usize,
    remove_PasswordChanging: usize,
    SelectAll: usize,
    PasteFromClipboard: usize,
}
windows_core::imp::define_interface!(
    IPersonPicture,
    IPersonPicture_Vtbl,
    0x30ec982c_0efa_5538_a356_e9ebd5aa363c
);
impl windows_core::RuntimeType for IPersonPicture {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPersonPicture {
    pub fn put_DisplayName(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_DisplayName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Initials(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Initials)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersonPicture_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_BadgeNumber: usize,
    put_BadgeNumber: usize,
    get_BadgeGlyph: usize,
    put_BadgeGlyph: usize,
    get_BadgeImageSource: usize,
    put_BadgeImageSource: usize,
    get_BadgeText: usize,
    put_BadgeText: usize,
    get_IsGroup: usize,
    put_IsGroup: usize,
    get_Contact: usize,
    put_Contact: usize,
    get_DisplayName: usize,
    pub put_DisplayName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Initials: usize,
    pub put_Initials: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_PreferSmallImage: usize,
    put_PreferSmallImage: usize,
    get_ProfilePicture: usize,
    put_ProfilePicture: usize,
    get_TemplateSettings: usize,
}
windows_core::imp::define_interface!(
    IPersonPictureFactory,
    IPersonPictureFactory_Vtbl,
    0x7ec0794c_f2cc_5282_a89d_cd3bb765b71a
);
impl windows_core::RuntimeType for IPersonPictureFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersonPictureFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPivot, IPivot_Vtbl, 0x1c6438e5_ecac_5fb6_8e8e_00de7e922303);
impl windows_core::RuntimeType for IPivot {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPivot {
    pub fn put_Title<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Title)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_SelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_SelectionChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<SelectionChangedEventArgs>,
            ) + 'static,
    {
        let handler: SelectionChangedEventHandler = {
            let com = windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::new(
                &SelectionChangedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectionChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivot_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Title: usize,
    pub put_Title: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TitleTemplate: usize,
    put_TitleTemplate: usize,
    get_LeftHeader: usize,
    put_LeftHeader: usize,
    get_LeftHeaderTemplate: usize,
    put_LeftHeaderTemplate: usize,
    get_RightHeader: usize,
    put_RightHeader: usize,
    get_RightHeaderTemplate: usize,
    put_RightHeaderTemplate: usize,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_SelectedIndex: usize,
    pub put_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_SelectedItem: usize,
    put_SelectedItem: usize,
    get_IsLocked: usize,
    put_IsLocked: usize,
    get_HeaderFocusVisualPlacement: usize,
    put_HeaderFocusVisualPlacement: usize,
    get_IsHeaderItemsCarouselEnabled: usize,
    put_IsHeaderItemsCarouselEnabled: usize,
    pub add_SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_PivotItemLoading: usize,
    remove_PivotItemLoading: usize,
    add_PivotItemLoaded: usize,
    remove_PivotItemLoaded: usize,
    add_PivotItemUnloading: usize,
    remove_PivotItemUnloading: usize,
    add_PivotItemUnloaded: usize,
    remove_PivotItemUnloaded: usize,
}
windows_core::imp::define_interface!(
    IPivotFactory,
    IPivotFactory_Vtbl,
    0xda9d033c_4782_5a69_90af_076ccdf071ae
);
impl windows_core::RuntimeType for IPivotFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPivotItem,
    IPivotItem_Vtbl,
    0x8108c195_63f5_5df9_abcf_418fa2dbfbec
);
impl windows_core::RuntimeType for IPivotItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPivotItem {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPivotItemFactory,
    IPivotItemFactory_Vtbl,
    0x9149a2ab_606c_55a9_8786_801d55ca8ed6
);
impl windows_core::RuntimeType for IPivotItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPointerPoint,
    IPointerPoint_Vtbl,
    0x0d430ee6_252c_59a4_b2a2_d44264dc6a40
);
impl windows_core::RuntimeType for IPointerPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPointerPoint {
    pub fn get_Properties(&self) -> windows_core::Result<PointerPointProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Properties)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPoint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_FrameId: usize,
    get_IsInContact: usize,
    get_PointerDeviceType: usize,
    get_PointerId: usize,
    get_Position: usize,
    pub get_Properties: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Timestamp: usize,
    GetTransformedPoint: usize,
}
windows_core::imp::define_interface!(
    IPointerPointProperties,
    IPointerPointProperties_Vtbl,
    0xd760ed77_4b10_57a5_b3cc_d9bf3413e996
);
impl windows_core::RuntimeType for IPointerPointProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPointerPointProperties {
    pub fn get_IsLeftButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsLeftButtonPressed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_IsMiddleButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsMiddleButtonPressed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_IsRightButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsRightButtonPressed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_ContactRect: usize,
    get_IsBarrelButtonPressed: usize,
    get_IsCanceled: usize,
    get_IsEraser: usize,
    get_IsHorizontalMouseWheel: usize,
    get_IsInRange: usize,
    get_IsInverted: usize,
    pub get_IsLeftButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub get_IsMiddleButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    get_IsPrimary: usize,
    pub get_IsRightButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    get_IsXButton1Pressed: usize,
    get_IsXButton2Pressed: usize,
    get_MouseWheelDelta: usize,
    get_Orientation: usize,
    get_PointerUpdateKind: usize,
    get_Pressure: usize,
    get_TouchConfidence: usize,
    get_Twist: usize,
    get_XTilt: usize,
    get_YTilt: usize,
}
windows_core::imp::define_interface!(
    IPointerRoutedEventArgs,
    IPointerRoutedEventArgs_Vtbl,
    0x66e78a9a_1bec_5f92_b1a1_ea6334ee511c
);
impl windows_core::RuntimeType for IPointerRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPointerRoutedEventArgs {
    pub fn GetCurrentPoint<P0>(&self, relativeto: P0) -> windows_core::Result<PointerPoint>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPoint)(
                windows_core::Interface::as_raw(self),
                relativeto.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Pointer: usize,
    get_KeyModifiers: usize,
    get_Handled: usize,
    put_Handled: usize,
    get_IsGenerated: usize,
    pub GetCurrentPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetIntermediatePoints: usize,
}
windows_core::imp::define_interface!(
    IProgressBar,
    IProgressBar_Vtbl,
    0x87555c8c_0aaf_52c1_8390_0db17f40438e
);
impl windows_core::RuntimeType for IProgressBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IProgressBar {
    pub fn put_IsIndeterminate(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsIndeterminate)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsIndeterminate: usize,
    pub put_IsIndeterminate:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_ShowError: usize,
    put_ShowError: usize,
    get_ShowPaused: usize,
    put_ShowPaused: usize,
    get_TemplateSettings: usize,
}
windows_core::imp::define_interface!(
    IProgressBarFactory,
    IProgressBarFactory_Vtbl,
    0x189826ad_f6f2_533e_9ddb_b6600e88675b
);
impl windows_core::RuntimeType for IProgressBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IProgressRing,
    IProgressRing_Vtbl,
    0x2670d03f_e28c_5652_bee2_b5212ebdf7ff
);
impl windows_core::RuntimeType for IProgressRing {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IProgressRing {
    pub fn put_IsActive(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsActive)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsIndeterminate(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsIndeterminate)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Minimum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Minimum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Maximum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Maximum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressRing_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsActive: usize,
    pub put_IsActive:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsIndeterminate: usize,
    pub put_IsIndeterminate:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_TemplateSettings: usize,
    get_Value: usize,
    put_Value: usize,
    get_Minimum: usize,
    pub put_Minimum:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Maximum: usize,
    pub put_Maximum:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IProgressRingFactory,
    IProgressRingFactory_Vtbl,
    0x092fa98c_62a7_5dbc_9a85_3e556ba81f79
);
impl windows_core::RuntimeType for IProgressRingFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressRingFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRadioButton,
    IRadioButton_Vtbl,
    0x38f30cee_e75a_5ba1_ae64_4474a3abeac7
);
impl windows_core::RuntimeType for IRadioButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRadioButton {
    pub fn put_GroupName(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_GroupName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_GroupName: usize,
    pub put_GroupName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRadioButtonFactory,
    IRadioButtonFactory_Vtbl,
    0x5772c79a_b3eb_5719_8005_2a513429495a
);
impl windows_core::RuntimeType for IRadioButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRadioButtons,
    IRadioButtons_Vtbl,
    0x966daf80_ee85_5d90_b6b3_80bec9134673
);
impl windows_core::RuntimeType for IRadioButtons {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRadioButtons {
    pub fn get_Items(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_SelectedIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_SelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_SelectionChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<SelectionChangedEventArgs>,
            ) + 'static,
    {
        let handler: SelectionChangedEventHandler = {
            let com = windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::new(
                &SelectionChangedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectionChanged,
            ))
        }
    }
    pub fn put_MaxColumns(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MaxColumns)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioButtons_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_ItemsSource: usize,
    put_ItemsSource: usize,
    pub get_Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ItemTemplate: usize,
    put_ItemTemplate: usize,
    ContainerFromIndex: usize,
    pub get_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_SelectedItem: usize,
    put_SelectedItem: usize,
    pub add_SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    get_MaxColumns: usize,
    pub put_MaxColumns:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
}
windows_core::imp::define_interface!(
    IRadioButtonsFactory,
    IRadioButtonsFactory_Vtbl,
    0x2cf95efb_a7a2_5d85_8ead_ea222baa3c55
);
impl windows_core::RuntimeType for IRadioButtonsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioButtonsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRangeBase,
    IRangeBase_Vtbl,
    0x540d6d61_8fac_5d5c_b5b0_e172a7dde103
);
impl windows_core::RuntimeType for IRangeBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRangeBase {
    pub fn put_Minimum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Minimum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Maximum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Maximum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_SmallChange(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SmallChange)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_Value(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_Value(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Value)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_ValueChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<RangeBaseValueChangedEventArgs>,
            ) + 'static,
    {
        let handler: RangeBaseValueChangedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RangeBaseValueChangedEventHandler, F>::new(
                &RangeBaseValueChangedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ValueChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ValueChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Minimum: usize,
    pub put_Minimum:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Maximum: usize,
    pub put_Maximum:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_SmallChange: usize,
    pub put_SmallChange:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_LargeChange: usize,
    put_LargeChange: usize,
    pub get_Value:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub add_ValueChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ValueChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRangeBaseValueChangedEventArgs,
    IRangeBaseValueChangedEventArgs_Vtbl,
    0xb0181692_9578_51c7_9d1c_adfcf8945aa9
);
impl windows_core::RuntimeType for IRangeBaseValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRangeBaseValueChangedEventArgs {
    pub fn get_NewValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NewValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_OldValue: usize,
    pub get_NewValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRatingControl,
    IRatingControl_Vtbl,
    0x5488193b_ea4b_52c6_8544_c063219bcd90
);
impl windows_core::RuntimeType for IRatingControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRatingControl {
    pub fn put_Caption(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Caption)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_IsReadOnly(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsReadOnly)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_MaxRating(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MaxRating)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_PlaceholderValue(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderValue)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_Value(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_Value(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Value)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_ValueChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<RatingControl>, windows_core::Ref<windows_core::IInspectable>)
            + 'static,
    {
        let handler: TypedEventHandler<RatingControl, windows_core::IInspectable> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<RatingControl, windows_core::IInspectable>,
                F,
            >::new(
                &TypedEventHandlerBox::<RatingControl, windows_core::IInspectable, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ValueChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ValueChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatingControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Caption: usize,
    pub put_Caption: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_InitialSetValue: usize,
    put_InitialSetValue: usize,
    get_IsClearEnabled: usize,
    put_IsClearEnabled: usize,
    get_IsReadOnly: usize,
    pub put_IsReadOnly:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_MaxRating: usize,
    pub put_MaxRating:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_PlaceholderValue: usize,
    pub put_PlaceholderValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_ItemInfo: usize,
    put_ItemInfo: usize,
    pub get_Value:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub add_ValueChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ValueChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRatingControlFactory,
    IRatingControlFactory_Vtbl,
    0xa53b9b73_bff9_548d_a294_ac63d819f78a
);
impl windows_core::RuntimeType for IRatingControlFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatingControlFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRectangle,
    IRectangle_Vtbl,
    0xbf7d30b9_152c_556e_9f10_d0b7eba4e52f
);
impl windows_core::RuntimeType for IRectangle {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRectangle {
    pub fn put_RadiusX(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_RadiusX)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_RadiusY(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_RadiusY)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangle_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_RadiusX: usize,
    pub put_RadiusX:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_RadiusY: usize,
    pub put_RadiusY:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRelativePanel,
    IRelativePanel_Vtbl,
    0xc432fcc4_88f2_59d8_9d0e_a237beaeb07f
);
impl windows_core::RuntimeType for IRelativePanel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRelativePanel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_BackgroundSizing: usize,
    put_BackgroundSizing: usize,
    get_BorderBrush: usize,
    put_BorderBrush: usize,
    get_BorderThickness: usize,
    put_BorderThickness: usize,
    get_CornerRadius: usize,
    put_CornerRadius: usize,
    get_Padding: usize,
    put_Padding: usize,
}
windows_core::imp::define_interface!(
    IRelativePanelFactory,
    IRelativePanelFactory_Vtbl,
    0xc85f1443_d973_50fd_9497_b867f492468f
);
impl windows_core::RuntimeType for IRelativePanelFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRelativePanelFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRelativePanelStatics,
    IRelativePanelStatics_Vtbl,
    0xbdd929a2_76cc_59c4_82c1_f14b5da4221a
);
impl windows_core::RuntimeType for IRelativePanelStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRelativePanelStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_BackgroundSizingProperty: usize,
    get_LeftOfProperty: usize,
    GetLeftOf: usize,
    SetLeftOf: usize,
    get_AboveProperty: usize,
    GetAbove: usize,
    SetAbove: usize,
    get_RightOfProperty: usize,
    GetRightOf: usize,
    SetRightOf: usize,
    get_BelowProperty: usize,
    GetBelow: usize,
    SetBelow: usize,
    get_AlignHorizontalCenterWithProperty: usize,
    GetAlignHorizontalCenterWith: usize,
    SetAlignHorizontalCenterWith: usize,
    get_AlignVerticalCenterWithProperty: usize,
    GetAlignVerticalCenterWith: usize,
    SetAlignVerticalCenterWith: usize,
    get_AlignLeftWithProperty: usize,
    GetAlignLeftWith: usize,
    SetAlignLeftWith: usize,
    get_AlignTopWithProperty: usize,
    GetAlignTopWith: usize,
    SetAlignTopWith: usize,
    get_AlignRightWithProperty: usize,
    GetAlignRightWith: usize,
    SetAlignRightWith: usize,
    get_AlignBottomWithProperty: usize,
    GetAlignBottomWith: usize,
    SetAlignBottomWith: usize,
    get_AlignLeftWithPanelProperty: usize,
    GetAlignLeftWithPanel: usize,
    pub SetAlignLeftWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    get_AlignTopWithPanelProperty: usize,
    GetAlignTopWithPanel: usize,
    pub SetAlignTopWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    get_AlignRightWithPanelProperty: usize,
    GetAlignRightWithPanel: usize,
    pub SetAlignRightWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    get_AlignBottomWithPanelProperty: usize,
    GetAlignBottomWithPanel: usize,
    pub SetAlignBottomWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    get_AlignHorizontalCenterWithPanelProperty: usize,
    GetAlignHorizontalCenterWithPanel: usize,
    pub SetAlignHorizontalCenterWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    get_AlignVerticalCenterWithPanelProperty: usize,
    GetAlignVerticalCenterWithPanel: usize,
    pub SetAlignVerticalCenterWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    get_BorderBrushProperty: usize,
    get_BorderThicknessProperty: usize,
    get_CornerRadiusProperty: usize,
    get_PaddingProperty: usize,
}
windows_core::imp::define_interface!(
    IRepeatButton,
    IRepeatButton_Vtbl,
    0x97f4c728_4a94_56b5_91e4_e7c6f6a1251a
);
impl windows_core::RuntimeType for IRepeatButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRepeatButton {
    pub fn put_Delay(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Delay)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Interval(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Interval)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepeatButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Delay: usize,
    pub put_Delay: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_Interval: usize,
    pub put_Interval:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IResourceDictionary,
    IResourceDictionary_Vtbl,
    0x1b690975_a710_5783_a6e1_15836f6186c2
);
impl windows_core::RuntimeType for IResourceDictionary {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IResourceDictionary {
    pub fn get_MergedDictionaries(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<ResourceDictionary>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MergedDictionaries)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceDictionary_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Source: usize,
    put_Source: usize,
    pub get_MergedDictionaries: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ThemeDictionaries: usize,
}
windows_core::imp::define_interface!(
    IResourceDictionaryFactory,
    IResourceDictionaryFactory_Vtbl,
    0xea22a48f_ab71_56f6_a392_d82310c8aa7b
);
impl windows_core::RuntimeType for IResourceDictionaryFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceDictionaryFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRichEditBox,
    IRichEditBox_Vtbl,
    0x699163db_723d_5514_a8c9_2c64d99e1ea6
);
impl windows_core::RuntimeType for IRichEditBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRichEditBox {
    pub fn put_IsReadOnly(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsReadOnly)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_Document(&self) -> windows_core::Result<RichEditTextDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Document)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_PlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn add_TextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_TextChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_TextChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsReadOnly: usize,
    pub put_IsReadOnly:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_AcceptsReturn: usize,
    put_AcceptsReturn: usize,
    get_TextAlignment: usize,
    put_TextAlignment: usize,
    get_TextWrapping: usize,
    put_TextWrapping: usize,
    get_IsSpellCheckEnabled: usize,
    put_IsSpellCheckEnabled: usize,
    get_IsTextPredictionEnabled: usize,
    put_IsTextPredictionEnabled: usize,
    pub get_Document: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_InputScope: usize,
    put_InputScope: usize,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_PlaceholderText: usize,
    pub put_PlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_SelectionHighlightColor: usize,
    put_SelectionHighlightColor: usize,
    get_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    put_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    get_IsColorFontEnabled: usize,
    put_IsColorFontEnabled: usize,
    get_SelectionHighlightColorWhenNotFocused: usize,
    put_SelectionHighlightColorWhenNotFocused: usize,
    get_MaxLength: usize,
    put_MaxLength: usize,
    get_HorizontalTextAlignment: usize,
    put_HorizontalTextAlignment: usize,
    get_CharacterCasing: usize,
    put_CharacterCasing: usize,
    get_DisabledFormattingAccelerators: usize,
    put_DisabledFormattingAccelerators: usize,
    get_TextDocument: usize,
    get_SelectionFlyout: usize,
    put_SelectionFlyout: usize,
    get_ProofingMenuFlyout: usize,
    get_Description: usize,
    put_Description: usize,
    pub add_TextChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_TextChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_SelectionChanged: usize,
    remove_SelectionChanged: usize,
    add_ContextMenuOpening: usize,
    remove_ContextMenuOpening: usize,
    add_Paste: usize,
    remove_Paste: usize,
    add_TextCompositionStarted: usize,
    remove_TextCompositionStarted: usize,
    add_TextCompositionChanged: usize,
    remove_TextCompositionChanged: usize,
    add_TextCompositionEnded: usize,
    remove_TextCompositionEnded: usize,
    add_CopyingToClipboard: usize,
    remove_CopyingToClipboard: usize,
    add_CuttingToClipboard: usize,
    remove_CuttingToClipboard: usize,
    add_SelectionChanging: usize,
    remove_SelectionChanging: usize,
    GetLinguisticAlternativesAsync: usize,
    get_TextReadingOrder: usize,
    put_TextReadingOrder: usize,
    get_ClipboardCopyFormat: usize,
    put_ClipboardCopyFormat: usize,
    get_DesiredCandidateWindowAlignment: usize,
    put_DesiredCandidateWindowAlignment: usize,
    add_CandidateWindowBoundsChanged: usize,
    remove_CandidateWindowBoundsChanged: usize,
    add_TextChanging: usize,
    remove_TextChanging: usize,
}
windows_core::imp::define_interface!(
    IRichEditBoxFactory,
    IRichEditBoxFactory_Vtbl,
    0x7c993c60_f5b6_589f_bba8_b68b9713e4ae
);
impl windows_core::RuntimeType for IRichEditBoxFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditBoxFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRichTextBlock,
    IRichTextBlock_Vtbl,
    0xd766e4db_a684_50b7_a202_c8e91fa26ff3
);
impl windows_core::RuntimeType for IRichTextBlock {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRichTextBlock {
    pub fn put_FontSize(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_FontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FontFamily>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontFamily)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_Foreground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Foreground)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_TextWrapping(&self, value: TextWrapping) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_TextWrapping)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_Blocks(&self) -> windows_core::Result<BlockCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Blocks)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_IsTextSelectionEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsTextSelectionEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichTextBlock_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_FontSize: usize,
    pub put_FontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_FontFamily: usize,
    pub put_FontFamily: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_FontWeight: usize,
    put_FontWeight: usize,
    get_FontStyle: usize,
    put_FontStyle: usize,
    get_FontStretch: usize,
    put_FontStretch: usize,
    get_Foreground: usize,
    pub put_Foreground: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TextWrapping: usize,
    pub put_TextWrapping:
        unsafe extern "system" fn(*mut core::ffi::c_void, TextWrapping) -> windows_core::HRESULT,
    get_TextTrimming: usize,
    put_TextTrimming: usize,
    get_TextAlignment: usize,
    put_TextAlignment: usize,
    pub get_Blocks: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Padding: usize,
    put_Padding: usize,
    get_LineHeight: usize,
    put_LineHeight: usize,
    get_LineStackingStrategy: usize,
    put_LineStackingStrategy: usize,
    get_CharacterSpacing: usize,
    put_CharacterSpacing: usize,
    get_OverflowContentTarget: usize,
    put_OverflowContentTarget: usize,
    get_IsTextSelectionEnabled: usize,
    pub put_IsTextSelectionEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_HasOverflowContent: usize,
    get_SelectedText: usize,
    get_ContentStart: usize,
    get_ContentEnd: usize,
    get_SelectionStart: usize,
    get_SelectionEnd: usize,
    get_BaselineOffset: usize,
    get_MaxLines: usize,
    put_MaxLines: usize,
    get_TextLineBounds: usize,
    put_TextLineBounds: usize,
    get_SelectionHighlightColor: usize,
    put_SelectionHighlightColor: usize,
    get_OpticalMarginAlignment: usize,
    put_OpticalMarginAlignment: usize,
    get_IsColorFontEnabled: usize,
    put_IsColorFontEnabled: usize,
    get_TextReadingOrder: usize,
    put_TextReadingOrder: usize,
    get_IsTextScaleFactorEnabled: usize,
    put_IsTextScaleFactorEnabled: usize,
    get_TextDecorations: usize,
    put_TextDecorations: usize,
    get_IsTextTrimmed: usize,
    get_HorizontalTextAlignment: usize,
    put_HorizontalTextAlignment: usize,
    get_TextHighlighters: usize,
    get_SelectionFlyout: usize,
    put_SelectionFlyout: usize,
    add_SelectionChanged: usize,
    remove_SelectionChanged: usize,
    add_ContextMenuOpening: usize,
    remove_ContextMenuOpening: usize,
    add_IsTextTrimmedChanged: usize,
    remove_IsTextTrimmedChanged: usize,
    SelectAll: usize,
    Select: usize,
    GetPositionFromPoint: usize,
    CopySelectionToClipboard: usize,
    get_TextIndent: usize,
    put_TextIndent: usize,
}
windows_core::imp::define_interface!(
    IRightTappedRoutedEventArgs,
    IRightTappedRoutedEventArgs_Vtbl,
    0x3972fafb_2915_5c62_bb6b_54ad84ff400d
);
impl windows_core::RuntimeType for IRightTappedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_PointerDeviceType: usize,
    get_Handled: usize,
    put_Handled: usize,
    GetPosition: usize,
}
windows_core::imp::define_interface!(
    IRoutedEventArgs,
    IRoutedEventArgs_Vtbl,
    0x0908c407_1c7d_5de3_9c50_d971c62ec8ec
);
impl windows_core::RuntimeType for IRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_OriginalSource: usize,
}
windows_core::imp::define_interface!(
    IRowDefinition,
    IRowDefinition_Vtbl,
    0xfe870f2f_89ef_5dac_9f33_968d0dc577c3
);
impl windows_core::RuntimeType for IRowDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRowDefinition {
    pub fn put_Height(&self, value: GridLength) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Height)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Height: usize,
    pub put_Height:
        unsafe extern "system" fn(*mut core::ffi::c_void, GridLength) -> windows_core::HRESULT,
    get_MaxHeight: usize,
    put_MaxHeight: usize,
    get_MinHeight: usize,
    put_MinHeight: usize,
    get_ActualHeight: usize,
}
windows_core::imp::define_interface!(IRun, IRun_Vtbl, 0x1f905239_37cb_590b_9132_3ffb7741906e);
impl windows_core::RuntimeType for IRun {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRun {
    pub fn put_Text(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Text)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRun_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Text: usize,
    pub put_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_FlowDirection: usize,
    put_FlowDirection: usize,
}
windows_core::imp::define_interface!(
    IScalarKeyFrameAnimation,
    IScalarKeyFrameAnimation_Vtbl,
    0x5a5f8abe_d129_5b25_8aff_8180fd9bfb22
);
impl windows_core::RuntimeType for IScalarKeyFrameAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IScalarKeyFrameAnimation {
    pub fn InsertKeyFrame(
        &self,
        normalizedprogresskey: f32,
        value: f32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).InsertKeyFrame)(
                windows_core::Interface::as_raw(self),
                normalizedprogresskey,
                value,
            )
            .ok()
        }
    }
    pub fn InsertKeyFrameWithEasingFunction<P2>(
        &self,
        normalizedprogresskey: f32,
        value: f32,
        easingfunction: P2,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<CompositionEasingFunction>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InsertKeyFrameWithEasingFunction)(
                windows_core::Interface::as_raw(self),
                normalizedprogresskey,
                value,
                easingfunction.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScalarKeyFrameAnimation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InsertKeyFrame:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32) -> windows_core::HRESULT,
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        f32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IScrollView,
    IScrollView_Vtbl,
    0x8c98c86d_378a_5102_a1e3_3352280fa010
);
impl windows_core::RuntimeType for IScrollView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IScrollView {
    pub fn put_Content<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Content)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_HorizontalScrollBarVisibility(
        &self,
        value: ScrollingScrollBarVisibility,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_HorizontalScrollBarVisibility)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_VerticalScrollBarVisibility(
        &self,
        value: ScrollingScrollBarVisibility,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_VerticalScrollBarVisibility)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Content: usize,
    pub put_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_CurrentAnchor: usize,
    get_ScrollPresenter: usize,
    get_ExpressionAnimationSources: usize,
    get_HorizontalOffset: usize,
    get_VerticalOffset: usize,
    get_ZoomFactor: usize,
    get_ExtentWidth: usize,
    get_ExtentHeight: usize,
    get_ViewportWidth: usize,
    get_ViewportHeight: usize,
    get_ScrollableWidth: usize,
    get_ScrollableHeight: usize,
    get_State: usize,
    get_HorizontalScrollBarVisibility: usize,
    pub put_HorizontalScrollBarVisibility: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ScrollingScrollBarVisibility,
    ) -> windows_core::HRESULT,
    get_VerticalScrollBarVisibility: usize,
    pub put_VerticalScrollBarVisibility: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ScrollingScrollBarVisibility,
    ) -> windows_core::HRESULT,
    get_ContentOrientation: usize,
    put_ContentOrientation: usize,
    get_HorizontalScrollChainMode: usize,
    put_HorizontalScrollChainMode: usize,
    get_VerticalScrollChainMode: usize,
    put_VerticalScrollChainMode: usize,
    get_HorizontalScrollRailMode: usize,
    put_HorizontalScrollRailMode: usize,
    get_VerticalScrollRailMode: usize,
    put_VerticalScrollRailMode: usize,
    get_HorizontalScrollMode: usize,
    put_HorizontalScrollMode: usize,
    get_VerticalScrollMode: usize,
    put_VerticalScrollMode: usize,
    get_ComputedHorizontalScrollBarVisibility: usize,
    get_ComputedVerticalScrollBarVisibility: usize,
    get_ComputedHorizontalScrollMode: usize,
    get_ComputedVerticalScrollMode: usize,
    get_ZoomChainMode: usize,
    put_ZoomChainMode: usize,
    get_ZoomMode: usize,
    put_ZoomMode: usize,
    get_IgnoredInputKinds: usize,
    put_IgnoredInputKinds: usize,
    get_MinZoomFactor: usize,
    put_MinZoomFactor: usize,
    get_MaxZoomFactor: usize,
    put_MaxZoomFactor: usize,
    get_HorizontalAnchorRatio: usize,
    put_HorizontalAnchorRatio: usize,
    get_VerticalAnchorRatio: usize,
    put_VerticalAnchorRatio: usize,
    RegisterAnchorCandidate: usize,
    UnregisterAnchorCandidate: usize,
    ScrollTo: usize,
    ScrollToWithOptions: usize,
    ScrollBy: usize,
    ScrollByWithOptions: usize,
    AddScrollVelocity: usize,
    ZoomTo: usize,
    ZoomToWithOptions: usize,
    ZoomBy: usize,
    ZoomByWithOptions: usize,
    AddZoomVelocity: usize,
    add_ExtentChanged: usize,
    remove_ExtentChanged: usize,
    add_StateChanged: usize,
    remove_StateChanged: usize,
    add_ViewChanged: usize,
    remove_ViewChanged: usize,
    add_ScrollAnimationStarting: usize,
    remove_ScrollAnimationStarting: usize,
    add_ZoomAnimationStarting: usize,
    remove_ZoomAnimationStarting: usize,
    add_ScrollCompleted: usize,
    remove_ScrollCompleted: usize,
    add_ZoomCompleted: usize,
    remove_ZoomCompleted: usize,
    add_BringingIntoView: usize,
    remove_BringingIntoView: usize,
    add_AnchorRequested: usize,
    remove_AnchorRequested: usize,
}
windows_core::imp::define_interface!(
    IScrollViewFactory,
    IScrollViewFactory_Vtbl,
    0xf3547344_22e4_5e6c_9ece_66504ef733ed
);
impl windows_core::RuntimeType for IScrollViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IScrollViewer,
    IScrollViewer_Vtbl,
    0x1dc28c2e_996c_5394_89c3_4dc656b4ad46
);
impl windows_core::RuntimeType for IScrollViewer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IScrollViewer {
    pub fn put_HorizontalScrollBarVisibility(
        &self,
        value: ScrollBarVisibility,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_HorizontalScrollBarVisibility)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_VerticalScrollBarVisibility(
        &self,
        value: ScrollBarVisibility,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_VerticalScrollBarVisibility)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollViewer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_HorizontalScrollBarVisibility: usize,
    pub put_HorizontalScrollBarVisibility: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ScrollBarVisibility,
    ) -> windows_core::HRESULT,
    get_VerticalScrollBarVisibility: usize,
    pub put_VerticalScrollBarVisibility: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ScrollBarVisibility,
    ) -> windows_core::HRESULT,
    get_IsHorizontalRailEnabled: usize,
    put_IsHorizontalRailEnabled: usize,
    get_IsVerticalRailEnabled: usize,
    put_IsVerticalRailEnabled: usize,
    get_IsHorizontalScrollChainingEnabled: usize,
    put_IsHorizontalScrollChainingEnabled: usize,
    get_IsVerticalScrollChainingEnabled: usize,
    put_IsVerticalScrollChainingEnabled: usize,
    get_IsZoomChainingEnabled: usize,
    put_IsZoomChainingEnabled: usize,
    get_IsScrollInertiaEnabled: usize,
    put_IsScrollInertiaEnabled: usize,
    get_IsZoomInertiaEnabled: usize,
    put_IsZoomInertiaEnabled: usize,
    get_HorizontalScrollMode: usize,
    put_HorizontalScrollMode: usize,
    get_VerticalScrollMode: usize,
    put_VerticalScrollMode: usize,
    get_ZoomMode: usize,
    put_ZoomMode: usize,
    get_HorizontalSnapPointsAlignment: usize,
    put_HorizontalSnapPointsAlignment: usize,
    get_VerticalSnapPointsAlignment: usize,
    put_VerticalSnapPointsAlignment: usize,
    get_HorizontalSnapPointsType: usize,
    put_HorizontalSnapPointsType: usize,
    get_VerticalSnapPointsType: usize,
    put_VerticalSnapPointsType: usize,
    get_ZoomSnapPointsType: usize,
    put_ZoomSnapPointsType: usize,
    get_HorizontalOffset: usize,
    get_ViewportWidth: usize,
    get_ScrollableWidth: usize,
    get_ComputedHorizontalScrollBarVisibility: usize,
    get_ExtentWidth: usize,
    get_VerticalOffset: usize,
    get_ViewportHeight: usize,
    get_ScrollableHeight: usize,
    get_ComputedVerticalScrollBarVisibility: usize,
    get_ExtentHeight: usize,
    get_MinZoomFactor: usize,
    put_MinZoomFactor: usize,
    get_MaxZoomFactor: usize,
    put_MaxZoomFactor: usize,
    get_ZoomFactor: usize,
    get_ZoomSnapPoints: usize,
    get_TopLeftHeader: usize,
    put_TopLeftHeader: usize,
    get_LeftHeader: usize,
    put_LeftHeader: usize,
    get_TopHeader: usize,
    put_TopHeader: usize,
    get_ReduceViewportForCoreInputViewOcclusions: usize,
    put_ReduceViewportForCoreInputViewOcclusions: usize,
    get_HorizontalAnchorRatio: usize,
    put_HorizontalAnchorRatio: usize,
    get_VerticalAnchorRatio: usize,
    put_VerticalAnchorRatio: usize,
    get_CanContentRenderOutsideBounds: usize,
    put_CanContentRenderOutsideBounds: usize,
    add_AnchorRequested: usize,
    remove_AnchorRequested: usize,
    add_ViewChanging: usize,
    remove_ViewChanging: usize,
    add_ViewChanged: usize,
    remove_ViewChanged: usize,
    add_DirectManipulationStarted: usize,
    remove_DirectManipulationStarted: usize,
    add_DirectManipulationCompleted: usize,
    remove_DirectManipulationCompleted: usize,
    ScrollToHorizontalOffset: usize,
    ScrollToVerticalOffset: usize,
    ZoomToFactor: usize,
    ChangeView: usize,
    ChangeViewWithOptionalAnimation: usize,
    InvalidateScrollInfo: usize,
    get_IsDeferredScrollingEnabled: usize,
    put_IsDeferredScrollingEnabled: usize,
    get_BringIntoViewOnFocusChange: usize,
    put_BringIntoViewOnFocusChange: usize,
}
windows_core::imp::define_interface!(
    ISelectionChangedEventArgs,
    ISelectionChangedEventArgs_Vtbl,
    0xb6c18076_4b76_5416_ad29_e2dc20c46246
);
impl windows_core::RuntimeType for ISelectionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_AddedItems: usize,
    get_RemovedItems: usize,
}
windows_core::imp::define_interface!(
    ISelector,
    ISelector_Vtbl,
    0x8f7e2159_e61d_576f_8476_f83fde3d689e
);
impl windows_core::RuntimeType for ISelector {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISelector {
    pub fn get_SelectedIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_SelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn add_SelectionChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<SelectionChangedEventArgs>,
            ) + 'static,
    {
        let handler: SelectionChangedEventHandler = {
            let com = windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::new(
                &SelectionChangedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectionChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelector_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_SelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    put_SelectedItem: usize,
    get_SelectedValue: usize,
    put_SelectedValue: usize,
    get_SelectedValuePath: usize,
    put_SelectedValuePath: usize,
    get_IsSynchronizedWithCurrentItem: usize,
    put_IsSynchronizedWithCurrentItem: usize,
    pub add_SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISelectorBar,
    ISelectorBar_Vtbl,
    0x7f4ad191_55ea_508e_bf47_7047d8677370
);
impl windows_core::RuntimeType for ISelectorBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISelectorBar {
    pub fn get_Items(&self) -> windows_core::Result<windows_collections::IVector<SelectorBarItem>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_SelectedItem(&self) -> windows_core::Result<SelectorBarItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn add_SelectionChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<SelectorBar>,
                windows_core::Ref<SelectorBarSelectionChangedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<SelectorBar, SelectorBarSelectionChangedEventArgs> = {
            let com = windows_core::imp::DelegateBox::< TypedEventHandler < SelectorBar , SelectorBarSelectionChangedEventArgs > , F >::new (& TypedEventHandlerBox::< SelectorBar , SelectorBarSelectionChangedEventArgs , F >::VTABLE , handler) ;
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectionChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_SelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    put_SelectedItem: usize,
    pub add_SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISelectorBarFactory,
    ISelectorBarFactory_Vtbl,
    0x71243dc7_b46c_5a04_9894_e420e462703f
);
impl windows_core::RuntimeType for ISelectorBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISelectorBarItem,
    ISelectorBarItem_Vtbl,
    0x3cdba1f9_a13a_56a2_b9a9_f954998d3658
);
impl windows_core::RuntimeType for ISelectorBarItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISelectorBarItem {
    pub fn get_Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn put_Text(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Text)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Icon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IconElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Icon)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorBarItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Icon: usize,
    pub put_Icon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISelectorBarItemFactory,
    ISelectorBarItemFactory_Vtbl,
    0xe46b62ea_e60d_5989_bea7_5470da326816
);
impl windows_core::RuntimeType for ISelectorBarItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorBarItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISelectorBarSelectionChangedEventArgs,
    ISelectorBarSelectionChangedEventArgs_Vtbl,
    0x73b3f6c5_5050_5c5a_899c_4e6e0474cb63
);
impl windows_core::RuntimeType for ISelectorBarSelectionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorBarSelectionChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ISelectorItem,
    ISelectorItem_Vtbl,
    0x5772c4de_60ea_5492_8c5e_b3323d5a3ca6
);
impl windows_core::RuntimeType for ISelectorItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsSelected: usize,
    put_IsSelected: usize,
}
windows_core::imp::define_interface!(
    ISetterBase,
    ISetterBase_Vtbl,
    0x5a7c1347_cda3_55be_bfef_5c7582213980
);
impl windows_core::RuntimeType for ISetterBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetterBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsSealed: usize,
}
windows_core::imp::define_interface!(IShape, IShape_Vtbl, 0x9941aad3_6af2_5ba2_9085_8506d5f2485e);
impl windows_core::RuntimeType for IShape {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IShape {
    pub fn put_Fill<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Fill)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_Stroke<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Stroke)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_StrokeThickness(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_StrokeThickness)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShape_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Fill: usize,
    pub put_Fill: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Stroke: usize,
    pub put_Stroke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_StrokeMiterLimit: usize,
    put_StrokeMiterLimit: usize,
    get_StrokeThickness: usize,
    pub put_StrokeThickness:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_StrokeStartLineCap: usize,
    put_StrokeStartLineCap: usize,
    get_StrokeEndLineCap: usize,
    put_StrokeEndLineCap: usize,
    get_StrokeLineJoin: usize,
    put_StrokeLineJoin: usize,
    get_StrokeDashOffset: usize,
    put_StrokeDashOffset: usize,
    get_StrokeDashCap: usize,
    put_StrokeDashCap: usize,
    get_StrokeDashArray: usize,
    put_StrokeDashArray: usize,
    get_Stretch: usize,
    put_Stretch: usize,
    get_GeometryTransform: usize,
    GetAlphaMask: usize,
}
windows_core::imp::define_interface!(
    ISizeChangedEventArgs,
    ISizeChangedEventArgs_Vtbl,
    0xfe76324e_6dfb_58b1_9dcd_886ca8f9a2ea
);
impl windows_core::RuntimeType for ISizeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISizeChangedEventArgs {
    pub fn get_NewSize(&self) -> windows_core::Result<Size> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NewSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISizeChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_PreviousSize: usize,
    pub get_NewSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Size) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISlider,
    ISlider_Vtbl,
    0xf7418ecf_7c35_5216_8bf1_d82d47cce5df
);
impl windows_core::RuntimeType for ISlider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISlider {
    pub fn put_StepFrequency(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_StepFrequency)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Orientation(&self, value: Orientation) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Orientation)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IntermediateValue: usize,
    put_IntermediateValue: usize,
    get_StepFrequency: usize,
    pub put_StepFrequency:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_SnapsTo: usize,
    put_SnapsTo: usize,
    get_TickFrequency: usize,
    put_TickFrequency: usize,
    get_TickPlacement: usize,
    put_TickPlacement: usize,
    get_Orientation: usize,
    pub put_Orientation:
        unsafe extern "system" fn(*mut core::ffi::c_void, Orientation) -> windows_core::HRESULT,
    get_IsDirectionReversed: usize,
    put_IsDirectionReversed: usize,
    get_IsThumbToolTipEnabled: usize,
    put_IsThumbToolTipEnabled: usize,
    get_ThumbToolTipValueConverter: usize,
    put_ThumbToolTipValueConverter: usize,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
}
windows_core::imp::define_interface!(
    ISliderFactory,
    ISliderFactory_Vtbl,
    0x06604d71_34ca_5f39_9656_29d81d3c110c
);
impl windows_core::RuntimeType for ISliderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISliderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISolidColorBrush,
    ISolidColorBrush_Vtbl,
    0xb3865c31_37c8_55c1_8a72_d41c67642e2a
);
impl windows_core::RuntimeType for ISolidColorBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISolidColorBrush {
    pub fn put_Color(&self, value: Color) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Color)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISolidColorBrush_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Color: usize,
    pub put_Color:
        unsafe extern "system" fn(*mut core::ffi::c_void, Color) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISplitButton,
    ISplitButton_Vtbl,
    0xf627202d_d2d7_5ff6_bb05_8c48eb6b1fc6
);
impl windows_core::RuntimeType for ISplitButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISplitButton {
    pub fn add_Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<SplitButton>, windows_core::Ref<SplitButtonClickEventArgs>)
            + 'static,
    {
        let handler: TypedEventHandler<SplitButton, SplitButtonClickEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<SplitButton, SplitButtonClickEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<SplitButton, SplitButtonClickEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Click)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Click,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Flyout: usize,
    put_Flyout: usize,
    get_Command: usize,
    put_Command: usize,
    get_CommandParameter: usize,
    put_CommandParameter: usize,
    pub add_Click: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Click:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISplitButtonClickEventArgs,
    ISplitButtonClickEventArgs_Vtbl,
    0x6af896c2_e65a_5998_9c82_2af8f3e0741f
);
impl windows_core::RuntimeType for ISplitButtonClickEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitButtonClickEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ISplitButtonFactory,
    ISplitButtonFactory_Vtbl,
    0x07510092_2612_55e7_981c_a536ddd4570e
);
impl windows_core::RuntimeType for ISplitButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISplitView,
    ISplitView_Vtbl,
    0x10ae18f7_1666_5897_bbce_1e687e7784a8
);
impl windows_core::RuntimeType for ISplitView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISplitView {
    pub fn put_Content<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Content)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_Pane<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Pane)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsPaneOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsPaneOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_OpenPaneLength(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_OpenPaneLength)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_CompactPaneLength(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_CompactPaneLength)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_DisplayMode(&self, value: SplitViewDisplayMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_DisplayMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_PaneClosed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<SplitView>, windows_core::Ref<windows_core::IInspectable>)
            + 'static,
    {
        let handler: TypedEventHandler<SplitView, windows_core::IInspectable> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<SplitView, windows_core::IInspectable>,
                F,
            >::new(
                &TypedEventHandlerBox::<SplitView, windows_core::IInspectable, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_PaneClosed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_PaneClosed,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Content: usize,
    pub put_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Pane: usize,
    pub put_Pane: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsPaneOpen: usize,
    pub put_IsPaneOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_OpenPaneLength: usize,
    pub put_OpenPaneLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_CompactPaneLength: usize,
    pub put_CompactPaneLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_PanePlacement: usize,
    put_PanePlacement: usize,
    get_DisplayMode: usize,
    pub put_DisplayMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        SplitViewDisplayMode,
    ) -> windows_core::HRESULT,
    get_TemplateSettings: usize,
    get_PaneBackground: usize,
    put_PaneBackground: usize,
    get_LightDismissOverlayMode: usize,
    put_LightDismissOverlayMode: usize,
    add_PaneClosing: usize,
    remove_PaneClosing: usize,
    pub add_PaneClosed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_PaneClosed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_PaneOpening: usize,
    remove_PaneOpening: usize,
    add_PaneOpened: usize,
    remove_PaneOpened: usize,
}
windows_core::imp::define_interface!(
    ISplitViewFactory,
    ISplitViewFactory_Vtbl,
    0x389ece72_75ce_561b_aad3_c52125ca6a50
);
impl windows_core::RuntimeType for ISplitViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IStackPanel,
    IStackPanel_Vtbl,
    0x493ab00b_3a6a_5e4a_9452_407cd5197406
);
impl windows_core::RuntimeType for IStackPanel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IStackPanel {
    pub fn put_Orientation(&self, value: Orientation) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Orientation)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Spacing(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Spacing)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStackPanel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_AreScrollSnapPointsRegular: usize,
    put_AreScrollSnapPointsRegular: usize,
    get_Orientation: usize,
    pub put_Orientation:
        unsafe extern "system" fn(*mut core::ffi::c_void, Orientation) -> windows_core::HRESULT,
    get_BackgroundSizing: usize,
    put_BackgroundSizing: usize,
    get_BorderBrush: usize,
    put_BorderBrush: usize,
    get_BorderThickness: usize,
    put_BorderThickness: usize,
    get_CornerRadius: usize,
    put_CornerRadius: usize,
    get_Padding: usize,
    put_Padding: usize,
    get_Spacing: usize,
    pub put_Spacing:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IStackPanelFactory,
    IStackPanelFactory_Vtbl,
    0x64c1d388_47a2_5a74_a75b_559d151ee5ac
);
impl windows_core::RuntimeType for IStackPanelFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStackPanelFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStyle, IStyle_Vtbl, 0x65e1d164_572f_5b0e_a80f_9c02441fac49);
impl windows_core::RuntimeType for IStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStyle_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsSealed: usize,
    get_Setters: usize,
    get_TargetType: usize,
    put_TargetType: usize,
    get_BasedOn: usize,
    put_BasedOn: usize,
    Seal: usize,
}
windows_core::imp::define_interface!(
    ISwapChainPanel,
    ISwapChainPanel_Vtbl,
    0x08844f85_aa1b_540d_bef2_b2bb7b257f8c
);
impl windows_core::RuntimeType for ISwapChainPanel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISwapChainPanel {
    pub fn get_CompositionScaleX(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CompositionScaleX)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_CompositionScaleY(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CompositionScaleY)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn add_CompositionScaleChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<SwapChainPanel>, windows_core::Ref<windows_core::IInspectable>)
            + 'static,
    {
        let handler: TypedEventHandler<SwapChainPanel, windows_core::IInspectable> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<SwapChainPanel, windows_core::IInspectable>,
                F,
            >::new(
                &TypedEventHandlerBox::<SwapChainPanel, windows_core::IInspectable, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_CompositionScaleChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_CompositionScaleChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_CompositionScaleX:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub get_CompositionScaleY:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub add_CompositionScaleChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_CompositionScaleChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    CreateCoreIndependentInputSource: usize,
}
windows_core::imp::define_interface!(
    ISwapChainPanelFactory,
    ISwapChainPanelFactory_Vtbl,
    0x38d00b69_5759_5f37_9e1c_3ae0d4288b26
);
impl windows_core::RuntimeType for ISwapChainPanelFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISwapChainPanelNative,
    ISwapChainPanelNative_Vtbl,
    0x63aad0b8_7c24_40ff_85a8_640d944cc325
);
windows_core::imp::interface_hierarchy!(ISwapChainPanelNative, windows_core::IUnknown);
impl ISwapChainPanelNative {
    pub unsafe fn SetSwapChain(
        &self,
        swapchain: *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSwapChain)(
                windows_core::Interface::as_raw(self),
                swapchain as _,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSwapChain: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISymbolIcon,
    ISymbolIcon_Vtbl,
    0xa4322906_0dbe_5eb7_8b64_3e832246eb7f
);
impl windows_core::RuntimeType for ISymbolIcon {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ISymbolIcon {
    pub fn get_Symbol(&self) -> windows_core::Result<Symbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Symbol)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISymbolIcon_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Symbol:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Symbol) -> windows_core::HRESULT,
    put_Symbol: usize,
}
windows_core::imp::define_interface!(
    ISymbolIconFactory,
    ISymbolIconFactory_Vtbl,
    0xd4430447_567c_5aad_996a_a547774e2c3c
);
impl windows_core::RuntimeType for ISymbolIconFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISymbolIconFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithSymbol: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        Symbol,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISystemBackdrop,
    ISystemBackdrop_Vtbl,
    0x5aeed5c4_37ac_5852_b73f_1b76ebc3205f
);
impl windows_core::RuntimeType for ISystemBackdrop {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdrop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    GetDefaultSystemBackdropConfiguration: usize,
}
windows_core::imp::define_interface!(
    ITabView,
    ITabView_Vtbl,
    0x07b509e1_1d38_551b_95f4_4732b049f6a6
);
impl windows_core::RuntimeType for ITabView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITabView {
    pub fn put_IsAddTabButtonVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsAddTabButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_TabCloseRequested<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TabView>, windows_core::Ref<TabViewTabCloseRequestedEventArgs>)
            + 'static,
    {
        let handler: TypedEventHandler<TabView, TabViewTabCloseRequestedEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<TabView, TabViewTabCloseRequestedEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<TabView, TabViewTabCloseRequestedEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_TabCloseRequested)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_TabCloseRequested,
            ))
        }
    }
    pub fn add_AddTabButtonClick<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TabView>, windows_core::Ref<windows_core::IInspectable>) + 'static,
    {
        let handler: TypedEventHandler<TabView, windows_core::IInspectable> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<TabView, windows_core::IInspectable>,
                F,
            >::new(
                &TypedEventHandlerBox::<TabView, windows_core::IInspectable, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_AddTabButtonClick)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_AddTabButtonClick,
            ))
        }
    }
    pub fn get_TabItems(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TabItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_CanReorderTabs(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_CanReorderTabs)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_SelectedIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_SelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_SelectionChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<SelectionChangedEventArgs>,
            ) + 'static,
    {
        let handler: SelectionChangedEventHandler = {
            let com = windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::new(
                &SelectionChangedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectionChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITabView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_TabWidthMode: usize,
    put_TabWidthMode: usize,
    get_CloseButtonOverlayMode: usize,
    put_CloseButtonOverlayMode: usize,
    get_TabStripHeader: usize,
    put_TabStripHeader: usize,
    get_TabStripHeaderTemplate: usize,
    put_TabStripHeaderTemplate: usize,
    get_TabStripFooter: usize,
    put_TabStripFooter: usize,
    get_TabStripFooterTemplate: usize,
    put_TabStripFooterTemplate: usize,
    get_IsAddTabButtonVisible: usize,
    pub put_IsAddTabButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_AddTabButtonCommand: usize,
    put_AddTabButtonCommand: usize,
    get_AddTabButtonCommandParameter: usize,
    put_AddTabButtonCommandParameter: usize,
    pub add_TabCloseRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_TabCloseRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_TabDroppedOutside: usize,
    remove_TabDroppedOutside: usize,
    pub add_AddTabButtonClick: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_AddTabButtonClick:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_TabItemsChanged: usize,
    remove_TabItemsChanged: usize,
    get_TabItemsSource: usize,
    put_TabItemsSource: usize,
    pub get_TabItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TabItemTemplate: usize,
    put_TabItemTemplate: usize,
    get_TabItemTemplateSelector: usize,
    put_TabItemTemplateSelector: usize,
    get_CanDragTabs: usize,
    put_CanDragTabs: usize,
    get_CanReorderTabs: usize,
    pub put_CanReorderTabs:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_AllowDropTabs: usize,
    put_AllowDropTabs: usize,
    pub get_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_SelectedItem: usize,
    put_SelectedItem: usize,
    ContainerFromItem: usize,
    ContainerFromIndex: usize,
    pub add_SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_TabDragStarting: usize,
    remove_TabDragStarting: usize,
    add_TabDragCompleted: usize,
    remove_TabDragCompleted: usize,
    add_TabStripDragOver: usize,
    remove_TabStripDragOver: usize,
    add_TabStripDrop: usize,
    remove_TabStripDrop: usize,
}
windows_core::imp::define_interface!(
    ITabViewFactory,
    ITabViewFactory_Vtbl,
    0xe7e83685_eedf_5106_9429_884435ab166b
);
impl windows_core::RuntimeType for ITabViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITabViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITabViewItem,
    ITabViewItem_Vtbl,
    0x64980afa_97af_5190_90b3_4ba277b1113d
);
impl windows_core::RuntimeType for ITabViewItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITabViewItem {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsClosable(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsClosable)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITabViewItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_IconSource: usize,
    put_IconSource: usize,
    get_IsClosable: usize,
    pub put_IsClosable:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_TabViewTemplateSettings: usize,
    add_CloseRequested: usize,
    remove_CloseRequested: usize,
}
windows_core::imp::define_interface!(
    ITabViewItemFactory,
    ITabViewItemFactory_Vtbl,
    0xb64c2423_7e56_5d41_8a84_1ee28f9826a4
);
impl windows_core::RuntimeType for ITabViewItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITabViewItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITabViewTabCloseRequestedEventArgs,
    ITabViewTabCloseRequestedEventArgs_Vtbl,
    0xd56ab9b2_e264_5c7e_a1cb_e41a16a6c6c6
);
impl windows_core::RuntimeType for ITabViewTabCloseRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITabViewTabCloseRequestedEventArgs {
    pub fn get_Tab(&self) -> windows_core::Result<TabViewItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Tab)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITabViewTabCloseRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Item: usize,
    pub get_Tab: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITappedRoutedEventArgs,
    ITappedRoutedEventArgs_Vtbl,
    0x73f74b8c_3709_547e_8e0c_51c03c89126a
);
impl windows_core::RuntimeType for ITappedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_PointerDeviceType: usize,
    get_Handled: usize,
    put_Handled: usize,
    GetPosition: usize,
}
windows_core::imp::define_interface!(
    ITeachingTip,
    ITeachingTip_Vtbl,
    0xdaebd5f7_3b47_5b12_b804_f4e1442b2113
);
impl windows_core::RuntimeType for ITeachingTip {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITeachingTip {
    pub fn put_Title(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Title)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Subtitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Subtitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_IsOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_ActionButtonContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_ActionButtonContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_CloseButtonContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_CloseButtonContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsLightDismissEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsLightDismissEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_PreferredPlacement(
        &self,
        value: TeachingTipPlacementMode,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PreferredPlacement)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_ActionButtonClick<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TeachingTip>, windows_core::Ref<windows_core::IInspectable>)
            + 'static,
    {
        let handler: TypedEventHandler<TeachingTip, windows_core::IInspectable> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<TeachingTip, windows_core::IInspectable>,
                F,
            >::new(
                &TypedEventHandlerBox::<TeachingTip, windows_core::IInspectable, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ActionButtonClick)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ActionButtonClick,
            ))
        }
    }
    pub fn add_Closed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TeachingTip>, windows_core::Ref<TeachingTipClosedEventArgs>)
            + 'static,
    {
        let handler: TypedEventHandler<TeachingTip, TeachingTipClosedEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<TeachingTip, TeachingTipClosedEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<TeachingTip, TeachingTipClosedEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Closed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Closed,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITeachingTip_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Title: usize,
    pub put_Title: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Subtitle: usize,
    pub put_Subtitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsOpen: usize,
    pub put_IsOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_Target: usize,
    put_Target: usize,
    get_TailVisibility: usize,
    put_TailVisibility: usize,
    get_ActionButtonContent: usize,
    pub put_ActionButtonContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ActionButtonStyle: usize,
    put_ActionButtonStyle: usize,
    get_ActionButtonCommand: usize,
    put_ActionButtonCommand: usize,
    get_ActionButtonCommandParameter: usize,
    put_ActionButtonCommandParameter: usize,
    get_CloseButtonContent: usize,
    pub put_CloseButtonContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_CloseButtonStyle: usize,
    put_CloseButtonStyle: usize,
    get_CloseButtonCommand: usize,
    put_CloseButtonCommand: usize,
    get_CloseButtonCommandParameter: usize,
    put_CloseButtonCommandParameter: usize,
    get_PlacementMargin: usize,
    put_PlacementMargin: usize,
    get_ShouldConstrainToRootBounds: usize,
    put_ShouldConstrainToRootBounds: usize,
    get_IsLightDismissEnabled: usize,
    pub put_IsLightDismissEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_PreferredPlacement: usize,
    pub put_PreferredPlacement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TeachingTipPlacementMode,
    ) -> windows_core::HRESULT,
    get_HeroContentPlacement: usize,
    put_HeroContentPlacement: usize,
    get_HeroContent: usize,
    put_HeroContent: usize,
    get_IconSource: usize,
    put_IconSource: usize,
    get_TemplateSettings: usize,
    pub add_ActionButtonClick: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ActionButtonClick:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_CloseButtonClick: usize,
    remove_CloseButtonClick: usize,
    add_Closing: usize,
    remove_Closing: usize,
    pub add_Closed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Closed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITeachingTipClosedEventArgs,
    ITeachingTipClosedEventArgs_Vtbl,
    0x2536f506_4038_59db_9e35_a9252fb5adb2
);
impl windows_core::RuntimeType for ITeachingTipClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITeachingTipClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Reason: usize,
}
windows_core::imp::define_interface!(
    ITeachingTipFactory,
    ITeachingTipFactory_Vtbl,
    0xa3ecd47d_2972_5d19_a62e_ddfbc5e1ad57
);
impl windows_core::RuntimeType for ITeachingTipFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITeachingTipFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITextBlock,
    ITextBlock_Vtbl,
    0x1ac8d84f_392c_5c7e_83f5_a53e3bf0abb0
);
impl windows_core::RuntimeType for ITextBlock {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITextBlock {
    pub fn put_FontSize(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_FontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FontFamily>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontFamily)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_FontWeight(&self, value: FontWeight) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontWeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Foreground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Foreground)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_TextWrapping(&self, value: TextWrapping) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_TextWrapping)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn put_Text(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Text)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_IsTextSelectionEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsTextSelectionEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextBlock_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_FontSize: usize,
    pub put_FontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_FontFamily: usize,
    pub put_FontFamily: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_FontWeight: usize,
    pub put_FontWeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, FontWeight) -> windows_core::HRESULT,
    get_FontStyle: usize,
    put_FontStyle: usize,
    get_FontStretch: usize,
    put_FontStretch: usize,
    get_CharacterSpacing: usize,
    put_CharacterSpacing: usize,
    get_Foreground: usize,
    pub put_Foreground: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TextWrapping: usize,
    pub put_TextWrapping:
        unsafe extern "system" fn(*mut core::ffi::c_void, TextWrapping) -> windows_core::HRESULT,
    get_TextTrimming: usize,
    put_TextTrimming: usize,
    get_TextAlignment: usize,
    put_TextAlignment: usize,
    pub get_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Inlines: usize,
    get_Padding: usize,
    put_Padding: usize,
    get_LineHeight: usize,
    put_LineHeight: usize,
    get_LineStackingStrategy: usize,
    put_LineStackingStrategy: usize,
    get_IsTextSelectionEnabled: usize,
    pub put_IsTextSelectionEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_SelectedText: usize,
    get_ContentStart: usize,
    get_ContentEnd: usize,
    get_SelectionStart: usize,
    get_SelectionEnd: usize,
    get_BaselineOffset: usize,
    get_SelectionHighlightColor: usize,
    put_SelectionHighlightColor: usize,
    get_MaxLines: usize,
    put_MaxLines: usize,
    get_TextLineBounds: usize,
    put_TextLineBounds: usize,
    get_OpticalMarginAlignment: usize,
    put_OpticalMarginAlignment: usize,
    get_IsColorFontEnabled: usize,
    put_IsColorFontEnabled: usize,
    get_TextReadingOrder: usize,
    put_TextReadingOrder: usize,
    get_IsTextScaleFactorEnabled: usize,
    put_IsTextScaleFactorEnabled: usize,
    get_TextDecorations: usize,
    put_TextDecorations: usize,
    get_IsTextTrimmed: usize,
    get_HorizontalTextAlignment: usize,
    put_HorizontalTextAlignment: usize,
    get_TextHighlighters: usize,
    get_SelectionFlyout: usize,
    put_SelectionFlyout: usize,
    add_SelectionChanged: usize,
    remove_SelectionChanged: usize,
    add_ContextMenuOpening: usize,
    remove_ContextMenuOpening: usize,
    add_IsTextTrimmedChanged: usize,
    remove_IsTextTrimmedChanged: usize,
    SelectAll: usize,
    Select: usize,
    GetAlphaMask: usize,
    CopySelectionToClipboard: usize,
}
windows_core::imp::define_interface!(
    ITextBlockStatics,
    ITextBlockStatics_Vtbl,
    0x3187104b_65c2_5e53_b889_c8272b1314cd
);
impl windows_core::RuntimeType for ITextBlockStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextBlockStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_FontSizeProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_FontFamilyProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_FontWeightProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_FontStyleProperty: usize,
    get_FontStretchProperty: usize,
    get_CharacterSpacingProperty: usize,
    pub get_ForegroundProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_TextWrappingProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TextTrimmingProperty: usize,
    get_TextAlignmentProperty: usize,
    pub get_TextProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_PaddingProperty: usize,
    get_LineHeightProperty: usize,
    get_LineStackingStrategyProperty: usize,
    pub get_IsTextSelectionEnabledProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_SelectedTextProperty: usize,
    get_SelectionHighlightColorProperty: usize,
    get_MaxLinesProperty: usize,
    get_TextLineBoundsProperty: usize,
    get_OpticalMarginAlignmentProperty: usize,
    get_IsColorFontEnabledProperty: usize,
    get_TextReadingOrderProperty: usize,
    get_IsTextScaleFactorEnabledProperty: usize,
    get_TextDecorationsProperty: usize,
    get_IsTextTrimmedProperty: usize,
    get_HorizontalTextAlignmentProperty: usize,
    get_SelectionFlyoutProperty: usize,
}
windows_core::imp::define_interface!(
    ITextBox,
    ITextBox_Vtbl,
    0x873af7c2_ab89_5d76_8dbe_3d6325669df5
);
impl windows_core::RuntimeType for ITextBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITextBox {
    pub fn get_Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn put_Text(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Text)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_AcceptsReturn(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_AcceptsReturn)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_TextWrapping(&self, value: TextWrapping) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_TextWrapping)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_PlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn add_TextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<TextChangedEventArgs>,
            ) + 'static,
    {
        let handler: TextChangedEventHandler = {
            let com = windows_core::imp::DelegateBox::<TextChangedEventHandler, F>::new(
                &TextChangedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_TextChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_TextChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_SelectedText: usize,
    put_SelectedText: usize,
    get_SelectionLength: usize,
    put_SelectionLength: usize,
    get_SelectionStart: usize,
    put_SelectionStart: usize,
    get_MaxLength: usize,
    put_MaxLength: usize,
    get_IsReadOnly: usize,
    put_IsReadOnly: usize,
    get_AcceptsReturn: usize,
    pub put_AcceptsReturn:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_TextAlignment: usize,
    put_TextAlignment: usize,
    get_TextWrapping: usize,
    pub put_TextWrapping:
        unsafe extern "system" fn(*mut core::ffi::c_void, TextWrapping) -> windows_core::HRESULT,
    get_IsSpellCheckEnabled: usize,
    put_IsSpellCheckEnabled: usize,
    get_IsTextPredictionEnabled: usize,
    put_IsTextPredictionEnabled: usize,
    get_InputScope: usize,
    put_InputScope: usize,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_PlaceholderText: usize,
    pub put_PlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_SelectionHighlightColor: usize,
    put_SelectionHighlightColor: usize,
    get_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    put_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    get_IsColorFontEnabled: usize,
    put_IsColorFontEnabled: usize,
    get_SelectionHighlightColorWhenNotFocused: usize,
    put_SelectionHighlightColorWhenNotFocused: usize,
    get_HorizontalTextAlignment: usize,
    put_HorizontalTextAlignment: usize,
    get_CharacterCasing: usize,
    put_CharacterCasing: usize,
    get_PlaceholderForeground: usize,
    put_PlaceholderForeground: usize,
    get_CanPasteClipboardContent: usize,
    get_CanUndo: usize,
    get_CanRedo: usize,
    get_SelectionFlyout: usize,
    put_SelectionFlyout: usize,
    get_ProofingMenuFlyout: usize,
    get_Description: usize,
    put_Description: usize,
    pub add_TextChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_TextChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_SelectionChanged: usize,
    remove_SelectionChanged: usize,
    add_ContextMenuOpening: usize,
    remove_ContextMenuOpening: usize,
    add_Paste: usize,
    remove_Paste: usize,
    add_TextCompositionStarted: usize,
    remove_TextCompositionStarted: usize,
    add_TextCompositionChanged: usize,
    remove_TextCompositionChanged: usize,
    add_TextCompositionEnded: usize,
    remove_TextCompositionEnded: usize,
    add_CopyingToClipboard: usize,
    remove_CopyingToClipboard: usize,
    add_CuttingToClipboard: usize,
    remove_CuttingToClipboard: usize,
    add_BeforeTextChanging: usize,
    remove_BeforeTextChanging: usize,
    add_SelectionChanging: usize,
    remove_SelectionChanging: usize,
    Select: usize,
    SelectAll: usize,
    GetRectFromCharacterIndex: usize,
    GetLinguisticAlternativesAsync: usize,
    Undo: usize,
    Redo: usize,
    PasteFromClipboard: usize,
    CopySelectionToClipboard: usize,
    CutSelectionToClipboard: usize,
    ClearUndoRedoHistory: usize,
    get_TextReadingOrder: usize,
    put_TextReadingOrder: usize,
    get_DesiredCandidateWindowAlignment: usize,
    put_DesiredCandidateWindowAlignment: usize,
    add_CandidateWindowBoundsChanged: usize,
    remove_CandidateWindowBoundsChanged: usize,
    add_TextChanging: usize,
    remove_TextChanging: usize,
}
windows_core::imp::define_interface!(
    ITextBoxFactory,
    ITextBoxFactory_Vtbl,
    0xe1d8b82e_bc60_5d27_b646_5ca4c4a69432
);
impl windows_core::RuntimeType for ITextBoxFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextBoxFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITextChangedEventArgs,
    ITextChangedEventArgs_Vtbl,
    0x71c37e43_7be7_52fc_bf8c_9867f44be5f4
);
impl windows_core::RuntimeType for ITextChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ITextDocument,
    ITextDocument_Vtbl,
    0x1149d57d_86a6_59dd_88d9_196f27bc5c85
);
impl windows_core::RuntimeType for ITextDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITextDocument {
    pub fn GetText(
        &self,
        options: TextGetOptions,
        value: &mut windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetText)(
                windows_core::Interface::as_raw(self),
                options,
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn SetText(&self, options: TextSetOptions, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetText)(
                windows_core::Interface::as_raw(self),
                options,
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_CaretType: usize,
    put_CaretType: usize,
    get_DefaultTabStop: usize,
    put_DefaultTabStop: usize,
    get_Selection: usize,
    get_UndoLimit: usize,
    put_UndoLimit: usize,
    CanCopy: usize,
    CanPaste: usize,
    CanRedo: usize,
    CanUndo: usize,
    ApplyDisplayUpdates: usize,
    BatchDisplayUpdates: usize,
    BeginUndoGroup: usize,
    EndUndoGroup: usize,
    GetDefaultCharacterFormat: usize,
    GetDefaultParagraphFormat: usize,
    GetRange: usize,
    GetRangeFromPoint: usize,
    pub GetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextGetOptions,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    LoadFromStream: usize,
    Redo: usize,
    SaveToStream: usize,
    SetDefaultCharacterFormat: usize,
    SetDefaultParagraphFormat: usize,
    pub SetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextSetOptions,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Undo: usize,
    get_AlignmentIncludesTrailingWhitespace: usize,
    put_AlignmentIncludesTrailingWhitespace: usize,
    get_IgnoreTrailingCharacterSpacing: usize,
    put_IgnoreTrailingCharacterSpacing: usize,
    ClearUndoRedoHistory: usize,
}
windows_core::imp::define_interface!(
    ITextElement,
    ITextElement_Vtbl,
    0xa122ba22_833f_5220_a47e_6cd507531abe
);
impl windows_core::RuntimeType for ITextElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITextElement {
    pub fn put_FontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FontFamily>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontFamily)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_FontWeight(&self, value: FontWeight) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_FontWeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Name: usize,
    get_FontSize: usize,
    put_FontSize: usize,
    get_FontFamily: usize,
    pub put_FontFamily: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_FontWeight: usize,
    pub put_FontWeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, FontWeight) -> windows_core::HRESULT,
    get_FontStyle: usize,
    put_FontStyle: usize,
    get_FontStretch: usize,
    put_FontStretch: usize,
    get_CharacterSpacing: usize,
    put_CharacterSpacing: usize,
    get_Foreground: usize,
    put_Foreground: usize,
    get_Language: usize,
    put_Language: usize,
    get_IsTextScaleFactorEnabled: usize,
    put_IsTextScaleFactorEnabled: usize,
    get_TextDecorations: usize,
    put_TextDecorations: usize,
    get_ContentStart: usize,
    get_ContentEnd: usize,
    get_ElementStart: usize,
    get_ElementEnd: usize,
    get_AllowFocusOnInteraction: usize,
    put_AllowFocusOnInteraction: usize,
    get_AccessKey: usize,
    put_AccessKey: usize,
    get_ExitDisplayModeOnAccessKeyInvoked: usize,
    put_ExitDisplayModeOnAccessKeyInvoked: usize,
    get_IsAccessKeyScope: usize,
    put_IsAccessKeyScope: usize,
    get_AccessKeyScopeOwner: usize,
    put_AccessKeyScopeOwner: usize,
    get_KeyTipPlacementMode: usize,
    put_KeyTipPlacementMode: usize,
    get_KeyTipHorizontalOffset: usize,
    put_KeyTipHorizontalOffset: usize,
    get_KeyTipVerticalOffset: usize,
    put_KeyTipVerticalOffset: usize,
    get_XamlRoot: usize,
    put_XamlRoot: usize,
    add_AccessKeyDisplayRequested: usize,
    remove_AccessKeyDisplayRequested: usize,
    add_AccessKeyDisplayDismissed: usize,
    remove_AccessKeyDisplayDismissed: usize,
    add_AccessKeyInvoked: usize,
    remove_AccessKeyInvoked: usize,
    FindName: usize,
}
windows_core::imp::define_interface!(
    ITimePicker,
    ITimePicker_Vtbl,
    0xed4baa33_c240_5934_9229_82d37b26f846
);
impl windows_core::RuntimeType for ITimePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITimePicker {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_ClockIdentifier(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_ClockIdentifier)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_MinuteIncrement(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MinuteIncrement)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_SelectedTimeChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<TimePicker>,
                windows_core::Ref<TimePickerSelectedValueChangedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<TimePicker, TimePickerSelectedValueChangedEventArgs> = {
            let com = windows_core::imp::DelegateBox::< TypedEventHandler < TimePicker , TimePickerSelectedValueChangedEventArgs > , F >::new (& TypedEventHandlerBox::< TimePicker , TimePickerSelectedValueChangedEventArgs , F >::VTABLE , handler) ;
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectedTimeChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectedTimeChanged,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimePicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_ClockIdentifier: usize,
    pub put_ClockIdentifier: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_MinuteIncrement: usize,
    pub put_MinuteIncrement:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_Time: usize,
    put_Time: usize,
    get_LightDismissOverlayMode: usize,
    put_LightDismissOverlayMode: usize,
    get_SelectedTime: usize,
    put_SelectedTime: usize,
    add_TimeChanged: usize,
    remove_TimeChanged: usize,
    pub add_SelectedTimeChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectedTimeChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITimePickerFactory,
    ITimePickerFactory_Vtbl,
    0x1584429e_aafe_5c65_89e9_8c066e5690e7
);
impl windows_core::RuntimeType for ITimePickerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimePickerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITimePickerSelectedValueChangedEventArgs,
    ITimePickerSelectedValueChangedEventArgs_Vtbl,
    0x6ed7edf1_9b0b_5e7b_9e10_f35660a29fd2
);
impl windows_core::RuntimeType for ITimePickerSelectedValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITimePickerSelectedValueChangedEventArgs {
    pub fn get_NewTime(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NewTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<windows_time::TimeSpan>| r__.Value())
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimePickerSelectedValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_OldTime: usize,
    pub get_NewTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITitleBar,
    ITitleBar_Vtbl,
    0xc552714d_5d30_5a2b_9c7a_d68bea3dde8d
);
impl windows_core::RuntimeType for ITitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITitleBar {
    pub fn put_Title(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Title)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Subtitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Subtitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Content<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Content)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_RightHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_RightHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsBackButtonVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsBackButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsBackButtonEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsBackButtonEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsPaneToggleButtonVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsPaneToggleButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_BackRequested<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TitleBar>, windows_core::Ref<windows_core::IInspectable>) + 'static,
    {
        let handler: TypedEventHandler<TitleBar, windows_core::IInspectable> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<TitleBar, windows_core::IInspectable>,
                F,
            >::new(
                &TypedEventHandlerBox::<TitleBar, windows_core::IInspectable, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_BackRequested)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_BackRequested,
            ))
        }
    }
    pub fn add_PaneToggleRequested<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TitleBar>, windows_core::Ref<windows_core::IInspectable>) + 'static,
    {
        let handler: TypedEventHandler<TitleBar, windows_core::IInspectable> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<TitleBar, windows_core::IInspectable>,
                F,
            >::new(
                &TypedEventHandlerBox::<TitleBar, windows_core::IInspectable, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_PaneToggleRequested)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_PaneToggleRequested,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITitleBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Title: usize,
    pub put_Title: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Subtitle: usize,
    pub put_Subtitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IconSource: usize,
    put_IconSource: usize,
    get_LeftHeader: usize,
    put_LeftHeader: usize,
    get_Content: usize,
    pub put_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_RightHeader: usize,
    pub put_RightHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsBackButtonVisible: usize,
    pub put_IsBackButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsBackButtonEnabled: usize,
    pub put_IsBackButtonEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsPaneToggleButtonVisible: usize,
    pub put_IsPaneToggleButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_TemplateSettings: usize,
    pub add_BackRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_BackRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_PaneToggleRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_PaneToggleRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITitleBarFactory,
    ITitleBarFactory_Vtbl,
    0xc4452799_4606_59ef_9392_a0548d48b82e
);
impl windows_core::RuntimeType for ITitleBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITitleBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToggleButton,
    IToggleButton_Vtbl,
    0x686fbaa4_c866_568b_8f75_481d8d545291
);
impl windows_core::RuntimeType for IToggleButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IToggleButton {
    pub fn get_IsChecked(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsChecked)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<bool>| r__.Value())
        }
    }
    pub fn put_IsChecked(&self, value: Option<bool>) -> windows_core::Result<()> {
        let value__ =
            value.map(<windows_reference::IReference<bool> as core::convert::From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).put_IsChecked)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn add_Checked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Checked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Checked,
            ))
        }
    }
    pub fn add_Unchecked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Unchecked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Unchecked,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_IsChecked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_IsChecked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsThreeState: usize,
    put_IsThreeState: usize,
    pub add_Checked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Checked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_Unchecked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Unchecked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_Indeterminate: usize,
    remove_Indeterminate: usize,
}
windows_core::imp::define_interface!(
    IToggleButtonFactory,
    IToggleButtonFactory_Vtbl,
    0x519511bb_d35b_5e2d_966c_8369405a4408
);
impl windows_core::RuntimeType for IToggleButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToggleSwitch,
    IToggleSwitch_Vtbl,
    0x1b17eeb1_74bf_5a83_8161_a86f0fdcdf24
);
impl windows_core::RuntimeType for IToggleSwitch {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IToggleSwitch {
    pub fn get_IsOn(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsOn)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_IsOn(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsOn)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_OnContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_OnContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_OffContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_OffContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn add_Toggled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Toggled)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Toggled,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleSwitch_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_IsOn:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub put_IsOn: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_OnContent: usize,
    pub put_OnContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_OnContentTemplate: usize,
    put_OnContentTemplate: usize,
    get_OffContent: usize,
    pub put_OffContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_OffContentTemplate: usize,
    put_OffContentTemplate: usize,
    get_TemplateSettings: usize,
    pub add_Toggled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Toggled:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToolTip,
    IToolTip_Vtbl,
    0x67e93d74_5e93_59a1_91bf_413efbeb904c
);
impl windows_core::RuntimeType for IToolTip {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IToolTip_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_HorizontalOffset: usize,
    put_HorizontalOffset: usize,
    get_IsOpen: usize,
    put_IsOpen: usize,
    get_Placement: usize,
    put_Placement: usize,
    get_PlacementTarget: usize,
    put_PlacementTarget: usize,
    get_PlacementRect: usize,
    put_PlacementRect: usize,
    get_VerticalOffset: usize,
    put_VerticalOffset: usize,
    get_TemplateSettings: usize,
    add_Closed: usize,
    remove_Closed: usize,
    add_Opened: usize,
    remove_Opened: usize,
}
windows_core::imp::define_interface!(
    IToolTipFactory,
    IToolTipFactory_Vtbl,
    0xbcbb3720_2db8_54e1_8806_fcbed38949a9
);
impl windows_core::RuntimeType for IToolTipFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IToolTipFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToolTipService,
    IToolTipService_Vtbl,
    0x01140768_2727_5f89_80e0_5210326a3431
);
impl windows_core::RuntimeType for IToolTipService {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IToolTipService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IToolTipServiceStatics,
    IToolTipServiceStatics_Vtbl,
    0x5aa38adc_9874_5e0a_8d8e_1574efc0b88f
);
impl windows_core::RuntimeType for IToolTipServiceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IToolTipServiceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_PlacementProperty: usize,
    pub GetPlacement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut PlacementMode,
    ) -> windows_core::HRESULT,
    pub SetPlacement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        PlacementMode,
    ) -> windows_core::HRESULT,
    get_PlacementTargetProperty: usize,
    GetPlacementTarget: usize,
    SetPlacementTarget: usize,
    get_ToolTipProperty: usize,
    pub GetToolTip: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetToolTip: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITreeView,
    ITreeView_Vtbl,
    0x1bef9af4_712c_50ef_9bb4_881b975232ab
);
impl windows_core::RuntimeType for ITreeView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITreeView {
    pub fn get_RootNodes(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<TreeViewNode>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RootNodes)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_SelectionMode(&self, value: TreeViewSelectionMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectionMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_ItemInvoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TreeView>, windows_core::Ref<TreeViewItemInvokedEventArgs>)
            + 'static,
    {
        let handler: TypedEventHandler<TreeView, TreeViewItemInvokedEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<TreeView, TreeViewItemInvokedEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<TreeView, TreeViewItemInvokedEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ItemInvoked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ItemInvoked,
            ))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITreeView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_RootNodes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_SelectionMode: usize,
    pub put_SelectionMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TreeViewSelectionMode,
    ) -> windows_core::HRESULT,
    get_SelectedNodes: usize,
    Expand: usize,
    Collapse: usize,
    SelectAll: usize,
    pub add_ItemInvoked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ItemInvoked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_Expanding: usize,
    remove_Expanding: usize,
    add_Collapsed: usize,
    remove_Collapsed: usize,
}
windows_core::imp::define_interface!(
    ITreeViewFactory,
    ITreeViewFactory_Vtbl,
    0x9c6220be_f9eb_518a_b30e_7e41de5efda9
);
impl windows_core::RuntimeType for ITreeViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITreeViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITreeViewItemInvokedEventArgs,
    ITreeViewItemInvokedEventArgs_Vtbl,
    0x1a05853c_b101_542c_9a1e_775a044c4652
);
impl windows_core::RuntimeType for ITreeViewItemInvokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITreeViewItemInvokedEventArgs {
    pub fn get_InvokedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InvokedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITreeViewItemInvokedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_InvokedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    put_Handled: usize,
    get_Handled: usize,
}
windows_core::imp::define_interface!(
    ITreeViewNode,
    ITreeViewNode_Vtbl,
    0x00378a74_790b_5328_8afa_7d65e22da426
);
impl windows_core::RuntimeType for ITreeViewNode {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITreeViewNode {
    pub fn get_Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_Content<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Content)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsExpanded(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsExpanded)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_Children(&self) -> windows_core::Result<windows_collections::IVector<TreeViewNode>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Children)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITreeViewNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Parent: usize,
    get_IsExpanded: usize,
    pub put_IsExpanded:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_HasChildren: usize,
    get_Depth: usize,
    get_HasUnrealizedChildren: usize,
    put_HasUnrealizedChildren: usize,
    pub get_Children: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITreeViewNodeFactory,
    ITreeViewNodeFactory_Vtbl,
    0xc105a5e5_cea8_5efd_8be8_3d89b54cbd5f
);
impl windows_core::RuntimeType for ITreeViewNodeFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITreeViewNodeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITriggerBase,
    ITriggerBase_Vtbl,
    0xd37da89d_0d71_58cf_a901_99a7d3e5e434
);
impl windows_core::RuntimeType for ITriggerBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITriggerBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IUIElement,
    IUIElement_Vtbl,
    0xc3c01020_320c_5cf6_9d24_d396bbfa4d8b
);
impl windows_core::RuntimeType for IUIElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IUIElement {
    pub fn put_Opacity(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Opacity)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsHitTestVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_KeyboardAccelerators(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<KeyboardAccelerator>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KeyboardAccelerators)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_KeyboardAcceleratorPlacementMode(
        &self,
        value: KeyboardAcceleratorPlacementMode,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_KeyboardAcceleratorPlacementMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_XamlRoot(&self) -> windows_core::Result<XamlRoot> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_XamlRoot)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_XamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<XamlRoot>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_XamlRoot)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn add_PointerPressed<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<PointerRoutedEventArgs>,
            ) + 'static,
    {
        let handler: PointerEventHandler = {
            let com = windows_core::imp::DelegateBox::<PointerEventHandler, F>::new(
                &PointerEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_PointerPressed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_PointerPressed,
            ))
        }
    }
    pub fn add_PointerReleased<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<PointerRoutedEventArgs>,
            ) + 'static,
    {
        let handler: PointerEventHandler = {
            let com = windows_core::imp::DelegateBox::<PointerEventHandler, F>::new(
                &PointerEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_PointerReleased)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_PointerReleased,
            ))
        }
    }
    pub fn add_PointerExited<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<PointerRoutedEventArgs>,
            ) + 'static,
    {
        let handler: PointerEventHandler = {
            let com = windows_core::imp::DelegateBox::<PointerEventHandler, F>::new(
                &PointerEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_PointerExited)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_PointerExited,
            ))
        }
    }
    pub fn add_Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<TappedRoutedEventArgs>,
            ) + 'static,
    {
        let handler: TappedEventHandler = {
            let com = windows_core::imp::DelegateBox::<TappedEventHandler, F>::new(
                &TappedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Tapped)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Tapped,
            ))
        }
    }
    pub fn add_RightTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<RightTappedRoutedEventArgs>,
            ) + 'static,
    {
        let handler: RightTappedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RightTappedEventHandler, F>::new(
                &RightTappedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_RightTapped)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_RightTapped,
            ))
        }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).UpdateLayout)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_DesiredSize: usize,
    get_AllowDrop: usize,
    put_AllowDrop: usize,
    get_Opacity: usize,
    pub put_Opacity:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Clip: usize,
    put_Clip: usize,
    get_RenderTransform: usize,
    put_RenderTransform: usize,
    get_Projection: usize,
    put_Projection: usize,
    get_Transform3D: usize,
    put_Transform3D: usize,
    get_RenderTransformOrigin: usize,
    put_RenderTransformOrigin: usize,
    get_IsHitTestVisible: usize,
    pub put_IsHitTestVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_Visibility: usize,
    put_Visibility: usize,
    get_RenderSize: usize,
    get_UseLayoutRounding: usize,
    put_UseLayoutRounding: usize,
    get_Transitions: usize,
    put_Transitions: usize,
    get_CacheMode: usize,
    put_CacheMode: usize,
    get_IsTapEnabled: usize,
    put_IsTapEnabled: usize,
    get_IsDoubleTapEnabled: usize,
    put_IsDoubleTapEnabled: usize,
    get_CanDrag: usize,
    put_CanDrag: usize,
    get_IsRightTapEnabled: usize,
    put_IsRightTapEnabled: usize,
    get_IsHoldingEnabled: usize,
    put_IsHoldingEnabled: usize,
    get_ManipulationMode: usize,
    put_ManipulationMode: usize,
    get_PointerCaptures: usize,
    get_ContextFlyout: usize,
    put_ContextFlyout: usize,
    get_CompositeMode: usize,
    put_CompositeMode: usize,
    get_Lights: usize,
    get_CanBeScrollAnchor: usize,
    put_CanBeScrollAnchor: usize,
    get_ExitDisplayModeOnAccessKeyInvoked: usize,
    put_ExitDisplayModeOnAccessKeyInvoked: usize,
    get_IsAccessKeyScope: usize,
    put_IsAccessKeyScope: usize,
    get_AccessKeyScopeOwner: usize,
    put_AccessKeyScopeOwner: usize,
    get_AccessKey: usize,
    put_AccessKey: usize,
    get_KeyTipPlacementMode: usize,
    put_KeyTipPlacementMode: usize,
    get_KeyTipHorizontalOffset: usize,
    put_KeyTipHorizontalOffset: usize,
    get_KeyTipVerticalOffset: usize,
    put_KeyTipVerticalOffset: usize,
    get_KeyTipTarget: usize,
    put_KeyTipTarget: usize,
    get_XYFocusKeyboardNavigation: usize,
    put_XYFocusKeyboardNavigation: usize,
    get_XYFocusUpNavigationStrategy: usize,
    put_XYFocusUpNavigationStrategy: usize,
    get_XYFocusDownNavigationStrategy: usize,
    put_XYFocusDownNavigationStrategy: usize,
    get_XYFocusLeftNavigationStrategy: usize,
    put_XYFocusLeftNavigationStrategy: usize,
    get_XYFocusRightNavigationStrategy: usize,
    put_XYFocusRightNavigationStrategy: usize,
    pub get_KeyboardAccelerators: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_KeyboardAcceleratorPlacementTarget: usize,
    put_KeyboardAcceleratorPlacementTarget: usize,
    get_KeyboardAcceleratorPlacementMode: usize,
    pub put_KeyboardAcceleratorPlacementMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        KeyboardAcceleratorPlacementMode,
    )
        -> windows_core::HRESULT,
    get_HighContrastAdjustment: usize,
    put_HighContrastAdjustment: usize,
    get_TabFocusNavigation: usize,
    put_TabFocusNavigation: usize,
    get_OpacityTransition: usize,
    put_OpacityTransition: usize,
    get_Translation: usize,
    put_Translation: usize,
    get_TranslationTransition: usize,
    put_TranslationTransition: usize,
    get_Rotation: usize,
    put_Rotation: usize,
    get_RotationTransition: usize,
    put_RotationTransition: usize,
    get_Scale: usize,
    put_Scale: usize,
    get_ScaleTransition: usize,
    put_ScaleTransition: usize,
    get_TransformMatrix: usize,
    put_TransformMatrix: usize,
    get_CenterPoint: usize,
    put_CenterPoint: usize,
    get_RotationAxis: usize,
    put_RotationAxis: usize,
    get_ActualOffset: usize,
    get_ActualSize: usize,
    pub get_XamlRoot: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_XamlRoot: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Shadow: usize,
    put_Shadow: usize,
    get_RasterizationScale: usize,
    put_RasterizationScale: usize,
    get_FocusState: usize,
    get_UseSystemFocusVisuals: usize,
    put_UseSystemFocusVisuals: usize,
    get_XYFocusLeft: usize,
    put_XYFocusLeft: usize,
    get_XYFocusRight: usize,
    put_XYFocusRight: usize,
    get_XYFocusUp: usize,
    put_XYFocusUp: usize,
    get_XYFocusDown: usize,
    put_XYFocusDown: usize,
    get_IsTabStop: usize,
    put_IsTabStop: usize,
    get_TabIndex: usize,
    put_TabIndex: usize,
    add_KeyUp: usize,
    remove_KeyUp: usize,
    add_KeyDown: usize,
    remove_KeyDown: usize,
    add_GotFocus: usize,
    remove_GotFocus: usize,
    add_LostFocus: usize,
    remove_LostFocus: usize,
    add_DragStarting: usize,
    remove_DragStarting: usize,
    add_DropCompleted: usize,
    remove_DropCompleted: usize,
    add_CharacterReceived: usize,
    remove_CharacterReceived: usize,
    add_DragEnter: usize,
    remove_DragEnter: usize,
    add_DragLeave: usize,
    remove_DragLeave: usize,
    add_DragOver: usize,
    remove_DragOver: usize,
    add_Drop: usize,
    remove_Drop: usize,
    pub add_PointerPressed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_PointerPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_PointerMoved: usize,
    remove_PointerMoved: usize,
    pub add_PointerReleased: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_PointerReleased:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_PointerEntered: usize,
    remove_PointerEntered: usize,
    pub add_PointerExited: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_PointerExited:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_PointerCaptureLost: usize,
    remove_PointerCaptureLost: usize,
    add_PointerCanceled: usize,
    remove_PointerCanceled: usize,
    add_PointerWheelChanged: usize,
    remove_PointerWheelChanged: usize,
    pub add_Tapped: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Tapped:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_DoubleTapped: usize,
    remove_DoubleTapped: usize,
    add_Holding: usize,
    remove_Holding: usize,
    add_ContextRequested: usize,
    remove_ContextRequested: usize,
    add_ContextCanceled: usize,
    remove_ContextCanceled: usize,
    pub add_RightTapped: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_RightTapped:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_ManipulationStarting: usize,
    remove_ManipulationStarting: usize,
    add_ManipulationInertiaStarting: usize,
    remove_ManipulationInertiaStarting: usize,
    add_ManipulationStarted: usize,
    remove_ManipulationStarted: usize,
    add_ManipulationDelta: usize,
    remove_ManipulationDelta: usize,
    add_ManipulationCompleted: usize,
    remove_ManipulationCompleted: usize,
    add_AccessKeyDisplayRequested: usize,
    remove_AccessKeyDisplayRequested: usize,
    add_AccessKeyDisplayDismissed: usize,
    remove_AccessKeyDisplayDismissed: usize,
    add_AccessKeyInvoked: usize,
    remove_AccessKeyInvoked: usize,
    add_ProcessKeyboardAccelerators: usize,
    remove_ProcessKeyboardAccelerators: usize,
    add_GettingFocus: usize,
    remove_GettingFocus: usize,
    add_LosingFocus: usize,
    remove_LosingFocus: usize,
    add_NoFocusCandidateFound: usize,
    remove_NoFocusCandidateFound: usize,
    add_PreviewKeyDown: usize,
    remove_PreviewKeyDown: usize,
    add_PreviewKeyUp: usize,
    remove_PreviewKeyUp: usize,
    add_BringIntoViewRequested: usize,
    remove_BringIntoViewRequested: usize,
    Measure: usize,
    Arrange: usize,
    CapturePointer: usize,
    ReleasePointerCapture: usize,
    ReleasePointerCaptures: usize,
    AddHandler: usize,
    RemoveHandler: usize,
    TransformToVisual: usize,
    InvalidateMeasure: usize,
    InvalidateArrange: usize,
    pub UpdateLayout: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    CancelDirectManipulations: usize,
    StartDragAsync: usize,
    StartBringIntoView: usize,
    StartBringIntoViewWithOptions: usize,
    TryInvokeKeyboardAccelerator: usize,
    Focus: usize,
    StartAnimation: usize,
    StopAnimation: usize,
}
windows_core::imp::define_interface!(
    IUriRuntimeClass,
    IUriRuntimeClass_Vtbl,
    0x9e365e57_48b2_4160_956f_c7385120bbfc
);
impl windows_core::RuntimeType for IUriRuntimeClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_AbsoluteUri: usize,
    get_DisplayUri: usize,
    get_Domain: usize,
    get_Extension: usize,
    get_Fragment: usize,
    get_Host: usize,
    get_Password: usize,
    get_Path: usize,
    get_Query: usize,
    get_QueryParsed: usize,
    get_RawUri: usize,
    get_SchemeName: usize,
    get_UserName: usize,
    get_Port: usize,
    get_Suspicious: usize,
    Equals: usize,
    CombineUri: usize,
}
windows_core::imp::define_interface!(
    IUriRuntimeClassFactory,
    IUriRuntimeClassFactory_Vtbl,
    0x44a9796f_723e_4fdf_a218_033e75b0c084
);
impl windows_core::RuntimeType for IUriRuntimeClassFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClassFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateWithRelativeUri: usize,
}
windows_core::imp::define_interface!(
    IVector3KeyFrameAnimation,
    IVector3KeyFrameAnimation_Vtbl,
    0xd7da980e_2dde_5dd1_a40c_d6868dd2449e
);
impl windows_core::RuntimeType for IVector3KeyFrameAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IVector3KeyFrameAnimation {
    pub fn InsertKeyFrame(
        &self,
        normalizedprogresskey: f32,
        value: windows_numerics::Vector3,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).InsertKeyFrame)(
                windows_core::Interface::as_raw(self),
                normalizedprogresskey,
                value,
            )
            .ok()
        }
    }
    pub fn InsertKeyFrameWithEasingFunction<P2>(
        &self,
        normalizedprogresskey: f32,
        value: windows_numerics::Vector3,
        easingfunction: P2,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<CompositionEasingFunction>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InsertKeyFrameWithEasingFunction)(
                windows_core::Interface::as_raw(self),
                normalizedprogresskey,
                value,
                easingfunction.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVector3KeyFrameAnimation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InsertKeyFrame: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    pub InsertKeyFrameWithEasingFunction: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        windows_numerics::Vector3,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IViewbox,
    IViewbox_Vtbl,
    0x510683e8_d0fe_5ef4_85bd_e1131076ac22
);
impl windows_core::RuntimeType for IViewbox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IViewbox {
    pub fn put_Child<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Child)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_Stretch(&self, value: Stretch) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Stretch)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewbox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Child: usize,
    pub put_Child: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Stretch: usize,
    pub put_Stretch:
        unsafe extern "system" fn(*mut core::ffi::c_void, Stretch) -> windows_core::HRESULT,
    get_StretchDirection: usize,
    put_StretchDirection: usize,
}
windows_core::imp::define_interface!(
    IVisual,
    IVisual_Vtbl,
    0xc0eeab6c_c897_5ac6_a1c9_63abd5055b9b
);
impl windows_core::RuntimeType for IVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IVisual {
    pub fn put_CenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_CenterPoint)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Scale)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_AnchorPoint: usize,
    put_AnchorPoint: usize,
    get_BackfaceVisibility: usize,
    put_BackfaceVisibility: usize,
    get_BorderMode: usize,
    put_BorderMode: usize,
    get_CenterPoint: usize,
    pub put_CenterPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    get_Clip: usize,
    put_Clip: usize,
    get_CompositeMode: usize,
    put_CompositeMode: usize,
    get_IsVisible: usize,
    put_IsVisible: usize,
    get_Offset: usize,
    put_Offset: usize,
    get_Opacity: usize,
    put_Opacity: usize,
    get_Orientation: usize,
    put_Orientation: usize,
    get_Parent: usize,
    get_RotationAngle: usize,
    put_RotationAngle: usize,
    get_RotationAngleInDegrees: usize,
    put_RotationAngleInDegrees: usize,
    get_RotationAxis: usize,
    put_RotationAxis: usize,
    pub get_Scale: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_numerics::Vector3,
    ) -> windows_core::HRESULT,
    put_Scale: usize,
    get_Size: usize,
    put_Size: usize,
    get_TransformMatrix: usize,
    put_TransformMatrix: usize,
}
windows_core::imp::define_interface!(
    IVisualTreeHelper,
    IVisualTreeHelper_Vtbl,
    0x5f69ac1e_6504_5e3f_a11c_87684c1db814
);
impl windows_core::RuntimeType for IVisualTreeHelper {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelper_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IVisualTreeHelperStatics,
    IVisualTreeHelperStatics_Vtbl,
    0x5aece43c_7651_5bb5_855c_2198496e455e
);
impl windows_core::RuntimeType for IVisualTreeHelperStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    FindElementsInHostCoordinatesPoint: usize,
    FindElementsInHostCoordinatesRect: usize,
    FindAllElementsInHostCoordinatesPoint: usize,
    FindAllElementsInHostCoordinatesRect: usize,
    pub GetChild: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetChildrenCount: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i32,
    ) -> windows_core::HRESULT,
    GetParent: usize,
    DisconnectChildrenRecursive: usize,
    GetOpenPopups: usize,
    GetOpenPopupsForXamlRoot: usize,
}
windows_core::imp::define_interface!(
    IWindow,
    IWindow_Vtbl,
    0x61f0ec79_5d52_56b5_86fb_40fa4af288b0
);
impl windows_core::RuntimeType for IWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IWindow {
    pub fn get_Content(&self) -> windows_core::Result<UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_Content<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Content)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_Title(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Title)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_ExtendsContentIntoTitleBar(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_ExtendsContentIntoTitleBar)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_Closed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<WindowEventArgs>)
            + 'static,
    {
        let handler: TypedEventHandler<windows_core::IInspectable, WindowEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<windows_core::IInspectable, WindowEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<windows_core::IInspectable, WindowEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Closed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Closed,
            ))
        }
    }
    pub fn Activate(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub fn SetTitleBar<P0>(&self, titlebar: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTitleBar)(
                windows_core::Interface::as_raw(self),
                titlebar.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindow_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Bounds: usize,
    get_Visible: usize,
    pub get_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub put_Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_CoreWindow: usize,
    get_Compositor: usize,
    get_Dispatcher: usize,
    get_DispatcherQueue: usize,
    get_Title: usize,
    pub put_Title: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ExtendsContentIntoTitleBar: usize,
    pub put_ExtendsContentIntoTitleBar:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    add_Activated: usize,
    remove_Activated: usize,
    pub add_Closed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Closed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_SizeChanged: usize,
    remove_SizeChanged: usize,
    add_VisibilityChanged: usize,
    remove_VisibilityChanged: usize,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    Close: usize,
    pub SetTitleBar: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWindow2,
    IWindow2_Vtbl,
    0x42febaa5_1c32_522a_a591_57618c6f665d
);
impl windows_core::RuntimeType for IWindow2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IWindow2 {
    pub fn put_SystemBackdrop<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SystemBackdrop>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_SystemBackdrop)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn get_AppWindow(&self) -> windows_core::Result<AppWindow> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AppWindow)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindow2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_SystemBackdrop: usize,
    pub put_SystemBackdrop: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_AppWindow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWindowEventArgs,
    IWindowEventArgs_Vtbl,
    0x1140827c_fe0a_5268_bc2b_f4492c2ccb49
);
impl windows_core::RuntimeType for IWindowEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IWindowEventArgs {
    pub fn put_Handled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Handled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Handled: usize,
    pub put_Handled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWindowFactory,
    IWindowFactory_Vtbl,
    0xf0441536_afef_5222_918f_324a9b2dec75
);
impl windows_core::RuntimeType for IWindowFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWindowNative,
    IWindowNative_Vtbl,
    0xeecdbf0e_bae9_4cb6_a68e_9598e1cb57bb
);
windows_core::imp::interface_hierarchy!(IWindowNative, windows_core::IUnknown);
impl IWindowNative {
    pub unsafe fn get_WindowHandle(
        &self,
        hwnd: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).get_WindowHandle)(
                windows_core::Interface::as_raw(self),
                hwnd as _,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_WindowHandle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXamlControlsResources,
    IXamlControlsResources_Vtbl,
    0x918ca043_f42c_5805_861b_62d6d1d0c162
);
impl windows_core::RuntimeType for IXamlControlsResources {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsResources_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_UseCompactResources: usize,
    put_UseCompactResources: usize,
}
windows_core::imp::define_interface!(
    IXamlMember,
    IXamlMember_Vtbl,
    0xbf3a2913_5c63_50ec_8660_61809be7b9b9
);
impl windows_core::RuntimeType for IXamlMember {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IXamlMember,
    windows_core::IUnknown,
    windows_core::IInspectable
);
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMember_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsAttachable: usize,
    get_IsDependencyProperty: usize,
    get_IsReadOnly: usize,
    get_Name: usize,
    get_TargetType: usize,
    get_Type: usize,
    GetValue: usize,
    SetValue: usize,
}
windows_core::imp::define_interface!(
    IXamlMetadataProvider,
    IXamlMetadataProvider_Vtbl,
    0xa96251f0_2214_5d53_8746_ce99a2593cd7
);
impl windows_core::RuntimeType for IXamlMetadataProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Markup.IXamlMetadataProvider",
    );
}
windows_core::imp::interface_hierarchy!(
    IXamlMetadataProvider,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IXamlMetadataProvider {
    pub fn GetXamlType(&self, r#type: &TypeName) -> windows_core::Result<IXamlType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXamlType)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(r#type),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXamlTypeByFullName(&self, fullname: &str) -> windows_core::Result<IXamlType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXamlTypeByFullName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(fullname)),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXmlnsDefinitions(
        &self,
    ) -> windows_core::Result<windows_core::Array<XmlnsDefinition>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetXmlnsDefinitions)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<XmlnsDefinition>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeName for IXamlMetadataProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IXamlMetadataProvider";
}
pub trait IXamlMetadataProvider_Impl: windows_core::IUnknownImpl {
    fn GetXamlType(&self, r#type: &TypeName) -> windows_core::Result<IXamlType>;
    fn GetXamlTypeByFullName(
        &self,
        fullName: &windows_core::HSTRING,
    ) -> windows_core::Result<IXamlType>;
    fn GetXmlnsDefinitions(&self) -> windows_core::Result<windows_core::Array<XmlnsDefinition>>;
}
impl IXamlMetadataProvider_Vtbl {
    pub const fn new<Identity: IXamlMetadataProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetXamlType<
            Identity: IXamlMetadataProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            r#type: core::mem::MaybeUninit<TypeName>,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlMetadataProvider_Impl::GetXamlType(this, core::mem::transmute(&r#type)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetXamlTypeByFullName<
            Identity: IXamlMetadataProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fullname: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlMetadataProvider_Impl::GetXamlTypeByFullName(
                    this,
                    core::mem::transmute(&fullname),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetXmlnsDefinitions<
            Identity: IXamlMetadataProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut core::mem::MaybeUninit<XmlnsDefinition>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlMetadataProvider_Impl::GetXmlnsDefinitions(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IXamlMetadataProvider, OFFSET>(
            ),
            GetXamlType: GetXamlType::<Identity, OFFSET>,
            GetXamlTypeByFullName: GetXamlTypeByFullName::<Identity, OFFSET>,
            GetXmlnsDefinitions: GetXmlnsDefinitions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlMetadataProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMetadataProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetXamlType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        core::mem::MaybeUninit<TypeName>,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetXamlTypeByFullName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetXmlnsDefinitions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut core::mem::MaybeUninit<XmlnsDefinition>,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXamlReader,
    IXamlReader_Vtbl,
    0x54ce54c8_38c6_50d9_ac98_4b03eddbde9f
);
impl windows_core::RuntimeType for IXamlReader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IXamlReaderStatics,
    IXamlReaderStatics_Vtbl,
    0x82a4cd9e_435e_5aeb_8c4f_300cece45cae
);
impl windows_core::RuntimeType for IXamlReaderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReaderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Load: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    LoadWithInitialTemplateValidation: usize,
}
windows_core::imp::define_interface!(
    IXamlRoot,
    IXamlRoot_Vtbl,
    0x60cb215a_ad15_520a_8b01_4416824f0441
);
impl windows_core::RuntimeType for IXamlRoot {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRoot_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Content: usize,
    get_Size: usize,
    get_RasterizationScale: usize,
    get_IsHostVisible: usize,
    add_Changed: usize,
    remove_Changed: usize,
}
windows_core::imp::define_interface!(
    IXamlType,
    IXamlType_Vtbl,
    0xd24219df_7ec9_57f1_a27b_6af251d9c5bc
);
impl windows_core::RuntimeType for IXamlType {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IXamlType,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IXamlType {
    pub fn get_BaseType(&self) -> windows_core::Result<IXamlType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BaseType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_ContentProperty(&self) -> windows_core::Result<IXamlMember> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ContentProperty)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_FullName(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FullName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn get_IsArray(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsArray)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_IsCollection(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsCollection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_IsConstructible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsConstructible)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_IsDictionary(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsDictionary)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_IsMarkupExtension(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsMarkupExtension)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_IsBindable(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsBindable)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn get_ItemType(&self) -> windows_core::Result<IXamlType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ItemType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_KeyType(&self) -> windows_core::Result<IXamlType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KeyType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_BoxedType(&self) -> windows_core::Result<IXamlType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BoxedType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_UnderlyingType(&self) -> windows_core::Result<TypeName> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UnderlyingType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ActivateInstance(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateInstance)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFromString(
        &self,
        value: &str,
    ) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFromString)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMember(&self, name: &str) -> windows_core::Result<IXamlMember> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMember)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(name)),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddToVector<P0, P1>(&self, instance: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddToVector)(
                windows_core::Interface::as_raw(self),
                instance.param().abi(),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AddToMap<P0, P1, P2>(&self, instance: P0, key: P1, value: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
        P2: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddToMap)(
                windows_core::Interface::as_raw(self),
                instance.param().abi(),
                key.param().abi(),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RunInitializer(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RunInitializer)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlType_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_BaseType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_ContentProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_FullName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_IsArray:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub get_IsCollection:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub get_IsConstructible:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub get_IsDictionary:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub get_IsMarkupExtension:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub get_IsBindable:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub get_ItemType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_KeyType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_BoxedType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_UnderlyingType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<TypeName>,
    ) -> windows_core::HRESULT,
    pub ActivateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateFromString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetMember: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddToVector: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddToMap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RunInitializer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IconElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    IconElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IconElement, FrameworkElement, UIElement, DependencyObject);
impl IconElement {}
impl windows_core::RuntimeType for IconElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IIconElement>();
}
unsafe impl windows_core::Interface for IconElement {
    type Vtable = <IIconElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IIconElement as windows_core::Interface>::IID;
}
impl core::ops::Deref for IconElement {
    type Target = IIconElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for IconElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IconElement";
}
unsafe impl Send for IconElement {}
unsafe impl Sync for IconElement {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Image, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Image, FrameworkElement, UIElement, DependencyObject);
impl Image {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Image, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Image {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IImage>();
}
unsafe impl windows_core::Interface for Image {
    type Vtable = <IImage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IImage as windows_core::Interface>::IID;
}
impl core::ops::Deref for Image {
    type Target = IImage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Image {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Image";
}
unsafe impl Send for Image {}
unsafe impl Sync for Image {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ImageSource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ImageSource, DependencyObject);
impl ImageSource {}
impl windows_core::RuntimeType for ImageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IImageSource>();
}
unsafe impl windows_core::Interface for ImageSource {
    type Vtable = <IImageSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IImageSource as windows_core::Interface>::IID;
}
impl core::ops::Deref for ImageSource {
    type Target = IImageSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ImageSource";
}
unsafe impl Send for ImageSource {}
unsafe impl Sync for ImageSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImplicitAnimationCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ImplicitAnimationCollection,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ImplicitAnimationCollection, CompositionObject);
impl ImplicitAnimationCollection {}
impl windows_core::RuntimeType for ImplicitAnimationCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IImplicitAnimationCollection>();
}
unsafe impl windows_core::Interface for ImplicitAnimationCollection {
    type Vtable = <IImplicitAnimationCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IImplicitAnimationCollection as windows_core::Interface>::IID;
}
impl core::ops::Deref for ImplicitAnimationCollection {
    type Target = IImplicitAnimationCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ImplicitAnimationCollection {
    const NAME: &'static str = "Microsoft.UI.Composition.ImplicitAnimationCollection";
}
unsafe impl Send for ImplicitAnimationCollection {}
unsafe impl Sync for ImplicitAnimationCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfoBadge(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InfoBadge,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    InfoBadge,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl InfoBadge {
    pub fn new() -> windows_core::Result<InfoBadge> {
        Self::IInfoBadgeFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<InfoBadge>
    where
        T: windows_core::Compose,
    {
        Self::IInfoBadgeFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IInfoBadgeFactory<R, F: FnOnce(&IInfoBadgeFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InfoBadge, IInfoBadgeFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InfoBadge {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInfoBadge>();
}
unsafe impl windows_core::Interface for InfoBadge {
    type Vtable = <IInfoBadge as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInfoBadge as windows_core::Interface>::IID;
}
impl core::ops::Deref for InfoBadge {
    type Target = IInfoBadge;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InfoBadge {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.InfoBadge";
}
unsafe impl Send for InfoBadge {}
unsafe impl Sync for InfoBadge {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfoBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InfoBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    InfoBar,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl InfoBar {
    pub fn new() -> windows_core::Result<InfoBar> {
        Self::IInfoBarFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<InfoBar>
    where
        T: windows_core::Compose,
    {
        Self::IInfoBarFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IInfoBarFactory<R, F: FnOnce(&IInfoBarFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InfoBar, IInfoBarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InfoBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInfoBar>();
}
unsafe impl windows_core::Interface for InfoBar {
    type Vtable = <IInfoBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInfoBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for InfoBar {
    type Target = IInfoBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InfoBar {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.InfoBar";
}
unsafe impl Send for InfoBar {}
unsafe impl Sync for InfoBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfoBarClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InfoBarClosedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl InfoBarClosedEventArgs {}
impl windows_core::RuntimeType for InfoBarClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInfoBarClosedEventArgs>();
}
unsafe impl windows_core::Interface for InfoBarClosedEventArgs {
    type Vtable = <IInfoBarClosedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInfoBarClosedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for InfoBarClosedEventArgs {
    type Target = IInfoBarClosedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InfoBarClosedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.InfoBarClosedEventArgs";
}
unsafe impl Send for InfoBarClosedEventArgs {}
unsafe impl Sync for InfoBarClosedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InfoBarSeverity(pub i32);
impl InfoBarSeverity {
    pub const Informational: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const Warning: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
}
impl windows_core::TypeKind for InfoBarSeverity {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for InfoBarSeverity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.InfoBarSeverity;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inline(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Inline, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Inline, TextElement, DependencyObject);
impl Inline {}
impl windows_core::RuntimeType for Inline {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInline>();
}
unsafe impl windows_core::Interface for Inline {
    type Vtable = <IInline as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInline as windows_core::Interface>::IID;
}
impl core::ops::Deref for Inline {
    type Target = IInline;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Inline {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Inline";
}
unsafe impl Send for Inline {}
unsafe impl Sync for Inline {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InlineCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InlineCollection,
    windows_core::IUnknown,
    windows_core::IInspectable,
    windows_collections::IVector<Inline>
);
impl InlineCollection {}
impl windows_core::RuntimeType for InlineCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVector<Inline>>();
}
unsafe impl windows_core::Interface for InlineCollection {
    type Vtable = <windows_collections::IVector<Inline> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <windows_collections::IVector<Inline> as windows_core::Interface>::IID;
}
impl core::ops::Deref for InlineCollection {
    type Target = windows_collections::IVector<Inline>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InlineCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.InlineCollection";
}
unsafe impl Send for InlineCollection {}
unsafe impl Sync for InlineCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ItemCollection,
    windows_core::IUnknown,
    windows_core::IInspectable,
    windows_collections::IObservableVector<windows_core::IInspectable>
);
impl ItemCollection {}
impl windows_core::RuntimeType for ItemCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        windows_collections::IObservableVector<windows_core::IInspectable>,
    >();
}
unsafe impl windows_core::Interface for ItemCollection {
    type Vtable = < windows_collections::IObservableVector < windows_core::IInspectable > as windows_core::Interface >::Vtable ;
    const IID: windows_core::GUID = <windows_collections::IObservableVector<
        windows_core::IInspectable,
    > as windows_core::Interface>::IID;
}
impl core::ops::Deref for ItemCollection {
    type Target = windows_collections::IObservableVector<windows_core::IInspectable>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ItemCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ItemCollection";
}
unsafe impl Send for ItemCollection {}
unsafe impl Sync for ItemCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemContainer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ItemContainer,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ItemContainer,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ItemContainer {}
impl windows_core::RuntimeType for ItemContainer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IItemContainer>();
}
unsafe impl windows_core::Interface for ItemContainer {
    type Vtable = <IItemContainer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IItemContainer as windows_core::Interface>::IID;
}
impl core::ops::Deref for ItemContainer {
    type Target = IItemContainer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ItemContainer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ItemContainer";
}
unsafe impl Send for ItemContainer {}
unsafe impl Sync for ItemContainer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemsControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ItemsControl,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ItemsControl {}
impl windows_core::RuntimeType for ItemsControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IItemsControl>();
}
unsafe impl windows_core::Interface for ItemsControl {
    type Vtable = <IItemsControl as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IItemsControl as windows_core::Interface>::IID;
}
impl core::ops::Deref for ItemsControl {
    type Target = IItemsControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ItemsControl {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ItemsControl";
}
unsafe impl Send for ItemsControl {}
unsafe impl Sync for ItemsControl {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyFrameAnimation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    KeyFrameAnimation,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    KeyFrameAnimation,
    ICompositionAnimationBase,
    CompositionAnimation,
    CompositionObject
);
impl KeyFrameAnimation {}
impl windows_core::RuntimeType for KeyFrameAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IKeyFrameAnimation>();
}
unsafe impl windows_core::Interface for KeyFrameAnimation {
    type Vtable = <IKeyFrameAnimation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IKeyFrameAnimation as windows_core::Interface>::IID;
}
impl core::ops::Deref for KeyFrameAnimation {
    type Target = IKeyFrameAnimation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for KeyFrameAnimation {
    const NAME: &'static str = "Microsoft.UI.Composition.KeyFrameAnimation";
}
unsafe impl Send for KeyFrameAnimation {}
unsafe impl Sync for KeyFrameAnimation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyboardAccelerator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    KeyboardAccelerator,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(KeyboardAccelerator, DependencyObject);
impl KeyboardAccelerator {
    pub fn new() -> windows_core::Result<KeyboardAccelerator> {
        Self::IKeyboardAcceleratorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<KeyboardAccelerator>
    where
        T: windows_core::Compose,
    {
        Self::IKeyboardAcceleratorFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IKeyboardAcceleratorFactory<
        R,
        F: FnOnce(&IKeyboardAcceleratorFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            KeyboardAccelerator,
            IKeyboardAcceleratorFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for KeyboardAccelerator {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IKeyboardAccelerator>();
}
unsafe impl windows_core::Interface for KeyboardAccelerator {
    type Vtable = <IKeyboardAccelerator as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IKeyboardAccelerator as windows_core::Interface>::IID;
}
impl core::ops::Deref for KeyboardAccelerator {
    type Target = IKeyboardAccelerator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for KeyboardAccelerator {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.KeyboardAccelerator";
}
unsafe impl Send for KeyboardAccelerator {}
unsafe impl Sync for KeyboardAccelerator {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyboardAcceleratorInvokedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    KeyboardAcceleratorInvokedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl KeyboardAcceleratorInvokedEventArgs {}
impl windows_core::RuntimeType for KeyboardAcceleratorInvokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IKeyboardAcceleratorInvokedEventArgs>();
}
unsafe impl windows_core::Interface for KeyboardAcceleratorInvokedEventArgs {
    type Vtable = <IKeyboardAcceleratorInvokedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IKeyboardAcceleratorInvokedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for KeyboardAcceleratorInvokedEventArgs {
    type Target = IKeyboardAcceleratorInvokedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for KeyboardAcceleratorInvokedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.KeyboardAcceleratorInvokedEventArgs";
}
unsafe impl Send for KeyboardAcceleratorInvokedEventArgs {}
unsafe impl Sync for KeyboardAcceleratorInvokedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KeyboardAcceleratorPlacementMode(pub i32);
impl KeyboardAcceleratorPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
}
impl windows_core::TypeKind for KeyboardAcceleratorPlacementMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for KeyboardAcceleratorPlacementMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Input.KeyboardAcceleratorPlacementMode;i4)",
    );
}
pub type LPARAM = isize;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaunchActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    LaunchActivatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl LaunchActivatedEventArgs {}
impl windows_core::RuntimeType for LaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ILaunchActivatedEventArgs>();
}
unsafe impl windows_core::Interface for LaunchActivatedEventArgs {
    type Vtable = <ILaunchActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILaunchActivatedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for LaunchActivatedEventArgs {
    type Target = ILaunchActivatedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for LaunchActivatedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.LaunchActivatedEventArgs";
}
unsafe impl Send for LaunchActivatedEventArgs {}
unsafe impl Sync for LaunchActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Line(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Line, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Line, Shape, FrameworkElement, UIElement, DependencyObject);
impl Line {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Line, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Line {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ILine>();
}
unsafe impl windows_core::Interface for Line {
    type Vtable = <ILine as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILine as windows_core::Interface>::IID;
}
impl core::ops::Deref for Line {
    type Target = ILine;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Line {
    const NAME: &'static str = "Microsoft.UI.Xaml.Shapes.Line";
}
unsafe impl Send for Line {}
unsafe impl Sync for Line {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinearEasingFunction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    LinearEasingFunction,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    LinearEasingFunction,
    CompositionEasingFunction,
    CompositionObject
);
impl LinearEasingFunction {}
impl windows_core::RuntimeType for LinearEasingFunction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ILinearEasingFunction>();
}
unsafe impl windows_core::Interface for LinearEasingFunction {
    type Vtable = <ILinearEasingFunction as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILinearEasingFunction as windows_core::Interface>::IID;
}
impl core::ops::Deref for LinearEasingFunction {
    type Target = ILinearEasingFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for LinearEasingFunction {
    const NAME: &'static str = "Microsoft.UI.Composition.LinearEasingFunction";
}
unsafe impl Send for LinearEasingFunction {}
unsafe impl Sync for LinearEasingFunction {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ListBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ListBox,
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ListBox {
    pub fn new() -> windows_core::Result<ListBox> {
        Self::IListBoxFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<ListBox>
    where
        T: windows_core::Compose,
    {
        Self::IListBoxFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IListBoxFactory<R, F: FnOnce(&IListBoxFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ListBox, IListBoxFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ListBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IListBox>();
}
unsafe impl windows_core::Interface for ListBox {
    type Vtable = <IListBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IListBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for ListBox {
    type Target = IListBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ListBox {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ListBox";
}
unsafe impl Send for ListBox {}
unsafe impl Sync for ListBox {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ListView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ListView,
    ListViewBase,
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ListView {
    pub fn new() -> windows_core::Result<ListView> {
        Self::IListViewFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<ListView>
    where
        T: windows_core::Compose,
    {
        Self::IListViewFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IListViewFactory<R, F: FnOnce(&IListViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ListView, IListViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ListView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IListView>();
}
unsafe impl windows_core::Interface for ListView {
    type Vtable = <IListView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IListView as windows_core::Interface>::IID;
}
impl core::ops::Deref for ListView {
    type Target = IListView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ListView {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ListView";
}
unsafe impl Send for ListView {}
unsafe impl Sync for ListView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListViewBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ListViewBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ListViewBase,
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ListViewBase {}
impl windows_core::RuntimeType for ListViewBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IListViewBase>();
}
unsafe impl windows_core::Interface for ListViewBase {
    type Vtable = <IListViewBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IListViewBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for ListViewBase {
    type Target = IListViewBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ListViewBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ListViewBase";
}
unsafe impl Send for ListViewBase {}
unsafe impl Sync for ListViewBase {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListViewItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ListViewItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ListViewItem,
    SelectorItem,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ListViewItem {}
impl windows_core::RuntimeType for ListViewItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IListViewItem>();
}
unsafe impl windows_core::Interface for ListViewItem {
    type Vtable = <IListViewItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IListViewItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for ListViewItem {
    type Target = IListViewItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ListViewItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ListViewItem";
}
unsafe impl Send for ListViewItem {}
unsafe impl Sync for ListViewItem {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ListViewSelectionMode(pub i32);
impl ListViewSelectionMode {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Multiple: Self = Self(2i32);
    pub const Extended: Self = Self(3i32);
}
impl windows_core::TypeKind for ListViewSelectionMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ListViewSelectionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.ListViewSelectionMode;i4)",
    );
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: RECT,
    pub rcWork: RECT,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONITORINFOEXW {
    pub monitorInfo: MONITORINFO,
    pub szDevice: [u16; 32],
}
impl Default for MONITORINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MONITOR_DEFAULTTONEAREST: MONITOR_FROM_FLAGS = 2u32;
pub type MONITOR_FROM_FLAGS = u32;
pub type MddBootstrapInitializeOptions = i32;
pub const MddBootstrapInitializeOptions_OnNoMatch_ShowUI: MddBootstrapInitializeOptions = 8i32;
pub const MddBootstrapInitializeOptions_OnPackageIdentity_NOOP: MddBootstrapInitializeOptions =
    16i32;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MenuBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    MenuBar,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl MenuBar {
    pub fn new() -> windows_core::Result<MenuBar> {
        Self::IMenuBarFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<MenuBar>
    where
        T: windows_core::Compose,
    {
        Self::IMenuBarFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IMenuBarFactory<R, F: FnOnce(&IMenuBarFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MenuBar, IMenuBarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MenuBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMenuBar>();
}
unsafe impl windows_core::Interface for MenuBar {
    type Vtable = <IMenuBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMenuBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for MenuBar {
    type Target = IMenuBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MenuBar {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.MenuBar";
}
unsafe impl Send for MenuBar {}
unsafe impl Sync for MenuBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuBarItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MenuBarItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    MenuBarItem,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl MenuBarItem {
    pub fn new() -> windows_core::Result<MenuBarItem> {
        Self::IMenuBarItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<MenuBarItem>
    where
        T: windows_core::Compose,
    {
        Self::IMenuBarItemFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IMenuBarItemFactory<R, F: FnOnce(&IMenuBarItemFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MenuBarItem, IMenuBarItemFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MenuBarItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMenuBarItem>();
}
unsafe impl windows_core::Interface for MenuBarItem {
    type Vtable = <IMenuBarItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMenuBarItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for MenuBarItem {
    type Target = IMenuBarItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MenuBarItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.MenuBarItem";
}
unsafe impl Send for MenuBarItem {}
unsafe impl Sync for MenuBarItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuFlyout(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MenuFlyout,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(MenuFlyout, FlyoutBase, DependencyObject);
impl MenuFlyout {
    pub fn new() -> windows_core::Result<MenuFlyout> {
        Self::IMenuFlyoutFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<MenuFlyout>
    where
        T: windows_core::Compose,
    {
        Self::IMenuFlyoutFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IMenuFlyoutFactory<R, F: FnOnce(&IMenuFlyoutFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MenuFlyout, IMenuFlyoutFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MenuFlyout {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMenuFlyout>();
}
unsafe impl windows_core::Interface for MenuFlyout {
    type Vtable = <IMenuFlyout as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMenuFlyout as windows_core::Interface>::IID;
}
impl core::ops::Deref for MenuFlyout {
    type Target = IMenuFlyout;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MenuFlyout {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.MenuFlyout";
}
unsafe impl Send for MenuFlyout {}
unsafe impl Sync for MenuFlyout {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuFlyoutItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MenuFlyoutItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    MenuFlyoutItem,
    MenuFlyoutItemBase,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl MenuFlyoutItem {
    pub fn new() -> windows_core::Result<MenuFlyoutItem> {
        Self::IMenuFlyoutItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<MenuFlyoutItem>
    where
        T: windows_core::Compose,
    {
        Self::IMenuFlyoutItemFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IMenuFlyoutItemFactory<R, F: FnOnce(&IMenuFlyoutItemFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MenuFlyoutItem, IMenuFlyoutItemFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MenuFlyoutItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMenuFlyoutItem>();
}
unsafe impl windows_core::Interface for MenuFlyoutItem {
    type Vtable = <IMenuFlyoutItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMenuFlyoutItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for MenuFlyoutItem {
    type Target = IMenuFlyoutItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MenuFlyoutItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.MenuFlyoutItem";
}
unsafe impl Send for MenuFlyoutItem {}
unsafe impl Sync for MenuFlyoutItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuFlyoutItemBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MenuFlyoutItemBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    MenuFlyoutItemBase,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl MenuFlyoutItemBase {}
impl windows_core::RuntimeType for MenuFlyoutItemBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMenuFlyoutItemBase>();
}
unsafe impl windows_core::Interface for MenuFlyoutItemBase {
    type Vtable = <IMenuFlyoutItemBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMenuFlyoutItemBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for MenuFlyoutItemBase {
    type Target = IMenuFlyoutItemBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MenuFlyoutItemBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.MenuFlyoutItemBase";
}
unsafe impl Send for MenuFlyoutItemBase {}
unsafe impl Sync for MenuFlyoutItemBase {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuFlyoutSeparator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MenuFlyoutSeparator,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    MenuFlyoutSeparator,
    MenuFlyoutItemBase,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl MenuFlyoutSeparator {
    pub fn new() -> windows_core::Result<MenuFlyoutSeparator> {
        Self::IMenuFlyoutSeparatorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<MenuFlyoutSeparator>
    where
        T: windows_core::Compose,
    {
        Self::IMenuFlyoutSeparatorFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IMenuFlyoutSeparatorFactory<
        R,
        F: FnOnce(&IMenuFlyoutSeparatorFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            MenuFlyoutSeparator,
            IMenuFlyoutSeparatorFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MenuFlyoutSeparator {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMenuFlyoutSeparator>();
}
unsafe impl windows_core::Interface for MenuFlyoutSeparator {
    type Vtable = <IMenuFlyoutSeparator as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMenuFlyoutSeparator as windows_core::Interface>::IID;
}
impl core::ops::Deref for MenuFlyoutSeparator {
    type Target = IMenuFlyoutSeparator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MenuFlyoutSeparator {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.MenuFlyoutSeparator";
}
unsafe impl Send for MenuFlyoutSeparator {}
unsafe impl Sync for MenuFlyoutSeparator {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuFlyoutSubItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MenuFlyoutSubItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    MenuFlyoutSubItem,
    MenuFlyoutItemBase,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl MenuFlyoutSubItem {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            MenuFlyoutSubItem,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MenuFlyoutSubItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMenuFlyoutSubItem>();
}
unsafe impl windows_core::Interface for MenuFlyoutSubItem {
    type Vtable = <IMenuFlyoutSubItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMenuFlyoutSubItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for MenuFlyoutSubItem {
    type Target = IMenuFlyoutSubItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MenuFlyoutSubItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.MenuFlyoutSubItem";
}
unsafe impl Send for MenuFlyoutSubItem {}
unsafe impl Sync for MenuFlyoutSubItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MicaBackdrop(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MicaBackdrop,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(MicaBackdrop, SystemBackdrop, DependencyObject);
impl MicaBackdrop {
    pub fn new() -> windows_core::Result<MicaBackdrop> {
        Self::IMicaBackdropFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<MicaBackdrop>
    where
        T: windows_core::Compose,
    {
        Self::IMicaBackdropFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IMicaBackdropFactory<R, F: FnOnce(&IMicaBackdropFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MicaBackdrop, IMicaBackdropFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MicaBackdrop {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMicaBackdrop>();
}
unsafe impl windows_core::Interface for MicaBackdrop {
    type Vtable = <IMicaBackdrop as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMicaBackdrop as windows_core::Interface>::IID;
}
impl core::ops::Deref for MicaBackdrop {
    type Target = IMicaBackdrop;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MicaBackdrop {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.MicaBackdrop";
}
unsafe impl Send for MicaBackdrop {}
unsafe impl Sync for MicaBackdrop {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MicaKind(pub i32);
impl MicaKind {
    pub const Base: Self = Self(0i32);
    pub const BaseAlt: Self = Self(1i32);
}
impl windows_core::TypeKind for MicaKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for MicaKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.SystemBackdrops.MicaKind;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    NavigationView,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl NavigationView {
    pub fn new() -> windows_core::Result<NavigationView> {
        Self::INavigationViewFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<NavigationView>
    where
        T: windows_core::Compose,
    {
        Self::INavigationViewFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn INavigationViewFactory<R, F: FnOnce(&INavigationViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NavigationView, INavigationViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NavigationView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INavigationView>();
}
unsafe impl windows_core::Interface for NavigationView {
    type Vtable = <INavigationView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INavigationView as windows_core::Interface>::IID;
}
impl core::ops::Deref for NavigationView {
    type Target = INavigationView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NavigationView {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationView";
}
unsafe impl Send for NavigationView {}
unsafe impl Sync for NavigationView {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NavigationViewBackButtonVisible(pub i32);
impl NavigationViewBackButtonVisible {
    pub const Collapsed: Self = Self(0i32);
    pub const Auto: Self = Self(2i32);
}
impl windows_core::TypeKind for NavigationViewBackButtonVisible {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NavigationViewBackButtonVisible {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.NavigationViewBackButtonVisible;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationViewBackRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationViewBackRequestedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl NavigationViewBackRequestedEventArgs {}
impl windows_core::RuntimeType for NavigationViewBackRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewBackRequestedEventArgs>();
}
unsafe impl windows_core::Interface for NavigationViewBackRequestedEventArgs {
    type Vtable = <INavigationViewBackRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <INavigationViewBackRequestedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for NavigationViewBackRequestedEventArgs {
    type Target = INavigationViewBackRequestedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NavigationViewBackRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewBackRequestedEventArgs";
}
unsafe impl Send for NavigationViewBackRequestedEventArgs {}
unsafe impl Sync for NavigationViewBackRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationViewItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationViewItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    NavigationViewItem,
    NavigationViewItemBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl NavigationViewItem {
    pub fn new() -> windows_core::Result<NavigationViewItem> {
        Self::INavigationViewItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<NavigationViewItem>
    where
        T: windows_core::Compose,
    {
        Self::INavigationViewItemFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn INavigationViewItemFactory<
        R,
        F: FnOnce(&INavigationViewItemFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            NavigationViewItem,
            INavigationViewItemFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NavigationViewItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewItem>();
}
unsafe impl windows_core::Interface for NavigationViewItem {
    type Vtable = <INavigationViewItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INavigationViewItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for NavigationViewItem {
    type Target = INavigationViewItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NavigationViewItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewItem";
}
unsafe impl Send for NavigationViewItem {}
unsafe impl Sync for NavigationViewItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationViewItemBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationViewItemBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    NavigationViewItemBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl NavigationViewItemBase {}
impl windows_core::RuntimeType for NavigationViewItemBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewItemBase>();
}
unsafe impl windows_core::Interface for NavigationViewItemBase {
    type Vtable = <INavigationViewItemBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INavigationViewItemBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for NavigationViewItemBase {
    type Target = INavigationViewItemBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NavigationViewItemBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewItemBase";
}
unsafe impl Send for NavigationViewItemBase {}
unsafe impl Sync for NavigationViewItemBase {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationViewItemHeader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationViewItemHeader,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    NavigationViewItemHeader,
    NavigationViewItemBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl NavigationViewItemHeader {
    pub fn new() -> windows_core::Result<NavigationViewItemHeader> {
        Self::INavigationViewItemHeaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<NavigationViewItemHeader>
    where
        T: windows_core::Compose,
    {
        Self::INavigationViewItemHeaderFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn INavigationViewItemHeaderFactory<
        R,
        F: FnOnce(&INavigationViewItemHeaderFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            NavigationViewItemHeader,
            INavigationViewItemHeaderFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NavigationViewItemHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewItemHeader>();
}
unsafe impl windows_core::Interface for NavigationViewItemHeader {
    type Vtable = <INavigationViewItemHeader as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INavigationViewItemHeader as windows_core::Interface>::IID;
}
impl core::ops::Deref for NavigationViewItemHeader {
    type Target = INavigationViewItemHeader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NavigationViewItemHeader {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewItemHeader";
}
unsafe impl Send for NavigationViewItemHeader {}
unsafe impl Sync for NavigationViewItemHeader {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NavigationViewPaneDisplayMode(pub i32);
impl NavigationViewPaneDisplayMode {
    pub const Auto: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Top: Self = Self(2i32);
    pub const LeftCompact: Self = Self(3i32);
    pub const LeftMinimal: Self = Self(4i32);
}
impl windows_core::TypeKind for NavigationViewPaneDisplayMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NavigationViewPaneDisplayMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.NavigationViewPaneDisplayMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationViewSelectionChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationViewSelectionChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl NavigationViewSelectionChangedEventArgs {}
impl windows_core::RuntimeType for NavigationViewSelectionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        INavigationViewSelectionChangedEventArgs,
    >();
}
unsafe impl windows_core::Interface for NavigationViewSelectionChangedEventArgs {
    type Vtable = <INavigationViewSelectionChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <INavigationViewSelectionChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for NavigationViewSelectionChangedEventArgs {
    type Target = INavigationViewSelectionChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NavigationViewSelectionChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewSelectionChangedEventArgs";
}
unsafe impl Send for NavigationViewSelectionChangedEventArgs {}
unsafe impl Sync for NavigationViewSelectionChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumberBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NumberBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    NumberBox,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl NumberBox {
    pub fn new() -> windows_core::Result<NumberBox> {
        Self::INumberBoxFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<NumberBox>
    where
        T: windows_core::Compose,
    {
        Self::INumberBoxFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn INumberBoxFactory<R, F: FnOnce(&INumberBoxFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NumberBox, INumberBoxFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NumberBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INumberBox>();
}
unsafe impl windows_core::Interface for NumberBox {
    type Vtable = <INumberBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INumberBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for NumberBox {
    type Target = INumberBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NumberBox {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NumberBox";
}
unsafe impl Send for NumberBox {}
unsafe impl Sync for NumberBox {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumberBoxValueChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NumberBoxValueChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl NumberBoxValueChangedEventArgs {}
impl windows_core::RuntimeType for NumberBoxValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INumberBoxValueChangedEventArgs>();
}
unsafe impl windows_core::Interface for NumberBoxValueChangedEventArgs {
    type Vtable = <INumberBoxValueChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <INumberBoxValueChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for NumberBoxValueChangedEventArgs {
    type Target = INumberBoxValueChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NumberBoxValueChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NumberBoxValueChangedEventArgs";
}
unsafe impl Send for NumberBoxValueChangedEventArgs {}
unsafe impl Sync for NumberBoxValueChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Orientation(pub i32);
impl Orientation {
    pub const Vertical: Self = Self(0i32);
    pub const Horizontal: Self = Self(1i32);
}
impl windows_core::TypeKind for Orientation {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Orientation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.Orientation;i4)",
    );
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PACKAGE_VERSION {
    pub Anonymous: PACKAGE_VERSION_0,
}
impl Default for PACKAGE_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub union PACKAGE_VERSION_0 {
    pub Version: u64,
    pub Anonymous: PACKAGE_VERSION_0_0,
}
impl Default for PACKAGE_VERSION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PACKAGE_VERSION_0_0 {
    pub Revision: u16,
    pub Build: u16,
    pub Minor: u16,
    pub Major: u16,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Panel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Panel, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Panel, FrameworkElement, UIElement, DependencyObject);
impl Panel {}
impl windows_core::RuntimeType for Panel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPanel>();
}
unsafe impl windows_core::Interface for Panel {
    type Vtable = <IPanel as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPanel as windows_core::Interface>::IID;
}
impl core::ops::Deref for Panel {
    type Target = IPanel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Panel {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Panel";
}
unsafe impl Send for Panel {}
unsafe impl Sync for Panel {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Paragraph(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Paragraph,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(Paragraph, Block, TextElement, DependencyObject);
impl Paragraph {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            Paragraph,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Paragraph {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IParagraph>();
}
unsafe impl windows_core::Interface for Paragraph {
    type Vtable = <IParagraph as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IParagraph as windows_core::Interface>::IID;
}
impl core::ops::Deref for Paragraph {
    type Target = IParagraph;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Paragraph {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Paragraph";
}
unsafe impl Send for Paragraph {}
unsafe impl Sync for Paragraph {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PasswordBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PasswordBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    PasswordBox,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl PasswordBox {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            PasswordBox,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PasswordBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPasswordBox>();
}
unsafe impl windows_core::Interface for PasswordBox {
    type Vtable = <IPasswordBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPasswordBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for PasswordBox {
    type Target = IPasswordBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PasswordBox {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.PasswordBox";
}
unsafe impl Send for PasswordBox {}
unsafe impl Sync for PasswordBox {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PasswordRevealMode(pub i32);
impl PasswordRevealMode {
    pub const Peek: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const Visible: Self = Self(2i32);
}
impl windows_core::TypeKind for PasswordRevealMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PasswordRevealMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.PasswordRevealMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PersonPicture(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PersonPicture,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    PersonPicture,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl PersonPicture {
    pub fn new() -> windows_core::Result<PersonPicture> {
        Self::IPersonPictureFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<PersonPicture>
    where
        T: windows_core::Compose,
    {
        Self::IPersonPictureFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IPersonPictureFactory<R, F: FnOnce(&IPersonPictureFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PersonPicture, IPersonPictureFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PersonPicture {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPersonPicture>();
}
unsafe impl windows_core::Interface for PersonPicture {
    type Vtable = <IPersonPicture as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPersonPicture as windows_core::Interface>::IID;
}
impl core::ops::Deref for PersonPicture {
    type Target = IPersonPicture;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PersonPicture {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.PersonPicture";
}
unsafe impl Send for PersonPicture {}
unsafe impl Sync for PersonPicture {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pivot(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Pivot, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(
    Pivot,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Pivot {
    pub fn new() -> windows_core::Result<Pivot> {
        Self::IPivotFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Pivot>
    where
        T: windows_core::Compose,
    {
        Self::IPivotFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IPivotFactory<R, F: FnOnce(&IPivotFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Pivot, IPivotFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Pivot {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPivot>();
}
unsafe impl windows_core::Interface for Pivot {
    type Vtable = <IPivot as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPivot as windows_core::Interface>::IID;
}
impl core::ops::Deref for Pivot {
    type Target = IPivot;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Pivot {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Pivot";
}
unsafe impl Send for Pivot {}
unsafe impl Sync for Pivot {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PivotItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PivotItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    PivotItem,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl PivotItem {
    pub fn new() -> windows_core::Result<PivotItem> {
        Self::IPivotItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<PivotItem>
    where
        T: windows_core::Compose,
    {
        Self::IPivotItemFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IPivotItemFactory<R, F: FnOnce(&IPivotItemFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PivotItem, IPivotItemFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PivotItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPivotItem>();
}
unsafe impl windows_core::Interface for PivotItem {
    type Vtable = <IPivotItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPivotItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for PivotItem {
    type Target = IPivotItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PivotItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.PivotItem";
}
unsafe impl Send for PivotItem {}
unsafe impl Sync for PivotItem {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlacementMode(pub i32);
impl PlacementMode {
    pub const Bottom: Self = Self(2i32);
    pub const Left: Self = Self(9i32);
    pub const Mouse: Self = Self(7i32);
    pub const Right: Self = Self(4i32);
    pub const Top: Self = Self(10i32);
}
impl windows_core::TypeKind for PlacementMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlacementMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.Primitives.PlacementMode;i4)",
    );
}
windows_core::imp::define_interface!(
    PointerEventHandler,
    PointerEventHandler_Vtbl,
    0xa48a71e1_8bb4_5597_9e31_903a3f6a04fb
);
impl windows_core::RuntimeType for PointerEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct PointerEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct PointerEventHandlerBox<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<PointerRoutedEventArgs>)
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<PointerRoutedEventArgs>)
        + 'static,
> PointerEventHandlerBox<F>
{
    const VTABLE: PointerEventHandler_Vtbl = PointerEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<PointerEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<PointerEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<PointerEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<PointerEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerPoint(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PointerPoint,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl PointerPoint {}
impl windows_core::RuntimeType for PointerPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPointerPoint>();
}
unsafe impl windows_core::Interface for PointerPoint {
    type Vtable = <IPointerPoint as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerPoint as windows_core::Interface>::IID;
}
impl core::ops::Deref for PointerPoint {
    type Target = IPointerPoint;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PointerPoint {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPoint";
}
unsafe impl Send for PointerPoint {}
unsafe impl Sync for PointerPoint {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerPointProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PointerPointProperties,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl PointerPointProperties {}
impl windows_core::RuntimeType for PointerPointProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPointerPointProperties>();
}
unsafe impl windows_core::Interface for PointerPointProperties {
    type Vtable = <IPointerPointProperties as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerPointProperties as windows_core::Interface>::IID;
}
impl core::ops::Deref for PointerPointProperties {
    type Target = IPointerPointProperties;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PointerPointProperties {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPointProperties";
}
unsafe impl Send for PointerPointProperties {}
unsafe impl Sync for PointerPointProperties {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PointerRoutedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(PointerRoutedEventArgs, RoutedEventArgs);
impl PointerRoutedEventArgs {}
impl windows_core::RuntimeType for PointerRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPointerRoutedEventArgs>();
}
unsafe impl windows_core::Interface for PointerRoutedEventArgs {
    type Vtable = <IPointerRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerRoutedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for PointerRoutedEventArgs {
    type Target = IPointerRoutedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PointerRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.PointerRoutedEventArgs";
}
unsafe impl Send for PointerRoutedEventArgs {}
unsafe impl Sync for PointerRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgressBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ProgressBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ProgressBar,
    RangeBase,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ProgressBar {
    pub fn new() -> windows_core::Result<ProgressBar> {
        Self::IProgressBarFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<ProgressBar>
    where
        T: windows_core::Compose,
    {
        Self::IProgressBarFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IProgressBarFactory<R, F: FnOnce(&IProgressBarFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProgressBar, IProgressBarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ProgressBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IProgressBar>();
}
unsafe impl windows_core::Interface for ProgressBar {
    type Vtable = <IProgressBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IProgressBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for ProgressBar {
    type Target = IProgressBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ProgressBar {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ProgressBar";
}
unsafe impl Send for ProgressBar {}
unsafe impl Sync for ProgressBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgressRing(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ProgressRing,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ProgressRing,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ProgressRing {
    pub fn new() -> windows_core::Result<ProgressRing> {
        Self::IProgressRingFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<ProgressRing>
    where
        T: windows_core::Compose,
    {
        Self::IProgressRingFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IProgressRingFactory<R, F: FnOnce(&IProgressRingFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProgressRing, IProgressRingFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ProgressRing {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IProgressRing>();
}
unsafe impl windows_core::Interface for ProgressRing {
    type Vtable = <IProgressRing as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IProgressRing as windows_core::Interface>::IID;
}
impl core::ops::Deref for ProgressRing {
    type Target = IProgressRing;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ProgressRing {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ProgressRing";
}
unsafe impl Send for ProgressRing {}
unsafe impl Sync for ProgressRing {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
pub const RPC_E_CHANGED_MODE: windows_core::HRESULT = windows_core::HRESULT(0x80010106_u32 as _);
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RadioButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RadioButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RadioButton,
    ToggleButton,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RadioButton {
    pub fn new() -> windows_core::Result<RadioButton> {
        Self::IRadioButtonFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<RadioButton>
    where
        T: windows_core::Compose,
    {
        Self::IRadioButtonFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IRadioButtonFactory<R, F: FnOnce(&IRadioButtonFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RadioButton, IRadioButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RadioButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRadioButton>();
}
unsafe impl windows_core::Interface for RadioButton {
    type Vtable = <IRadioButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRadioButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for RadioButton {
    type Target = IRadioButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RadioButton {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.RadioButton";
}
unsafe impl Send for RadioButton {}
unsafe impl Sync for RadioButton {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RadioButtons(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RadioButtons,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RadioButtons,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RadioButtons {
    pub fn new() -> windows_core::Result<RadioButtons> {
        Self::IRadioButtonsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<RadioButtons>
    where
        T: windows_core::Compose,
    {
        Self::IRadioButtonsFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IRadioButtonsFactory<R, F: FnOnce(&IRadioButtonsFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RadioButtons, IRadioButtonsFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RadioButtons {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRadioButtons>();
}
unsafe impl windows_core::Interface for RadioButtons {
    type Vtable = <IRadioButtons as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRadioButtons as windows_core::Interface>::IID;
}
impl core::ops::Deref for RadioButtons {
    type Target = IRadioButtons;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RadioButtons {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.RadioButtons";
}
unsafe impl Send for RadioButtons {}
unsafe impl Sync for RadioButtons {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RangeBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RangeBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RangeBase,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RangeBase {}
impl windows_core::RuntimeType for RangeBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRangeBase>();
}
unsafe impl windows_core::Interface for RangeBase {
    type Vtable = <IRangeBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRangeBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for RangeBase {
    type Target = IRangeBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RangeBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.RangeBase";
}
unsafe impl Send for RangeBase {}
unsafe impl Sync for RangeBase {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RangeBaseValueChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RangeBaseValueChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(RangeBaseValueChangedEventArgs, RoutedEventArgs);
impl RangeBaseValueChangedEventArgs {}
impl windows_core::RuntimeType for RangeBaseValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRangeBaseValueChangedEventArgs>();
}
unsafe impl windows_core::Interface for RangeBaseValueChangedEventArgs {
    type Vtable = <IRangeBaseValueChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IRangeBaseValueChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for RangeBaseValueChangedEventArgs {
    type Target = IRangeBaseValueChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RangeBaseValueChangedEventArgs {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.Primitives.RangeBaseValueChangedEventArgs";
}
unsafe impl Send for RangeBaseValueChangedEventArgs {}
unsafe impl Sync for RangeBaseValueChangedEventArgs {}
windows_core::imp::define_interface!(
    RangeBaseValueChangedEventHandler,
    RangeBaseValueChangedEventHandler_Vtbl,
    0x23f0e209_9455_54cb_b8bc_0b49553c7dcc
);
impl windows_core::RuntimeType for RangeBaseValueChangedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct RangeBaseValueChangedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct RangeBaseValueChangedEventHandlerBox<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<RangeBaseValueChangedEventArgs>,
        ) + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<RangeBaseValueChangedEventArgs>,
        ) + 'static,
> RangeBaseValueChangedEventHandlerBox<F>
{
    const VTABLE: RangeBaseValueChangedEventHandler_Vtbl = RangeBaseValueChangedEventHandler_Vtbl {
        base__:
            windows_core::IUnknown_Vtbl {
                QueryInterface: windows_core::imp::DelegateBox::<
                    RangeBaseValueChangedEventHandler,
                    F,
                >::QueryInterface,
                AddRef:
                    windows_core::imp::DelegateBox::<RangeBaseValueChangedEventHandler, F>::AddRef,
                Release:
                    windows_core::imp::DelegateBox::<RangeBaseValueChangedEventHandler, F>::Release,
            },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<RangeBaseValueChangedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RatingControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RatingControl,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RatingControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RatingControl {
    pub fn new() -> windows_core::Result<RatingControl> {
        Self::IRatingControlFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<RatingControl>
    where
        T: windows_core::Compose,
    {
        Self::IRatingControlFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IRatingControlFactory<R, F: FnOnce(&IRatingControlFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RatingControl, IRatingControlFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RatingControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRatingControl>();
}
unsafe impl windows_core::Interface for RatingControl {
    type Vtable = <IRatingControl as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRatingControl as windows_core::Interface>::IID;
}
impl core::ops::Deref for RatingControl {
    type Target = IRatingControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RatingControl {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.RatingControl";
}
unsafe impl Send for RatingControl {}
unsafe impl Sync for RatingControl {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rectangle(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Rectangle,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    Rectangle,
    Shape,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Rectangle {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            Rectangle,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Rectangle {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRectangle>();
}
unsafe impl windows_core::Interface for Rectangle {
    type Vtable = <IRectangle as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRectangle as windows_core::Interface>::IID;
}
impl core::ops::Deref for Rectangle {
    type Target = IRectangle;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Rectangle {
    const NAME: &'static str = "Microsoft.UI.Xaml.Shapes.Rectangle";
}
unsafe impl Send for Rectangle {}
unsafe impl Sync for Rectangle {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelativePanel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RelativePanel,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RelativePanel,
    Panel,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RelativePanel {
    pub fn new() -> windows_core::Result<RelativePanel> {
        Self::IRelativePanelFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<RelativePanel>
    where
        T: windows_core::Compose,
    {
        Self::IRelativePanelFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    pub fn SetAlignLeftWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IRelativePanelStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAlignLeftWithPanel)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetAlignTopWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IRelativePanelStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAlignTopWithPanel)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetAlignRightWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IRelativePanelStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAlignRightWithPanel)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetAlignBottomWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IRelativePanelStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAlignBottomWithPanel)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetAlignHorizontalCenterWithPanel<P0>(
        element: P0,
        value: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IRelativePanelStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAlignHorizontalCenterWithPanel)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetAlignVerticalCenterWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        Self::IRelativePanelStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAlignVerticalCenterWithPanel)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    fn IRelativePanelFactory<R, F: FnOnce(&IRelativePanelFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RelativePanel, IRelativePanelFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IRelativePanelStatics<R, F: FnOnce(&IRelativePanelStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RelativePanel, IRelativePanelStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RelativePanel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRelativePanel>();
}
unsafe impl windows_core::Interface for RelativePanel {
    type Vtable = <IRelativePanel as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRelativePanel as windows_core::Interface>::IID;
}
impl core::ops::Deref for RelativePanel {
    type Target = IRelativePanel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RelativePanel {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.RelativePanel";
}
unsafe impl Send for RelativePanel {}
unsafe impl Sync for RelativePanel {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepeatButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RepeatButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RepeatButton,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RepeatButton {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            RepeatButton,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RepeatButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRepeatButton>();
}
unsafe impl windows_core::Interface for RepeatButton {
    type Vtable = <IRepeatButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRepeatButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for RepeatButton {
    type Target = IRepeatButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RepeatButton {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.RepeatButton";
}
unsafe impl Send for RepeatButton {}
unsafe impl Sync for RepeatButton {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceDictionary(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ResourceDictionary,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ResourceDictionary, DependencyObject);
impl ResourceDictionary {
    pub fn new() -> windows_core::Result<ResourceDictionary> {
        Self::IResourceDictionaryFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<ResourceDictionary>
    where
        T: windows_core::Compose,
    {
        Self::IResourceDictionaryFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IResourceDictionaryFactory<
        R,
        F: FnOnce(&IResourceDictionaryFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            ResourceDictionary,
            IResourceDictionaryFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ResourceDictionary {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IResourceDictionary>();
}
unsafe impl windows_core::Interface for ResourceDictionary {
    type Vtable = <IResourceDictionary as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IResourceDictionary as windows_core::Interface>::IID;
}
impl core::ops::Deref for ResourceDictionary {
    type Target = IResourceDictionary;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ResourceDictionary {
    const NAME: &'static str = "Microsoft.UI.Xaml.ResourceDictionary";
}
unsafe impl Send for ResourceDictionary {}
unsafe impl Sync for ResourceDictionary {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RichEditBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RichEditBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RichEditBox,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RichEditBox {
    pub fn new() -> windows_core::Result<RichEditBox> {
        Self::IRichEditBoxFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<RichEditBox>
    where
        T: windows_core::Compose,
    {
        Self::IRichEditBoxFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IRichEditBoxFactory<R, F: FnOnce(&IRichEditBoxFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RichEditBox, IRichEditBoxFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RichEditBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRichEditBox>();
}
unsafe impl windows_core::Interface for RichEditBox {
    type Vtable = <IRichEditBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRichEditBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for RichEditBox {
    type Target = IRichEditBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RichEditBox {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.RichEditBox";
}
unsafe impl Send for RichEditBox {}
unsafe impl Sync for RichEditBox {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RichEditTextDocument(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RichEditTextDocument,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl RichEditTextDocument {}
impl windows_core::RuntimeType for RichEditTextDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITextDocument>();
}
unsafe impl windows_core::Interface for RichEditTextDocument {
    type Vtable = <ITextDocument as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextDocument as windows_core::Interface>::IID;
}
impl core::ops::Deref for RichEditTextDocument {
    type Target = ITextDocument;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RichEditTextDocument {
    const NAME: &'static str = "Microsoft.UI.Text.RichEditTextDocument";
}
unsafe impl Send for RichEditTextDocument {}
unsafe impl Sync for RichEditTextDocument {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RichTextBlock(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RichTextBlock,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    RichTextBlock,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl RichTextBlock {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            RichTextBlock,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RichTextBlock {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRichTextBlock>();
}
unsafe impl windows_core::Interface for RichTextBlock {
    type Vtable = <IRichTextBlock as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRichTextBlock as windows_core::Interface>::IID;
}
impl core::ops::Deref for RichTextBlock {
    type Target = IRichTextBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RichTextBlock {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.RichTextBlock";
}
unsafe impl Send for RichTextBlock {}
unsafe impl Sync for RichTextBlock {}
windows_core::imp::define_interface!(
    RightTappedEventHandler,
    RightTappedEventHandler_Vtbl,
    0x5070e32f_3dc7_56cf_8fdd_de1b40d0b472
);
impl windows_core::RuntimeType for RightTappedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct RightTappedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct RightTappedEventHandlerBox<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<RightTappedRoutedEventArgs>,
        ) + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<RightTappedRoutedEventArgs>,
        ) + 'static,
> RightTappedEventHandlerBox<F>
{
    const VTABLE: RightTappedEventHandler_Vtbl = RightTappedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<RightTappedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<RightTappedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<RightTappedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<RightTappedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RightTappedRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RightTappedRoutedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(RightTappedRoutedEventArgs, RoutedEventArgs);
impl RightTappedRoutedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            RightTappedRoutedEventArgs,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RightTappedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRightTappedRoutedEventArgs>();
}
unsafe impl windows_core::Interface for RightTappedRoutedEventArgs {
    type Vtable = <IRightTappedRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRightTappedRoutedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for RightTappedRoutedEventArgs {
    type Target = IRightTappedRoutedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RightTappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.RightTappedRoutedEventArgs";
}
unsafe impl Send for RightTappedRoutedEventArgs {}
unsafe impl Sync for RightTappedRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RoutedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl RoutedEventArgs {}
impl windows_core::RuntimeType for RoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRoutedEventArgs>();
}
unsafe impl windows_core::Interface for RoutedEventArgs {
    type Vtable = <IRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRoutedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for RoutedEventArgs {
    type Target = IRoutedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.RoutedEventArgs";
}
unsafe impl Send for RoutedEventArgs {}
unsafe impl Sync for RoutedEventArgs {}
windows_core::imp::define_interface!(
    RoutedEventHandler,
    RoutedEventHandler_Vtbl,
    0xdae23d85_69ca_5bdf_805b_6161a3a215cc
);
impl windows_core::RuntimeType for RoutedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct RoutedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct RoutedEventHandlerBox<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) + 'static,
> RoutedEventHandlerBox<F>
{
    const VTABLE: RoutedEventHandler_Vtbl = RoutedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<RoutedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<RoutedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<RoutedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<RoutedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RowDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RowDefinition,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(RowDefinition, DependencyObject);
impl RowDefinition {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            RowDefinition,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RowDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRowDefinition>();
}
unsafe impl windows_core::Interface for RowDefinition {
    type Vtable = <IRowDefinition as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRowDefinition as windows_core::Interface>::IID;
}
impl core::ops::Deref for RowDefinition {
    type Target = IRowDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RowDefinition {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.RowDefinition";
}
unsafe impl Send for RowDefinition {}
unsafe impl Sync for RowDefinition {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RowDefinitionCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RowDefinitionCollection,
    windows_core::IUnknown,
    windows_core::IInspectable,
    windows_collections::IVector<RowDefinition>
);
impl RowDefinitionCollection {}
impl windows_core::RuntimeType for RowDefinitionCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        windows_collections::IVector<RowDefinition>,
    >();
}
unsafe impl windows_core::Interface for RowDefinitionCollection {
    type Vtable = <windows_collections::IVector<RowDefinition> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <windows_collections::IVector<RowDefinition> as windows_core::Interface>::IID;
}
impl core::ops::Deref for RowDefinitionCollection {
    type Target = windows_collections::IVector<RowDefinition>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RowDefinitionCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.RowDefinitionCollection";
}
unsafe impl Send for RowDefinitionCollection {}
unsafe impl Sync for RowDefinitionCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Run(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Run, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Run, Inline, TextElement, DependencyObject);
impl Run {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Run, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Run {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRun>();
}
unsafe impl windows_core::Interface for Run {
    type Vtable = <IRun as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRun as windows_core::Interface>::IID;
}
impl core::ops::Deref for Run {
    type Target = IRun;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Run {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Run";
}
unsafe impl Send for Run {}
unsafe impl Sync for Run {}
pub type SET_WINDOW_POS_FLAGS = u32;
pub const SWP_NOACTIVATE: SET_WINDOW_POS_FLAGS = 16u32;
pub const SWP_NOSIZE: SET_WINDOW_POS_FLAGS = 1u32;
pub const SWP_NOZORDER: SET_WINDOW_POS_FLAGS = 4u32;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScalarKeyFrameAnimation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ScalarKeyFrameAnimation,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ScalarKeyFrameAnimation,
    ICompositionAnimationBase,
    KeyFrameAnimation,
    CompositionAnimation,
    CompositionObject
);
impl ScalarKeyFrameAnimation {}
impl windows_core::RuntimeType for ScalarKeyFrameAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IScalarKeyFrameAnimation>();
}
unsafe impl windows_core::Interface for ScalarKeyFrameAnimation {
    type Vtable = <IScalarKeyFrameAnimation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IScalarKeyFrameAnimation as windows_core::Interface>::IID;
}
impl core::ops::Deref for ScalarKeyFrameAnimation {
    type Target = IScalarKeyFrameAnimation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ScalarKeyFrameAnimation {
    const NAME: &'static str = "Microsoft.UI.Composition.ScalarKeyFrameAnimation";
}
unsafe impl Send for ScalarKeyFrameAnimation {}
unsafe impl Sync for ScalarKeyFrameAnimation {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ScrollBarVisibility(pub i32);
impl ScrollBarVisibility {
    pub const Disabled: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
    pub const Visible: Self = Self(3i32);
}
impl windows_core::TypeKind for ScrollBarVisibility {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ScrollBarVisibility {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.ScrollBarVisibility;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ScrollIntoViewAlignment(pub i32);
impl ScrollIntoViewAlignment {
    pub const Default: Self = Self(0i32);
    pub const Leading: Self = Self(1i32);
}
impl windows_core::TypeKind for ScrollIntoViewAlignment {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ScrollIntoViewAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.ScrollIntoViewAlignment;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScrollView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ScrollView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ScrollView,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ScrollView {
    pub fn new() -> windows_core::Result<ScrollView> {
        Self::IScrollViewFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<ScrollView>
    where
        T: windows_core::Compose,
    {
        Self::IScrollViewFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IScrollViewFactory<R, F: FnOnce(&IScrollViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ScrollView, IScrollViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ScrollView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IScrollView>();
}
unsafe impl windows_core::Interface for ScrollView {
    type Vtable = <IScrollView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IScrollView as windows_core::Interface>::IID;
}
impl core::ops::Deref for ScrollView {
    type Target = IScrollView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ScrollView {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ScrollView";
}
unsafe impl Send for ScrollView {}
unsafe impl Sync for ScrollView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScrollViewer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ScrollViewer,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ScrollViewer,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ScrollViewer {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            ScrollViewer,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ScrollViewer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IScrollViewer>();
}
unsafe impl windows_core::Interface for ScrollViewer {
    type Vtable = <IScrollViewer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IScrollViewer as windows_core::Interface>::IID;
}
impl core::ops::Deref for ScrollViewer {
    type Target = IScrollViewer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ScrollViewer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ScrollViewer";
}
unsafe impl Send for ScrollViewer {}
unsafe impl Sync for ScrollViewer {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ScrollingScrollBarVisibility(pub i32);
impl ScrollingScrollBarVisibility {
    pub const Auto: Self = Self(0i32);
    pub const Visible: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
}
impl windows_core::TypeKind for ScrollingScrollBarVisibility {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ScrollingScrollBarVisibility {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.ScrollingScrollBarVisibility;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SelectionChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(SelectionChangedEventArgs, RoutedEventArgs);
impl SelectionChangedEventArgs {}
impl windows_core::RuntimeType for SelectionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISelectionChangedEventArgs>();
}
unsafe impl windows_core::Interface for SelectionChangedEventArgs {
    type Vtable = <ISelectionChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISelectionChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for SelectionChangedEventArgs {
    type Target = ISelectionChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SelectionChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.SelectionChangedEventArgs";
}
unsafe impl Send for SelectionChangedEventArgs {}
unsafe impl Sync for SelectionChangedEventArgs {}
windows_core::imp::define_interface!(
    SelectionChangedEventHandler,
    SelectionChangedEventHandler_Vtbl,
    0xa232390d_0e34_595e_8931_fa928a9909f4
);
impl windows_core::RuntimeType for SelectionChangedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct SelectionChangedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct SelectionChangedEventHandlerBox<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) + 'static,
> SelectionChangedEventHandlerBox<F>
{
    const VTABLE: SelectionChangedEventHandler_Vtbl = SelectionChangedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<SelectionChangedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SelectionMode(pub i32);
impl SelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
    pub const Extended: Self = Self(2i32);
}
impl windows_core::TypeKind for SelectionMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SelectionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.SelectionMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selector(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Selector,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    Selector,
    ItemsControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Selector {}
impl windows_core::RuntimeType for Selector {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISelector>();
}
unsafe impl windows_core::Interface for Selector {
    type Vtable = <ISelector as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISelector as windows_core::Interface>::IID;
}
impl core::ops::Deref for Selector {
    type Target = ISelector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Selector {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.Selector";
}
unsafe impl Send for Selector {}
unsafe impl Sync for Selector {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectorBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SelectorBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    SelectorBar,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl SelectorBar {
    pub fn new() -> windows_core::Result<SelectorBar> {
        Self::ISelectorBarFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<SelectorBar>
    where
        T: windows_core::Compose,
    {
        Self::ISelectorBarFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ISelectorBarFactory<R, F: FnOnce(&ISelectorBarFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SelectorBar, ISelectorBarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SelectorBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISelectorBar>();
}
unsafe impl windows_core::Interface for SelectorBar {
    type Vtable = <ISelectorBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISelectorBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for SelectorBar {
    type Target = ISelectorBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SelectorBar {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.SelectorBar";
}
unsafe impl Send for SelectorBar {}
unsafe impl Sync for SelectorBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectorBarItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SelectorBarItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    SelectorBarItem,
    ItemContainer,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl SelectorBarItem {
    pub fn new() -> windows_core::Result<SelectorBarItem> {
        Self::ISelectorBarItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<SelectorBarItem>
    where
        T: windows_core::Compose,
    {
        Self::ISelectorBarItemFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ISelectorBarItemFactory<
        R,
        F: FnOnce(&ISelectorBarItemFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SelectorBarItem, ISelectorBarItemFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SelectorBarItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISelectorBarItem>();
}
unsafe impl windows_core::Interface for SelectorBarItem {
    type Vtable = <ISelectorBarItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISelectorBarItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for SelectorBarItem {
    type Target = ISelectorBarItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SelectorBarItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.SelectorBarItem";
}
unsafe impl Send for SelectorBarItem {}
unsafe impl Sync for SelectorBarItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectorBarSelectionChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SelectorBarSelectionChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl SelectorBarSelectionChangedEventArgs {}
impl windows_core::RuntimeType for SelectorBarSelectionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISelectorBarSelectionChangedEventArgs>();
}
unsafe impl windows_core::Interface for SelectorBarSelectionChangedEventArgs {
    type Vtable = <ISelectorBarSelectionChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ISelectorBarSelectionChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for SelectorBarSelectionChangedEventArgs {
    type Target = ISelectorBarSelectionChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SelectorBarSelectionChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.SelectorBarSelectionChangedEventArgs";
}
unsafe impl Send for SelectorBarSelectionChangedEventArgs {}
unsafe impl Sync for SelectorBarSelectionChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectorItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SelectorItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    SelectorItem,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl SelectorItem {}
impl windows_core::RuntimeType for SelectorItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISelectorItem>();
}
unsafe impl windows_core::Interface for SelectorItem {
    type Vtable = <ISelectorItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISelectorItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for SelectorItem {
    type Target = ISelectorItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SelectorItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.SelectorItem";
}
unsafe impl Send for SelectorItem {}
unsafe impl Sync for SelectorItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetterBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SetterBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(SetterBase, DependencyObject);
impl SetterBase {}
impl windows_core::RuntimeType for SetterBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISetterBase>();
}
unsafe impl windows_core::Interface for SetterBase {
    type Vtable = <ISetterBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISetterBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for SetterBase {
    type Target = ISetterBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SetterBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.SetterBase";
}
unsafe impl Send for SetterBase {}
unsafe impl Sync for SetterBase {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Shape(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Shape, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Shape, FrameworkElement, UIElement, DependencyObject);
impl Shape {}
impl windows_core::RuntimeType for Shape {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IShape>();
}
unsafe impl windows_core::Interface for Shape {
    type Vtable = <IShape as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IShape as windows_core::Interface>::IID;
}
impl core::ops::Deref for Shape {
    type Target = IShape;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Shape {
    const NAME: &'static str = "Microsoft.UI.Xaml.Shapes.Shape";
}
unsafe impl Send for Shape {}
unsafe impl Sync for Shape {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub Width: f32,
    pub Height: f32,
}
impl windows_core::TypeKind for Size {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Size {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Size;f4;f4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SizeChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SizeChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(SizeChangedEventArgs, RoutedEventArgs);
impl SizeChangedEventArgs {}
impl windows_core::RuntimeType for SizeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISizeChangedEventArgs>();
}
unsafe impl windows_core::Interface for SizeChangedEventArgs {
    type Vtable = <ISizeChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISizeChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for SizeChangedEventArgs {
    type Target = ISizeChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SizeChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.SizeChangedEventArgs";
}
unsafe impl Send for SizeChangedEventArgs {}
unsafe impl Sync for SizeChangedEventArgs {}
windows_core::imp::define_interface!(
    SizeChangedEventHandler,
    SizeChangedEventHandler_Vtbl,
    0x8d7b1a58_14c6_51c9_892c_9fcce368e77d
);
impl windows_core::RuntimeType for SizeChangedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct SizeChangedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct SizeChangedEventHandlerBox<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<SizeChangedEventArgs>)
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<SizeChangedEventArgs>)
        + 'static,
> SizeChangedEventHandlerBox<F>
{
    const VTABLE: SizeChangedEventHandler_Vtbl = SizeChangedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<SizeChangedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<SizeChangedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<SizeChangedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<SizeChangedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SizeInt32 {
    pub Width: i32,
    pub Height: i32,
}
impl windows_core::TypeKind for SizeInt32 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SizeInt32 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.SizeInt32;i4;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Slider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Slider, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(
    Slider,
    RangeBase,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Slider {
    pub fn new() -> windows_core::Result<Slider> {
        Self::ISliderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Slider>
    where
        T: windows_core::Compose,
    {
        Self::ISliderFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ISliderFactory<R, F: FnOnce(&ISliderFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Slider, ISliderFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Slider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISlider>();
}
unsafe impl windows_core::Interface for Slider {
    type Vtable = <ISlider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISlider as windows_core::Interface>::IID;
}
impl core::ops::Deref for Slider {
    type Target = ISlider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Slider {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Slider";
}
unsafe impl Send for Slider {}
unsafe impl Sync for Slider {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolidColorBrush(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SolidColorBrush,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(SolidColorBrush, Brush, DependencyObject);
impl SolidColorBrush {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            SolidColorBrush,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SolidColorBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISolidColorBrush>();
}
unsafe impl windows_core::Interface for SolidColorBrush {
    type Vtable = <ISolidColorBrush as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISolidColorBrush as windows_core::Interface>::IID;
}
impl core::ops::Deref for SolidColorBrush {
    type Target = ISolidColorBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SolidColorBrush {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.SolidColorBrush";
}
unsafe impl Send for SolidColorBrush {}
unsafe impl Sync for SolidColorBrush {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SplitButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    SplitButton,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl SplitButton {
    pub fn new() -> windows_core::Result<SplitButton> {
        Self::ISplitButtonFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<SplitButton>
    where
        T: windows_core::Compose,
    {
        Self::ISplitButtonFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ISplitButtonFactory<R, F: FnOnce(&ISplitButtonFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SplitButton, ISplitButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SplitButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISplitButton>();
}
unsafe impl windows_core::Interface for SplitButton {
    type Vtable = <ISplitButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISplitButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for SplitButton {
    type Target = ISplitButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SplitButton {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.SplitButton";
}
unsafe impl Send for SplitButton {}
unsafe impl Sync for SplitButton {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitButtonClickEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SplitButtonClickEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl SplitButtonClickEventArgs {}
impl windows_core::RuntimeType for SplitButtonClickEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISplitButtonClickEventArgs>();
}
unsafe impl windows_core::Interface for SplitButtonClickEventArgs {
    type Vtable = <ISplitButtonClickEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISplitButtonClickEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for SplitButtonClickEventArgs {
    type Target = ISplitButtonClickEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SplitButtonClickEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.SplitButtonClickEventArgs";
}
unsafe impl Send for SplitButtonClickEventArgs {}
unsafe impl Sync for SplitButtonClickEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SplitView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    SplitView,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl SplitView {
    pub fn new() -> windows_core::Result<SplitView> {
        Self::ISplitViewFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<SplitView>
    where
        T: windows_core::Compose,
    {
        Self::ISplitViewFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ISplitViewFactory<R, F: FnOnce(&ISplitViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SplitView, ISplitViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SplitView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISplitView>();
}
unsafe impl windows_core::Interface for SplitView {
    type Vtable = <ISplitView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISplitView as windows_core::Interface>::IID;
}
impl core::ops::Deref for SplitView {
    type Target = ISplitView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SplitView {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.SplitView";
}
unsafe impl Send for SplitView {}
unsafe impl Sync for SplitView {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SplitViewDisplayMode(pub i32);
impl SplitViewDisplayMode {
    pub const Overlay: Self = Self(0i32);
    pub const Inline: Self = Self(1i32);
    pub const CompactOverlay: Self = Self(2i32);
    pub const CompactInline: Self = Self(3i32);
}
impl windows_core::TypeKind for SplitViewDisplayMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SplitViewDisplayMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.SplitViewDisplayMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackPanel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    StackPanel,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    StackPanel,
    Panel,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl StackPanel {
    pub fn new() -> windows_core::Result<StackPanel> {
        Self::IStackPanelFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<StackPanel>
    where
        T: windows_core::Compose,
    {
        Self::IStackPanelFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IStackPanelFactory<R, F: FnOnce(&IStackPanelFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StackPanel, IStackPanelFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StackPanel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IStackPanel>();
}
unsafe impl windows_core::Interface for StackPanel {
    type Vtable = <IStackPanel as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStackPanel as windows_core::Interface>::IID;
}
impl core::ops::Deref for StackPanel {
    type Target = IStackPanel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for StackPanel {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.StackPanel";
}
unsafe impl Send for StackPanel {}
unsafe impl Sync for StackPanel {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Stretch(pub i32);
impl Stretch {
    pub const None: Self = Self(0i32);
    pub const Fill: Self = Self(1i32);
    pub const Uniform: Self = Self(2i32);
    pub const UniformToFill: Self = Self(3i32);
}
impl windows_core::TypeKind for Stretch {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Stretch {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Media.Stretch;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Style(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Style, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Style, DependencyObject);
impl Style {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Style, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Style {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IStyle>();
}
unsafe impl windows_core::Interface for Style {
    type Vtable = <IStyle as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStyle as windows_core::Interface>::IID;
}
impl core::ops::Deref for Style {
    type Target = IStyle;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Style {
    const NAME: &'static str = "Microsoft.UI.Xaml.Style";
}
unsafe impl Send for Style {}
unsafe impl Sync for Style {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapChainPanel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SwapChainPanel,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    SwapChainPanel,
    Grid,
    Panel,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl SwapChainPanel {
    pub fn new() -> windows_core::Result<SwapChainPanel> {
        Self::ISwapChainPanelFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<SwapChainPanel>
    where
        T: windows_core::Compose,
    {
        Self::ISwapChainPanelFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ISwapChainPanelFactory<R, F: FnOnce(&ISwapChainPanelFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SwapChainPanel, ISwapChainPanelFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SwapChainPanel {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISwapChainPanel>();
}
unsafe impl windows_core::Interface for SwapChainPanel {
    type Vtable = <ISwapChainPanel as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISwapChainPanel as windows_core::Interface>::IID;
}
impl core::ops::Deref for SwapChainPanel {
    type Target = ISwapChainPanel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SwapChainPanel {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.SwapChainPanel";
}
unsafe impl Send for SwapChainPanel {}
unsafe impl Sync for SwapChainPanel {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Symbol(pub i32);
impl Symbol {
    pub const Previous: Self = Self(57600i32);
    pub const Next: Self = Self(57601i32);
    pub const Play: Self = Self(57602i32);
    pub const Pause: Self = Self(57603i32);
    pub const Edit: Self = Self(57604i32);
    pub const Save: Self = Self(57605i32);
    pub const Clear: Self = Self(57606i32);
    pub const Delete: Self = Self(57607i32);
    pub const Remove: Self = Self(57608i32);
    pub const Add: Self = Self(57609i32);
    pub const Cancel: Self = Self(57610i32);
    pub const Accept: Self = Self(57611i32);
    pub const More: Self = Self(57612i32);
    pub const Redo: Self = Self(57613i32);
    pub const Undo: Self = Self(57614i32);
    pub const Home: Self = Self(57615i32);
    pub const Up: Self = Self(57616i32);
    pub const Forward: Self = Self(57617i32);
    pub const Back: Self = Self(57618i32);
    pub const Favorite: Self = Self(57619i32);
    pub const Camera: Self = Self(57620i32);
    pub const Setting: Self = Self(57621i32);
    pub const Video: Self = Self(57622i32);
    pub const Sync: Self = Self(57623i32);
    pub const Download: Self = Self(57624i32);
    pub const Mail: Self = Self(57625i32);
    pub const Find: Self = Self(57626i32);
    pub const Help: Self = Self(57627i32);
    pub const Upload: Self = Self(57628i32);
    pub const Emoji: Self = Self(57629i32);
    pub const TwoPage: Self = Self(57630i32);
    pub const LeaveChat: Self = Self(57631i32);
    pub const MailForward: Self = Self(57632i32);
    pub const Clock: Self = Self(57633i32);
    pub const Send: Self = Self(57634i32);
    pub const Crop: Self = Self(57635i32);
    pub const RotateCamera: Self = Self(57636i32);
    pub const People: Self = Self(57637i32);
    pub const OpenPane: Self = Self(57638i32);
    pub const ClosePane: Self = Self(57639i32);
    pub const World: Self = Self(57640i32);
    pub const Flag: Self = Self(57641i32);
    pub const PreviewLink: Self = Self(57642i32);
    pub const Globe: Self = Self(57643i32);
    pub const Trim: Self = Self(57644i32);
    pub const AttachCamera: Self = Self(57645i32);
    pub const ZoomIn: Self = Self(57646i32);
    pub const Bookmarks: Self = Self(57647i32);
    pub const Document: Self = Self(57648i32);
    pub const ProtectedDocument: Self = Self(57649i32);
    pub const Page: Self = Self(57650i32);
    pub const Bullets: Self = Self(57651i32);
    pub const Comment: Self = Self(57652i32);
    pub const MailFilled: Self = Self(57653i32);
    pub const ContactInfo: Self = Self(57654i32);
    pub const HangUp: Self = Self(57655i32);
    pub const ViewAll: Self = Self(57656i32);
    pub const MapPin: Self = Self(57657i32);
    pub const Phone: Self = Self(57658i32);
    pub const VideoChat: Self = Self(57659i32);
    pub const Switch: Self = Self(57660i32);
    pub const Contact: Self = Self(57661i32);
    pub const Rename: Self = Self(57662i32);
    pub const Pin: Self = Self(57665i32);
    pub const MusicInfo: Self = Self(57666i32);
    pub const Go: Self = Self(57667i32);
    pub const Keyboard: Self = Self(57668i32);
    pub const DockLeft: Self = Self(57669i32);
    pub const DockRight: Self = Self(57670i32);
    pub const DockBottom: Self = Self(57671i32);
    pub const Remote: Self = Self(57672i32);
    pub const Refresh: Self = Self(57673i32);
    pub const Rotate: Self = Self(57674i32);
    pub const Shuffle: Self = Self(57675i32);
    pub const List: Self = Self(57676i32);
    pub const Shop: Self = Self(57677i32);
    pub const SelectAll: Self = Self(57678i32);
    pub const Orientation: Self = Self(57679i32);
    pub const Import: Self = Self(57680i32);
    pub const ImportAll: Self = Self(57681i32);
    pub const BrowsePhotos: Self = Self(57685i32);
    pub const WebCam: Self = Self(57686i32);
    pub const Pictures: Self = Self(57688i32);
    pub const SaveLocal: Self = Self(57689i32);
    pub const Caption: Self = Self(57690i32);
    pub const Stop: Self = Self(57691i32);
    pub const ShowResults: Self = Self(57692i32);
    pub const Volume: Self = Self(57693i32);
    pub const Repair: Self = Self(57694i32);
    pub const Message: Self = Self(57695i32);
    pub const Page2: Self = Self(57696i32);
    pub const CalendarDay: Self = Self(57697i32);
    pub const CalendarWeek: Self = Self(57698i32);
    pub const Calendar: Self = Self(57699i32);
    pub const Character: Self = Self(57700i32);
    pub const MailReplyAll: Self = Self(57701i32);
    pub const Read: Self = Self(57702i32);
    pub const Link: Self = Self(57703i32);
    pub const Account: Self = Self(57704i32);
    pub const ShowBcc: Self = Self(57705i32);
    pub const HideBcc: Self = Self(57706i32);
    pub const Cut: Self = Self(57707i32);
    pub const Attach: Self = Self(57708i32);
    pub const Paste: Self = Self(57709i32);
    pub const Filter: Self = Self(57710i32);
    pub const Copy: Self = Self(57711i32);
    pub const Emoji2: Self = Self(57712i32);
    pub const Important: Self = Self(57713i32);
    pub const MailReply: Self = Self(57714i32);
    pub const SlideShow: Self = Self(57715i32);
    pub const Sort: Self = Self(57716i32);
    pub const Manage: Self = Self(57720i32);
    pub const AllApps: Self = Self(57721i32);
    pub const DisconnectDrive: Self = Self(57722i32);
    pub const MapDrive: Self = Self(57723i32);
    pub const NewWindow: Self = Self(57724i32);
    pub const OpenWith: Self = Self(57725i32);
    pub const ContactPresence: Self = Self(57729i32);
    pub const Priority: Self = Self(57730i32);
    pub const GoToToday: Self = Self(57732i32);
    pub const Font: Self = Self(57733i32);
    pub const FontColor: Self = Self(57734i32);
    pub const Contact2: Self = Self(57735i32);
    pub const Folder: Self = Self(57736i32);
    pub const Audio: Self = Self(57737i32);
    pub const Placeholder: Self = Self(57738i32);
    pub const View: Self = Self(57739i32);
    pub const SetLockScreen: Self = Self(57740i32);
    pub const SetTile: Self = Self(57741i32);
    pub const ClosedCaption: Self = Self(57744i32);
    pub const StopSlideShow: Self = Self(57745i32);
    pub const Permissions: Self = Self(57746i32);
    pub const Highlight: Self = Self(57747i32);
    pub const DisableUpdates: Self = Self(57748i32);
    pub const UnFavorite: Self = Self(57749i32);
    pub const UnPin: Self = Self(57750i32);
    pub const OpenLocal: Self = Self(57751i32);
    pub const Mute: Self = Self(57752i32);
    pub const Italic: Self = Self(57753i32);
    pub const Underline: Self = Self(57754i32);
    pub const Bold: Self = Self(57755i32);
    pub const MoveToFolder: Self = Self(57756i32);
    pub const LikeDislike: Self = Self(57757i32);
    pub const Dislike: Self = Self(57758i32);
    pub const Like: Self = Self(57759i32);
    pub const AlignRight: Self = Self(57760i32);
    pub const AlignCenter: Self = Self(57761i32);
    pub const AlignLeft: Self = Self(57762i32);
    pub const Zoom: Self = Self(57763i32);
    pub const ZoomOut: Self = Self(57764i32);
    pub const OpenFile: Self = Self(57765i32);
    pub const OtherUser: Self = Self(57766i32);
    pub const Admin: Self = Self(57767i32);
    pub const Street: Self = Self(57795i32);
    pub const Map: Self = Self(57796i32);
    pub const ClearSelection: Self = Self(57797i32);
    pub const FontDecrease: Self = Self(57798i32);
    pub const FontIncrease: Self = Self(57799i32);
    pub const FontSize: Self = Self(57800i32);
    pub const CellPhone: Self = Self(57801i32);
    pub const ReShare: Self = Self(57802i32);
    pub const Tag: Self = Self(57803i32);
    pub const RepeatOne: Self = Self(57804i32);
    pub const RepeatAll: Self = Self(57805i32);
    pub const OutlineStar: Self = Self(57806i32);
    pub const SolidStar: Self = Self(57807i32);
    pub const Calculator: Self = Self(57808i32);
    pub const Directions: Self = Self(57809i32);
    pub const Target: Self = Self(57810i32);
    pub const Library: Self = Self(57811i32);
    pub const PhoneBook: Self = Self(57812i32);
    pub const Memo: Self = Self(57813i32);
    pub const Microphone: Self = Self(57814i32);
    pub const PostUpdate: Self = Self(57815i32);
    pub const BackToWindow: Self = Self(57816i32);
    pub const FullScreen: Self = Self(57817i32);
    pub const NewFolder: Self = Self(57818i32);
    pub const CalendarReply: Self = Self(57819i32);
    pub const UnSyncFolder: Self = Self(57821i32);
    pub const ReportHacked: Self = Self(57822i32);
    pub const SyncFolder: Self = Self(57823i32);
    pub const BlockContact: Self = Self(57824i32);
    pub const SwitchApps: Self = Self(57825i32);
    pub const AddFriend: Self = Self(57826i32);
    pub const TouchPointer: Self = Self(57827i32);
    pub const GoToStart: Self = Self(57828i32);
    pub const ZeroBars: Self = Self(57829i32);
    pub const OneBar: Self = Self(57830i32);
    pub const TwoBars: Self = Self(57831i32);
    pub const ThreeBars: Self = Self(57832i32);
    pub const FourBars: Self = Self(57833i32);
    pub const Scan: Self = Self(58004i32);
    pub const Preview: Self = Self(58005i32);
    pub const GlobalNavigationButton: Self = Self(59136i32);
    pub const Share: Self = Self(59181i32);
    pub const Print: Self = Self(59209i32);
    pub const XboxOneConsole: Self = Self(59792i32);
}
impl windows_core::TypeKind for Symbol {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Symbol {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Controls.Symbol;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolIcon(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SymbolIcon,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    SymbolIcon,
    IconElement,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl SymbolIcon {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            SymbolIcon,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CreateInstanceWithSymbol(symbol: Symbol) -> windows_core::Result<SymbolIcon> {
        Self::ISymbolIconFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithSymbol)(
                windows_core::Interface::as_raw(this),
                symbol,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISymbolIconFactory<R, F: FnOnce(&ISymbolIconFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SymbolIcon, ISymbolIconFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SymbolIcon {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISymbolIcon>();
}
unsafe impl windows_core::Interface for SymbolIcon {
    type Vtable = <ISymbolIcon as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISymbolIcon as windows_core::Interface>::IID;
}
impl core::ops::Deref for SymbolIcon {
    type Target = ISymbolIcon;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SymbolIcon {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.SymbolIcon";
}
unsafe impl Send for SymbolIcon {}
unsafe impl Sync for SymbolIcon {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemBackdrop(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SystemBackdrop,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(SystemBackdrop, DependencyObject);
impl SystemBackdrop {}
impl windows_core::RuntimeType for SystemBackdrop {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISystemBackdrop>();
}
unsafe impl windows_core::Interface for SystemBackdrop {
    type Vtable = <ISystemBackdrop as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISystemBackdrop as windows_core::Interface>::IID;
}
impl core::ops::Deref for SystemBackdrop {
    type Target = ISystemBackdrop;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SystemBackdrop {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.SystemBackdrop";
}
unsafe impl Send for SystemBackdrop {}
unsafe impl Sync for SystemBackdrop {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TabView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    TabView,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl TabView {
    pub fn new() -> windows_core::Result<TabView> {
        Self::ITabViewFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<TabView>
    where
        T: windows_core::Compose,
    {
        Self::ITabViewFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ITabViewFactory<R, F: FnOnce(&ITabViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TabView, ITabViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TabView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITabView>();
}
unsafe impl windows_core::Interface for TabView {
    type Vtable = <ITabView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITabView as windows_core::Interface>::IID;
}
impl core::ops::Deref for TabView {
    type Target = ITabView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TabView {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TabView";
}
unsafe impl Send for TabView {}
unsafe impl Sync for TabView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabViewItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TabViewItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    TabViewItem,
    ListViewItem,
    SelectorItem,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl TabViewItem {
    pub fn new() -> windows_core::Result<TabViewItem> {
        Self::ITabViewItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<TabViewItem>
    where
        T: windows_core::Compose,
    {
        Self::ITabViewItemFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ITabViewItemFactory<R, F: FnOnce(&ITabViewItemFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TabViewItem, ITabViewItemFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TabViewItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITabViewItem>();
}
unsafe impl windows_core::Interface for TabViewItem {
    type Vtable = <ITabViewItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITabViewItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for TabViewItem {
    type Target = ITabViewItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TabViewItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TabViewItem";
}
unsafe impl Send for TabViewItem {}
unsafe impl Sync for TabViewItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabViewTabCloseRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TabViewTabCloseRequestedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl TabViewTabCloseRequestedEventArgs {}
impl windows_core::RuntimeType for TabViewTabCloseRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITabViewTabCloseRequestedEventArgs>();
}
unsafe impl windows_core::Interface for TabViewTabCloseRequestedEventArgs {
    type Vtable = <ITabViewTabCloseRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ITabViewTabCloseRequestedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for TabViewTabCloseRequestedEventArgs {
    type Target = ITabViewTabCloseRequestedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TabViewTabCloseRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TabViewTabCloseRequestedEventArgs";
}
unsafe impl Send for TabViewTabCloseRequestedEventArgs {}
unsafe impl Sync for TabViewTabCloseRequestedEventArgs {}
windows_core::imp::define_interface!(
    TappedEventHandler,
    TappedEventHandler_Vtbl,
    0xb60074f3_125b_534e_8f9c_9769bd3f0f64
);
impl windows_core::RuntimeType for TappedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct TappedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct TappedEventHandlerBox<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<TappedRoutedEventArgs>)
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<TappedRoutedEventArgs>)
        + 'static,
> TappedEventHandlerBox<F>
{
    const VTABLE: TappedEventHandler_Vtbl = TappedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<TappedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<TappedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<TappedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<TappedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TappedRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TappedRoutedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(TappedRoutedEventArgs, RoutedEventArgs);
impl TappedRoutedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            TappedRoutedEventArgs,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TappedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITappedRoutedEventArgs>();
}
unsafe impl windows_core::Interface for TappedRoutedEventArgs {
    type Vtable = <ITappedRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITappedRoutedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for TappedRoutedEventArgs {
    type Target = ITappedRoutedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.TappedRoutedEventArgs";
}
unsafe impl Send for TappedRoutedEventArgs {}
unsafe impl Sync for TappedRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeachingTip(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TeachingTip,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    TeachingTip,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl TeachingTip {
    pub fn new() -> windows_core::Result<TeachingTip> {
        Self::ITeachingTipFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<TeachingTip>
    where
        T: windows_core::Compose,
    {
        Self::ITeachingTipFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ITeachingTipFactory<R, F: FnOnce(&ITeachingTipFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TeachingTip, ITeachingTipFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TeachingTip {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITeachingTip>();
}
unsafe impl windows_core::Interface for TeachingTip {
    type Vtable = <ITeachingTip as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITeachingTip as windows_core::Interface>::IID;
}
impl core::ops::Deref for TeachingTip {
    type Target = ITeachingTip;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TeachingTip {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TeachingTip";
}
unsafe impl Send for TeachingTip {}
unsafe impl Sync for TeachingTip {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeachingTipClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TeachingTipClosedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl TeachingTipClosedEventArgs {}
impl windows_core::RuntimeType for TeachingTipClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITeachingTipClosedEventArgs>();
}
unsafe impl windows_core::Interface for TeachingTipClosedEventArgs {
    type Vtable = <ITeachingTipClosedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITeachingTipClosedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for TeachingTipClosedEventArgs {
    type Target = ITeachingTipClosedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TeachingTipClosedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TeachingTipClosedEventArgs";
}
unsafe impl Send for TeachingTipClosedEventArgs {}
unsafe impl Sync for TeachingTipClosedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TeachingTipPlacementMode(pub i32);
impl TeachingTipPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
    pub const TopRight: Self = Self(5i32);
    pub const TopLeft: Self = Self(6i32);
    pub const BottomRight: Self = Self(7i32);
    pub const BottomLeft: Self = Self(8i32);
    pub const LeftTop: Self = Self(9i32);
    pub const LeftBottom: Self = Self(10i32);
    pub const RightTop: Self = Self(11i32);
    pub const RightBottom: Self = Self(12i32);
    pub const Center: Self = Self(13i32);
}
impl windows_core::TypeKind for TeachingTipPlacementMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TeachingTipPlacementMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.TeachingTipPlacementMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextBlock(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TextBlock,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(TextBlock, FrameworkElement, UIElement, DependencyObject);
impl TextBlock {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            TextBlock,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn get_FontSizeProperty() -> windows_core::Result<DependencyProperty> {
        Self::ITextBlockStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_FontSizeProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_FontFamilyProperty() -> windows_core::Result<DependencyProperty> {
        Self::ITextBlockStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_FontFamilyProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_FontWeightProperty() -> windows_core::Result<DependencyProperty> {
        Self::ITextBlockStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_FontWeightProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_ForegroundProperty() -> windows_core::Result<DependencyProperty> {
        Self::ITextBlockStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_ForegroundProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_TextWrappingProperty() -> windows_core::Result<DependencyProperty> {
        Self::ITextBlockStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_TextWrappingProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_TextProperty() -> windows_core::Result<DependencyProperty> {
        Self::ITextBlockStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_TextProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn get_IsTextSelectionEnabledProperty() -> windows_core::Result<DependencyProperty> {
        Self::ITextBlockStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).get_IsTextSelectionEnabledProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITextBlockStatics<R, F: FnOnce(&ITextBlockStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TextBlock, ITextBlockStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TextBlock {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITextBlock>();
}
unsafe impl windows_core::Interface for TextBlock {
    type Vtable = <ITextBlock as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextBlock as windows_core::Interface>::IID;
}
impl core::ops::Deref for TextBlock {
    type Target = ITextBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TextBlock {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TextBlock";
}
unsafe impl Send for TextBlock {}
unsafe impl Sync for TextBlock {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TextBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    TextBox,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl TextBox {
    pub fn new() -> windows_core::Result<TextBox> {
        Self::ITextBoxFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<TextBox>
    where
        T: windows_core::Compose,
    {
        Self::ITextBoxFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ITextBoxFactory<R, F: FnOnce(&ITextBoxFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TextBox, ITextBoxFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TextBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITextBox>();
}
unsafe impl windows_core::Interface for TextBox {
    type Vtable = <ITextBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for TextBox {
    type Target = ITextBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TextBox {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TextBox";
}
unsafe impl Send for TextBox {}
unsafe impl Sync for TextBox {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TextChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(TextChangedEventArgs, RoutedEventArgs);
impl TextChangedEventArgs {}
impl windows_core::RuntimeType for TextChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITextChangedEventArgs>();
}
unsafe impl windows_core::Interface for TextChangedEventArgs {
    type Vtable = <ITextChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for TextChangedEventArgs {
    type Target = ITextChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TextChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TextChangedEventArgs";
}
unsafe impl Send for TextChangedEventArgs {}
unsafe impl Sync for TextChangedEventArgs {}
windows_core::imp::define_interface!(
    TextChangedEventHandler,
    TextChangedEventHandler_Vtbl,
    0x5d8ddcff_45d8_5e7c_9b8b_c41d2893c6a1
);
impl windows_core::RuntimeType for TextChangedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct TextChangedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct TextChangedEventHandlerBox<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<TextChangedEventArgs>)
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<TextChangedEventArgs>)
        + 'static,
> TextChangedEventHandlerBox<F>
{
    const VTABLE: TextChangedEventHandler_Vtbl = TextChangedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<TextChangedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<TextChangedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<TextChangedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<TextChangedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TextElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(TextElement, DependencyObject);
impl TextElement {}
impl windows_core::RuntimeType for TextElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITextElement>();
}
unsafe impl windows_core::Interface for TextElement {
    type Vtable = <ITextElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextElement as windows_core::Interface>::IID;
}
impl core::ops::Deref for TextElement {
    type Target = ITextElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TextElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextElement";
}
unsafe impl Send for TextElement {}
unsafe impl Sync for TextElement {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextGetOptions(pub u32);
impl TextGetOptions {
    pub const None: Self = Self(0u32);
    pub const AdjustCrlf: Self = Self(1u32);
    pub const UseCrlf: Self = Self(2u32);
    pub const UseObjectText: Self = Self(4u32);
    pub const AllowFinalEop: Self = Self(8u32);
    pub const NoHidden: Self = Self(32u32);
    pub const IncludeNumbering: Self = Self(64u32);
    pub const FormatRtf: Self = Self(8192u32);
    pub const UseLf: Self = Self(16777216u32);
}
impl windows_core::TypeKind for TextGetOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextGetOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TextGetOptions;u4)");
}
impl TextGetOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TextGetOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TextGetOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TextGetOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for TextGetOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for TextGetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextSetOptions(pub u32);
impl TextSetOptions {
    pub const None: Self = Self(0u32);
    pub const UnicodeBidi: Self = Self(1u32);
    pub const Unlink: Self = Self(8u32);
    pub const Unhide: Self = Self(16u32);
    pub const CheckTextLimit: Self = Self(32u32);
    pub const FormatRtf: Self = Self(8192u32);
    pub const ApplyRtfDocumentDefaults: Self = Self(16384u32);
}
impl windows_core::TypeKind for TextSetOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextSetOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TextSetOptions;u4)");
}
impl TextSetOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TextSetOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TextSetOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TextSetOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for TextSetOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for TextSetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextWrapping(pub i32);
impl TextWrapping {
    pub const NoWrap: Self = Self(1i32);
    pub const Wrap: Self = Self(2i32);
    pub const WrapWholeWords: Self = Self(3i32);
}
impl windows_core::TypeKind for TextWrapping {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextWrapping {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.TextWrapping;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Thickness {
    pub Left: f64,
    pub Top: f64,
    pub Right: f64,
    pub Bottom: f64,
}
impl windows_core::TypeKind for Thickness {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Thickness {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Thickness;f8;f8;f8;f8)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimePicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TimePicker,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    TimePicker,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl TimePicker {
    pub fn new() -> windows_core::Result<TimePicker> {
        Self::ITimePickerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<TimePicker>
    where
        T: windows_core::Compose,
    {
        Self::ITimePickerFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ITimePickerFactory<R, F: FnOnce(&ITimePickerFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TimePicker, ITimePickerFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TimePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITimePicker>();
}
unsafe impl windows_core::Interface for TimePicker {
    type Vtable = <ITimePicker as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITimePicker as windows_core::Interface>::IID;
}
impl core::ops::Deref for TimePicker {
    type Target = ITimePicker;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TimePicker {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TimePicker";
}
unsafe impl Send for TimePicker {}
unsafe impl Sync for TimePicker {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimePickerSelectedValueChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TimePickerSelectedValueChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl TimePickerSelectedValueChangedEventArgs {}
impl windows_core::RuntimeType for TimePickerSelectedValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        ITimePickerSelectedValueChangedEventArgs,
    >();
}
unsafe impl windows_core::Interface for TimePickerSelectedValueChangedEventArgs {
    type Vtable = <ITimePickerSelectedValueChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ITimePickerSelectedValueChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for TimePickerSelectedValueChangedEventArgs {
    type Target = ITimePickerSelectedValueChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TimePickerSelectedValueChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TimePickerSelectedValueChangedEventArgs";
}
unsafe impl Send for TimePickerSelectedValueChangedEventArgs {}
unsafe impl Sync for TimePickerSelectedValueChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TitleBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TitleBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    TitleBar,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl TitleBar {
    pub fn new() -> windows_core::Result<TitleBar> {
        Self::ITitleBarFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<TitleBar>
    where
        T: windows_core::Compose,
    {
        Self::ITitleBarFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ITitleBarFactory<R, F: FnOnce(&ITitleBarFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TitleBar, ITitleBarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITitleBar>();
}
unsafe impl windows_core::Interface for TitleBar {
    type Vtable = <ITitleBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITitleBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for TitleBar {
    type Target = ITitleBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TitleBar {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TitleBar";
}
unsafe impl Send for TitleBar {}
unsafe impl Sync for TitleBar {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TitleBarHeightOption(pub i32);
impl TitleBarHeightOption {
    pub const Standard: Self = Self(0i32);
    pub const Tall: Self = Self(1i32);
}
impl windows_core::TypeKind for TitleBarHeightOption {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TitleBarHeightOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.TitleBarHeightOption;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TitleBarTheme(pub i32);
impl TitleBarTheme {
    pub const UseDefaultAppMode: Self = Self(1i32);
    pub const Light: Self = Self(2i32);
    pub const Dark: Self = Self(3i32);
}
impl windows_core::TypeKind for TitleBarTheme {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TitleBarTheme {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.TitleBarTheme;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToggleButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ToggleButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ToggleButton,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ToggleButton {
    pub fn new() -> windows_core::Result<ToggleButton> {
        Self::IToggleButtonFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<ToggleButton>
    where
        T: windows_core::Compose,
    {
        Self::IToggleButtonFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IToggleButtonFactory<R, F: FnOnce(&IToggleButtonFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToggleButton, IToggleButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToggleButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IToggleButton>();
}
unsafe impl windows_core::Interface for ToggleButton {
    type Vtable = <IToggleButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IToggleButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for ToggleButton {
    type Target = IToggleButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ToggleButton {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ToggleButton";
}
unsafe impl Send for ToggleButton {}
unsafe impl Sync for ToggleButton {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToggleSwitch(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ToggleSwitch,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ToggleSwitch,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ToggleSwitch {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            ToggleSwitch,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToggleSwitch {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IToggleSwitch>();
}
unsafe impl windows_core::Interface for ToggleSwitch {
    type Vtable = <IToggleSwitch as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IToggleSwitch as windows_core::Interface>::IID;
}
impl core::ops::Deref for ToggleSwitch {
    type Target = IToggleSwitch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ToggleSwitch {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ToggleSwitch";
}
unsafe impl Send for ToggleSwitch {}
unsafe impl Sync for ToggleSwitch {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolTip(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ToolTip,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ToolTip,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ToolTip {
    pub fn new() -> windows_core::Result<ToolTip> {
        Self::IToolTipFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<ToolTip>
    where
        T: windows_core::Compose,
    {
        Self::IToolTipFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IToolTipFactory<R, F: FnOnce(&IToolTipFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToolTip, IToolTipFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToolTip {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IToolTip>();
}
unsafe impl windows_core::Interface for ToolTip {
    type Vtable = <IToolTip as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IToolTip as windows_core::Interface>::IID;
}
impl core::ops::Deref for ToolTip {
    type Target = IToolTip;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ToolTip {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ToolTip";
}
unsafe impl Send for ToolTip {}
unsafe impl Sync for ToolTip {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolTipService(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ToolTipService,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ToolTipService {
    pub fn GetPlacement<P0>(element: P0) -> windows_core::Result<PlacementMode>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IToolTipServiceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPlacement)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn SetPlacement<P0>(element: P0, value: PlacementMode) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IToolTipServiceStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetPlacement)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn GetToolTip<P0>(element: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IToolTipServiceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetToolTip)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SetToolTip<P0, P1>(element: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        Self::IToolTipServiceStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetToolTip)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value.param().abi(),
            )
            .ok()
        })
    }
    fn IToolTipServiceStatics<R, F: FnOnce(&IToolTipServiceStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToolTipService, IToolTipServiceStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToolTipService {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IToolTipService>();
}
unsafe impl windows_core::Interface for ToolTipService {
    type Vtable = <IToolTipService as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IToolTipService as windows_core::Interface>::IID;
}
impl core::ops::Deref for ToolTipService {
    type Target = IToolTipService;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ToolTipService {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ToolTipService";
}
unsafe impl Send for ToolTipService {}
unsafe impl Sync for ToolTipService {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TreeView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    TreeView,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl TreeView {
    pub fn new() -> windows_core::Result<TreeView> {
        Self::ITreeViewFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<TreeView>
    where
        T: windows_core::Compose,
    {
        Self::ITreeViewFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ITreeViewFactory<R, F: FnOnce(&ITreeViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TreeView, ITreeViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TreeView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITreeView>();
}
unsafe impl windows_core::Interface for TreeView {
    type Vtable = <ITreeView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITreeView as windows_core::Interface>::IID;
}
impl core::ops::Deref for TreeView {
    type Target = ITreeView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TreeView {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TreeView";
}
unsafe impl Send for TreeView {}
unsafe impl Sync for TreeView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeViewItemInvokedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TreeViewItemInvokedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl TreeViewItemInvokedEventArgs {}
impl windows_core::RuntimeType for TreeViewItemInvokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITreeViewItemInvokedEventArgs>();
}
unsafe impl windows_core::Interface for TreeViewItemInvokedEventArgs {
    type Vtable = <ITreeViewItemInvokedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITreeViewItemInvokedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for TreeViewItemInvokedEventArgs {
    type Target = ITreeViewItemInvokedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TreeViewItemInvokedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TreeViewItemInvokedEventArgs";
}
unsafe impl Send for TreeViewItemInvokedEventArgs {}
unsafe impl Sync for TreeViewItemInvokedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeViewNode(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TreeViewNode,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(TreeViewNode, DependencyObject);
impl TreeViewNode {
    pub fn new() -> windows_core::Result<TreeViewNode> {
        Self::ITreeViewNodeFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<TreeViewNode>
    where
        T: windows_core::Compose,
    {
        Self::ITreeViewNodeFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn ITreeViewNodeFactory<R, F: FnOnce(&ITreeViewNodeFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TreeViewNode, ITreeViewNodeFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TreeViewNode {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITreeViewNode>();
}
unsafe impl windows_core::Interface for TreeViewNode {
    type Vtable = <ITreeViewNode as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITreeViewNode as windows_core::Interface>::IID;
}
impl core::ops::Deref for TreeViewNode {
    type Target = ITreeViewNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TreeViewNode {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TreeViewNode";
}
unsafe impl Send for TreeViewNode {}
unsafe impl Sync for TreeViewNode {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TreeViewSelectionMode(pub i32);
impl TreeViewSelectionMode {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Multiple: Self = Self(2i32);
}
impl windows_core::TypeKind for TreeViewSelectionMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TreeViewSelectionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.TreeViewSelectionMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TriggerBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TriggerBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(TriggerBase, DependencyObject);
impl TriggerBase {}
impl windows_core::RuntimeType for TriggerBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITriggerBase>();
}
unsafe impl windows_core::Interface for TriggerBase {
    type Vtable = <ITriggerBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITriggerBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for TriggerBase {
    type Target = ITriggerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TriggerBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.TriggerBase";
}
unsafe impl Send for TriggerBase {}
unsafe impl Sync for TriggerBase {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TypeKind(pub i32);
impl TypeKind {
    pub const Primitive: Self = Self(0i32);
    pub const Metadata: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
}
impl windows_core::TypeKind for TypeKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TypeKind {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Interop.TypeKind;i4)");
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TypeName {
    pub Name: windows_core::HSTRING,
    pub Kind: TypeKind,
}
impl windows_core::TypeKind for TypeName {
    type TypeKind = windows_core::CloneType;
}
impl windows_core::RuntimeType for TypeName {
    const SIGNATURE : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"struct(Windows.UI.Xaml.Interop.TypeName;string;enum(Windows.UI.Xaml.Interop.TypeKind;i4))") ;
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedEventHandler<TSender, TResult>(
    windows_core::IUnknown,
    core::marker::PhantomData<TSender>,
    core::marker::PhantomData<TResult>,
)
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static;
unsafe impl<
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static,
> windows_core::Interface for TypedEventHandler<TSender, TResult>
{
    type Vtable = TypedEventHandler_Vtbl<TSender, TResult>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static>
    windows_core::RuntimeType for TypedEventHandler<TSender, TResult>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({9de1c534-6ae1-11e0-84e1-18a905bcc53f}")
        .push_slice(b";")
        .push_other(TSender::SIGNATURE)
        .push_slice(b";")
        .push_other(TResult::SIGNATURE)
        .push_slice(b")");
}
#[repr(C)]
#[doc(hidden)]
pub struct TypedEventHandler_Vtbl<TSender, TResult>
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: windows_core::AbiType<TSender>,
        args: windows_core::AbiType<TResult>,
    ) -> windows_core::HRESULT,
    TSender: core::marker::PhantomData<TSender>,
    TResult: core::marker::PhantomData<TResult>,
}
struct TypedEventHandlerBox<
    TSender,
    TResult,
    F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) + 'static,
>(core::marker::PhantomData<(TSender, TResult, fn() -> F)>)
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static;
impl<
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static,
    F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) + 'static,
> TypedEventHandlerBox<TSender, TResult, F>
{
    const VTABLE : TypedEventHandler_Vtbl < TSender , TResult , > = TypedEventHandler_Vtbl::< TSender , TResult , > { base__ : windows_core::IUnknown_Vtbl { QueryInterface : windows_core::imp::DelegateBox::< TypedEventHandler < TSender , TResult > , F >::QueryInterface , AddRef : windows_core::imp::DelegateBox::< TypedEventHandler < TSender , TResult > , F >::AddRef , Release : windows_core::imp::DelegateBox::< TypedEventHandler < TSender , TResult > , F >::Release , } , Invoke : Self::Invoke , TSender : core::marker::PhantomData::< TSender > , TResult : core::marker::PhantomData::< TResult > } ;
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: windows_core::AbiType<TSender>,
        args: windows_core::AbiType<TResult>,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<TypedEventHandler<TSender, TResult>, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&args),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UIElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    UIElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(UIElement, DependencyObject);
impl UIElement {}
impl windows_core::RuntimeType for UIElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IUIElement>();
}
unsafe impl windows_core::Interface for UIElement {
    type Vtable = <IUIElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUIElement as windows_core::Interface>::IID;
}
impl core::ops::Deref for UIElement {
    type Target = IUIElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for UIElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.UIElement";
}
unsafe impl Send for UIElement {}
unsafe impl Sync for UIElement {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UIElementCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    UIElementCollection,
    windows_core::IUnknown,
    windows_core::IInspectable,
    windows_collections::IVector<UIElement>
);
impl UIElementCollection {}
impl windows_core::RuntimeType for UIElementCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVector<UIElement>>(
        );
}
unsafe impl windows_core::Interface for UIElementCollection {
    type Vtable = <windows_collections::IVector<UIElement> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <windows_collections::IVector<UIElement> as windows_core::Interface>::IID;
}
impl core::ops::Deref for UIElementCollection {
    type Target = windows_collections::IVector<UIElement>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for UIElementCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.UIElementCollection";
}
unsafe impl Send for UIElementCollection {}
unsafe impl Sync for UIElementCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uri(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Uri, windows_core::IUnknown, windows_core::IInspectable);
impl Uri {
    pub fn CreateUri(uri: &str) -> windows_core::Result<Uri> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUri)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&windows_core::HSTRING::from(uri)),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IUriRuntimeClassFactory<
        R,
        F: FnOnce(&IUriRuntimeClassFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Uri, IUriRuntimeClassFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Uri {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IUriRuntimeClass>();
}
unsafe impl windows_core::Interface for Uri {
    type Vtable = <IUriRuntimeClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUriRuntimeClass as windows_core::Interface>::IID;
}
impl core::ops::Deref for Uri {
    type Target = IUriRuntimeClass;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Uri {
    const NAME: &'static str = "Windows.Foundation.Uri";
}
unsafe impl Send for Uri {}
unsafe impl Sync for Uri {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vector3KeyFrameAnimation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Vector3KeyFrameAnimation,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    Vector3KeyFrameAnimation,
    ICompositionAnimationBase,
    KeyFrameAnimation,
    CompositionAnimation,
    CompositionObject
);
impl Vector3KeyFrameAnimation {}
impl windows_core::RuntimeType for Vector3KeyFrameAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IVector3KeyFrameAnimation>();
}
unsafe impl windows_core::Interface for Vector3KeyFrameAnimation {
    type Vtable = <IVector3KeyFrameAnimation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVector3KeyFrameAnimation as windows_core::Interface>::IID;
}
impl core::ops::Deref for Vector3KeyFrameAnimation {
    type Target = IVector3KeyFrameAnimation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Vector3KeyFrameAnimation {
    const NAME: &'static str = "Microsoft.UI.Composition.Vector3KeyFrameAnimation";
}
unsafe impl Send for Vector3KeyFrameAnimation {}
unsafe impl Sync for Vector3KeyFrameAnimation {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VerticalAlignment(pub i32);
impl VerticalAlignment {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Stretch: Self = Self(3i32);
}
impl windows_core::TypeKind for VerticalAlignment {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for VerticalAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.VerticalAlignment;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Viewbox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Viewbox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(Viewbox, FrameworkElement, UIElement, DependencyObject);
impl Viewbox {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            Viewbox,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Viewbox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IViewbox>();
}
unsafe impl windows_core::Interface for Viewbox {
    type Vtable = <IViewbox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IViewbox as windows_core::Interface>::IID;
}
impl core::ops::Deref for Viewbox {
    type Target = IViewbox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Viewbox {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Viewbox";
}
unsafe impl Send for Viewbox {}
unsafe impl Sync for Viewbox {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VirtualKey(pub i32);
impl VirtualKey {
    pub const None: Self = Self(0i32);
    pub const LeftButton: Self = Self(1i32);
    pub const RightButton: Self = Self(2i32);
    pub const Cancel: Self = Self(3i32);
    pub const MiddleButton: Self = Self(4i32);
    pub const XButton1: Self = Self(5i32);
    pub const XButton2: Self = Self(6i32);
    pub const Back: Self = Self(8i32);
    pub const Tab: Self = Self(9i32);
    pub const Clear: Self = Self(12i32);
    pub const Enter: Self = Self(13i32);
    pub const Shift: Self = Self(16i32);
    pub const Control: Self = Self(17i32);
    pub const Menu: Self = Self(18i32);
    pub const Pause: Self = Self(19i32);
    pub const CapitalLock: Self = Self(20i32);
    pub const Kana: Self = Self(21i32);
    pub const Hangul: Self = Self(21i32);
    pub const ImeOn: Self = Self(22i32);
    pub const Junja: Self = Self(23i32);
    pub const Final: Self = Self(24i32);
    pub const Hanja: Self = Self(25i32);
    pub const Kanji: Self = Self(25i32);
    pub const ImeOff: Self = Self(26i32);
    pub const Escape: Self = Self(27i32);
    pub const Convert: Self = Self(28i32);
    pub const NonConvert: Self = Self(29i32);
    pub const Accept: Self = Self(30i32);
    pub const ModeChange: Self = Self(31i32);
    pub const Space: Self = Self(32i32);
    pub const PageUp: Self = Self(33i32);
    pub const PageDown: Self = Self(34i32);
    pub const End: Self = Self(35i32);
    pub const Home: Self = Self(36i32);
    pub const Left: Self = Self(37i32);
    pub const Up: Self = Self(38i32);
    pub const Right: Self = Self(39i32);
    pub const Down: Self = Self(40i32);
    pub const Select: Self = Self(41i32);
    pub const Print: Self = Self(42i32);
    pub const Execute: Self = Self(43i32);
    pub const Snapshot: Self = Self(44i32);
    pub const Insert: Self = Self(45i32);
    pub const Delete: Self = Self(46i32);
    pub const Help: Self = Self(47i32);
    pub const Number0: Self = Self(48i32);
    pub const Number1: Self = Self(49i32);
    pub const Number2: Self = Self(50i32);
    pub const Number3: Self = Self(51i32);
    pub const Number4: Self = Self(52i32);
    pub const Number5: Self = Self(53i32);
    pub const Number6: Self = Self(54i32);
    pub const Number7: Self = Self(55i32);
    pub const Number8: Self = Self(56i32);
    pub const Number9: Self = Self(57i32);
    pub const A: Self = Self(65i32);
    pub const B: Self = Self(66i32);
    pub const C: Self = Self(67i32);
    pub const D: Self = Self(68i32);
    pub const E: Self = Self(69i32);
    pub const F: Self = Self(70i32);
    pub const G: Self = Self(71i32);
    pub const H: Self = Self(72i32);
    pub const I: Self = Self(73i32);
    pub const J: Self = Self(74i32);
    pub const K: Self = Self(75i32);
    pub const L: Self = Self(76i32);
    pub const M: Self = Self(77i32);
    pub const N: Self = Self(78i32);
    pub const O: Self = Self(79i32);
    pub const P: Self = Self(80i32);
    pub const Q: Self = Self(81i32);
    pub const R: Self = Self(82i32);
    pub const S: Self = Self(83i32);
    pub const T: Self = Self(84i32);
    pub const U: Self = Self(85i32);
    pub const V: Self = Self(86i32);
    pub const W: Self = Self(87i32);
    pub const X: Self = Self(88i32);
    pub const Y: Self = Self(89i32);
    pub const Z: Self = Self(90i32);
    pub const LeftWindows: Self = Self(91i32);
    pub const RightWindows: Self = Self(92i32);
    pub const Application: Self = Self(93i32);
    pub const Sleep: Self = Self(95i32);
    pub const NumberPad0: Self = Self(96i32);
    pub const NumberPad1: Self = Self(97i32);
    pub const NumberPad2: Self = Self(98i32);
    pub const NumberPad3: Self = Self(99i32);
    pub const NumberPad4: Self = Self(100i32);
    pub const NumberPad5: Self = Self(101i32);
    pub const NumberPad6: Self = Self(102i32);
    pub const NumberPad7: Self = Self(103i32);
    pub const NumberPad8: Self = Self(104i32);
    pub const NumberPad9: Self = Self(105i32);
    pub const Multiply: Self = Self(106i32);
    pub const Add: Self = Self(107i32);
    pub const Separator: Self = Self(108i32);
    pub const Subtract: Self = Self(109i32);
    pub const Decimal: Self = Self(110i32);
    pub const Divide: Self = Self(111i32);
    pub const F1: Self = Self(112i32);
    pub const F2: Self = Self(113i32);
    pub const F3: Self = Self(114i32);
    pub const F4: Self = Self(115i32);
    pub const F5: Self = Self(116i32);
    pub const F6: Self = Self(117i32);
    pub const F7: Self = Self(118i32);
    pub const F8: Self = Self(119i32);
    pub const F9: Self = Self(120i32);
    pub const F10: Self = Self(121i32);
    pub const F11: Self = Self(122i32);
    pub const F12: Self = Self(123i32);
    pub const F13: Self = Self(124i32);
    pub const F14: Self = Self(125i32);
    pub const F15: Self = Self(126i32);
    pub const F16: Self = Self(127i32);
    pub const F17: Self = Self(128i32);
    pub const F18: Self = Self(129i32);
    pub const F19: Self = Self(130i32);
    pub const F20: Self = Self(131i32);
    pub const F21: Self = Self(132i32);
    pub const F22: Self = Self(133i32);
    pub const F23: Self = Self(134i32);
    pub const F24: Self = Self(135i32);
    pub const NavigationView: Self = Self(136i32);
    pub const NavigationMenu: Self = Self(137i32);
    pub const NavigationUp: Self = Self(138i32);
    pub const NavigationDown: Self = Self(139i32);
    pub const NavigationLeft: Self = Self(140i32);
    pub const NavigationRight: Self = Self(141i32);
    pub const NavigationAccept: Self = Self(142i32);
    pub const NavigationCancel: Self = Self(143i32);
    pub const NumberKeyLock: Self = Self(144i32);
    pub const Scroll: Self = Self(145i32);
    pub const LeftShift: Self = Self(160i32);
    pub const RightShift: Self = Self(161i32);
    pub const LeftControl: Self = Self(162i32);
    pub const RightControl: Self = Self(163i32);
    pub const LeftMenu: Self = Self(164i32);
    pub const RightMenu: Self = Self(165i32);
    pub const GoBack: Self = Self(166i32);
    pub const GoForward: Self = Self(167i32);
    pub const Refresh: Self = Self(168i32);
    pub const Stop: Self = Self(169i32);
    pub const Search: Self = Self(170i32);
    pub const Favorites: Self = Self(171i32);
    pub const GoHome: Self = Self(172i32);
    pub const GamepadA: Self = Self(195i32);
    pub const GamepadB: Self = Self(196i32);
    pub const GamepadX: Self = Self(197i32);
    pub const GamepadY: Self = Self(198i32);
    pub const GamepadRightShoulder: Self = Self(199i32);
    pub const GamepadLeftShoulder: Self = Self(200i32);
    pub const GamepadLeftTrigger: Self = Self(201i32);
    pub const GamepadRightTrigger: Self = Self(202i32);
    pub const GamepadDPadUp: Self = Self(203i32);
    pub const GamepadDPadDown: Self = Self(204i32);
    pub const GamepadDPadLeft: Self = Self(205i32);
    pub const GamepadDPadRight: Self = Self(206i32);
    pub const GamepadMenu: Self = Self(207i32);
    pub const GamepadView: Self = Self(208i32);
    pub const GamepadLeftThumbstickButton: Self = Self(209i32);
    pub const GamepadRightThumbstickButton: Self = Self(210i32);
    pub const GamepadLeftThumbstickUp: Self = Self(211i32);
    pub const GamepadLeftThumbstickDown: Self = Self(212i32);
    pub const GamepadLeftThumbstickRight: Self = Self(213i32);
    pub const GamepadLeftThumbstickLeft: Self = Self(214i32);
    pub const GamepadRightThumbstickUp: Self = Self(215i32);
    pub const GamepadRightThumbstickDown: Self = Self(216i32);
    pub const GamepadRightThumbstickRight: Self = Self(217i32);
    pub const GamepadRightThumbstickLeft: Self = Self(218i32);
}
impl windows_core::TypeKind for VirtualKey {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for VirtualKey {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKey;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VirtualKeyModifiers(pub u32);
impl VirtualKeyModifiers {
    pub const None: Self = Self(0u32);
    pub const Control: Self = Self(1u32);
    pub const Menu: Self = Self(2u32);
    pub const Shift: Self = Self(4u32);
    pub const Windows: Self = Self(8u32);
}
impl windows_core::TypeKind for VirtualKeyModifiers {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for VirtualKeyModifiers {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKeyModifiers;u4)");
}
impl VirtualKeyModifiers {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VirtualKeyModifiers {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VirtualKeyModifiers {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VirtualKeyModifiers {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for VirtualKeyModifiers {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for VirtualKeyModifiers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Visual(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Visual, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Visual, CompositionObject);
impl Visual {}
impl windows_core::RuntimeType for Visual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IVisual>();
}
unsafe impl windows_core::Interface for Visual {
    type Vtable = <IVisual as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVisual as windows_core::Interface>::IID;
}
impl core::ops::Deref for Visual {
    type Target = IVisual;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Visual {
    const NAME: &'static str = "Microsoft.UI.Composition.Visual";
}
unsafe impl Send for Visual {}
unsafe impl Sync for Visual {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VisualTreeHelper(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    VisualTreeHelper,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl VisualTreeHelper {
    pub fn GetChild<P0>(reference: P0, childindex: i32) -> windows_core::Result<DependencyObject>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChild)(
                windows_core::Interface::as_raw(this),
                reference.param().abi(),
                childindex,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetChildrenCount<P0>(reference: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenCount)(
                windows_core::Interface::as_raw(this),
                reference.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    fn IVisualTreeHelperStatics<
        R,
        F: FnOnce(&IVisualTreeHelperStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VisualTreeHelper, IVisualTreeHelperStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VisualTreeHelper {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IVisualTreeHelper>();
}
unsafe impl windows_core::Interface for VisualTreeHelper {
    type Vtable = <IVisualTreeHelper as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVisualTreeHelper as windows_core::Interface>::IID;
}
impl core::ops::Deref for VisualTreeHelper {
    type Target = IVisualTreeHelper;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for VisualTreeHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.VisualTreeHelper";
}
unsafe impl Send for VisualTreeHelper {}
unsafe impl Sync for VisualTreeHelper {}
pub const WINDOWSAPPSDK_RELEASE_MAJORMINOR: i32 = 131072i32;
pub const WINDOWSAPPSDK_RELEASE_VERSION_TAG_W: windows_core::PCWSTR = windows_core::w!("");
pub const WINDOWSAPPSDK_RUNTIME_VERSION_UINT64: u64 = 562949953486848u64;
pub const WM_MOUSEMOVE: u32 = 512u32;
pub const WM_SETCURSOR: u32 = 32u32;
pub type WPARAM = usize;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Window(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Window, windows_core::IUnknown, windows_core::IInspectable);
impl Window {
    pub fn new() -> windows_core::Result<Window> {
        Self::IWindowFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Window>
    where
        T: windows_core::Compose,
    {
        Self::IWindowFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IWindowFactory<R, F: FnOnce(&IWindowFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Window, IWindowFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Window {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWindow>();
}
unsafe impl windows_core::Interface for Window {
    type Vtable = <IWindow as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindow as windows_core::Interface>::IID;
}
impl core::ops::Deref for Window {
    type Target = IWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Window {
    const NAME: &'static str = "Microsoft.UI.Xaml.Window";
}
unsafe impl Send for Window {}
unsafe impl Sync for Window {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WindowEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl WindowEventArgs {}
impl windows_core::RuntimeType for WindowEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWindowEventArgs>();
}
unsafe impl windows_core::Interface for WindowEventArgs {
    type Vtable = <IWindowEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for WindowEventArgs {
    type Target = IWindowEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for WindowEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.WindowEventArgs";
}
unsafe impl Send for WindowEventArgs {}
unsafe impl Sync for WindowEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlControlsResources(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlControlsResources,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(XamlControlsResources, ResourceDictionary, DependencyObject);
impl XamlControlsResources {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            XamlControlsResources,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XamlControlsResources {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IXamlControlsResources>();
}
unsafe impl windows_core::Interface for XamlControlsResources {
    type Vtable = <IXamlControlsResources as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlControlsResources as windows_core::Interface>::IID;
}
impl core::ops::Deref for XamlControlsResources {
    type Target = IXamlControlsResources;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for XamlControlsResources {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.XamlControlsResources";
}
unsafe impl Send for XamlControlsResources {}
unsafe impl Sync for XamlControlsResources {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlControlsXamlMetaDataProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlControlsXamlMetaDataProvider,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IXamlMetadataProvider
);
impl XamlControlsXamlMetaDataProvider {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            XamlControlsXamlMetaDataProvider,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XamlControlsXamlMetaDataProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IXamlMetadataProvider>();
}
unsafe impl windows_core::Interface for XamlControlsXamlMetaDataProvider {
    type Vtable = <IXamlMetadataProvider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlMetadataProvider as windows_core::Interface>::IID;
}
impl core::ops::Deref for XamlControlsXamlMetaDataProvider {
    type Target = IXamlMetadataProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for XamlControlsXamlMetaDataProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider";
}
unsafe impl Send for XamlControlsXamlMetaDataProvider {}
unsafe impl Sync for XamlControlsXamlMetaDataProvider {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlReader,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl XamlReader {
    pub fn Load(xaml: &str) -> windows_core::Result<windows_core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Load)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&windows_core::HSTRING::from(xaml)),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IXamlReaderStatics<R, F: FnOnce(&IXamlReaderStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XamlReader, IXamlReaderStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XamlReader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IXamlReader>();
}
unsafe impl windows_core::Interface for XamlReader {
    type Vtable = <IXamlReader as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlReader as windows_core::Interface>::IID;
}
impl core::ops::Deref for XamlReader {
    type Target = IXamlReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for XamlReader {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlReader";
}
unsafe impl Send for XamlReader {}
unsafe impl Sync for XamlReader {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlRoot(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlRoot,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl XamlRoot {}
impl windows_core::RuntimeType for XamlRoot {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IXamlRoot>();
}
unsafe impl windows_core::Interface for XamlRoot {
    type Vtable = <IXamlRoot as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlRoot as windows_core::Interface>::IID;
}
impl core::ops::Deref for XamlRoot {
    type Target = IXamlRoot;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for XamlRoot {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlRoot";
}
unsafe impl Send for XamlRoot {}
unsafe impl Sync for XamlRoot {}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct XmlnsDefinition {
    pub XmlNamespace: windows_core::HSTRING,
    pub Namespace: windows_core::HSTRING,
}
impl windows_core::TypeKind for XmlnsDefinition {
    type TypeKind = windows_core::CloneType;
}
impl windows_core::RuntimeType for XmlnsDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Markup.XmlnsDefinition;string;string)",
    );
}
