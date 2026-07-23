windows_core::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_core::HRESULT);
windows_core::link!("user32.dll" "system" fn GetDpiForWindow(hwnd : HWND) -> u32);
windows_core::link!("user32.dll" "system" fn GetMonitorInfoW(hmonitor : HMONITOR, lpmi : *mut MONITORINFO) -> windows_core::BOOL);
windows_core::link!("microsoft.windowsappruntime.bootstrap.dll" "system" fn MddBootstrapInitialize2(majorminorversion : u32, versiontag : *const u16, minversion : PACKAGE_VERSION, options : MddBootstrapInitializeOptions) -> windows_core::HRESULT);
windows_core::link!("user32.dll" "system" fn MonitorFromWindow(hwnd : HWND, dwflags : u32) -> HMONITOR);
windows_core::link!("user32.dll" "system" fn PostMessageW(hwnd : HWND, msg : u32, wparam : WPARAM, lparam : LPARAM) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn SetProcessDpiAwarenessContext(value : DPI_AWARENESS_CONTEXT) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn SetWindowPos(hwnd : HWND, hwndinsertafter : HWND, x : i32, y : i32, cx : i32, cy : i32, uflags : u32) -> windows_core::BOOL);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Default: Self = Self(0);
    pub const CompactOverlay: Self = Self(1);
    pub const FullScreen: Self = Self(2);
    pub const Overlapped: Self = Self(3);
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
    pub(crate) fn compose<T>(compose: T) -> windows_core::Result<Self>
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
    pub(crate) fn Current() -> windows_core::Result<Self> {
        Self::IApplicationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub(crate) fn Start<P0>(callback: P0) -> windows_core::Result<()>
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
    pub(crate) fn new<
        F: Fn(windows_core::Ref<ApplicationInitializationCallbackParams>) + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &ApplicationInitializationCallbackBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const UserInput: Self = Self(0);
    pub const ProgrammaticChange: Self = Self(1);
    pub const SuggestionChosen: Self = Self(2);
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
    pub const None: Self = Self(0);
    pub const Level1: Self = Self(1);
    pub const Level2: Self = Self(2);
    pub const Level3: Self = Self(3);
    pub const Level4: Self = Self(4);
    pub const Level5: Self = Self(5);
    pub const Level6: Self = Self(6);
    pub const Level7: Self = Self(7);
    pub const Level8: Self = Self(8);
    pub const Level9: Self = Self(9);
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
    pub const Off: Self = Self(0);
    pub const Polite: Self = Self(1);
    pub const Assertive: Self = Self(2);
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
pub struct AutomationProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AutomationProperties,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl AutomationProperties {
    pub(crate) fn SetAutomationId<P0>(element: P0, value: &str) -> windows_core::Result<()>
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
    pub(crate) fn SetHelpText<P0>(element: P0, value: &str) -> windows_core::Result<()>
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
    pub(crate) fn SetName<P0>(element: P0, value: &str) -> windows_core::Result<()>
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
    pub(crate) fn SetLiveSetting<P0>(
        element: P0,
        value: AutomationLiveSetting,
    ) -> windows_core::Result<()>
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
    pub(crate) fn SetHeadingLevel<P0>(
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
pub struct BitmapIcon(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BitmapIcon,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    BitmapIcon,
    IconElement,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl BitmapIcon {
    pub(crate) fn new() -> windows_core::Result<Self> {
        Self::IBitmapIconFactory(|this| unsafe {
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
    fn IBitmapIconFactory<R, F: FnOnce(&IBitmapIconFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BitmapIcon, IBitmapIconFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BitmapIcon {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBitmapIcon>();
}
unsafe impl windows_core::Interface for BitmapIcon {
    type Vtable = <IBitmapIcon as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapIcon as windows_core::Interface>::IID;
}
impl core::ops::Deref for BitmapIcon {
    type Target = IBitmapIcon;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BitmapIcon {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.BitmapIcon";
}
unsafe impl Send for BitmapIcon {}
unsafe impl Sync for BitmapIcon {}
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
pub const COINIT_APARTMENTTHREADED: COINIT = 2;
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn SetLeft<P0>(element: P0, length: f64) -> windows_core::Result<()>
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
    pub(crate) fn SetTop<P0>(element: P0, length: f64) -> windows_core::Result<()>
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
    pub(crate) fn SetZIndex<P0>(element: P0, value: i32) -> windows_core::Result<()>
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Bottom: Self = Self(0);
    pub const Right: Self = Self(1);
    pub const Collapsed: Self = Self(2);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
pub struct CompositionObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionObject,
    windows_core::IUnknown,
    windows_core::IInspectable
);
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
    pub(crate) fn Rendering<F>(handler: F) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        Self::ICompositionTargetStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Rendering)(
                windows_core::Interface::as_raw(this),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                this.clone(),
                token__,
                windows_core::Interface::vtable(this).RemoveRendering,
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
pub struct ContainerContentChangingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ContainerContentChangingEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for ContainerContentChangingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IContainerContentChangingEventArgs>();
}
unsafe impl windows_core::Interface for ContainerContentChangingEventArgs {
    type Vtable = <IContainerContentChangingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IContainerContentChangingEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ContainerContentChangingEventArgs {
    type Target = IContainerContentChangingEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ContainerContentChangingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ContainerContentChangingEventArgs";
}
unsafe impl Send for ContainerContentChangingEventArgs {}
unsafe impl Sync for ContainerContentChangingEventArgs {}
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
pub struct ContentDialogResult(pub i32);
impl ContentDialogResult {
    pub const None: Self = Self(0);
    pub const Primary: Self = Self(1);
    pub const Secondary: Self = Self(2);
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
    pub top_left: f64,
    pub top_right: f64,
    pub bottom_right: f64,
    pub bottom_left: f64,
}
impl windows_core::TypeKind for CornerRadius {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CornerRadius {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.CornerRadius;f8;f8;f8;f8)",
    );
}
pub type DPI_AWARENESS_CONTEXT = *mut core::ffi::c_void;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = -4 as _;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DataPackageOperation(pub u32);
impl DataPackageOperation {
    pub const None: Self = Self(0);
    pub const Copy: Self = Self(1);
    pub const Move: Self = Self(2);
    pub const Link: Self = Self(4);
    pub const NewTarget: Self = Self(1073741824);
    pub const BackgroundTarget: Self = Self(536870912);
}
impl windows_core::TypeKind for DataPackageOperation {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DataPackageOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.DataTransfer.DataPackageOperation;u4)",
    );
}
impl DataPackageOperation {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DataPackageOperation {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DataPackageOperation {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DataPackageOperation {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for DataPackageOperation {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for DataPackageOperation {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataPackageView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DataPackageView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DataPackageView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDataPackageView>();
}
unsafe impl windows_core::Interface for DataPackageView {
    type Vtable = <IDataPackageView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDataPackageView as windows_core::Interface>::IID;
}
impl core::ops::Deref for DataPackageView {
    type Target = IDataPackageView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DataPackageView {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataPackageView";
}
unsafe impl Send for DataPackageView {}
unsafe impl Sync for DataPackageView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataTemplate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DataTemplate,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(DataTemplate, FrameworkTemplate, DependencyObject);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
pub struct DesktopAcrylicBackdrop(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DesktopAcrylicBackdrop,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(DesktopAcrylicBackdrop, SystemBackdrop, DependencyObject);
impl DesktopAcrylicBackdrop {
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn GetForCurrentThread() -> windows_core::Result<Self> {
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
    pub(crate) fn new<F: Fn() + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &DispatcherQueueHandlerBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
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
    pub const Low: Self = Self(-10);
    pub const Normal: Self = Self(0);
    pub const High: Self = Self(10);
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
pub struct DragEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DragEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(DragEventArgs, RoutedEventArgs);
impl windows_core::RuntimeType for DragEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDragEventArgs>();
}
unsafe impl windows_core::Interface for DragEventArgs {
    type Vtable = <IDragEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDragEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for DragEventArgs {
    type Target = IDragEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DragEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.DragEventArgs";
}
unsafe impl Send for DragEventArgs {}
unsafe impl Sync for DragEventArgs {}
windows_core::imp::define_interface!(
    DragEventHandler,
    DragEventHandler_Vtbl,
    0x277afc83_cb67_56c8_b601_1b9c0f1c3d32
);
impl windows_core::RuntimeType for DragEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct DragEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct DragEventHandlerBox<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<DragEventArgs>) + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<DragEventArgs>) + 'static,
> DragEventHandlerBox<F>
{
    const VTABLE: DragEventHandler_Vtbl = DragEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<DragEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<DragEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<DragEventHandler, F>::Release,
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
                as *mut windows_core::imp::DelegateBox<DragEventHandler, F>);
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
pub struct DragItemsCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DragItemsCompletedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DragItemsCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDragItemsCompletedEventArgs>();
}
unsafe impl windows_core::Interface for DragItemsCompletedEventArgs {
    type Vtable = <IDragItemsCompletedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDragItemsCompletedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for DragItemsCompletedEventArgs {
    type Target = IDragItemsCompletedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DragItemsCompletedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.DragItemsCompletedEventArgs";
}
unsafe impl Send for DragItemsCompletedEventArgs {}
unsafe impl Sync for DragItemsCompletedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DragOperationDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DragOperationDeferral,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DragOperationDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDragOperationDeferral>();
}
unsafe impl windows_core::Interface for DragOperationDeferral {
    type Vtable = <IDragOperationDeferral as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDragOperationDeferral as windows_core::Interface>::IID;
}
impl core::ops::Deref for DragOperationDeferral {
    type Target = IDragOperationDeferral;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DragOperationDeferral {
    const NAME: &'static str = "Microsoft.UI.Xaml.DragOperationDeferral";
}
unsafe impl Send for DragOperationDeferral {}
unsafe impl Sync for DragOperationDeferral {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DragUIOverride(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DragUIOverride,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DragUIOverride {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDragUIOverride>();
}
unsafe impl windows_core::Interface for DragUIOverride {
    type Vtable = <IDragUIOverride as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDragUIOverride as windows_core::Interface>::IID;
}
impl core::ops::Deref for DragUIOverride {
    type Target = IDragUIOverride;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DragUIOverride {
    const NAME: &'static str = "Microsoft.UI.Xaml.DragUIOverride";
}
unsafe impl Send for DragUIOverride {}
unsafe impl Sync for DragUIOverride {}
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
pub const E_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80004005_u32 as _);
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElementCompositionPreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ElementCompositionPreview,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ElementCompositionPreview {
    pub(crate) fn GetElementVisual<P0>(element: P0) -> windows_core::Result<Visual>
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
    pub(crate) fn SetElementChildVisual<P0, P1>(element: P0, visual: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
        P1: windows_core::Param<Visual>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetElementChildVisual)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                visual.param().abi(),
            )
            .ok()
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
    pub const Default: Self = Self(0);
    pub const Light: Self = Self(1);
    pub const Dark: Self = Self(2);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FileAttributes(pub u32);
impl FileAttributes {
    pub const Normal: Self = Self(0);
    pub const ReadOnly: Self = Self(1);
    pub const Directory: Self = Self(16);
    pub const Archive: Self = Self(32);
    pub const Temporary: Self = Self(256);
    pub const LocallyIncomplete: Self = Self(512);
}
impl windows_core::TypeKind for FileAttributes {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FileAttributes {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.FileAttributes;u4)");
}
impl FileAttributes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FileAttributes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FileAttributes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FileAttributes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for FileAttributes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for FileAttributes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Top: Self = Self(0);
    pub const Bottom: Self = Self(1);
    pub const Left: Self = Self(2);
    pub const Right: Self = Self(3);
    pub const Full: Self = Self(4);
    pub const TopEdgeAlignedLeft: Self = Self(5);
    pub const TopEdgeAlignedRight: Self = Self(6);
    pub const BottomEdgeAlignedLeft: Self = Self(7);
    pub const BottomEdgeAlignedRight: Self = Self(8);
    pub const LeftEdgeAlignedTop: Self = Self(9);
    pub const LeftEdgeAlignedBottom: Self = Self(10);
    pub const RightEdgeAlignedTop: Self = Self(11);
    pub const RightEdgeAlignedBottom: Self = Self(12);
    pub const Auto: Self = Self(13);
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
    pub(crate) fn CreateInstanceWithName(familyname: &str) -> windows_core::Result<Self> {
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
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontIcon(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FontIcon,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    FontIcon,
    IconElement,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl FontIcon {
    pub(crate) fn new() -> windows_core::Result<Self> {
        Self::IFontIconFactory(|this| unsafe {
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
    fn IFontIconFactory<R, F: FnOnce(&IFontIconFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FontIcon, IFontIconFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FontIcon {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFontIcon>();
}
unsafe impl windows_core::Interface for FontIcon {
    type Vtable = <IFontIcon as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFontIcon as windows_core::Interface>::IID;
}
impl core::ops::Deref for FontIcon {
    type Target = IFontIcon;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FontIcon {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.FontIcon";
}
unsafe impl Send for FontIcon {}
unsafe impl Sync for FontIcon {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FontWeight {
    pub weight: u16,
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
pub struct FrameworkTemplate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FrameworkTemplate,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(FrameworkTemplate, DependencyObject);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn SetRow<P0>(element: P0, value: i32) -> windows_core::Result<()>
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
    pub(crate) fn SetColumn<P0>(element: P0, value: i32) -> windows_core::Result<()>
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
    pub(crate) fn SetRowSpan<P0>(element: P0, value: i32) -> windows_core::Result<()>
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
    pub(crate) fn SetColumnSpan<P0>(element: P0, value: i32) -> windows_core::Result<()>
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
    pub value: f64,
    pub grid_unit_type: GridUnitType,
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
    pub const Auto: Self = Self(0);
    pub const Pixel: Self = Self(1);
    pub const Star: Self = Self(2);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
pub const HTCLIENT: u32 = 1;
pub type HWND = *mut core::ffi::c_void;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HorizontalAlignment(pub i32);
impl HorizontalAlignment {
    pub const Left: Self = Self(0);
    pub const Center: Self = Self(1);
    pub const Right: Self = Self(2);
    pub const Stretch: Self = Self(3);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
pub struct IAppBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn Label(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub(crate) fn SetLabel(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetLabel)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetIcon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IconElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetIcon)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppBarButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Icon: usize,
    pub SetIcon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetLabel(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetLabel)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetIcon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IconElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetIcon)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppBarToggleButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Label: usize,
    pub SetLabel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Icon: usize,
    pub SetIcon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn Presenter(&self) -> windows_core::Result<AppWindowPresenter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Presenter)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn Size(&self) -> windows_core::Result<SizeInt32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn TitleBar(&self) -> windows_core::Result<AppWindowTitleBar> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TitleBar)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetIcon(&self, iconpath: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIcon)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(iconpath)),
            )
            .ok()
        }
    }
    pub(crate) fn SetPresenterByKind(
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
pub struct IAppWindow_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Id: usize,
    IsShownInSwitchers: usize,
    SetIsShownInSwitchers: usize,
    IsVisible: usize,
    OwnerWindowId: usize,
    Position: usize,
    pub Presenter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Size:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut SizeInt32) -> windows_core::HRESULT,
    Title: usize,
    SetTitle: usize,
    pub TitleBar: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Destroy: usize,
    Hide: usize,
    Move: usize,
    MoveAndResize: usize,
    MoveAndResizeRelativeToDisplayArea: usize,
    Resize: usize,
    pub SetIcon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SetIconWithIconId: usize,
    SetPresenter: usize,
    pub SetPresenterByKind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        AppWindowPresenterKind,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn ClientSize(&self) -> windows_core::Result<SizeInt32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn ResizeClient(&self, size: SizeInt32) -> windows_core::Result<()> {
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
pub struct IAppWindow2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ClientSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut SizeInt32) -> windows_core::HRESULT,
    MoveInZOrderAtBottom: usize,
    MoveInZOrderAtTop: usize,
    MoveInZOrderBelow: usize,
    pub ResizeClient:
        unsafe extern "system" fn(*mut core::ffi::c_void, SizeInt32) -> windows_core::HRESULT,
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
pub struct IAppWindowPresenter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
pub struct IAppWindowTitleBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn SetPreferredHeightOption(
        &self,
        value: TitleBarHeightOption,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredHeightOption)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppWindowTitleBar2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PreferredHeightOption: usize,
    pub SetPreferredHeightOption: unsafe extern "system" fn(
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
    pub(crate) fn SetPreferredTheme(&self, value: TitleBarTheme) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredTheme)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppWindowTitleBar3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PreferredTheme: usize,
    pub SetPreferredTheme:
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
    pub(crate) fn Resources(&self) -> windows_core::Result<ResourceDictionary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Resources)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IApplication_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Resources: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
pub struct IApplicationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub(crate) fn SetText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetPlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SuggestionChosen<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SuggestionChosen)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveSuggestionChosen,
            ))
        }
    }
    pub(crate) fn TextChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<AutoSuggestBox>,
                windows_core::Ref<AutoSuggestBoxTextChangedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<AutoSuggestBox, AutoSuggestBoxTextChangedEventArgs> = {
            let com = windows_core::imp::DelegateBox::< TypedEventHandler < AutoSuggestBox , AutoSuggestBoxTextChangedEventArgs > , F >::new (& TypedEventHandlerBox::< AutoSuggestBox , AutoSuggestBoxTextChangedEventArgs , F >::VTABLE , handler) ;
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).TextChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveTextChanged,
            ))
        }
    }
    pub(crate) fn QuerySubmitted<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).QuerySubmitted)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveQuerySubmitted,
            ))
        }
    }
}
#[repr(C)]
pub struct IAutoSuggestBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    MaxSuggestionListHeight: usize,
    SetMaxSuggestionListHeight: usize,
    IsSuggestionListOpen: usize,
    SetIsSuggestionListOpen: usize,
    TextMemberPath: usize,
    SetTextMemberPath: usize,
    pub Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    UpdateTextOnSelect: usize,
    SetUpdateTextOnSelect: usize,
    PlaceholderText: usize,
    pub SetPlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    AutoMaximizeSuggestionArea: usize,
    SetAutoMaximizeSuggestionArea: usize,
    TextBoxStyle: usize,
    SetTextBoxStyle: usize,
    QueryIcon: usize,
    SetQueryIcon: usize,
    LightDismissOverlayMode: usize,
    SetLightDismissOverlayMode: usize,
    Description: usize,
    SetDescription: usize,
    pub SuggestionChosen: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSuggestionChosen:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub TextChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveTextChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub QuerySubmitted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveQuerySubmitted:
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
    pub(crate) fn QueryText(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryText)(
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
pub struct IAutoSuggestBoxQuerySubmittedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub QueryText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IAutoSuggestBoxSuggestionChosenEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectedItem: unsafe extern "system" fn(
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
    pub(crate) fn Reason(&self) -> windows_core::Result<AutoSuggestionBoxTextChangeReason> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reason)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IAutoSuggestBoxTextChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AutoSuggestionBoxTextChangeReason,
    ) -> windows_core::HRESULT,
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
pub struct IAutomationPropertiesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    AcceleratorKeyProperty: usize,
    GetAcceleratorKey: usize,
    SetAcceleratorKey: usize,
    AccessKeyProperty: usize,
    GetAccessKey: usize,
    SetAccessKey: usize,
    AutomationIdProperty: usize,
    GetAutomationId: usize,
    pub SetAutomationId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    HelpTextProperty: usize,
    GetHelpText: usize,
    pub SetHelpText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    IsRequiredForFormProperty: usize,
    GetIsRequiredForForm: usize,
    SetIsRequiredForForm: usize,
    ItemStatusProperty: usize,
    GetItemStatus: usize,
    SetItemStatus: usize,
    ItemTypeProperty: usize,
    GetItemType: usize,
    SetItemType: usize,
    LabeledByProperty: usize,
    GetLabeledBy: usize,
    SetLabeledBy: usize,
    NameProperty: usize,
    GetName: usize,
    pub SetName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    LiveSettingProperty: usize,
    GetLiveSetting: usize,
    pub SetLiveSetting: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        AutomationLiveSetting,
    ) -> windows_core::HRESULT,
    AccessibilityViewProperty: usize,
    GetAccessibilityView: usize,
    SetAccessibilityView: usize,
    ControlledPeersProperty: usize,
    GetControlledPeers: usize,
    PositionInSetProperty: usize,
    GetPositionInSet: usize,
    SetPositionInSet: usize,
    SizeOfSetProperty: usize,
    GetSizeOfSet: usize,
    SetSizeOfSet: usize,
    LevelProperty: usize,
    GetLevel: usize,
    SetLevel: usize,
    AnnotationsProperty: usize,
    GetAnnotations: usize,
    LandmarkTypeProperty: usize,
    GetLandmarkType: usize,
    SetLandmarkType: usize,
    LocalizedLandmarkTypeProperty: usize,
    GetLocalizedLandmarkType: usize,
    SetLocalizedLandmarkType: usize,
    IsPeripheralProperty: usize,
    GetIsPeripheral: usize,
    SetIsPeripheral: usize,
    IsDataValidForFormProperty: usize,
    GetIsDataValidForForm: usize,
    SetIsDataValidForForm: usize,
    FullDescriptionProperty: usize,
    GetFullDescription: usize,
    SetFullDescription: usize,
    LocalizedControlTypeProperty: usize,
    GetLocalizedControlType: usize,
    SetLocalizedControlType: usize,
    DescribedByProperty: usize,
    GetDescribedBy: usize,
    FlowsToProperty: usize,
    GetFlowsTo: usize,
    FlowsFromProperty: usize,
    GetFlowsFrom: usize,
    CultureProperty: usize,
    GetCulture: usize,
    SetCulture: usize,
    HeadingLevelProperty: usize,
    GetHeadingLevel: usize,
    pub SetHeadingLevel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        AutomationHeadingLevel,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IBitmapIcon,
    IBitmapIcon_Vtbl,
    0xc370bc29_805b_5bad_b615_ec640e579dbb
);
impl windows_core::RuntimeType for IBitmapIcon {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IBitmapIcon {
    pub(crate) fn SetUriSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Uri>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetUriSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetShowAsMonochrome(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetShowAsMonochrome)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IBitmapIcon_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    UriSource: usize,
    pub SetUriSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ShowAsMonochrome: usize,
    pub SetShowAsMonochrome:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IBitmapIconFactory,
    IBitmapIconFactory_Vtbl,
    0xb43b5ddc_cdb5_5ad6_8ac1_2fcca33be39e
);
impl windows_core::RuntimeType for IBitmapIconFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBitmapIconFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetUriSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Uri>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetUriSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IBitmapImage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CreateOptions: usize,
    SetCreateOptions: usize,
    UriSource: usize,
    pub SetUriSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
pub struct IBitmapSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IBlock, IBlock_Vtbl, 0x8149d507_672f_5fd5_a10a_351389ba9659);
impl windows_core::RuntimeType for IBlock {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBlock_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn SetBorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetBorderBrush)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetBorderThickness(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetBorderThickness)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetBackground)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetCornerRadius(&self, value: CornerRadius) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCornerRadius)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetPadding(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPadding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetChild<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetChild)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IBorder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    BorderBrush: usize,
    pub SetBorderBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    BorderThickness: usize,
    pub SetBorderThickness:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    Background: usize,
    pub SetBackground: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    BackgroundSizing: usize,
    SetBackgroundSizing: usize,
    CornerRadius: usize,
    pub SetCornerRadius:
        unsafe extern "system" fn(*mut core::ffi::c_void, CornerRadius) -> windows_core::HRESULT,
    Padding: usize,
    pub SetPadding:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    Child: usize,
    pub SetChild: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetItemsSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetItemsSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn ItemClicked<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<BreadcrumbBar>,
                windows_core::Ref<BreadcrumbBarItemClickedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<BreadcrumbBar, BreadcrumbBarItemClickedEventArgs> = {
            let com = windows_core::imp::DelegateBox::< TypedEventHandler < BreadcrumbBar , BreadcrumbBarItemClickedEventArgs > , F >::new (& TypedEventHandlerBox::< BreadcrumbBar , BreadcrumbBarItemClickedEventArgs , F >::VTABLE , handler) ;
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ItemClicked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveItemClicked,
            ))
        }
    }
}
#[repr(C)]
pub struct IBreadcrumbBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ItemsSource: usize,
    pub SetItemsSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ItemTemplate: usize,
    SetItemTemplate: usize,
    pub ItemClicked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveItemClicked:
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
    pub(crate) fn Index(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Index)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IBreadcrumbBarItemClickedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBrush, IBrush_Vtbl, 0x2de3cb83_1329_5679_88f8_c822bc5442cb);
impl windows_core::RuntimeType for IBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBrush_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn Flyout(&self) -> windows_core::Result<FlyoutBase> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Flyout)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetFlyout<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FlyoutBase>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFlyout)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Flyout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetFlyout: unsafe extern "system" fn(
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
    pub(crate) fn Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Click)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveClick,
            ))
        }
    }
}
#[repr(C)]
pub struct IButtonBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ClickMode: usize,
    SetClickMode: usize,
    IsPointerOver: usize,
    IsPressed: usize,
    Command: usize,
    SetCommand: usize,
    CommandParameter: usize,
    SetCommandParameter: usize,
    pub Click: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveClick:
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
    pub(crate) fn SetIsCalendarOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsCalendarOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetPlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsTodayHighlighted(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsTodayHighlighted)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn DateChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).DateChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveDateChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct ICalendarDatePicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Date: usize,
    SetDate: usize,
    IsCalendarOpen: usize,
    pub SetIsCalendarOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    DateFormat: usize,
    SetDateFormat: usize,
    PlaceholderText: usize,
    pub SetPlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    CalendarViewStyle: usize,
    SetCalendarViewStyle: usize,
    LightDismissOverlayMode: usize,
    SetLightDismissOverlayMode: usize,
    Description: usize,
    SetDescription: usize,
    MinDate: usize,
    SetMinDate: usize,
    MaxDate: usize,
    SetMaxDate: usize,
    IsTodayHighlighted: usize,
    pub SetIsTodayHighlighted:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    DisplayMode: usize,
    SetDisplayMode: usize,
    FirstDayOfWeek: usize,
    SetFirstDayOfWeek: usize,
    DayOfWeekFormat: usize,
    SetDayOfWeekFormat: usize,
    CalendarIdentifier: usize,
    SetCalendarIdentifier: usize,
    IsOutOfScopeEnabled: usize,
    SetIsOutOfScopeEnabled: usize,
    IsGroupLabelVisible: usize,
    SetIsGroupLabelVisible: usize,
    CalendarViewDayItemChanging: usize,
    RemoveCalendarViewDayItemChanging: usize,
    pub DateChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveDateChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
    pub(crate) fn NewDate(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewDate)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<windows_time::DateTime>| r__.Value())
        }
    }
}
#[repr(C)]
pub struct ICalendarDatePickerDateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NewDate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetIsGroupLabelVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsGroupLabelVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsTodayHighlighted(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsTodayHighlighted)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SelectedDatesChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SelectedDatesChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveSelectedDatesChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct ICalendarView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CalendarIdentifier: usize,
    SetCalendarIdentifier: usize,
    DayOfWeekFormat: usize,
    SetDayOfWeekFormat: usize,
    IsGroupLabelVisible: usize,
    pub SetIsGroupLabelVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    DisplayMode: usize,
    SetDisplayMode: usize,
    FirstDayOfWeek: usize,
    SetFirstDayOfWeek: usize,
    IsOutOfScopeEnabled: usize,
    SetIsOutOfScopeEnabled: usize,
    IsTodayHighlighted: usize,
    pub SetIsTodayHighlighted:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    MaxDate: usize,
    SetMaxDate: usize,
    MinDate: usize,
    SetMinDate: usize,
    NumberOfWeeksInView: usize,
    SetNumberOfWeeksInView: usize,
    SelectedDates: usize,
    SelectionMode: usize,
    SetSelectionMode: usize,
    TemplateSettings: usize,
    FocusBorderBrush: usize,
    SetFocusBorderBrush: usize,
    SelectedHoverBorderBrush: usize,
    SetSelectedHoverBorderBrush: usize,
    SelectedPressedBorderBrush: usize,
    SetSelectedPressedBorderBrush: usize,
    SelectedDisabledBorderBrush: usize,
    SetSelectedDisabledBorderBrush: usize,
    SelectedBorderBrush: usize,
    SetSelectedBorderBrush: usize,
    HoverBorderBrush: usize,
    SetHoverBorderBrush: usize,
    PressedBorderBrush: usize,
    SetPressedBorderBrush: usize,
    TodaySelectedInnerBorderBrush: usize,
    SetTodaySelectedInnerBorderBrush: usize,
    BlackoutStrikethroughBrush: usize,
    SetBlackoutStrikethroughBrush: usize,
    CalendarItemBorderBrush: usize,
    SetCalendarItemBorderBrush: usize,
    BlackoutBackground: usize,
    SetBlackoutBackground: usize,
    OutOfScopeBackground: usize,
    SetOutOfScopeBackground: usize,
    CalendarItemBackground: usize,
    SetCalendarItemBackground: usize,
    CalendarItemHoverBackground: usize,
    SetCalendarItemHoverBackground: usize,
    CalendarItemPressedBackground: usize,
    SetCalendarItemPressedBackground: usize,
    CalendarItemDisabledBackground: usize,
    SetCalendarItemDisabledBackground: usize,
    TodayBackground: usize,
    SetTodayBackground: usize,
    TodayBlackoutBackground: usize,
    SetTodayBlackoutBackground: usize,
    TodayHoverBackground: usize,
    SetTodayHoverBackground: usize,
    TodayPressedBackground: usize,
    SetTodayPressedBackground: usize,
    TodayDisabledBackground: usize,
    SetTodayDisabledBackground: usize,
    PressedForeground: usize,
    SetPressedForeground: usize,
    TodayForeground: usize,
    SetTodayForeground: usize,
    BlackoutForeground: usize,
    SetBlackoutForeground: usize,
    TodayBlackoutForeground: usize,
    SetTodayBlackoutForeground: usize,
    SelectedForeground: usize,
    SetSelectedForeground: usize,
    SelectedHoverForeground: usize,
    SetSelectedHoverForeground: usize,
    SelectedPressedForeground: usize,
    SetSelectedPressedForeground: usize,
    SelectedDisabledForeground: usize,
    SetSelectedDisabledForeground: usize,
    OutOfScopeForeground: usize,
    SetOutOfScopeForeground: usize,
    OutOfScopeHoverForeground: usize,
    SetOutOfScopeHoverForeground: usize,
    OutOfScopePressedForeground: usize,
    SetOutOfScopePressedForeground: usize,
    CalendarItemForeground: usize,
    SetCalendarItemForeground: usize,
    DisabledForeground: usize,
    SetDisabledForeground: usize,
    DayItemFontFamily: usize,
    SetDayItemFontFamily: usize,
    DayItemFontSize: usize,
    SetDayItemFontSize: usize,
    DayItemFontStyle: usize,
    SetDayItemFontStyle: usize,
    DayItemFontWeight: usize,
    SetDayItemFontWeight: usize,
    TodayFontWeight: usize,
    SetTodayFontWeight: usize,
    FirstOfMonthLabelFontFamily: usize,
    SetFirstOfMonthLabelFontFamily: usize,
    FirstOfMonthLabelFontSize: usize,
    SetFirstOfMonthLabelFontSize: usize,
    FirstOfMonthLabelFontStyle: usize,
    SetFirstOfMonthLabelFontStyle: usize,
    FirstOfMonthLabelFontWeight: usize,
    SetFirstOfMonthLabelFontWeight: usize,
    MonthYearItemFontFamily: usize,
    SetMonthYearItemFontFamily: usize,
    MonthYearItemFontSize: usize,
    SetMonthYearItemFontSize: usize,
    MonthYearItemFontStyle: usize,
    SetMonthYearItemFontStyle: usize,
    MonthYearItemFontWeight: usize,
    SetMonthYearItemFontWeight: usize,
    FirstOfYearDecadeLabelFontFamily: usize,
    SetFirstOfYearDecadeLabelFontFamily: usize,
    FirstOfYearDecadeLabelFontSize: usize,
    SetFirstOfYearDecadeLabelFontSize: usize,
    FirstOfYearDecadeLabelFontStyle: usize,
    SetFirstOfYearDecadeLabelFontStyle: usize,
    FirstOfYearDecadeLabelFontWeight: usize,
    SetFirstOfYearDecadeLabelFontWeight: usize,
    DayItemMargin: usize,
    SetDayItemMargin: usize,
    MonthYearItemMargin: usize,
    SetMonthYearItemMargin: usize,
    FirstOfMonthLabelMargin: usize,
    SetFirstOfMonthLabelMargin: usize,
    FirstOfYearDecadeLabelMargin: usize,
    SetFirstOfYearDecadeLabelMargin: usize,
    HorizontalDayItemAlignment: usize,
    SetHorizontalDayItemAlignment: usize,
    VerticalDayItemAlignment: usize,
    SetVerticalDayItemAlignment: usize,
    HorizontalFirstOfMonthLabelAlignment: usize,
    SetHorizontalFirstOfMonthLabelAlignment: usize,
    VerticalFirstOfMonthLabelAlignment: usize,
    SetVerticalFirstOfMonthLabelAlignment: usize,
    CalendarItemBorderThickness: usize,
    SetCalendarItemBorderThickness: usize,
    CalendarViewDayItemStyle: usize,
    SetCalendarViewDayItemStyle: usize,
    CalendarItemCornerRadius: usize,
    SetCalendarItemCornerRadius: usize,
    CalendarViewDayItemChanging: usize,
    RemoveCalendarViewDayItemChanging: usize,
    pub SelectedDatesChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSelectedDatesChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
#[repr(C)]
pub struct ICalendarViewSelectedDatesChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
pub struct ICanvasStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    LeftProperty: usize,
    GetLeft: usize,
    pub SetLeft: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        f64,
    ) -> windows_core::HRESULT,
    TopProperty: usize,
    GetTop: usize,
    pub SetTop: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        f64,
    ) -> windows_core::HRESULT,
    ZIndexProperty: usize,
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
    pub(crate) fn NewColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IColorChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    OldColor: usize,
    pub NewColor:
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
    pub(crate) fn SetColor(&self, value: Color) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetColor)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsAlphaEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsAlphaEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsColorSliderVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsColorSliderVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsColorChannelTextInputVisible(
        &self,
        value: bool,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsColorChannelTextInputVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsHexInputVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsHexInputVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn ColorChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ColorChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveColorChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IColorPicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Color: usize,
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, Color) -> windows_core::HRESULT,
    PreviousColor: usize,
    SetPreviousColor: usize,
    IsAlphaEnabled: usize,
    pub SetIsAlphaEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    IsColorSpectrumVisible: usize,
    SetIsColorSpectrumVisible: usize,
    IsColorPreviewVisible: usize,
    SetIsColorPreviewVisible: usize,
    IsColorSliderVisible: usize,
    pub SetIsColorSliderVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    IsAlphaSliderVisible: usize,
    SetIsAlphaSliderVisible: usize,
    IsMoreButtonVisible: usize,
    SetIsMoreButtonVisible: usize,
    IsColorChannelTextInputVisible: usize,
    pub SetIsColorChannelTextInputVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    IsAlphaTextInputVisible: usize,
    SetIsAlphaTextInputVisible: usize,
    IsHexInputVisible: usize,
    pub SetIsHexInputVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    MinHue: usize,
    SetMinHue: usize,
    MaxHue: usize,
    SetMaxHue: usize,
    MinSaturation: usize,
    SetMinSaturation: usize,
    MaxSaturation: usize,
    SetMaxSaturation: usize,
    MinValue: usize,
    SetMinValue: usize,
    MaxValue: usize,
    SetMaxValue: usize,
    ColorSpectrumShape: usize,
    SetColorSpectrumShape: usize,
    ColorSpectrumComponents: usize,
    SetColorSpectrumComponents: usize,
    pub ColorChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveColorChanged:
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
    pub(crate) fn SetWidth(&self, value: GridLength) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetWidth)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IColumnDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Width: usize,
    pub SetWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, GridLength) -> windows_core::HRESULT,
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
    pub(crate) fn SetIsEditable(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsEditable)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetPlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IComboBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsDropDownOpen: usize,
    SetIsDropDownOpen: usize,
    IsEditable: usize,
    pub SetIsEditable:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    IsSelectionBoxHighlighted: usize,
    MaxDropDownHeight: usize,
    SetMaxDropDownHeight: usize,
    SelectionBoxItem: usize,
    SelectionBoxItemTemplate: usize,
    TemplateSettings: usize,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    PlaceholderText: usize,
    pub SetPlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn PrimaryCommands(
        &self,
    ) -> windows_core::Result<windows_collections::IObservableVector<ICommandBarElement>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrimaryCommands)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SecondaryCommands(
        &self,
    ) -> windows_core::Result<windows_collections::IObservableVector<ICommandBarElement>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SecondaryCommands)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetDefaultLabelPosition(
        &self,
        value: CommandBarDefaultLabelPosition,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultLabelPosition)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICommandBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrimaryCommands: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SecondaryCommands: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CommandBarOverflowPresenterStyle: usize,
    SetCommandBarOverflowPresenterStyle: usize,
    CommandBarTemplateSettings: usize,
    DefaultLabelPosition: usize,
    pub SetDefaultLabelPosition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        CommandBarDefaultLabelPosition,
    ) -> windows_core::HRESULT,
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
pub struct ICommandBarElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn PrimaryCommands(
        &self,
    ) -> windows_core::Result<windows_collections::IObservableVector<ICommandBarElement>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrimaryCommands)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SecondaryCommands(
        &self,
    ) -> windows_core::Result<windows_collections::IObservableVector<ICommandBarElement>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SecondaryCommands)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICommandBarFlyout_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrimaryCommands: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SecondaryCommands: unsafe extern "system" fn(
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
    ICompositionObject,
    ICompositionObject_Vtbl,
    0x0e583d49_fb5e_5481_a426_d3c41e059a5a
);
impl windows_core::RuntimeType for ICompositionObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositionObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
pub struct ICompositionTargetStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Rendering: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveRendering:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContainerContentChangingEventArgs,
    IContainerContentChangingEventArgs_Vtbl,
    0xf4c8c937_b070_53ce_a76c_074ee5750a71
);
impl windows_core::RuntimeType for IContainerContentChangingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IContainerContentChangingEventArgs {
    pub(crate) fn ItemContainer(&self) -> windows_core::Result<SelectorItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ItemContainer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn InRecycleQueue(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InRecycleQueue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn ItemIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ItemIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHandled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IContainerContentChangingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ItemContainer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub InRecycleQueue:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ItemIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    Item: usize,
    Phase: usize,
    Handled: usize,
    pub SetHandled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
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
    pub(crate) fn Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn ContentTemplateRoot(&self) -> windows_core::Result<UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentTemplateRoot)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IContentControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ContentTemplate: usize,
    SetContentTemplate: usize,
    ContentTemplateSelector: usize,
    SetContentTemplateSelector: usize,
    ContentTransitions: usize,
    SetContentTransitions: usize,
    pub ContentTemplateRoot: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetTitle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTitle)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetPrimaryButtonText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrimaryButtonText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetSecondaryButtonText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSecondaryButtonText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetCloseButtonText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCloseButtonText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsPrimaryButtonEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsPrimaryButtonEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsSecondaryButtonEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsSecondaryButtonEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Closed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Closed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveClosed,
            ))
        }
    }
    pub(crate) fn Hide(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Hide)(windows_core::Interface::as_raw(self)).ok()
        }
    }
    pub(crate) fn ShowAsync(
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
}
#[repr(C)]
pub struct IContentDialog_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Title: usize,
    pub SetTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    TitleTemplate: usize,
    SetTitleTemplate: usize,
    FullSizeDesired: usize,
    SetFullSizeDesired: usize,
    PrimaryButtonText: usize,
    pub SetPrimaryButtonText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SecondaryButtonText: usize,
    pub SetSecondaryButtonText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CloseButtonText: usize,
    pub SetCloseButtonText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    PrimaryButtonCommand: usize,
    SetPrimaryButtonCommand: usize,
    SecondaryButtonCommand: usize,
    SetSecondaryButtonCommand: usize,
    CloseButtonCommand: usize,
    SetCloseButtonCommand: usize,
    PrimaryButtonCommandParameter: usize,
    SetPrimaryButtonCommandParameter: usize,
    SecondaryButtonCommandParameter: usize,
    SetSecondaryButtonCommandParameter: usize,
    CloseButtonCommandParameter: usize,
    SetCloseButtonCommandParameter: usize,
    IsPrimaryButtonEnabled: usize,
    pub SetIsPrimaryButtonEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    IsSecondaryButtonEnabled: usize,
    pub SetIsSecondaryButtonEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    PrimaryButtonStyle: usize,
    SetPrimaryButtonStyle: usize,
    SecondaryButtonStyle: usize,
    SetSecondaryButtonStyle: usize,
    CloseButtonStyle: usize,
    SetCloseButtonStyle: usize,
    DefaultButton: usize,
    SetDefaultButton: usize,
    Closing: usize,
    RemoveClosing: usize,
    pub Closed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveClosed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Opened: usize,
    RemoveOpened: usize,
    PrimaryButtonClick: usize,
    RemovePrimaryButtonClick: usize,
    SecondaryButtonClick: usize,
    RemoveSecondaryButtonClick: usize,
    CloseButtonClick: usize,
    RemoveCloseButtonClick: usize,
    pub Hide: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
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
    pub(crate) fn Result(&self) -> windows_core::Result<ContentDialogResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Result)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IContentDialogClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(
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
    pub(crate) fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FontFamily>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontFamily)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetFontWeight(&self, value: FontWeight) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontWeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetForeground)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetPadding(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPadding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetBackground)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetBorderThickness(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetBorderThickness)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetBorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetBorderBrush)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsFocusEngagementEnabled: usize,
    SetIsFocusEngagementEnabled: usize,
    IsFocusEngaged: usize,
    SetIsFocusEngaged: usize,
    RequiresPointer: usize,
    SetRequiresPointer: usize,
    FontSize: usize,
    pub SetFontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    FontFamily: usize,
    pub SetFontFamily: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    FontWeight: usize,
    pub SetFontWeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, FontWeight) -> windows_core::HRESULT,
    FontStyle: usize,
    SetFontStyle: usize,
    FontStretch: usize,
    SetFontStretch: usize,
    CharacterSpacing: usize,
    SetCharacterSpacing: usize,
    Foreground: usize,
    pub SetForeground: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    IsTextScaleFactorEnabled: usize,
    SetIsTextScaleFactorEnabled: usize,
    IsEnabled: usize,
    pub SetIsEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    TabNavigation: usize,
    SetTabNavigation: usize,
    Template: usize,
    SetTemplate: usize,
    Padding: usize,
    pub SetPadding:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    HorizontalContentAlignment: usize,
    SetHorizontalContentAlignment: usize,
    VerticalContentAlignment: usize,
    SetVerticalContentAlignment: usize,
    Background: usize,
    pub SetBackground: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    BackgroundSizing: usize,
    SetBackgroundSizing: usize,
    BorderThickness: usize,
    pub SetBorderThickness:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    BorderBrush: usize,
    pub SetBorderBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDataPackageView,
    IDataPackageView_Vtbl,
    0x7b840471_5900_4d85_a90b_10cb85fe3552
);
impl windows_core::RuntimeType for IDataPackageView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDataPackageView {
    pub(crate) fn AvailableFormats(
        &self,
    ) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AvailableFormats)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn GetTextAsync(
        &self,
    ) -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn GetStorageItemsAsync(
        &self,
    ) -> windows_core::Result<
        windows_future::IAsyncOperation<windows_collections::IVectorView<IStorageItem>>,
    > {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStorageItemsAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDataPackageView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Properties: usize,
    RequestedOperation: usize,
    ReportOperationCompleted: usize,
    pub AvailableFormats: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Contains: usize,
    GetDataAsync: usize,
    pub GetTextAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetCustomTextAsync: usize,
    GetUriAsync: usize,
    GetHtmlFormatAsync: usize,
    GetResourceMapAsync: usize,
    GetRtfAsync: usize,
    GetBitmapAsync: usize,
    pub GetStorageItemsAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
pub struct IDataTemplate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetDayVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDayVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetMonthVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMonthVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetYearVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetYearVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SelectedDateChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SelectedDateChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveSelectedDateChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IDatePicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    CalendarIdentifier: usize,
    SetCalendarIdentifier: usize,
    Date: usize,
    SetDate: usize,
    DayVisible: usize,
    pub SetDayVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    MonthVisible: usize,
    pub SetMonthVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    YearVisible: usize,
    pub SetYearVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    DayFormat: usize,
    SetDayFormat: usize,
    MonthFormat: usize,
    SetMonthFormat: usize,
    YearFormat: usize,
    SetYearFormat: usize,
    MinYear: usize,
    SetMinYear: usize,
    MaxYear: usize,
    SetMaxYear: usize,
    Orientation: usize,
    SetOrientation: usize,
    LightDismissOverlayMode: usize,
    SetLightDismissOverlayMode: usize,
    SelectedDate: usize,
    SetSelectedDate: usize,
    DateChanged: usize,
    RemoveDateChanged: usize,
    pub SelectedDateChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSelectedDateChanged:
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
    pub(crate) fn NewDate(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewDate)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<windows_time::DateTime>| r__.Value())
        }
    }
}
#[repr(C)]
pub struct IDatePickerSelectedValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    OldDate: usize,
    pub NewDate: unsafe extern "system" fn(
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
#[repr(C)]
pub struct IDependencyObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn CreateTimer(&self) -> windows_core::Result<DispatcherQueueTimer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTimer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn TryEnqueueWithPriority<P1>(
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
pub struct IDispatcherQueue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateTimer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    TryEnqueue: usize,
    pub TryEnqueueWithPriority: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DispatcherQueuePriority,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetInterval(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInterval)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsRepeating(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsRepeating)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Start(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub(crate) fn Stop(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
        }
    }
    pub(crate) fn Tick<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<DispatcherQueueTimer>,
                windows_core::Ref<windows_core::IInspectable>,
            ) + 'static,
    {
        let handler: TypedEventHandler<DispatcherQueueTimer, windows_core::IInspectable> = {
            let com = windows_core::imp::DelegateBox::< TypedEventHandler < DispatcherQueueTimer , windows_core::IInspectable > , F >::new (& TypedEventHandlerBox::< DispatcherQueueTimer , windows_core::IInspectable , F >::VTABLE , handler) ;
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Tick)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveTick,
            ))
        }
    }
}
#[repr(C)]
pub struct IDispatcherQueueTimer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Interval: usize,
    pub SetInterval: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    IsRunning: usize,
    IsRepeating: usize,
    pub SetIsRepeating:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Tick: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveTick: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDragEventArgs,
    IDragEventArgs_Vtbl,
    0x47ac5757_e4bc_52ba_8ab9_1bf81aad7900
);
impl windows_core::RuntimeType for IDragEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDragEventArgs {
    pub(crate) fn DataView(&self) -> windows_core::Result<DataPackageView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataView)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn DragUIOverride(&self) -> windows_core::Result<DragUIOverride> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DragUIOverride)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetAcceptedOperation(
        &self,
        value: DataPackageOperation,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAcceptedOperation)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn GetDeferral(&self) -> windows_core::Result<DragOperationDeferral> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeferral)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDragEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Handled: usize,
    SetHandled: usize,
    Data: usize,
    SetData: usize,
    pub DataView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DragUIOverride: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Modifiers: usize,
    AcceptedOperation: usize,
    pub SetAcceptedOperation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DataPackageOperation,
    ) -> windows_core::HRESULT,
    AllowedOperations: usize,
    pub GetDeferral: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDragItemsCompletedEventArgs,
    IDragItemsCompletedEventArgs_Vtbl,
    0xc0138552_f467_5c3e_8af4_593607762844
);
impl windows_core::RuntimeType for IDragItemsCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDragItemsCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IDragOperationDeferral,
    IDragOperationDeferral_Vtbl,
    0x462c1880_fc6a_5035_8abf_564bacb78158
);
impl windows_core::RuntimeType for IDragOperationDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDragOperationDeferral {
    pub(crate) fn Complete(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
pub struct IDragOperationDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDragUIOverride,
    IDragUIOverride_Vtbl,
    0x3260b18b_70df_5df2_b98a_56beb0601f79
);
impl windows_core::RuntimeType for IDragUIOverride {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDragUIOverride {
    pub(crate) fn SetCaption(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCaption)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsContentVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsContentVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsGlyphVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsGlyphVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IDragUIOverride_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Caption: usize,
    pub SetCaption: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    IsContentVisible: usize,
    pub SetIsContentVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    IsCaptionVisible: usize,
    SetIsCaptionVisible: usize,
    IsGlyphVisible: usize,
    pub SetIsGlyphVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
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
pub struct IElementCompositionPreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetElementVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetElementChildVisual: usize,
    pub SetElementChildVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsExpanded(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsExpanded)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Expanding<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Expanding)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveExpanding,
            ))
        }
    }
    pub(crate) fn Collapsed<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Collapsed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveCollapsed,
            ))
        }
    }
}
#[repr(C)]
pub struct IExpander_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    HeaderTemplateSelector: usize,
    SetHeaderTemplateSelector: usize,
    IsExpanded: usize,
    pub SetIsExpanded:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    ExpandDirection: usize,
    SetExpandDirection: usize,
    pub Expanding: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveExpanding:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Collapsed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveCollapsed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
pub struct IFlipView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFlyout_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Content: usize,
    pub SetContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetPlacement(&self, value: FlyoutPlacementMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPlacement)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFlyoutBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Placement: usize,
    pub SetPlacement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        FlyoutPlacementMode,
    ) -> windows_core::HRESULT,
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
pub struct IFontFamily_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    IFontIcon,
    IFontIcon_Vtbl,
    0x6eba5ed9_d233_5f5e_91a8_f5134292658a
);
impl windows_core::RuntimeType for IFontIcon {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFontIcon {
    pub(crate) fn SetGlyph(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetGlyph)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FontFamily>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontFamily)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFontIcon_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Glyph: usize,
    pub SetGlyph: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    FontSize: usize,
    SetFontSize: usize,
    FontFamily: usize,
    pub SetFontFamily: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFontIconFactory,
    IFontIconFactory_Vtbl,
    0xaa9a24fe_bef8_564a_b200_694cd6f6ba4e
);
impl windows_core::RuntimeType for IFontIconFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFontIconFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
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
    pub(crate) fn Resources(&self) -> windows_core::Result<ResourceDictionary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Resources)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Tag)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTag)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn ActualWidth(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActualWidth)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn ActualHeight(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActualHeight)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetWidth)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMinWidth)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaxWidth)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMinHeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaxHeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetHorizontalAlignment(
        &self,
        value: HorizontalAlignment,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHorizontalAlignment)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetVerticalAlignment(
        &self,
        value: VerticalAlignment,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetVerticalAlignment)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetMargin(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMargin)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Style>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetStyle)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetRequestedTheme(&self, value: ElementTheme) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRequestedTheme)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn ActualTheme(&self) -> windows_core::Result<ElementTheme> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActualTheme)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn Loaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Loaded)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveLoaded,
            ))
        }
    }
    pub(crate) fn SizeChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SizeChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveSizeChanged,
            ))
        }
    }
    pub(crate) fn ActualThemeChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ActualThemeChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveActualThemeChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IFrameworkElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Triggers: usize,
    pub Resources: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SetResources: usize,
    pub Tag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Language: usize,
    SetLanguage: usize,
    pub ActualWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ActualHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    Width: usize,
    pub SetWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    Height: usize,
    pub SetHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    MinWidth: usize,
    pub SetMinWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    MaxWidth: usize,
    pub SetMaxWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    MinHeight: usize,
    pub SetMinHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    MaxHeight: usize,
    pub SetMaxHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    HorizontalAlignment: usize,
    pub SetHorizontalAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HorizontalAlignment,
    ) -> windows_core::HRESULT,
    VerticalAlignment: usize,
    pub SetVerticalAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VerticalAlignment,
    ) -> windows_core::HRESULT,
    Margin: usize,
    pub SetMargin:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    Name: usize,
    SetName: usize,
    BaseUri: usize,
    DataContext: usize,
    SetDataContext: usize,
    AllowFocusOnInteraction: usize,
    SetAllowFocusOnInteraction: usize,
    FocusVisualMargin: usize,
    SetFocusVisualMargin: usize,
    FocusVisualSecondaryThickness: usize,
    SetFocusVisualSecondaryThickness: usize,
    FocusVisualPrimaryThickness: usize,
    SetFocusVisualPrimaryThickness: usize,
    FocusVisualSecondaryBrush: usize,
    SetFocusVisualSecondaryBrush: usize,
    FocusVisualPrimaryBrush: usize,
    SetFocusVisualPrimaryBrush: usize,
    AllowFocusWhenDisabled: usize,
    SetAllowFocusWhenDisabled: usize,
    Style: usize,
    pub SetStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Parent: usize,
    FlowDirection: usize,
    SetFlowDirection: usize,
    RequestedTheme: usize,
    pub SetRequestedTheme:
        unsafe extern "system" fn(*mut core::ffi::c_void, ElementTheme) -> windows_core::HRESULT,
    IsLoaded: usize,
    pub ActualTheme: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ElementTheme,
    ) -> windows_core::HRESULT,
    pub Loaded: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveLoaded:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Unloaded: usize,
    RemoveUnloaded: usize,
    DataContextChanged: usize,
    RemoveDataContextChanged: usize,
    pub SizeChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSizeChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    LayoutUpdated: usize,
    RemoveLayoutUpdated: usize,
    Loading: usize,
    RemoveLoading: usize,
    pub ActualThemeChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveActualThemeChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
pub struct IFrameworkTemplate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IGrid, IGrid_Vtbl, 0xc4496219_9014_58a1_b4ad_c5044913a5bb);
impl windows_core::RuntimeType for IGrid {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IGrid {
    pub(crate) fn RowDefinitions(&self) -> windows_core::Result<RowDefinitionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RowDefinitions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn ColumnDefinitions(&self) -> windows_core::Result<ColumnDefinitionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ColumnDefinitions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetPadding(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPadding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetRowSpacing(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRowSpacing)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetColumnSpacing(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetColumnSpacing)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IGrid_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RowDefinitions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ColumnDefinitions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    BackgroundSizing: usize,
    SetBackgroundSizing: usize,
    BorderBrush: usize,
    SetBorderBrush: usize,
    BorderThickness: usize,
    SetBorderThickness: usize,
    CornerRadius: usize,
    SetCornerRadius: usize,
    Padding: usize,
    pub SetPadding:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    RowSpacing: usize,
    pub SetRowSpacing:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    ColumnSpacing: usize,
    pub SetColumnSpacing:
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
pub struct IGridStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    BackgroundSizingProperty: usize,
    BorderBrushProperty: usize,
    BorderThicknessProperty: usize,
    CornerRadiusProperty: usize,
    PaddingProperty: usize,
    RowSpacingProperty: usize,
    ColumnSpacingProperty: usize,
    RowProperty: usize,
    GetRow: usize,
    pub SetRow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
    ) -> windows_core::HRESULT,
    ColumnProperty: usize,
    GetColumn: usize,
    pub SetColumn: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
    ) -> windows_core::HRESULT,
    RowSpanProperty: usize,
    GetRowSpan: usize,
    pub SetRowSpan: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
    ) -> windows_core::HRESULT,
    ColumnSpanProperty: usize,
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
    pub(crate) fn SetNavigateUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Uri>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetNavigateUri)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IHyperlinkButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    NavigateUri: usize,
    pub SetNavigateUri: unsafe extern "system" fn(
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
pub struct IIconElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IImage, IImage_Vtbl, 0x220d3d8d_66de_53a1_a215_ba9c165565ab);
impl windows_core::RuntimeType for IImage {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IImage {
    pub(crate) fn SetSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ImageSource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetStretch(&self, value: Stretch) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetStretch)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IImage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Source: usize,
    pub SetSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Stretch: usize,
    pub SetStretch:
        unsafe extern "system" fn(*mut core::ffi::c_void, Stretch) -> windows_core::HRESULT,
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
pub struct IImageSource_Vtbl {
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
    pub(crate) fn SetValue(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetValue)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IInfoBadge_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Value: usize,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
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
    pub(crate) fn SetIsOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetTitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetMessage(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMessage)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetSeverity(&self, value: InfoBarSeverity) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSeverity)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsClosable(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsClosable)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Closed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Closed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveClosed,
            ))
        }
    }
}
#[repr(C)]
pub struct IInfoBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsOpen: usize,
    pub SetIsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    Title: usize,
    pub SetTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Message: usize,
    pub SetMessage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Severity: usize,
    pub SetSeverity:
        unsafe extern "system" fn(*mut core::ffi::c_void, InfoBarSeverity) -> windows_core::HRESULT,
    IconSource: usize,
    SetIconSource: usize,
    IsIconVisible: usize,
    SetIsIconVisible: usize,
    IsClosable: usize,
    pub SetIsClosable:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    CloseButtonStyle: usize,
    SetCloseButtonStyle: usize,
    CloseButtonCommand: usize,
    SetCloseButtonCommand: usize,
    CloseButtonCommandParameter: usize,
    SetCloseButtonCommandParameter: usize,
    ActionButton: usize,
    SetActionButton: usize,
    Content: usize,
    SetContent: usize,
    ContentTemplate: usize,
    SetContentTemplate: usize,
    TemplateSettings: usize,
    CloseButtonClick: usize,
    RemoveCloseButtonClick: usize,
    Closing: usize,
    RemoveClosing: usize,
    pub Closed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveClosed:
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
pub struct IInfoBarClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
pub struct IItemContainer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn SetItemsSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetItemsSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn Items(&self) -> windows_core::Result<ItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetItemTemplate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DataTemplate>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetItemTemplate)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IItemsControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ItemsSource: usize,
    pub SetItemsSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ItemTemplate: usize,
    pub SetItemTemplate: unsafe extern "system" fn(
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
    pub(crate) fn SetKey(&self, value: VirtualKey) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetKey)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetModifiers(&self, value: VirtualKeyModifiers) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetModifiers)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Invoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Invoked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveInvoked,
            ))
        }
    }
}
#[repr(C)]
pub struct IKeyboardAccelerator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Key: usize,
    pub SetKey:
        unsafe extern "system" fn(*mut core::ffi::c_void, VirtualKey) -> windows_core::HRESULT,
    Modifiers: usize,
    pub SetModifiers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VirtualKeyModifiers,
    ) -> windows_core::HRESULT,
    IsEnabled: usize,
    SetIsEnabled: usize,
    ScopeOwner: usize,
    SetScopeOwner: usize,
    pub Invoked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveInvoked:
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
    pub(crate) fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHandled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IKeyboardAcceleratorInvokedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Handled: usize,
    pub SetHandled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
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
pub struct ILaunchActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ILine, ILine_Vtbl, 0x507b3858_af7e_559b_a87e_4cc6a5d8ba96);
impl windows_core::RuntimeType for ILine {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ILine {
    pub(crate) fn SetX1(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetX1)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetY1(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetY1)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetX2(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetX2)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetY2(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetY2)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ILine_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    X1: usize,
    pub SetX1: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    Y1: usize,
    pub SetY1: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    X2: usize,
    pub SetX2: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    Y2: usize,
    pub SetY2: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
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
#[repr(C)]
pub struct IListBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn SetSelectionMode(
        &self,
        value: ListViewSelectionMode,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSelectionMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetCanDragItems(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCanDragItems)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetCanReorderItems(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCanReorderItems)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn DragItemsCompleted<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<ListViewBase>, windows_core::Ref<DragItemsCompletedEventArgs>)
            + 'static,
    {
        let handler: TypedEventHandler<ListViewBase, DragItemsCompletedEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<ListViewBase, DragItemsCompletedEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<ListViewBase, DragItemsCompletedEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).DragItemsCompleted)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveDragItemsCompleted,
            ))
        }
    }
    pub(crate) fn ContainerContentChanging<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<ListViewBase>,
                windows_core::Ref<ContainerContentChangingEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<ListViewBase, ContainerContentChangingEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<ListViewBase, ContainerContentChangingEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<ListViewBase, ContainerContentChangingEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ContainerContentChanging)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveContainerContentChanging,
            ))
        }
    }
    pub(crate) fn ScrollIntoView<P0>(&self, item: P0) -> windows_core::Result<()>
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
}
#[repr(C)]
pub struct IListViewBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    SelectedItems: usize,
    SelectionMode: usize,
    pub SetSelectionMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ListViewSelectionMode,
    ) -> windows_core::HRESULT,
    IsSwipeEnabled: usize,
    SetIsSwipeEnabled: usize,
    CanDragItems: usize,
    pub SetCanDragItems:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    CanReorderItems: usize,
    pub SetCanReorderItems:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    IsItemClickEnabled: usize,
    SetIsItemClickEnabled: usize,
    DataFetchSize: usize,
    SetDataFetchSize: usize,
    IncrementalLoadingThreshold: usize,
    SetIncrementalLoadingThreshold: usize,
    IncrementalLoadingTrigger: usize,
    SetIncrementalLoadingTrigger: usize,
    ShowsScrollingPlaceholders: usize,
    SetShowsScrollingPlaceholders: usize,
    ReorderMode: usize,
    SetReorderMode: usize,
    SelectedRanges: usize,
    IsMultiSelectCheckBoxEnabled: usize,
    SetIsMultiSelectCheckBoxEnabled: usize,
    SingleSelectionFollowsFocus: usize,
    SetSingleSelectionFollowsFocus: usize,
    ItemClick: usize,
    RemoveItemClick: usize,
    DragItemsStarting: usize,
    RemoveDragItemsStarting: usize,
    pub DragItemsCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveDragItemsCompleted:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ContainerContentChanging: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveContainerContentChanging:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ChoosingItemContainer: usize,
    RemoveChoosingItemContainer: usize,
    ChoosingGroupHeaderContainer: usize,
    RemoveChoosingGroupHeaderContainer: usize,
    pub ScrollIntoView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
pub struct IListViewItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn Items(&self) -> windows_core::Result<windows_collections::IVector<MenuBarItem>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IMenuBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Items: unsafe extern "system" fn(
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
    pub(crate) fn SetTitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn Items(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<MenuFlyoutItemBase>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IMenuBarItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Title: usize,
    pub SetTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Items: unsafe extern "system" fn(
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
    pub(crate) fn Items(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<MenuFlyoutItemBase>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IMenuFlyout_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub(crate) fn SetText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Click)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveClick,
            ))
        }
    }
}
#[repr(C)]
pub struct IMenuFlyoutItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Command: usize,
    SetCommand: usize,
    CommandParameter: usize,
    SetCommandParameter: usize,
    Icon: usize,
    SetIcon: usize,
    KeyboardAcceleratorTextOverride: usize,
    SetKeyboardAcceleratorTextOverride: usize,
    TemplateSettings: usize,
    pub Click: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveClick:
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
    pub(crate) fn Items(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<MenuFlyoutItemBase>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IMenuFlyoutSubItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Text: usize,
    pub SetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetKind(&self, value: MicaKind) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetKind)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IMicaBackdrop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Kind: usize,
    pub SetKind:
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
    pub(crate) fn SetIsPaneOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsPaneOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetPaneFooter<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPaneFooter)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsSettingsVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsSettingsVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsPaneToggleButtonVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsPaneToggleButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetOpenPaneLength(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetOpenPaneLength)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetSelectedItem<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetSelectedItem)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn MenuItems(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MenuItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn AutoSuggestBox(&self) -> windows_core::Result<AutoSuggestBox> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoSuggestBox)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetAutoSuggestBox<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AutoSuggestBox>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetAutoSuggestBox)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SelectionChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveSelectionChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct INavigationView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsPaneOpen: usize,
    pub SetIsPaneOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    CompactModeThresholdWidth: usize,
    SetCompactModeThresholdWidth: usize,
    ExpandedModeThresholdWidth: usize,
    SetExpandedModeThresholdWidth: usize,
    FooterMenuItems: usize,
    FooterMenuItemsSource: usize,
    SetFooterMenuItemsSource: usize,
    PaneFooter: usize,
    pub SetPaneFooter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    DisplayMode: usize,
    IsSettingsVisible: usize,
    pub SetIsSettingsVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    IsPaneToggleButtonVisible: usize,
    pub SetIsPaneToggleButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    AlwaysShowHeader: usize,
    SetAlwaysShowHeader: usize,
    CompactPaneLength: usize,
    SetCompactPaneLength: usize,
    OpenPaneLength: usize,
    pub SetOpenPaneLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    PaneToggleButtonStyle: usize,
    SetPaneToggleButtonStyle: usize,
    SelectedItem: usize,
    pub SetSelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    MenuItemsSource: usize,
    SetMenuItemsSource: usize,
    SettingsItem: usize,
    pub AutoSuggestBox: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetAutoSuggestBox: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    MenuItemTemplate: usize,
    SetMenuItemTemplate: usize,
    MenuItemTemplateSelector: usize,
    SetMenuItemTemplateSelector: usize,
    MenuItemContainerStyle: usize,
    SetMenuItemContainerStyle: usize,
    MenuItemContainerStyleSelector: usize,
    SetMenuItemContainerStyleSelector: usize,
    MenuItemFromContainer: usize,
    ContainerFromMenuItem: usize,
    pub SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
    pub(crate) fn SetIsBackButtonVisible(
        &self,
        value: NavigationViewBackButtonVisible,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsBackButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsBackEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsBackEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetPaneTitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPaneTitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn BackRequested<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).BackRequested)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveBackRequested,
            ))
        }
    }
    pub(crate) fn SetPaneDisplayMode(
        &self,
        value: NavigationViewPaneDisplayMode,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPaneDisplayMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct INavigationView2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsBackButtonVisible: usize,
    pub SetIsBackButtonVisible: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        NavigationViewBackButtonVisible,
    ) -> windows_core::HRESULT,
    IsBackEnabled: usize,
    pub SetIsBackEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    PaneTitle: usize,
    pub SetPaneTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub BackRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveBackRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PaneClosed: usize,
    RemovePaneClosed: usize,
    PaneClosing: usize,
    RemovePaneClosing: usize,
    PaneOpened: usize,
    RemovePaneOpened: usize,
    PaneOpening: usize,
    RemovePaneOpening: usize,
    PaneDisplayMode: usize,
    pub SetPaneDisplayMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        NavigationViewPaneDisplayMode,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetIcon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IconElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetIcon)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct INavigationViewItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Icon: usize,
    pub SetIcon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn MenuItems(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MenuItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct INavigationViewItem2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    SelectsOnInvoked: usize,
    SetSelectsOnInvoked: usize,
    IsExpanded: usize,
    SetIsExpanded: usize,
    HasUnrealizedChildren: usize,
    SetHasUnrealizedChildren: usize,
    IsChildSelected: usize,
    SetIsChildSelected: usize,
    pub MenuItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct INavigationViewSelectionChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetMinimum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMinimum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetMaximum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaximum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetValue(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetValue)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn ValueChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ValueChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveValueChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct INumberBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Minimum: usize,
    pub SetMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    Maximum: usize,
    pub SetMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    Value: usize,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    SmallChange: usize,
    SetSmallChange: usize,
    LargeChange: usize,
    SetLargeChange: usize,
    Text: usize,
    SetText: usize,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    PlaceholderText: usize,
    SetPlaceholderText: usize,
    SelectionFlyout: usize,
    SetSelectionFlyout: usize,
    SelectionHighlightColor: usize,
    SetSelectionHighlightColor: usize,
    TextReadingOrder: usize,
    SetTextReadingOrder: usize,
    PreventKeyboardDisplayOnProgrammaticFocus: usize,
    SetPreventKeyboardDisplayOnProgrammaticFocus: usize,
    Description: usize,
    SetDescription: usize,
    ValidationMode: usize,
    SetValidationMode: usize,
    SpinButtonPlacementMode: usize,
    SetSpinButtonPlacementMode: usize,
    IsWrapEnabled: usize,
    SetIsWrapEnabled: usize,
    AcceptsExpression: usize,
    SetAcceptsExpression: usize,
    NumberFormatter: usize,
    SetNumberFormatter: usize,
    pub ValueChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveValueChanged:
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
    pub(crate) fn NewValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct INumberBoxValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    OldValue: usize,
    pub NewValue:
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
    pub(crate) fn SetPreferredMinimumHeight(&self, value: Option<i32>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<i32> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredMinimumHeight)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetPreferredMinimumWidth(&self, value: Option<i32>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<i32> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredMinimumWidth)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetPreferredMaximumWidth(&self, value: Option<i32>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<i32> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredMaximumWidth)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetPreferredMaximumHeight(&self, value: Option<i32>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<i32> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredMaximumHeight)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IOverlappedPresenter3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PreferredMinimumHeight: usize,
    pub SetPreferredMinimumHeight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    PreferredMinimumWidth: usize,
    pub SetPreferredMinimumWidth: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    PreferredMaximumWidth: usize,
    pub SetPreferredMaximumWidth: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    PreferredMaximumHeight: usize,
    pub SetPreferredMaximumHeight: unsafe extern "system" fn(
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
    pub(crate) fn Children(&self) -> windows_core::Result<UIElementCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Children)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetBackground)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IPanel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Children: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Background: usize,
    pub SetBackground: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn Inlines(&self) -> windows_core::Result<InlineCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Inlines)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IParagraph_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Inlines: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn Password(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Password)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub(crate) fn SetPassword(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPassword)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsPasswordRevealButtonEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsPasswordRevealButtonEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetPlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetPasswordRevealMode(
        &self,
        value: PasswordRevealMode,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPasswordRevealMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn PasswordChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PasswordChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemovePasswordChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IPasswordBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Password: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    PasswordChar: usize,
    SetPasswordChar: usize,
    IsPasswordRevealButtonEnabled: usize,
    pub SetIsPasswordRevealButtonEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    MaxLength: usize,
    SetMaxLength: usize,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    PlaceholderText: usize,
    pub SetPlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SelectionHighlightColor: usize,
    SetSelectionHighlightColor: usize,
    PreventKeyboardDisplayOnProgrammaticFocus: usize,
    SetPreventKeyboardDisplayOnProgrammaticFocus: usize,
    PasswordRevealMode: usize,
    pub SetPasswordRevealMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PasswordRevealMode,
    ) -> windows_core::HRESULT,
    TextReadingOrder: usize,
    SetTextReadingOrder: usize,
    InputScope: usize,
    SetInputScope: usize,
    CanPasteClipboardContent: usize,
    SelectionFlyout: usize,
    SetSelectionFlyout: usize,
    Description: usize,
    SetDescription: usize,
    pub PasswordChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePasswordChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
    pub(crate) fn SetDisplayName(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDisplayName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetInitials(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInitials)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IPersonPicture_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    BadgeNumber: usize,
    SetBadgeNumber: usize,
    BadgeGlyph: usize,
    SetBadgeGlyph: usize,
    BadgeImageSource: usize,
    SetBadgeImageSource: usize,
    BadgeText: usize,
    SetBadgeText: usize,
    IsGroup: usize,
    SetIsGroup: usize,
    Contact: usize,
    SetContact: usize,
    DisplayName: usize,
    pub SetDisplayName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Initials: usize,
    pub SetInitials: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetTitle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTitle)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetSelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSelectedIndex)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SelectionChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveSelectionChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IPivot_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Title: usize,
    pub SetTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    TitleTemplate: usize,
    SetTitleTemplate: usize,
    LeftHeader: usize,
    SetLeftHeader: usize,
    LeftHeaderTemplate: usize,
    SetLeftHeaderTemplate: usize,
    RightHeader: usize,
    SetRightHeader: usize,
    RightHeaderTemplate: usize,
    SetRightHeaderTemplate: usize,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    SelectedIndex: usize,
    pub SetSelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    SelectedItem: usize,
    SetSelectedItem: usize,
    IsLocked: usize,
    SetIsLocked: usize,
    HeaderFocusVisualPlacement: usize,
    SetHeaderFocusVisualPlacement: usize,
    IsHeaderItemsCarouselEnabled: usize,
    SetIsHeaderItemsCarouselEnabled: usize,
    pub SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IPivotItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
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
    pub(crate) fn Position(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn Properties(&self) -> windows_core::Result<PointerPointProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IPointerPoint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    FrameId: usize,
    IsInContact: usize,
    PointerDeviceType: usize,
    PointerId: usize,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn IsLeftButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLeftButtonPressed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn IsMiddleButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMiddleButtonPressed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn IsRightButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRightButtonPressed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IPointerPointProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ContactRect: usize,
    IsBarrelButtonPressed: usize,
    IsCanceled: usize,
    IsEraser: usize,
    IsHorizontalMouseWheel: usize,
    IsInRange: usize,
    IsInverted: usize,
    pub IsLeftButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMiddleButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    IsPrimary: usize,
    pub IsRightButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
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
    pub(crate) fn GetCurrentPoint<P0>(&self, relativeto: P0) -> windows_core::Result<PointerPoint>
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
pub struct IPointerRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Pointer: usize,
    KeyModifiers: usize,
    Handled: usize,
    SetHandled: usize,
    IsGenerated: usize,
    pub GetCurrentPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetIsIndeterminate(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsIndeterminate)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IProgressBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsIndeterminate: usize,
    pub SetIsIndeterminate:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
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
    pub(crate) fn SetIsActive(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsActive)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsIndeterminate(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsIndeterminate)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetValue(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetValue)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetMinimum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMinimum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetMaximum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaximum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IProgressRing_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsActive: usize,
    pub SetIsActive:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    IsIndeterminate: usize,
    pub SetIsIndeterminate:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    TemplateSettings: usize,
    Value: usize,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    Minimum: usize,
    pub SetMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    Maximum: usize,
    pub SetMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
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
    pub(crate) fn SetGroupName(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetGroupName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRadioButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    GroupName: usize,
    pub SetGroupName: unsafe extern "system" fn(
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
    pub(crate) fn Items(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SelectedIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn SetSelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSelectedIndex)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SelectionChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveSelectionChanged,
            ))
        }
    }
    pub(crate) fn SetMaxColumns(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaxColumns)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRadioButtons_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ItemsSource: usize,
    SetItemsSource: usize,
    pub Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ItemTemplate: usize,
    SetItemTemplate: usize,
    ContainerFromIndex: usize,
    pub SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    SelectedItem: usize,
    SetSelectedItem: usize,
    pub SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    MaxColumns: usize,
    pub SetMaxColumns:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetMinimum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMinimum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetMaximum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaximum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetSmallChange(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSmallChange)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetValue(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetValue)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn ValueChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ValueChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveValueChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IRangeBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Minimum: usize,
    pub SetMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    Maximum: usize,
    pub SetMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    SmallChange: usize,
    pub SetSmallChange:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    LargeChange: usize,
    SetLargeChange: usize,
    Value: usize,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ValueChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveValueChanged:
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
    pub(crate) fn NewValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IRangeBaseValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    OldValue: usize,
    pub NewValue:
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
    pub(crate) fn SetCaption(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCaption)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsReadOnly(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsReadOnly)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetMaxRating(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaxRating)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetPlaceholderValue(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPlaceholderValue)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Value(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn SetValue(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetValue)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn ValueChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ValueChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveValueChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IRatingControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Caption: usize,
    pub SetCaption: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    InitialSetValue: usize,
    SetInitialSetValue: usize,
    IsClearEnabled: usize,
    SetIsClearEnabled: usize,
    IsReadOnly: usize,
    pub SetIsReadOnly:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    MaxRating: usize,
    pub SetMaxRating:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    PlaceholderValue: usize,
    pub SetPlaceholderValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    ItemInfo: usize,
    SetItemInfo: usize,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ValueChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveValueChanged:
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
    pub(crate) fn SetRadiusX(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRadiusX)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetRadiusY(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRadiusY)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRectangle_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    RadiusX: usize,
    pub SetRadiusX: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    RadiusY: usize,
    pub SetRadiusY: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
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
pub struct IRelativePanel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
pub struct IRelativePanelStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    BackgroundSizingProperty: usize,
    LeftOfProperty: usize,
    GetLeftOf: usize,
    SetLeftOf: usize,
    AboveProperty: usize,
    GetAbove: usize,
    SetAbove: usize,
    RightOfProperty: usize,
    GetRightOf: usize,
    SetRightOf: usize,
    BelowProperty: usize,
    GetBelow: usize,
    SetBelow: usize,
    AlignHorizontalCenterWithProperty: usize,
    GetAlignHorizontalCenterWith: usize,
    SetAlignHorizontalCenterWith: usize,
    AlignVerticalCenterWithProperty: usize,
    GetAlignVerticalCenterWith: usize,
    SetAlignVerticalCenterWith: usize,
    AlignLeftWithProperty: usize,
    GetAlignLeftWith: usize,
    SetAlignLeftWith: usize,
    AlignTopWithProperty: usize,
    GetAlignTopWith: usize,
    SetAlignTopWith: usize,
    AlignRightWithProperty: usize,
    GetAlignRightWith: usize,
    SetAlignRightWith: usize,
    AlignBottomWithProperty: usize,
    GetAlignBottomWith: usize,
    SetAlignBottomWith: usize,
    AlignLeftWithPanelProperty: usize,
    GetAlignLeftWithPanel: usize,
    pub SetAlignLeftWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    AlignTopWithPanelProperty: usize,
    GetAlignTopWithPanel: usize,
    pub SetAlignTopWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    AlignRightWithPanelProperty: usize,
    GetAlignRightWithPanel: usize,
    pub SetAlignRightWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    AlignBottomWithPanelProperty: usize,
    GetAlignBottomWithPanel: usize,
    pub SetAlignBottomWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    AlignHorizontalCenterWithPanelProperty: usize,
    GetAlignHorizontalCenterWithPanel: usize,
    pub SetAlignHorizontalCenterWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    AlignVerticalCenterWithPanelProperty: usize,
    GetAlignVerticalCenterWithPanel: usize,
    pub SetAlignVerticalCenterWithPanel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetDelay(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDelay)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetInterval(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInterval)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRepeatButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Delay: usize,
    pub SetDelay: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    Interval: usize,
    pub SetInterval:
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
    pub(crate) fn MergedDictionaries(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<ResourceDictionary>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MergedDictionaries)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IResourceDictionary_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Source: usize,
    SetSource: usize,
    pub MergedDictionaries: unsafe extern "system" fn(
        *mut core::ffi::c_void,
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
    pub(crate) fn SetIsReadOnly(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsReadOnly)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Document(&self) -> windows_core::Result<RichEditTextDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Document)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetPlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn TextChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).TextChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveTextChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IRichEditBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsReadOnly: usize,
    pub SetIsReadOnly:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    AcceptsReturn: usize,
    SetAcceptsReturn: usize,
    TextAlignment: usize,
    SetTextAlignment: usize,
    TextWrapping: usize,
    SetTextWrapping: usize,
    IsSpellCheckEnabled: usize,
    SetIsSpellCheckEnabled: usize,
    IsTextPredictionEnabled: usize,
    SetIsTextPredictionEnabled: usize,
    pub Document: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    InputScope: usize,
    SetInputScope: usize,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    PlaceholderText: usize,
    pub SetPlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SelectionHighlightColor: usize,
    SetSelectionHighlightColor: usize,
    PreventKeyboardDisplayOnProgrammaticFocus: usize,
    SetPreventKeyboardDisplayOnProgrammaticFocus: usize,
    IsColorFontEnabled: usize,
    SetIsColorFontEnabled: usize,
    SelectionHighlightColorWhenNotFocused: usize,
    SetSelectionHighlightColorWhenNotFocused: usize,
    MaxLength: usize,
    SetMaxLength: usize,
    HorizontalTextAlignment: usize,
    SetHorizontalTextAlignment: usize,
    CharacterCasing: usize,
    SetCharacterCasing: usize,
    DisabledFormattingAccelerators: usize,
    SetDisabledFormattingAccelerators: usize,
    TextDocument: usize,
    SelectionFlyout: usize,
    SetSelectionFlyout: usize,
    ProofingMenuFlyout: usize,
    Description: usize,
    SetDescription: usize,
    pub TextChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveTextChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
    pub(crate) fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FontFamily>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontFamily)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetForeground)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetTextWrapping(&self, value: TextWrapping) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextWrapping)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Blocks(&self) -> windows_core::Result<BlockCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Blocks)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetPadding(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPadding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsTextSelectionEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsTextSelectionEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRichTextBlock_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    FontSize: usize,
    pub SetFontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    FontFamily: usize,
    pub SetFontFamily: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    FontWeight: usize,
    SetFontWeight: usize,
    FontStyle: usize,
    SetFontStyle: usize,
    FontStretch: usize,
    SetFontStretch: usize,
    Foreground: usize,
    pub SetForeground: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    TextWrapping: usize,
    pub SetTextWrapping:
        unsafe extern "system" fn(*mut core::ffi::c_void, TextWrapping) -> windows_core::HRESULT,
    TextTrimming: usize,
    SetTextTrimming: usize,
    TextAlignment: usize,
    SetTextAlignment: usize,
    pub Blocks: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Padding: usize,
    pub SetPadding:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    LineHeight: usize,
    SetLineHeight: usize,
    LineStackingStrategy: usize,
    SetLineStackingStrategy: usize,
    CharacterSpacing: usize,
    SetCharacterSpacing: usize,
    OverflowContentTarget: usize,
    SetOverflowContentTarget: usize,
    IsTextSelectionEnabled: usize,
    pub SetIsTextSelectionEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
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
pub struct IRightTappedRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
pub struct IRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn SetHeight(&self, value: GridLength) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRowDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Height: usize,
    pub SetHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, GridLength) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRun, IRun_Vtbl, 0x1f905239_37cb_590b_9132_3ffb7741906e);
impl windows_core::RuntimeType for IRun {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRun {
    pub(crate) fn SetText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRun_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Text: usize,
    pub SetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
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
    pub(crate) fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetHorizontalScrollBarVisibility(
        &self,
        value: ScrollingScrollBarVisibility,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHorizontalScrollBarVisibility)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetVerticalScrollBarVisibility(
        &self,
        value: ScrollingScrollBarVisibility,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetVerticalScrollBarVisibility)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IScrollView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Content: usize,
    pub SetContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CurrentAnchor: usize,
    ScrollPresenter: usize,
    ExpressionAnimationSources: usize,
    HorizontalOffset: usize,
    VerticalOffset: usize,
    ZoomFactor: usize,
    ExtentWidth: usize,
    ExtentHeight: usize,
    ViewportWidth: usize,
    ViewportHeight: usize,
    ScrollableWidth: usize,
    ScrollableHeight: usize,
    State: usize,
    HorizontalScrollBarVisibility: usize,
    pub SetHorizontalScrollBarVisibility: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ScrollingScrollBarVisibility,
    ) -> windows_core::HRESULT,
    VerticalScrollBarVisibility: usize,
    pub SetVerticalScrollBarVisibility: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ScrollingScrollBarVisibility,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetHorizontalScrollBarVisibility(
        &self,
        value: ScrollBarVisibility,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHorizontalScrollBarVisibility)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetVerticalScrollBarVisibility(
        &self,
        value: ScrollBarVisibility,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetVerticalScrollBarVisibility)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IScrollViewer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    HorizontalScrollBarVisibility: usize,
    pub SetHorizontalScrollBarVisibility: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ScrollBarVisibility,
    ) -> windows_core::HRESULT,
    VerticalScrollBarVisibility: usize,
    pub SetVerticalScrollBarVisibility: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ScrollBarVisibility,
    ) -> windows_core::HRESULT,
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
pub struct ISelectionChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn SelectedIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn SetSelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSelectedIndex)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SelectionChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveSelectionChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct ISelector_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    SelectedItem: usize,
    SetSelectedItem: usize,
    SelectedValue: usize,
    SetSelectedValue: usize,
    SelectedValuePath: usize,
    SetSelectedValuePath: usize,
    IsSynchronizedWithCurrentItem: usize,
    SetIsSynchronizedWithCurrentItem: usize,
    pub SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSelectionChanged:
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
    pub(crate) fn Items(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<SelectorBarItem>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SelectedItem(&self) -> windows_core::Result<SelectorBarItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SelectionChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveSelectionChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct ISelectorBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SetSelectedItem: usize,
    pub SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSelectionChanged:
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
    pub(crate) fn Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub(crate) fn SetText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetIcon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IconElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetIcon)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ISelectorBarItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Icon: usize,
    pub SetIcon: unsafe extern "system" fn(
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
pub struct ISelectorItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
pub struct ISetterBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IShape, IShape_Vtbl, 0x9941aad3_6af2_5ba2_9085_8506d5f2485e);
impl windows_core::RuntimeType for IShape {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IShape {
    pub(crate) fn SetFill<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFill)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetStroke<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetStroke)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetStrokeThickness(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetStrokeThickness)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IShape_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Fill: usize,
    pub SetFill: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Stroke: usize,
    pub SetStroke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    StrokeMiterLimit: usize,
    SetStrokeMiterLimit: usize,
    StrokeThickness: usize,
    pub SetStrokeThickness:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
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
    pub(crate) fn NewSize(&self) -> windows_core::Result<Size> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ISizeChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PreviousSize: usize,
    pub NewSize:
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
    pub(crate) fn SetStepFrequency(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetStepFrequency)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetOrientation(&self, value: Orientation) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetOrientation)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ISlider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IntermediateValue: usize,
    SetIntermediateValue: usize,
    StepFrequency: usize,
    pub SetStepFrequency:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    SnapsTo: usize,
    SetSnapsTo: usize,
    TickFrequency: usize,
    SetTickFrequency: usize,
    TickPlacement: usize,
    SetTickPlacement: usize,
    Orientation: usize,
    pub SetOrientation:
        unsafe extern "system" fn(*mut core::ffi::c_void, Orientation) -> windows_core::HRESULT,
    IsDirectionReversed: usize,
    SetIsDirectionReversed: usize,
    IsThumbToolTipEnabled: usize,
    SetIsThumbToolTipEnabled: usize,
    ThumbToolTipValueConverter: usize,
    SetThumbToolTipValueConverter: usize,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetColor(&self, value: Color) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetColor)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ISolidColorBrush_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Color: usize,
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, Color) -> windows_core::HRESULT,
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
    pub(crate) fn Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Click)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveClick,
            ))
        }
    }
}
#[repr(C)]
pub struct ISplitButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Flyout: usize,
    SetFlyout: usize,
    Command: usize,
    SetCommand: usize,
    CommandParameter: usize,
    SetCommandParameter: usize,
    pub Click: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveClick:
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
    pub(crate) fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetPane<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPane)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsPaneOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsPaneOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetOpenPaneLength(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetOpenPaneLength)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetCompactPaneLength(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCompactPaneLength)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetDisplayMode(&self, value: SplitViewDisplayMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDisplayMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn PaneClosed<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PaneClosed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemovePaneClosed,
            ))
        }
    }
}
#[repr(C)]
pub struct ISplitView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Content: usize,
    pub SetContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Pane: usize,
    pub SetPane: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    IsPaneOpen: usize,
    pub SetIsPaneOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    OpenPaneLength: usize,
    pub SetOpenPaneLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    CompactPaneLength: usize,
    pub SetCompactPaneLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    PanePlacement: usize,
    SetPanePlacement: usize,
    DisplayMode: usize,
    pub SetDisplayMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        SplitViewDisplayMode,
    ) -> windows_core::HRESULT,
    TemplateSettings: usize,
    PaneBackground: usize,
    SetPaneBackground: usize,
    LightDismissOverlayMode: usize,
    SetLightDismissOverlayMode: usize,
    PaneClosing: usize,
    RemovePaneClosing: usize,
    pub PaneClosed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePaneClosed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
    pub(crate) fn SetOrientation(&self, value: Orientation) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetOrientation)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetPadding(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPadding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetSpacing(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSpacing)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IStackPanel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    AreScrollSnapPointsRegular: usize,
    SetAreScrollSnapPointsRegular: usize,
    Orientation: usize,
    pub SetOrientation:
        unsafe extern "system" fn(*mut core::ffi::c_void, Orientation) -> windows_core::HRESULT,
    BackgroundSizing: usize,
    SetBackgroundSizing: usize,
    BorderBrush: usize,
    SetBorderBrush: usize,
    BorderThickness: usize,
    SetBorderThickness: usize,
    CornerRadius: usize,
    SetCornerRadius: usize,
    Padding: usize,
    pub SetPadding:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    Spacing: usize,
    pub SetSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
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
pub struct IStackPanelFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IStorageItem,
    IStorageItem_Vtbl,
    0x4207a996_ca2f_42f7_bde8_8b10457a7f30
);
impl windows_core::RuntimeType for IStorageItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IStorageItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IStorageItem {
    pub(crate) fn Name(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub(crate) fn Path(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub(crate) fn Attributes(&self) -> windows_core::Result<FileAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Attributes)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IStorageItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    RenameAsyncOverloadDefaultOptions: usize,
    RenameAsync: usize,
    DeleteAsyncOverloadDefaultOptions: usize,
    DeleteAsync: usize,
    GetBasicPropertiesAsync: usize,
    pub Name: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Attributes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FileAttributes,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStyle, IStyle_Vtbl, 0x65e1d164_572f_5b0e_a80f_9c02441fac49);
impl windows_core::RuntimeType for IStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStyle_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ISurfaceImageSource,
    ISurfaceImageSource_Vtbl,
    0xac078d9c_d0e0_5ff9_b73e_98e82e4c8d36
);
impl windows_core::RuntimeType for ISurfaceImageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISurfaceImageSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ISurfaceImageSourceFactory,
    ISurfaceImageSourceFactory_Vtbl,
    0x09a26ed2_11b3_5ef1_ac56_20d064ccca34
);
impl windows_core::RuntimeType for ISurfaceImageSourceFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISurfaceImageSourceFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithDimensions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        i32,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISurfaceImageSourceNativeWithD2D,
    ISurfaceImageSourceNativeWithD2D_Vtbl,
    0xcb833102_d5d1_448b_a31a_52a9509f24e6
);
windows_core::imp::interface_hierarchy!(ISurfaceImageSourceNativeWithD2D, windows_core::IUnknown);
impl ISurfaceImageSourceNativeWithD2D {
    pub(crate) unsafe fn SetDevice(&self, device: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetDevice)(
                windows_core::Interface::as_raw(self),
                device as _,
            )
        }
    }
    pub(crate) unsafe fn BeginDraw(
        &self,
        updaterect: *const RECT,
        iid: *const windows_core::GUID,
        updateobject: *mut *mut core::ffi::c_void,
        offset: *mut POINT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).BeginDraw)(
                windows_core::Interface::as_raw(self),
                updaterect,
                iid,
                updateobject as _,
                offset as _,
            )
        }
    }
    pub(crate) unsafe fn EndDraw(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn SuspendDraw(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SuspendDraw)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub(crate) unsafe fn ResumeDraw(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ResumeDraw)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
}
#[repr(C)]
pub struct ISurfaceImageSourceNativeWithD2D_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDevice: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub BeginDraw: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const RECT,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
        *mut POINT,
    ) -> windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
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
    pub(crate) fn CompositionScaleX(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompositionScaleX)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn CompositionScaleY(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompositionScaleY)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn CompositionScaleChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).CompositionScaleChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveCompositionScaleChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct ISwapChainPanel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CompositionScaleX:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub CompositionScaleY:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub CompositionScaleChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveCompositionScaleChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
    pub(crate) unsafe fn SetSwapChain(
        &self,
        swapchain: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetSwapChain)(
                windows_core::Interface::as_raw(self),
                swapchain as _,
            )
        }
    }
}
#[repr(C)]
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
#[repr(C)]
pub struct ISymbolIcon_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
pub struct ISystemBackdrop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn SetIsAddTabButtonVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsAddTabButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn TabCloseRequested<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).TabCloseRequested)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveTabCloseRequested,
            ))
        }
    }
    pub(crate) fn AddTabButtonClick<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).AddTabButtonClick)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveAddTabButtonClick,
            ))
        }
    }
    pub(crate) fn TabItems(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TabItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetCanReorderTabs(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCanReorderTabs)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SelectedIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn SetSelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSelectedIndex)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SelectionChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveSelectionChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct ITabView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    TabWidthMode: usize,
    SetTabWidthMode: usize,
    CloseButtonOverlayMode: usize,
    SetCloseButtonOverlayMode: usize,
    TabStripHeader: usize,
    SetTabStripHeader: usize,
    TabStripHeaderTemplate: usize,
    SetTabStripHeaderTemplate: usize,
    TabStripFooter: usize,
    SetTabStripFooter: usize,
    TabStripFooterTemplate: usize,
    SetTabStripFooterTemplate: usize,
    IsAddTabButtonVisible: usize,
    pub SetIsAddTabButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    AddTabButtonCommand: usize,
    SetAddTabButtonCommand: usize,
    AddTabButtonCommandParameter: usize,
    SetAddTabButtonCommandParameter: usize,
    pub TabCloseRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveTabCloseRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    TabDroppedOutside: usize,
    RemoveTabDroppedOutside: usize,
    pub AddTabButtonClick: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveAddTabButtonClick:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    TabItemsChanged: usize,
    RemoveTabItemsChanged: usize,
    TabItemsSource: usize,
    SetTabItemsSource: usize,
    pub TabItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    TabItemTemplate: usize,
    SetTabItemTemplate: usize,
    TabItemTemplateSelector: usize,
    SetTabItemTemplateSelector: usize,
    CanDragTabs: usize,
    SetCanDragTabs: usize,
    CanReorderTabs: usize,
    pub SetCanReorderTabs:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    AllowDropTabs: usize,
    SetAllowDropTabs: usize,
    pub SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    SelectedItem: usize,
    SetSelectedItem: usize,
    ContainerFromItem: usize,
    ContainerFromIndex: usize,
    pub SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsClosable(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsClosable)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ITabViewItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    IconSource: usize,
    SetIconSource: usize,
    IsClosable: usize,
    pub SetIsClosable:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
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
    pub(crate) fn Tab(&self) -> windows_core::Result<TabViewItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Tab)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ITabViewTabCloseRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Item: usize,
    pub Tab: unsafe extern "system" fn(
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
pub struct ITappedRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn SetTitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetSubtitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSubtitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetActionButtonContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetActionButtonContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetCloseButtonContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetCloseButtonContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsLightDismissEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsLightDismissEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetPreferredPlacement(
        &self,
        value: TeachingTipPlacementMode,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredPlacement)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn ActionButtonClick<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ActionButtonClick)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveActionButtonClick,
            ))
        }
    }
    pub(crate) fn Closed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Closed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveClosed,
            ))
        }
    }
}
#[repr(C)]
pub struct ITeachingTip_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Title: usize,
    pub SetTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Subtitle: usize,
    pub SetSubtitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    IsOpen: usize,
    pub SetIsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    Target: usize,
    SetTarget: usize,
    TailVisibility: usize,
    SetTailVisibility: usize,
    ActionButtonContent: usize,
    pub SetActionButtonContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ActionButtonStyle: usize,
    SetActionButtonStyle: usize,
    ActionButtonCommand: usize,
    SetActionButtonCommand: usize,
    ActionButtonCommandParameter: usize,
    SetActionButtonCommandParameter: usize,
    CloseButtonContent: usize,
    pub SetCloseButtonContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CloseButtonStyle: usize,
    SetCloseButtonStyle: usize,
    CloseButtonCommand: usize,
    SetCloseButtonCommand: usize,
    CloseButtonCommandParameter: usize,
    SetCloseButtonCommandParameter: usize,
    PlacementMargin: usize,
    SetPlacementMargin: usize,
    ShouldConstrainToRootBounds: usize,
    SetShouldConstrainToRootBounds: usize,
    IsLightDismissEnabled: usize,
    pub SetIsLightDismissEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    PreferredPlacement: usize,
    pub SetPreferredPlacement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TeachingTipPlacementMode,
    ) -> windows_core::HRESULT,
    HeroContentPlacement: usize,
    SetHeroContentPlacement: usize,
    HeroContent: usize,
    SetHeroContent: usize,
    IconSource: usize,
    SetIconSource: usize,
    TemplateSettings: usize,
    pub ActionButtonClick: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveActionButtonClick:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    CloseButtonClick: usize,
    RemoveCloseButtonClick: usize,
    Closing: usize,
    RemoveClosing: usize,
    pub Closed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveClosed:
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
pub struct ITeachingTipClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FontFamily>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontFamily)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetFontWeight(&self, value: FontWeight) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontWeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetForeground)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetTextWrapping(&self, value: TextWrapping) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextWrapping)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub(crate) fn SetText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetPadding(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPadding)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsTextSelectionEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsTextSelectionEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ITextBlock_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    FontSize: usize,
    pub SetFontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    FontFamily: usize,
    pub SetFontFamily: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    FontWeight: usize,
    pub SetFontWeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, FontWeight) -> windows_core::HRESULT,
    FontStyle: usize,
    SetFontStyle: usize,
    FontStretch: usize,
    SetFontStretch: usize,
    CharacterSpacing: usize,
    SetCharacterSpacing: usize,
    Foreground: usize,
    pub SetForeground: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    TextWrapping: usize,
    pub SetTextWrapping:
        unsafe extern "system" fn(*mut core::ffi::c_void, TextWrapping) -> windows_core::HRESULT,
    TextTrimming: usize,
    SetTextTrimming: usize,
    TextAlignment: usize,
    SetTextAlignment: usize,
    pub Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Inlines: usize,
    Padding: usize,
    pub SetPadding:
        unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    LineHeight: usize,
    SetLineHeight: usize,
    LineStackingStrategy: usize,
    SetLineStackingStrategy: usize,
    IsTextSelectionEnabled: usize,
    pub SetIsTextSelectionEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
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
    pub(crate) fn Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub(crate) fn SetText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetAcceptsReturn(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAcceptsReturn)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetTextWrapping(&self, value: TextWrapping) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextWrapping)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetPlaceholderText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPlaceholderText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn TextChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).TextChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveTextChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct ITextBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SelectedText: usize,
    SetSelectedText: usize,
    SelectionLength: usize,
    SetSelectionLength: usize,
    SelectionStart: usize,
    SetSelectionStart: usize,
    MaxLength: usize,
    SetMaxLength: usize,
    IsReadOnly: usize,
    SetIsReadOnly: usize,
    AcceptsReturn: usize,
    pub SetAcceptsReturn:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    TextAlignment: usize,
    SetTextAlignment: usize,
    TextWrapping: usize,
    pub SetTextWrapping:
        unsafe extern "system" fn(*mut core::ffi::c_void, TextWrapping) -> windows_core::HRESULT,
    IsSpellCheckEnabled: usize,
    SetIsSpellCheckEnabled: usize,
    IsTextPredictionEnabled: usize,
    SetIsTextPredictionEnabled: usize,
    InputScope: usize,
    SetInputScope: usize,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    PlaceholderText: usize,
    pub SetPlaceholderText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SelectionHighlightColor: usize,
    SetSelectionHighlightColor: usize,
    PreventKeyboardDisplayOnProgrammaticFocus: usize,
    SetPreventKeyboardDisplayOnProgrammaticFocus: usize,
    IsColorFontEnabled: usize,
    SetIsColorFontEnabled: usize,
    SelectionHighlightColorWhenNotFocused: usize,
    SetSelectionHighlightColorWhenNotFocused: usize,
    HorizontalTextAlignment: usize,
    SetHorizontalTextAlignment: usize,
    CharacterCasing: usize,
    SetCharacterCasing: usize,
    PlaceholderForeground: usize,
    SetPlaceholderForeground: usize,
    CanPasteClipboardContent: usize,
    CanUndo: usize,
    CanRedo: usize,
    SelectionFlyout: usize,
    SetSelectionFlyout: usize,
    ProofingMenuFlyout: usize,
    Description: usize,
    SetDescription: usize,
    pub TextChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveTextChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
    pub(crate) fn GetText(
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
    pub(crate) fn SetText(&self, options: TextSetOptions, value: &str) -> windows_core::Result<()> {
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
pub struct ITextDocument_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CaretType: usize,
    SetCaretType: usize,
    DefaultTabStop: usize,
    SetDefaultTabStop: usize,
    Selection: usize,
    UndoLimit: usize,
    SetUndoLimit: usize,
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
    pub(crate) fn SetFontWeight(&self, value: FontWeight) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontWeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ITextElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Name: usize,
    FontSize: usize,
    SetFontSize: usize,
    FontFamily: usize,
    SetFontFamily: usize,
    FontWeight: usize,
    pub SetFontWeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, FontWeight) -> windows_core::HRESULT,
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
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetClockIdentifier(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetClockIdentifier)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetMinuteIncrement(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMinuteIncrement)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SelectedTimeChanged<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SelectedTimeChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveSelectedTimeChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct ITimePicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    ClockIdentifier: usize,
    pub SetClockIdentifier: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    MinuteIncrement: usize,
    pub SetMinuteIncrement:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    Time: usize,
    SetTime: usize,
    LightDismissOverlayMode: usize,
    SetLightDismissOverlayMode: usize,
    SelectedTime: usize,
    SetSelectedTime: usize,
    TimeChanged: usize,
    RemoveTimeChanged: usize,
    pub SelectedTimeChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSelectedTimeChanged:
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
    pub(crate) fn NewTime(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<windows_time::TimeSpan>| r__.Value())
        }
    }
}
#[repr(C)]
pub struct ITimePickerSelectedValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    OldTime: usize,
    pub NewTime: unsafe extern "system" fn(
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
    pub(crate) fn SetTitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetSubtitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSubtitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetRightHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetRightHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsBackButtonVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsBackButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsBackButtonEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsBackButtonEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetIsPaneToggleButtonVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsPaneToggleButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn BackRequested<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).BackRequested)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveBackRequested,
            ))
        }
    }
    pub(crate) fn PaneToggleRequested<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PaneToggleRequested)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemovePaneToggleRequested,
            ))
        }
    }
}
#[repr(C)]
pub struct ITitleBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Title: usize,
    pub SetTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Subtitle: usize,
    pub SetSubtitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    IconSource: usize,
    SetIconSource: usize,
    LeftHeader: usize,
    SetLeftHeader: usize,
    Content: usize,
    pub SetContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    RightHeader: usize,
    pub SetRightHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    IsBackButtonVisible: usize,
    pub SetIsBackButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    IsBackButtonEnabled: usize,
    pub SetIsBackButtonEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    IsPaneToggleButtonVisible: usize,
    pub SetIsPaneToggleButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    TemplateSettings: usize,
    pub BackRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveBackRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PaneToggleRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePaneToggleRequested:
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
    pub(crate) fn SetIsChecked(&self, value: Option<bool>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<bool> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetIsChecked)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub(crate) fn Checked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Checked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveChecked,
            ))
        }
    }
    pub(crate) fn Unchecked<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Unchecked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveUnchecked,
            ))
        }
    }
}
#[repr(C)]
pub struct IToggleButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsChecked: usize,
    pub SetIsChecked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    IsThreeState: usize,
    SetIsThreeState: usize,
    pub Checked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveChecked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Unchecked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveUnchecked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
    pub(crate) fn IsOn(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOn)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn SetIsOn(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsOn)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetOnContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetOnContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetOffContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetOffContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn Toggled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Toggled)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveToggled,
            ))
        }
    }
}
#[repr(C)]
pub struct IToggleSwitch_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsOn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsOn: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    Header: usize,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    OnContent: usize,
    pub SetOnContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    OnContentTemplate: usize,
    SetOnContentTemplate: usize,
    OffContent: usize,
    pub SetOffContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    OffContentTemplate: usize,
    SetOffContentTemplate: usize,
    TemplateSettings: usize,
    pub Toggled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveToggled:
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
pub struct IToolTip_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
pub struct IToolTipServiceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PlacementProperty: usize,
    GetPlacement: usize,
    pub SetPlacement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        PlacementMode,
    ) -> windows_core::HRESULT,
    PlacementTargetProperty: usize,
    GetPlacementTarget: usize,
    SetPlacementTarget: usize,
    ToolTipProperty: usize,
    GetToolTip: usize,
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
    pub(crate) fn RootNodes(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<TreeViewNode>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RootNodes)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetSelectionMode(
        &self,
        value: TreeViewSelectionMode,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSelectionMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn ItemInvoked<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ItemInvoked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveItemInvoked,
            ))
        }
    }
}
#[repr(C)]
pub struct ITreeView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RootNodes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SelectionMode: usize,
    pub SetSelectionMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TreeViewSelectionMode,
    ) -> windows_core::HRESULT,
    SelectedNodes: usize,
    Expand: usize,
    Collapse: usize,
    SelectAll: usize,
    pub ItemInvoked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveItemInvoked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
    pub(crate) fn InvokedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InvokedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ITreeViewItemInvokedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InvokedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetIsExpanded(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsExpanded)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Children(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<TreeViewNode>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Children)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ITreeViewNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Parent: usize,
    IsExpanded: usize,
    pub SetIsExpanded:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    HasChildren: usize,
    Depth: usize,
    HasUnrealizedChildren: usize,
    SetHasUnrealizedChildren: usize,
    pub Children: unsafe extern "system" fn(
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
    pub(crate) fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAllowDrop)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetOpacity)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn KeyboardAccelerators(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<KeyboardAccelerator>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyboardAccelerators)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetKeyboardAcceleratorPlacementMode(
        &self,
        value: KeyboardAcceleratorPlacementMode,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetKeyboardAcceleratorPlacementMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn XamlRoot(&self) -> windows_core::Result<XamlRoot> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XamlRoot)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<XamlRoot>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetXamlRoot)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn DragEnter<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<DragEventArgs>)
            + 'static,
    {
        let handler: DragEventHandler = {
            let com = windows_core::imp::DelegateBox::<DragEventHandler, F>::new(
                &DragEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).DragEnter)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveDragEnter,
            ))
        }
    }
    pub(crate) fn DragLeave<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<DragEventArgs>)
            + 'static,
    {
        let handler: DragEventHandler = {
            let com = windows_core::imp::DelegateBox::<DragEventHandler, F>::new(
                &DragEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).DragLeave)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveDragLeave,
            ))
        }
    }
    pub(crate) fn DragOver<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<DragEventArgs>)
            + 'static,
    {
        let handler: DragEventHandler = {
            let com = windows_core::imp::DelegateBox::<DragEventHandler, F>::new(
                &DragEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).DragOver)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveDragOver,
            ))
        }
    }
    pub(crate) fn Drop<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<DragEventArgs>)
            + 'static,
    {
        let handler: DragEventHandler = {
            let com = windows_core::imp::DelegateBox::<DragEventHandler, F>::new(
                &DragEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Drop)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveDrop,
            ))
        }
    }
    pub(crate) fn PointerPressed<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerPressed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemovePointerPressed,
            ))
        }
    }
    pub(crate) fn PointerMoved<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerMoved)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemovePointerMoved,
            ))
        }
    }
    pub(crate) fn PointerReleased<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerReleased)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemovePointerReleased,
            ))
        }
    }
    pub(crate) fn PointerEntered<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerEntered)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemovePointerEntered,
            ))
        }
    }
    pub(crate) fn PointerExited<F>(
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerExited)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemovePointerExited,
            ))
        }
    }
    pub(crate) fn Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Tapped)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveTapped,
            ))
        }
    }
    pub(crate) fn RightTapped<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).RightTapped)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveRightTapped,
            ))
        }
    }
}
#[repr(C)]
pub struct IUIElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    DesiredSize: usize,
    AllowDrop: usize,
    pub SetAllowDrop:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    Opacity: usize,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    Clip: usize,
    SetClip: usize,
    RenderTransform: usize,
    SetRenderTransform: usize,
    Projection: usize,
    SetProjection: usize,
    Transform3D: usize,
    SetTransform3D: usize,
    RenderTransformOrigin: usize,
    SetRenderTransformOrigin: usize,
    IsHitTestVisible: usize,
    SetIsHitTestVisible: usize,
    Visibility: usize,
    SetVisibility: usize,
    RenderSize: usize,
    UseLayoutRounding: usize,
    SetUseLayoutRounding: usize,
    Transitions: usize,
    SetTransitions: usize,
    CacheMode: usize,
    SetCacheMode: usize,
    IsTapEnabled: usize,
    SetIsTapEnabled: usize,
    IsDoubleTapEnabled: usize,
    SetIsDoubleTapEnabled: usize,
    CanDrag: usize,
    SetCanDrag: usize,
    IsRightTapEnabled: usize,
    SetIsRightTapEnabled: usize,
    IsHoldingEnabled: usize,
    SetIsHoldingEnabled: usize,
    ManipulationMode: usize,
    SetManipulationMode: usize,
    PointerCaptures: usize,
    ContextFlyout: usize,
    SetContextFlyout: usize,
    CompositeMode: usize,
    SetCompositeMode: usize,
    Lights: usize,
    CanBeScrollAnchor: usize,
    SetCanBeScrollAnchor: usize,
    ExitDisplayModeOnAccessKeyInvoked: usize,
    SetExitDisplayModeOnAccessKeyInvoked: usize,
    IsAccessKeyScope: usize,
    SetIsAccessKeyScope: usize,
    AccessKeyScopeOwner: usize,
    SetAccessKeyScopeOwner: usize,
    AccessKey: usize,
    SetAccessKey: usize,
    KeyTipPlacementMode: usize,
    SetKeyTipPlacementMode: usize,
    KeyTipHorizontalOffset: usize,
    SetKeyTipHorizontalOffset: usize,
    KeyTipVerticalOffset: usize,
    SetKeyTipVerticalOffset: usize,
    KeyTipTarget: usize,
    SetKeyTipTarget: usize,
    XYFocusKeyboardNavigation: usize,
    SetXYFocusKeyboardNavigation: usize,
    XYFocusUpNavigationStrategy: usize,
    SetXYFocusUpNavigationStrategy: usize,
    XYFocusDownNavigationStrategy: usize,
    SetXYFocusDownNavigationStrategy: usize,
    XYFocusLeftNavigationStrategy: usize,
    SetXYFocusLeftNavigationStrategy: usize,
    XYFocusRightNavigationStrategy: usize,
    SetXYFocusRightNavigationStrategy: usize,
    pub KeyboardAccelerators: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    KeyboardAcceleratorPlacementTarget: usize,
    SetKeyboardAcceleratorPlacementTarget: usize,
    KeyboardAcceleratorPlacementMode: usize,
    pub SetKeyboardAcceleratorPlacementMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        KeyboardAcceleratorPlacementMode,
    )
        -> windows_core::HRESULT,
    HighContrastAdjustment: usize,
    SetHighContrastAdjustment: usize,
    TabFocusNavigation: usize,
    SetTabFocusNavigation: usize,
    OpacityTransition: usize,
    SetOpacityTransition: usize,
    Translation: usize,
    SetTranslation: usize,
    TranslationTransition: usize,
    SetTranslationTransition: usize,
    Rotation: usize,
    SetRotation: usize,
    RotationTransition: usize,
    SetRotationTransition: usize,
    Scale: usize,
    SetScale: usize,
    ScaleTransition: usize,
    SetScaleTransition: usize,
    TransformMatrix: usize,
    SetTransformMatrix: usize,
    CenterPoint: usize,
    SetCenterPoint: usize,
    RotationAxis: usize,
    SetRotationAxis: usize,
    ActualOffset: usize,
    ActualSize: usize,
    pub XamlRoot: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetXamlRoot: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Shadow: usize,
    SetShadow: usize,
    RasterizationScale: usize,
    SetRasterizationScale: usize,
    FocusState: usize,
    UseSystemFocusVisuals: usize,
    SetUseSystemFocusVisuals: usize,
    XYFocusLeft: usize,
    SetXYFocusLeft: usize,
    XYFocusRight: usize,
    SetXYFocusRight: usize,
    XYFocusUp: usize,
    SetXYFocusUp: usize,
    XYFocusDown: usize,
    SetXYFocusDown: usize,
    IsTabStop: usize,
    SetIsTabStop: usize,
    TabIndex: usize,
    SetTabIndex: usize,
    KeyUp: usize,
    RemoveKeyUp: usize,
    KeyDown: usize,
    RemoveKeyDown: usize,
    GotFocus: usize,
    RemoveGotFocus: usize,
    LostFocus: usize,
    RemoveLostFocus: usize,
    DragStarting: usize,
    RemoveDragStarting: usize,
    DropCompleted: usize,
    RemoveDropCompleted: usize,
    CharacterReceived: usize,
    RemoveCharacterReceived: usize,
    pub DragEnter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveDragEnter:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub DragLeave: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveDragLeave:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub DragOver: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveDragOver:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Drop: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveDrop: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerPressed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerMoved: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerMoved:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerReleased: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerReleased:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerEntered: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerEntered:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerExited: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerExited:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerCaptureLost: usize,
    RemovePointerCaptureLost: usize,
    PointerCanceled: usize,
    RemovePointerCanceled: usize,
    PointerWheelChanged: usize,
    RemovePointerWheelChanged: usize,
    pub Tapped: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveTapped:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    DoubleTapped: usize,
    RemoveDoubleTapped: usize,
    Holding: usize,
    RemoveHolding: usize,
    ContextRequested: usize,
    RemoveContextRequested: usize,
    ContextCanceled: usize,
    RemoveContextCanceled: usize,
    pub RightTapped: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveRightTapped:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
pub struct IUriRuntimeClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
pub struct IUriRuntimeClassFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
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
    pub(crate) fn SetChild<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetChild)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetStretch(&self, value: Stretch) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetStretch)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IViewbox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Child: usize,
    pub SetChild: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Stretch: usize,
    pub SetStretch:
        unsafe extern "system" fn(*mut core::ffi::c_void, Stretch) -> windows_core::HRESULT,
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
#[repr(C)]
pub struct IVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IWebView2,
    IWebView2_Vtbl,
    0x2b2c76c2_997c_5069_a8f0_9b84cd7e624b
);
impl windows_core::RuntimeType for IWebView2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebView2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IWebView2Factory,
    IWebView2Factory_Vtbl,
    0xfb4ec2ce_3074_5c42_b655_64fb81fbd040
);
impl windows_core::RuntimeType for IWebView2Factory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebView2Factory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub(crate) fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn SetTitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub(crate) fn SetExtendsContentIntoTitleBar(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetExtendsContentIntoTitleBar)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub(crate) fn Closed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
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
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Closed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveClosed,
            ))
        }
    }
    pub(crate) fn Activate(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub(crate) fn Close(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub(crate) fn SetTitleBar<P0>(&self, titlebar: P0) -> windows_core::Result<()>
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
pub struct IWindow_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Bounds: usize,
    Visible: usize,
    Content: usize,
    pub SetContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CoreWindow: usize,
    Compositor: usize,
    Dispatcher: usize,
    DispatcherQueue: usize,
    Title: usize,
    pub SetTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ExtendsContentIntoTitleBar: usize,
    pub SetExtendsContentIntoTitleBar:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    Activated: usize,
    RemoveActivated: usize,
    pub Closed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveClosed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    SizeChanged: usize,
    RemoveSizeChanged: usize,
    VisibilityChanged: usize,
    RemoveVisibilityChanged: usize,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
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
    pub(crate) fn SetSystemBackdrop<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SystemBackdrop>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetSystemBackdrop)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub(crate) fn AppWindow(&self) -> windows_core::Result<AppWindow> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppWindow)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IWindow2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    SystemBackdrop: usize,
    pub SetSystemBackdrop: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AppWindow: unsafe extern "system" fn(
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
#[repr(C)]
pub struct IWindowEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) unsafe fn WindowHandle(
        &self,
        hwnd: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).WindowHandle)(
                windows_core::Interface::as_raw(self),
                hwnd as _,
            )
        }
    }
}
#[repr(C)]
pub struct IWindowNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WindowHandle: unsafe extern "system" fn(
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
pub struct IXamlControlsResources_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn GetXamlType(&self, r#type: &TypeName) -> windows_core::Result<IXamlType> {
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
    pub(crate) fn GetXamlTypeByFullName(&self, fullname: &str) -> windows_core::Result<IXamlType> {
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
    pub(crate) fn GetXmlnsDefinitions(
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
                        result__.write(ok_data__);
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
pub struct IXamlReaderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Load: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
impl IXamlRoot {
    pub(crate) fn RasterizationScale(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RasterizationScale)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn Changed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<XamlRoot>, windows_core::Ref<XamlRootChangedEventArgs>) + 'static,
    {
        let handler: TypedEventHandler<XamlRoot, XamlRootChangedEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<XamlRoot, XamlRootChangedEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<XamlRoot, XamlRootChangedEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Changed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IXamlRoot_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Content: usize,
    Size: usize,
    pub RasterizationScale:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    IsHostVisible: usize,
    pub Changed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXamlRootChangedEventArgs,
    IXamlRootChangedEventArgs_Vtbl,
    0x61d2c719_f8a1_515a_902c_cfa498ba7a7f
);
impl windows_core::RuntimeType for IXamlRootChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXamlRootChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
#[repr(C)]
pub struct IXamlType_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Informational: Self = Self(0);
    pub const Success: Self = Self(1);
    pub const Warning: Self = Self(2);
    pub const Error: Self = Self(3);
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
pub struct KeyboardAccelerator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    KeyboardAccelerator,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(KeyboardAccelerator, DependencyObject);
impl KeyboardAccelerator {
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Auto: Self = Self(0);
    pub const Hidden: Self = Self(1);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const None: Self = Self(0);
    pub const Single: Self = Self(1);
    pub const Multiple: Self = Self(2);
    pub const Extended: Self = Self(3);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: RECT,
    pub rcWork: RECT,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MONITORINFOEXW {
    pub Base: MONITORINFO,
    pub szDevice: [u16; 32],
}
impl Default for MONITORINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MONITOR_DEFAULTTONEAREST: u32 = 2;
pub type MddBootstrapInitializeOptions = i32;
pub const MddBootstrapInitializeOptions_OnNoMatch_ShowUI: MddBootstrapInitializeOptions = 8;
pub const MddBootstrapInitializeOptions_OnPackageIdentity_NOOP: MddBootstrapInitializeOptions = 16;
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Base: Self = Self(0);
    pub const BaseAlt: Self = Self(1);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Collapsed: Self = Self(0);
    pub const Auto: Self = Self(2);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Auto: Self = Self(0);
    pub const Left: Self = Self(1);
    pub const Top: Self = Self(2);
    pub const LeftCompact: Self = Self(3);
    pub const LeftMinimal: Self = Self(4);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Vertical: Self = Self(0);
    pub const Horizontal: Self = Self(1);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PACKAGE_VERSION_0_0 {
    pub Revision: u16,
    pub Build: u16,
    pub Minor: u16,
    pub Major: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Panel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Panel, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Panel, FrameworkElement, UIElement, DependencyObject);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Peek: Self = Self(0);
    pub const Hidden: Self = Self(1);
    pub const Visible: Self = Self(2);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Bottom: Self = Self(2);
    pub const Left: Self = Self(9);
    pub const Mouse: Self = Self(7);
    pub const Right: Self = Self(4);
    pub const Top: Self = Self(10);
}
impl windows_core::TypeKind for PlacementMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlacementMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.Primitives.PlacementMode;i4)",
    );
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
impl windows_core::TypeKind for Point {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Point {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Point;f4;f4)");
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn SetAlignLeftWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
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
    pub(crate) fn SetAlignTopWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
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
    pub(crate) fn SetAlignRightWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
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
    pub(crate) fn SetAlignBottomWithPanel<P0>(element: P0, value: bool) -> windows_core::Result<()>
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
    pub(crate) fn SetAlignHorizontalCenterWithPanel<P0>(
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
    pub(crate) fn SetAlignVerticalCenterWithPanel<P0>(
        element: P0,
        value: bool,
    ) -> windows_core::Result<()>
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
pub const SWP_NOACTIVATE: u32 = 16;
pub const SWP_NOSIZE: u32 = 1;
pub const SWP_NOZORDER: u32 = 4;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ScrollBarVisibility(pub i32);
impl ScrollBarVisibility {
    pub const Disabled: Self = Self(0);
    pub const Auto: Self = Self(1);
    pub const Hidden: Self = Self(2);
    pub const Visible: Self = Self(3);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Auto: Self = Self(0);
    pub const Visible: Self = Self(1);
    pub const Hidden: Self = Self(2);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub width: f32,
    pub height: f32,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SizeInt32 {
    pub width: i32,
    pub height: i32,
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Overlay: Self = Self(0);
    pub const Inline: Self = Self(1);
    pub const CompactOverlay: Self = Self(2);
    pub const CompactInline: Self = Self(3);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const None: Self = Self(0);
    pub const Fill: Self = Self(1);
    pub const Uniform: Self = Self(2);
    pub const UniformToFill: Self = Self(3);
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
pub struct SurfaceImageSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SurfaceImageSource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(SurfaceImageSource, ImageSource, DependencyObject);
impl SurfaceImageSource {
    pub(crate) fn CreateInstanceWithDimensions(
        pixelwidth: i32,
        pixelheight: i32,
    ) -> windows_core::Result<Self> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithDimensions)(
                windows_core::Interface::as_raw(this),
                pixelwidth,
                pixelheight,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISurfaceImageSourceFactory<
        R,
        F: FnOnce(&ISurfaceImageSourceFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            SurfaceImageSource,
            ISurfaceImageSourceFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SurfaceImageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISurfaceImageSource>();
}
unsafe impl windows_core::Interface for SurfaceImageSource {
    type Vtable = <ISurfaceImageSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISurfaceImageSource as windows_core::Interface>::IID;
}
impl core::ops::Deref for SurfaceImageSource {
    type Target = ISurfaceImageSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SurfaceImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SurfaceImageSource";
}
unsafe impl Send for SurfaceImageSource {}
unsafe impl Sync for SurfaceImageSource {}
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Previous: Self = Self(57600);
    pub const Next: Self = Self(57601);
    pub const Play: Self = Self(57602);
    pub const Pause: Self = Self(57603);
    pub const Edit: Self = Self(57604);
    pub const Save: Self = Self(57605);
    pub const Clear: Self = Self(57606);
    pub const Delete: Self = Self(57607);
    pub const Remove: Self = Self(57608);
    pub const Add: Self = Self(57609);
    pub const Cancel: Self = Self(57610);
    pub const Accept: Self = Self(57611);
    pub const More: Self = Self(57612);
    pub const Redo: Self = Self(57613);
    pub const Undo: Self = Self(57614);
    pub const Home: Self = Self(57615);
    pub const Up: Self = Self(57616);
    pub const Forward: Self = Self(57617);
    pub const Back: Self = Self(57618);
    pub const Favorite: Self = Self(57619);
    pub const Camera: Self = Self(57620);
    pub const Setting: Self = Self(57621);
    pub const Video: Self = Self(57622);
    pub const Sync: Self = Self(57623);
    pub const Download: Self = Self(57624);
    pub const Mail: Self = Self(57625);
    pub const Find: Self = Self(57626);
    pub const Help: Self = Self(57627);
    pub const Upload: Self = Self(57628);
    pub const Emoji: Self = Self(57629);
    pub const TwoPage: Self = Self(57630);
    pub const LeaveChat: Self = Self(57631);
    pub const MailForward: Self = Self(57632);
    pub const Clock: Self = Self(57633);
    pub const Send: Self = Self(57634);
    pub const Crop: Self = Self(57635);
    pub const RotateCamera: Self = Self(57636);
    pub const People: Self = Self(57637);
    pub const OpenPane: Self = Self(57638);
    pub const ClosePane: Self = Self(57639);
    pub const World: Self = Self(57640);
    pub const Flag: Self = Self(57641);
    pub const PreviewLink: Self = Self(57642);
    pub const Globe: Self = Self(57643);
    pub const Trim: Self = Self(57644);
    pub const AttachCamera: Self = Self(57645);
    pub const ZoomIn: Self = Self(57646);
    pub const Bookmarks: Self = Self(57647);
    pub const Document: Self = Self(57648);
    pub const ProtectedDocument: Self = Self(57649);
    pub const Page: Self = Self(57650);
    pub const Bullets: Self = Self(57651);
    pub const Comment: Self = Self(57652);
    pub const MailFilled: Self = Self(57653);
    pub const ContactInfo: Self = Self(57654);
    pub const HangUp: Self = Self(57655);
    pub const ViewAll: Self = Self(57656);
    pub const MapPin: Self = Self(57657);
    pub const Phone: Self = Self(57658);
    pub const VideoChat: Self = Self(57659);
    pub const Switch: Self = Self(57660);
    pub const Contact: Self = Self(57661);
    pub const Rename: Self = Self(57662);
    pub const Pin: Self = Self(57665);
    pub const MusicInfo: Self = Self(57666);
    pub const Go: Self = Self(57667);
    pub const Keyboard: Self = Self(57668);
    pub const DockLeft: Self = Self(57669);
    pub const DockRight: Self = Self(57670);
    pub const DockBottom: Self = Self(57671);
    pub const Remote: Self = Self(57672);
    pub const Refresh: Self = Self(57673);
    pub const Rotate: Self = Self(57674);
    pub const Shuffle: Self = Self(57675);
    pub const List: Self = Self(57676);
    pub const Shop: Self = Self(57677);
    pub const SelectAll: Self = Self(57678);
    pub const Orientation: Self = Self(57679);
    pub const Import: Self = Self(57680);
    pub const ImportAll: Self = Self(57681);
    pub const BrowsePhotos: Self = Self(57685);
    pub const WebCam: Self = Self(57686);
    pub const Pictures: Self = Self(57688);
    pub const SaveLocal: Self = Self(57689);
    pub const Caption: Self = Self(57690);
    pub const Stop: Self = Self(57691);
    pub const ShowResults: Self = Self(57692);
    pub const Volume: Self = Self(57693);
    pub const Repair: Self = Self(57694);
    pub const Message: Self = Self(57695);
    pub const Page2: Self = Self(57696);
    pub const CalendarDay: Self = Self(57697);
    pub const CalendarWeek: Self = Self(57698);
    pub const Calendar: Self = Self(57699);
    pub const Character: Self = Self(57700);
    pub const MailReplyAll: Self = Self(57701);
    pub const Read: Self = Self(57702);
    pub const Link: Self = Self(57703);
    pub const Account: Self = Self(57704);
    pub const ShowBcc: Self = Self(57705);
    pub const HideBcc: Self = Self(57706);
    pub const Cut: Self = Self(57707);
    pub const Attach: Self = Self(57708);
    pub const Paste: Self = Self(57709);
    pub const Filter: Self = Self(57710);
    pub const Copy: Self = Self(57711);
    pub const Emoji2: Self = Self(57712);
    pub const Important: Self = Self(57713);
    pub const MailReply: Self = Self(57714);
    pub const SlideShow: Self = Self(57715);
    pub const Sort: Self = Self(57716);
    pub const Manage: Self = Self(57720);
    pub const AllApps: Self = Self(57721);
    pub const DisconnectDrive: Self = Self(57722);
    pub const MapDrive: Self = Self(57723);
    pub const NewWindow: Self = Self(57724);
    pub const OpenWith: Self = Self(57725);
    pub const ContactPresence: Self = Self(57729);
    pub const Priority: Self = Self(57730);
    pub const GoToToday: Self = Self(57732);
    pub const Font: Self = Self(57733);
    pub const FontColor: Self = Self(57734);
    pub const Contact2: Self = Self(57735);
    pub const Folder: Self = Self(57736);
    pub const Audio: Self = Self(57737);
    pub const Placeholder: Self = Self(57738);
    pub const View: Self = Self(57739);
    pub const SetLockScreen: Self = Self(57740);
    pub const SetTile: Self = Self(57741);
    pub const ClosedCaption: Self = Self(57744);
    pub const StopSlideShow: Self = Self(57745);
    pub const Permissions: Self = Self(57746);
    pub const Highlight: Self = Self(57747);
    pub const DisableUpdates: Self = Self(57748);
    pub const UnFavorite: Self = Self(57749);
    pub const UnPin: Self = Self(57750);
    pub const OpenLocal: Self = Self(57751);
    pub const Mute: Self = Self(57752);
    pub const Italic: Self = Self(57753);
    pub const Underline: Self = Self(57754);
    pub const Bold: Self = Self(57755);
    pub const MoveToFolder: Self = Self(57756);
    pub const LikeDislike: Self = Self(57757);
    pub const Dislike: Self = Self(57758);
    pub const Like: Self = Self(57759);
    pub const AlignRight: Self = Self(57760);
    pub const AlignCenter: Self = Self(57761);
    pub const AlignLeft: Self = Self(57762);
    pub const Zoom: Self = Self(57763);
    pub const ZoomOut: Self = Self(57764);
    pub const OpenFile: Self = Self(57765);
    pub const OtherUser: Self = Self(57766);
    pub const Admin: Self = Self(57767);
    pub const Street: Self = Self(57795);
    pub const Map: Self = Self(57796);
    pub const ClearSelection: Self = Self(57797);
    pub const FontDecrease: Self = Self(57798);
    pub const FontIncrease: Self = Self(57799);
    pub const FontSize: Self = Self(57800);
    pub const CellPhone: Self = Self(57801);
    pub const ReShare: Self = Self(57802);
    pub const Tag: Self = Self(57803);
    pub const RepeatOne: Self = Self(57804);
    pub const RepeatAll: Self = Self(57805);
    pub const OutlineStar: Self = Self(57806);
    pub const SolidStar: Self = Self(57807);
    pub const Calculator: Self = Self(57808);
    pub const Directions: Self = Self(57809);
    pub const Target: Self = Self(57810);
    pub const Library: Self = Self(57811);
    pub const PhoneBook: Self = Self(57812);
    pub const Memo: Self = Self(57813);
    pub const Microphone: Self = Self(57814);
    pub const PostUpdate: Self = Self(57815);
    pub const BackToWindow: Self = Self(57816);
    pub const FullScreen: Self = Self(57817);
    pub const NewFolder: Self = Self(57818);
    pub const CalendarReply: Self = Self(57819);
    pub const UnSyncFolder: Self = Self(57821);
    pub const ReportHacked: Self = Self(57822);
    pub const SyncFolder: Self = Self(57823);
    pub const BlockContact: Self = Self(57824);
    pub const SwitchApps: Self = Self(57825);
    pub const AddFriend: Self = Self(57826);
    pub const TouchPointer: Self = Self(57827);
    pub const GoToStart: Self = Self(57828);
    pub const ZeroBars: Self = Self(57829);
    pub const OneBar: Self = Self(57830);
    pub const TwoBars: Self = Self(57831);
    pub const ThreeBars: Self = Self(57832);
    pub const FourBars: Self = Self(57833);
    pub const Scan: Self = Self(58004);
    pub const Preview: Self = Self(58005);
    pub const GlobalNavigationButton: Self = Self(59136);
    pub const Share: Self = Self(59181);
    pub const Print: Self = Self(59209);
    pub const XboxOneConsole: Self = Self(59792);
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
    pub(crate) fn CreateInstanceWithSymbol(symbol: Symbol) -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Auto: Self = Self(0);
    pub const Top: Self = Self(1);
    pub const Bottom: Self = Self(2);
    pub const Left: Self = Self(3);
    pub const Right: Self = Self(4);
    pub const TopRight: Self = Self(5);
    pub const TopLeft: Self = Self(6);
    pub const BottomRight: Self = Self(7);
    pub const BottomLeft: Self = Self(8);
    pub const LeftTop: Self = Self(9);
    pub const LeftBottom: Self = Self(10);
    pub const RightTop: Self = Self(11);
    pub const RightBottom: Self = Self(12);
    pub const Center: Self = Self(13);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const None: Self = Self(0);
    pub const AdjustCrlf: Self = Self(1);
    pub const UseCrlf: Self = Self(2);
    pub const UseObjectText: Self = Self(4);
    pub const AllowFinalEop: Self = Self(8);
    pub const NoHidden: Self = Self(32);
    pub const IncludeNumbering: Self = Self(64);
    pub const FormatRtf: Self = Self(8192);
    pub const UseLf: Self = Self(16777216);
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
    pub const None: Self = Self(0);
    pub const UnicodeBidi: Self = Self(1);
    pub const Unlink: Self = Self(8);
    pub const Unhide: Self = Self(16);
    pub const CheckTextLimit: Self = Self(32);
    pub const FormatRtf: Self = Self(8192);
    pub const ApplyRtfDocumentDefaults: Self = Self(16384);
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
    pub const NoWrap: Self = Self(1);
    pub const Wrap: Self = Self(2);
    pub const WrapWholeWords: Self = Self(3);
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
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const Standard: Self = Self(0);
    pub const Tall: Self = Self(1);
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
    pub const UseDefaultAppMode: Self = Self(1);
    pub const Light: Self = Self(2);
    pub const Dark: Self = Self(3);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn SetPlacement<P0>(element: P0, value: PlacementMode) -> windows_core::Result<()>
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
    pub(crate) fn SetToolTip<P0, P1>(element: P0, value: P1) -> windows_core::Result<()>
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const None: Self = Self(0);
    pub const Single: Self = Self(1);
    pub const Multiple: Self = Self(2);
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
    pub const Primitive: Self = Self(0);
    pub const Metadata: Self = Self(1);
    pub const Custom: Self = Self(2);
}
impl windows_core::TypeKind for TypeKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TypeKind {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Interop.TypeKind;i4)");
}
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TypeName {
    pub name: windows_core::HSTRING,
    pub kind: TypeKind,
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
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static>
    TypedEventHandler<TSender, TResult>
{
    pub(crate) fn new<F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) + 'static>(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &TypedEventHandlerBox::<TSender, TResult, F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
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
    pub(crate) fn CreateUri(uri: &str) -> windows_core::Result<Self> {
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VerticalAlignment(pub i32);
impl VerticalAlignment {
    pub const Top: Self = Self(0);
    pub const Center: Self = Self(1);
    pub const Bottom: Self = Self(2);
    pub const Stretch: Self = Self(3);
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub const None: Self = Self(0);
    pub const LeftButton: Self = Self(1);
    pub const RightButton: Self = Self(2);
    pub const Cancel: Self = Self(3);
    pub const MiddleButton: Self = Self(4);
    pub const XButton1: Self = Self(5);
    pub const XButton2: Self = Self(6);
    pub const Back: Self = Self(8);
    pub const Tab: Self = Self(9);
    pub const Clear: Self = Self(12);
    pub const Enter: Self = Self(13);
    pub const Shift: Self = Self(16);
    pub const Control: Self = Self(17);
    pub const Menu: Self = Self(18);
    pub const Pause: Self = Self(19);
    pub const CapitalLock: Self = Self(20);
    pub const Kana: Self = Self(21);
    pub const Hangul: Self = Self(21);
    pub const ImeOn: Self = Self(22);
    pub const Junja: Self = Self(23);
    pub const Final: Self = Self(24);
    pub const Hanja: Self = Self(25);
    pub const Kanji: Self = Self(25);
    pub const ImeOff: Self = Self(26);
    pub const Escape: Self = Self(27);
    pub const Convert: Self = Self(28);
    pub const NonConvert: Self = Self(29);
    pub const Accept: Self = Self(30);
    pub const ModeChange: Self = Self(31);
    pub const Space: Self = Self(32);
    pub const PageUp: Self = Self(33);
    pub const PageDown: Self = Self(34);
    pub const End: Self = Self(35);
    pub const Home: Self = Self(36);
    pub const Left: Self = Self(37);
    pub const Up: Self = Self(38);
    pub const Right: Self = Self(39);
    pub const Down: Self = Self(40);
    pub const Select: Self = Self(41);
    pub const Print: Self = Self(42);
    pub const Execute: Self = Self(43);
    pub const Snapshot: Self = Self(44);
    pub const Insert: Self = Self(45);
    pub const Delete: Self = Self(46);
    pub const Help: Self = Self(47);
    pub const Number0: Self = Self(48);
    pub const Number1: Self = Self(49);
    pub const Number2: Self = Self(50);
    pub const Number3: Self = Self(51);
    pub const Number4: Self = Self(52);
    pub const Number5: Self = Self(53);
    pub const Number6: Self = Self(54);
    pub const Number7: Self = Self(55);
    pub const Number8: Self = Self(56);
    pub const Number9: Self = Self(57);
    pub const A: Self = Self(65);
    pub const B: Self = Self(66);
    pub const C: Self = Self(67);
    pub const D: Self = Self(68);
    pub const E: Self = Self(69);
    pub const F: Self = Self(70);
    pub const G: Self = Self(71);
    pub const H: Self = Self(72);
    pub const I: Self = Self(73);
    pub const J: Self = Self(74);
    pub const K: Self = Self(75);
    pub const L: Self = Self(76);
    pub const M: Self = Self(77);
    pub const N: Self = Self(78);
    pub const O: Self = Self(79);
    pub const P: Self = Self(80);
    pub const Q: Self = Self(81);
    pub const R: Self = Self(82);
    pub const S: Self = Self(83);
    pub const T: Self = Self(84);
    pub const U: Self = Self(85);
    pub const V: Self = Self(86);
    pub const W: Self = Self(87);
    pub const X: Self = Self(88);
    pub const Y: Self = Self(89);
    pub const Z: Self = Self(90);
    pub const LeftWindows: Self = Self(91);
    pub const RightWindows: Self = Self(92);
    pub const Application: Self = Self(93);
    pub const Sleep: Self = Self(95);
    pub const NumberPad0: Self = Self(96);
    pub const NumberPad1: Self = Self(97);
    pub const NumberPad2: Self = Self(98);
    pub const NumberPad3: Self = Self(99);
    pub const NumberPad4: Self = Self(100);
    pub const NumberPad5: Self = Self(101);
    pub const NumberPad6: Self = Self(102);
    pub const NumberPad7: Self = Self(103);
    pub const NumberPad8: Self = Self(104);
    pub const NumberPad9: Self = Self(105);
    pub const Multiply: Self = Self(106);
    pub const Add: Self = Self(107);
    pub const Separator: Self = Self(108);
    pub const Subtract: Self = Self(109);
    pub const Decimal: Self = Self(110);
    pub const Divide: Self = Self(111);
    pub const F1: Self = Self(112);
    pub const F2: Self = Self(113);
    pub const F3: Self = Self(114);
    pub const F4: Self = Self(115);
    pub const F5: Self = Self(116);
    pub const F6: Self = Self(117);
    pub const F7: Self = Self(118);
    pub const F8: Self = Self(119);
    pub const F9: Self = Self(120);
    pub const F10: Self = Self(121);
    pub const F11: Self = Self(122);
    pub const F12: Self = Self(123);
    pub const F13: Self = Self(124);
    pub const F14: Self = Self(125);
    pub const F15: Self = Self(126);
    pub const F16: Self = Self(127);
    pub const F17: Self = Self(128);
    pub const F18: Self = Self(129);
    pub const F19: Self = Self(130);
    pub const F20: Self = Self(131);
    pub const F21: Self = Self(132);
    pub const F22: Self = Self(133);
    pub const F23: Self = Self(134);
    pub const F24: Self = Self(135);
    pub const NavigationView: Self = Self(136);
    pub const NavigationMenu: Self = Self(137);
    pub const NavigationUp: Self = Self(138);
    pub const NavigationDown: Self = Self(139);
    pub const NavigationLeft: Self = Self(140);
    pub const NavigationRight: Self = Self(141);
    pub const NavigationAccept: Self = Self(142);
    pub const NavigationCancel: Self = Self(143);
    pub const NumberKeyLock: Self = Self(144);
    pub const Scroll: Self = Self(145);
    pub const LeftShift: Self = Self(160);
    pub const RightShift: Self = Self(161);
    pub const LeftControl: Self = Self(162);
    pub const RightControl: Self = Self(163);
    pub const LeftMenu: Self = Self(164);
    pub const RightMenu: Self = Self(165);
    pub const GoBack: Self = Self(166);
    pub const GoForward: Self = Self(167);
    pub const Refresh: Self = Self(168);
    pub const Stop: Self = Self(169);
    pub const Search: Self = Self(170);
    pub const Favorites: Self = Self(171);
    pub const GoHome: Self = Self(172);
    pub const GamepadA: Self = Self(195);
    pub const GamepadB: Self = Self(196);
    pub const GamepadX: Self = Self(197);
    pub const GamepadY: Self = Self(198);
    pub const GamepadRightShoulder: Self = Self(199);
    pub const GamepadLeftShoulder: Self = Self(200);
    pub const GamepadLeftTrigger: Self = Self(201);
    pub const GamepadRightTrigger: Self = Self(202);
    pub const GamepadDPadUp: Self = Self(203);
    pub const GamepadDPadDown: Self = Self(204);
    pub const GamepadDPadLeft: Self = Self(205);
    pub const GamepadDPadRight: Self = Self(206);
    pub const GamepadMenu: Self = Self(207);
    pub const GamepadView: Self = Self(208);
    pub const GamepadLeftThumbstickButton: Self = Self(209);
    pub const GamepadRightThumbstickButton: Self = Self(210);
    pub const GamepadLeftThumbstickUp: Self = Self(211);
    pub const GamepadLeftThumbstickDown: Self = Self(212);
    pub const GamepadLeftThumbstickRight: Self = Self(213);
    pub const GamepadLeftThumbstickLeft: Self = Self(214);
    pub const GamepadRightThumbstickUp: Self = Self(215);
    pub const GamepadRightThumbstickDown: Self = Self(216);
    pub const GamepadRightThumbstickRight: Self = Self(217);
    pub const GamepadRightThumbstickLeft: Self = Self(218);
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
    pub const None: Self = Self(0);
    pub const Control: Self = Self(1);
    pub const Menu: Self = Self(2);
    pub const Shift: Self = Self(4);
    pub const Windows: Self = Self(8);
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
pub const WINDOWSAPPSDK_RELEASE_MAJORMINOR: i32 = 131072;
pub const WINDOWSAPPSDK_RELEASE_VERSION_TAG_W: windows_core::PCWSTR = windows_core::w!("");
pub const WINDOWSAPPSDK_RUNTIME_VERSION_UINT64: u64 = 562949953486848;
pub const WM_MOUSEMOVE: u32 = 512;
pub const WM_SETCURSOR: u32 = 32;
pub type WPARAM = usize;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebView2(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WebView2,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(WebView2, FrameworkElement, UIElement, DependencyObject);
impl WebView2 {
    pub(crate) fn new() -> windows_core::Result<Self> {
        Self::IWebView2Factory(|this| unsafe {
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
    fn IWebView2Factory<R, F: FnOnce(&IWebView2Factory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebView2, IWebView2Factory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WebView2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWebView2>();
}
unsafe impl windows_core::Interface for WebView2 {
    type Vtable = <IWebView2 as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebView2 as windows_core::Interface>::IID;
}
impl core::ops::Deref for WebView2 {
    type Target = IWebView2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for WebView2 {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.WebView2";
}
unsafe impl Send for WebView2 {}
unsafe impl Sync for WebView2 {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Window(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Window, windows_core::IUnknown, windows_core::IInspectable);
impl Window {
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn new() -> windows_core::Result<Self> {
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
    pub(crate) fn Load(xaml: &str) -> windows_core::Result<windows_core::IInspectable> {
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
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlRootChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlRootChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for XamlRootChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IXamlRootChangedEventArgs>();
}
unsafe impl windows_core::Interface for XamlRootChangedEventArgs {
    type Vtable = <IXamlRootChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlRootChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for XamlRootChangedEventArgs {
    type Target = IXamlRootChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for XamlRootChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlRootChangedEventArgs";
}
unsafe impl Send for XamlRootChangedEventArgs {}
unsafe impl Sync for XamlRootChangedEventArgs {}
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct XmlnsDefinition {
    pub xml_namespace: windows_core::HSTRING,
    pub namespace: windows_core::HSTRING,
}
impl windows_core::TypeKind for XmlnsDefinition {
    type TypeKind = windows_core::CloneType;
}
impl windows_core::RuntimeType for XmlnsDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Markup.XmlnsDefinition;string;string)",
    );
}
