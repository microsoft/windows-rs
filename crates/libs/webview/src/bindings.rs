windows_core::link!("webview2loader.dll" "C" fn CreateCoreWebView2Environment(environmentcreatedhandler : *mut core::ffi::c_void) -> windows_core::HRESULT);
pub type HWND = *mut core::ffi::c_void;
windows_core::imp::define_interface!(
    ICoreWebView2,
    ICoreWebView2_Vtbl,
    0x76eceacb_0462_4d94_ac83_423a6793775e
);
windows_core::imp::interface_hierarchy!(ICoreWebView2, windows_core::IUnknown);
impl ICoreWebView2 {
    pub unsafe fn Navigate(&self, uri: LPCWSTR) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Navigate)(
                windows_core::Interface::as_raw(self),
                uri,
            )
            .ok()
        }
    }
    pub unsafe fn ExecuteScript<P1>(
        &self,
        javascript: LPCWSTR,
        handler: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ICoreWebView2ExecuteScriptCompletedHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ExecuteScript)(
                windows_core::Interface::as_raw(self),
                javascript,
                handler.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Settings: usize,
    Source: usize,
    pub Navigate:
        unsafe extern "system" fn(*mut core::ffi::c_void, LPCWSTR) -> windows_core::HRESULT,
    NavigateToString: usize,
    add_NavigationStarting: usize,
    remove_NavigationStarting: usize,
    add_ContentLoading: usize,
    remove_ContentLoading: usize,
    add_SourceChanged: usize,
    remove_SourceChanged: usize,
    add_HistoryChanged: usize,
    remove_HistoryChanged: usize,
    add_NavigationCompleted: usize,
    remove_NavigationCompleted: usize,
    add_FrameNavigationStarting: usize,
    remove_FrameNavigationStarting: usize,
    add_FrameNavigationCompleted: usize,
    remove_FrameNavigationCompleted: usize,
    add_ScriptDialogOpening: usize,
    remove_ScriptDialogOpening: usize,
    add_PermissionRequested: usize,
    remove_PermissionRequested: usize,
    add_ProcessFailed: usize,
    remove_ProcessFailed: usize,
    AddScriptToExecuteOnDocumentCreated: usize,
    RemoveScriptToExecuteOnDocumentCreated: usize,
    pub ExecuteScript: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        LPCWSTR,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CapturePreview: usize,
    Reload: usize,
    PostWebMessageAsJson: usize,
    PostWebMessageAsString: usize,
    add_WebMessageReceived: usize,
    remove_WebMessageReceived: usize,
    CallDevToolsProtocolMethod: usize,
    BrowserProcessId: usize,
    CanGoBack: usize,
    CanGoForward: usize,
    GoBack: usize,
    GoForward: usize,
    GetDevToolsProtocolEventReceiver: usize,
    Stop: usize,
    add_NewWindowRequested: usize,
    remove_NewWindowRequested: usize,
    add_DocumentTitleChanged: usize,
    remove_DocumentTitleChanged: usize,
    DocumentTitle: usize,
    AddHostObjectToScript: usize,
    RemoveHostObjectFromScript: usize,
    OpenDevToolsWindow: usize,
    add_ContainsFullScreenElementChanged: usize,
    remove_ContainsFullScreenElementChanged: usize,
    ContainsFullScreenElement: usize,
    add_WebResourceRequested: usize,
    remove_WebResourceRequested: usize,
    AddWebResourceRequestedFilter: usize,
    RemoveWebResourceRequestedFilter: usize,
    add_WindowCloseRequested: usize,
    remove_WindowCloseRequested: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2Controller,
    ICoreWebView2Controller_Vtbl,
    0x4d00c0d1_9434_4eb6_8078_8697a560334f
);
windows_core::imp::interface_hierarchy!(ICoreWebView2Controller, windows_core::IUnknown);
impl ICoreWebView2Controller {
    pub unsafe fn SetIsVisible(&self, isvisible: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsVisible)(
                windows_core::Interface::as_raw(self),
                isvisible.into(),
            )
            .ok()
        }
    }
    pub unsafe fn SetBounds(&self, bounds: RECT) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetBounds)(
                windows_core::Interface::as_raw(self),
                bounds,
            )
            .ok()
        }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub unsafe fn CoreWebView2(&self) -> windows_core::Result<ICoreWebView2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CoreWebView2)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Controller_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    IsVisible: usize,
    pub SetIsVisible: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    Bounds: usize,
    pub SetBounds: unsafe extern "system" fn(*mut core::ffi::c_void, RECT) -> windows_core::HRESULT,
    ZoomFactor: usize,
    SetZoomFactor: usize,
    add_ZoomFactorChanged: usize,
    remove_ZoomFactorChanged: usize,
    SetBoundsAndZoomFactor: usize,
    MoveFocus: usize,
    add_MoveFocusRequested: usize,
    remove_MoveFocusRequested: usize,
    add_GotFocus: usize,
    remove_GotFocus: usize,
    add_LostFocus: usize,
    remove_LostFocus: usize,
    add_AcceleratorKeyPressed: usize,
    remove_AcceleratorKeyPressed: usize,
    ParentWindow: usize,
    SetParentWindow: usize,
    NotifyParentWindowPositionChanged: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CoreWebView2: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2CreateCoreWebView2ControllerCompletedHandler,
    ICoreWebView2CreateCoreWebView2ControllerCompletedHandler_Vtbl,
    0x6c4819f3_c9b7_4260_8127_c9f5bde7f68c
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2CreateCoreWebView2ControllerCompletedHandler,
    windows_core::IUnknown
);
impl ICoreWebView2CreateCoreWebView2ControllerCompletedHandler {
    pub unsafe fn Invoke<P1>(
        &self,
        errorcode: windows_core::HRESULT,
        result: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ICoreWebView2Controller>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                errorcode,
                result.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2CreateCoreWebView2ControllerCompletedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::HRESULT,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2CreateCoreWebView2ControllerCompletedHandler_Impl:
    windows_core::IUnknownImpl
{
    fn Invoke(
        &self,
        errorcode: windows_core::HRESULT,
        result: windows_core::Ref<ICoreWebView2Controller>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2CreateCoreWebView2ControllerCompletedHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2CreateCoreWebView2ControllerCompletedHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2CreateCoreWebView2ControllerCompletedHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            errorcode: windows_core::HRESULT,
            result: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2CreateCoreWebView2ControllerCompletedHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&errorcode),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == & < ICoreWebView2CreateCoreWebView2ControllerCompletedHandler as windows_core::Interface >::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2CreateCoreWebView2ControllerCompletedHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler,
    ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler_Vtbl,
    0x4e8a3389_c9d8_4bd2_b6b5_124fee6cc14d
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler,
    windows_core::IUnknown
);
impl ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler {
    pub unsafe fn Invoke<P1>(
        &self,
        errorcode: windows_core::HRESULT,
        result: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ICoreWebView2Environment>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                errorcode,
                result.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::HRESULT,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler_Impl:
    windows_core::IUnknownImpl
{
    fn Invoke(
        &self,
        errorcode: windows_core::HRESULT,
        result: windows_core::Ref<ICoreWebView2Environment>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            errorcode: windows_core::HRESULT,
            result: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&errorcode),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == & < ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler as windows_core::Interface >::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2Environment,
    ICoreWebView2Environment_Vtbl,
    0xb96d755e_0319_4e92_a296_23436f46a1fc
);
windows_core::imp::interface_hierarchy!(ICoreWebView2Environment, windows_core::IUnknown);
impl ICoreWebView2Environment {
    pub unsafe fn CreateCoreWebView2Controller<P1>(
        &self,
        parentwindow: HWND,
        handler: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ICoreWebView2CreateCoreWebView2ControllerCompletedHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateCoreWebView2Controller)(
                windows_core::Interface::as_raw(self),
                parentwindow,
                handler.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Environment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateCoreWebView2Controller: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HWND,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateWebResourceResponse: usize,
    BrowserVersionString: usize,
    add_NewBrowserVersionAvailable: usize,
    remove_NewBrowserVersionAvailable: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2ExecuteScriptCompletedHandler,
    ICoreWebView2ExecuteScriptCompletedHandler_Vtbl,
    0x49511172_cc67_4bca_9923_137112f4c4cc
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2ExecuteScriptCompletedHandler,
    windows_core::IUnknown
);
impl ICoreWebView2ExecuteScriptCompletedHandler {
    pub unsafe fn Invoke(
        &self,
        errorcode: windows_core::HRESULT,
        result: LPCWSTR,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                errorcode,
                result,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2ExecuteScriptCompletedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::HRESULT,
        LPCWSTR,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2ExecuteScriptCompletedHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, errorcode: windows_core::HRESULT, result: LPCWSTR)
    -> windows_core::Result<()>;
}
impl ICoreWebView2ExecuteScriptCompletedHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2ExecuteScriptCompletedHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2ExecuteScriptCompletedHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            errorcode: windows_core::HRESULT,
            result: LPCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2ExecuteScriptCompletedHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&errorcode),
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWebView2ExecuteScriptCompletedHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2ExecuteScriptCompletedHandler {}
pub type LPCWSTR = *const WCHAR;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
pub type WCHAR = u16;
