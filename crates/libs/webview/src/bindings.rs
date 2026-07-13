windows_core::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_core::HRESULT);
windows_core::link!("ole32.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
windows_core::link!("ole32.dll" "system" fn CoTaskMemFree(pv : *mut core::ffi::c_void));
windows_core::link!("webview2loader.dll" "system" fn CreateCoreWebView2Environment(environmentcreatedhandler : *mut core::ffi::c_void) -> windows_core::HRESULT);
windows_core::link!("webview2loader.dll" "system" fn CreateCoreWebView2EnvironmentWithOptions(browserexecutablefolder : windows_core::PCWSTR, userdatafolder : windows_core::PCWSTR, environmentoptions : *mut core::ffi::c_void, environmentcreatedhandler : *mut core::ffi::c_void) -> windows_core::HRESULT);
windows_core::link!("user32.dll" "system" fn DispatchMessageW(lpmsg : *const MSG) -> LRESULT);
windows_core::link!("user32.dll" "system" fn GetMessageW(lpmsg : *mut MSG, hwnd : HWND, wmsgfiltermin : u32, wmsgfiltermax : u32) -> windows_core::BOOL);
windows_core::link!("shlwapi.dll" "system" fn SHCreateMemStream(pinit : *const u8, cbinit : u32) -> Option < IStream >);
windows_core::link!("user32.dll" "system" fn TranslateMessage(lpmsg : *const MSG) -> windows_core::BOOL);
pub type COINIT = i32;
pub const COINIT_APARTMENTTHREADED: COINIT = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COREWEBVIEW2_COLOR {
    pub A: u8,
    pub R: u8,
    pub G: u8,
    pub B: u8,
}
pub type COREWEBVIEW2_COOKIE_SAME_SITE_KIND = i32;
pub type COREWEBVIEW2_DOWNLOAD_INTERRUPT_REASON = i32;
pub type COREWEBVIEW2_DOWNLOAD_STATE = i32;
pub type COREWEBVIEW2_HOST_RESOURCE_ACCESS_KIND = i32;
pub type COREWEBVIEW2_KEY_EVENT_KIND = i32;
pub type COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL = i32;
pub type COREWEBVIEW2_MOVE_FOCUS_REASON = i32;
pub type COREWEBVIEW2_PERMISSION_KIND = i32;
pub type COREWEBVIEW2_PERMISSION_STATE = i32;
pub type COREWEBVIEW2_PREFERRED_COLOR_SCHEME = i32;
pub type COREWEBVIEW2_PROCESS_FAILED_KIND = i32;
pub type COREWEBVIEW2_SCROLLBAR_STYLE = i32;
pub type COREWEBVIEW2_WEB_RESOURCE_CONTEXT = i32;
pub type COREWEBVIEW2_WEB_RESOURCE_REQUEST_SOURCE_KINDS = u32;
pub const E_OUTOFMEMORY: windows_core::HRESULT = windows_core::HRESULT(0x8007000E_u32 as _);
pub type HWND = *mut core::ffi::c_void;
windows_core::imp::define_interface!(
    ICoreWebView2,
    ICoreWebView2_Vtbl,
    0x76eceacb_0462_4d94_ac83_423a6793775e
);
windows_core::imp::interface_hierarchy!(ICoreWebView2, windows_core::IUnknown);
impl ICoreWebView2 {
    pub(crate) unsafe fn Settings(&self) -> windows_core::Result<ICoreWebView2Settings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Settings)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn Source(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Source)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn Navigate<P0>(&self, uri: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Navigate)(
                windows_core::Interface::as_raw(self),
                uri.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn NavigateToString<P0>(&self, htmlcontent: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).NavigateToString)(
                windows_core::Interface::as_raw(self),
                htmlcontent.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn add_NavigationStarting<P0>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2NavigationStartingEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_NavigationStarting)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_NavigationStarting(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_NavigationStarting)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn add_ContentLoading<P0>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2ContentLoadingEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_ContentLoading)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_ContentLoading(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_ContentLoading)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn add_NavigationCompleted<P0>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2NavigationCompletedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_NavigationCompleted)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_NavigationCompleted(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_NavigationCompleted)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn add_PermissionRequested<P0>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2PermissionRequestedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_PermissionRequested)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_PermissionRequested(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_PermissionRequested)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn add_ProcessFailed<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2ProcessFailedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_ProcessFailed)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_ProcessFailed(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_ProcessFailed)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn AddScriptToExecuteOnDocumentCreated<P0, P1>(
        &self,
        javascript: P0,
        handler: P1,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddScriptToExecuteOnDocumentCreated)(
                windows_core::Interface::as_raw(self),
                javascript.param().abi(),
                handler.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn RemoveScriptToExecuteOnDocumentCreated<P0>(
        &self,
        id: P0,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveScriptToExecuteOnDocumentCreated)(
                windows_core::Interface::as_raw(self),
                id.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn ExecuteScript<P0, P1>(
        &self,
        javascript: P0,
        handler: P1,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICoreWebView2ExecuteScriptCompletedHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ExecuteScript)(
                windows_core::Interface::as_raw(self),
                javascript.param().abi(),
                handler.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn Reload(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reload)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn PostWebMessageAsJson<P0>(
        &self,
        webmessageasjson: P0,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PostWebMessageAsJson)(
                windows_core::Interface::as_raw(self),
                webmessageasjson.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn PostWebMessageAsString<P0>(
        &self,
        webmessageasstring: P0,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PostWebMessageAsString)(
                windows_core::Interface::as_raw(self),
                webmessageasstring.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn add_WebMessageReceived<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2WebMessageReceivedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_WebMessageReceived)(
                windows_core::Interface::as_raw(self),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_WebMessageReceived(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_WebMessageReceived)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn GoBack(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GoBack)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn GoForward(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GoForward)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn add_NewWindowRequested<P0>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2NewWindowRequestedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_NewWindowRequested)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_NewWindowRequested(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_NewWindowRequested)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn add_DocumentTitleChanged<P0>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2DocumentTitleChangedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_DocumentTitleChanged)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_DocumentTitleChanged(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_DocumentTitleChanged)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn DocumentTitle(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DocumentTitle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn OpenDevToolsWindow(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).OpenDevToolsWindow)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn add_ContainsFullScreenElementChanged<P0>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2ContainsFullScreenElementChangedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_ContainsFullScreenElementChanged)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_ContainsFullScreenElementChanged(
        &self,
        token: i64,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_ContainsFullScreenElementChanged)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn ContainsFullScreenElement(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContainsFullScreenElement)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn add_WebResourceRequested<P0>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2WebResourceRequestedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_WebResourceRequested)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_WebResourceRequested(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_WebResourceRequested)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn AddWebResourceRequestedFilter<P0>(
        &self,
        uri: P0,
        resourcecontext: COREWEBVIEW2_WEB_RESOURCE_CONTEXT,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddWebResourceRequestedFilter)(
                windows_core::Interface::as_raw(self),
                uri.param().abi(),
                resourcecontext,
            )
        }
    }
    pub(crate) unsafe fn RemoveWebResourceRequestedFilter<P0>(
        &self,
        uri: P0,
        resourcecontext: COREWEBVIEW2_WEB_RESOURCE_CONTEXT,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveWebResourceRequestedFilter)(
                windows_core::Interface::as_raw(self),
                uri.param().abi(),
                resourcecontext,
            )
        }
    }
    pub(crate) unsafe fn add_WindowCloseRequested<P0>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2WindowCloseRequestedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_WindowCloseRequested)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_WindowCloseRequested(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_WindowCloseRequested)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Settings: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Source:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub Navigate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub NavigateToString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub add_NavigationStarting: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_NavigationStarting:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_ContentLoading: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ContentLoading:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_SourceChanged: usize,
    remove_SourceChanged: usize,
    add_HistoryChanged: usize,
    remove_HistoryChanged: usize,
    pub add_NavigationCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_NavigationCompleted:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_FrameNavigationStarting: usize,
    remove_FrameNavigationStarting: usize,
    add_FrameNavigationCompleted: usize,
    remove_FrameNavigationCompleted: usize,
    add_ScriptDialogOpening: usize,
    remove_ScriptDialogOpening: usize,
    pub add_PermissionRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_PermissionRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_ProcessFailed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ProcessFailed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub AddScriptToExecuteOnDocumentCreated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    pub RemoveScriptToExecuteOnDocumentCreated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    )
        -> windows_core::HRESULT,
    pub ExecuteScript: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CapturePreview: usize,
    pub Reload: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PostWebMessageAsJson: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub PostWebMessageAsString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub add_WebMessageReceived: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_WebMessageReceived:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    CallDevToolsProtocolMethod: usize,
    BrowserProcessId: usize,
    CanGoBack: usize,
    CanGoForward: usize,
    pub GoBack: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GoForward: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    GetDevToolsProtocolEventReceiver: usize,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub add_NewWindowRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_NewWindowRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_DocumentTitleChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_DocumentTitleChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub DocumentTitle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    AddHostObjectToScript: usize,
    RemoveHostObjectFromScript: usize,
    pub OpenDevToolsWindow:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub add_ContainsFullScreenElementChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    )
        -> windows_core::HRESULT,
    pub remove_ContainsFullScreenElementChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ContainsFullScreenElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub add_WebResourceRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_WebResourceRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub AddWebResourceRequestedFilter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        COREWEBVIEW2_WEB_RESOURCE_CONTEXT,
    ) -> windows_core::HRESULT,
    pub RemoveWebResourceRequestedFilter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        COREWEBVIEW2_WEB_RESOURCE_CONTEXT,
    ) -> windows_core::HRESULT,
    pub add_WindowCloseRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_WindowCloseRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2AcceleratorKeyPressedEventArgs,
    ICoreWebView2AcceleratorKeyPressedEventArgs_Vtbl,
    0x9f760f8a_fb79_42be_9990_7b56900fa9c7
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2AcceleratorKeyPressedEventArgs,
    windows_core::IUnknown
);
impl ICoreWebView2AcceleratorKeyPressedEventArgs {
    pub(crate) unsafe fn KeyEventKind(&self) -> windows_core::Result<COREWEBVIEW2_KEY_EVENT_KIND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyEventKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn VirtualKey(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VirtualKey)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn Handled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetHandled(&self, handled: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetHandled)(
                windows_core::Interface::as_raw(self),
                handled.into(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2AcceleratorKeyPressedEventArgs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub KeyEventKind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut COREWEBVIEW2_KEY_EVENT_KIND,
    ) -> windows_core::HRESULT,
    pub VirtualKey:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    KeyEventLParam: usize,
    PhysicalKeyStatus: usize,
    pub Handled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2AcceleratorKeyPressedEventHandler,
    ICoreWebView2AcceleratorKeyPressedEventHandler_Vtbl,
    0xb29c7e28_fa79_41a8_8e44_65811c76dcb2
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2AcceleratorKeyPressedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2AcceleratorKeyPressedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2AcceleratorKeyPressedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2Controller>,
        args: windows_core::Ref<ICoreWebView2AcceleratorKeyPressedEventArgs>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2AcceleratorKeyPressedEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2AcceleratorKeyPressedEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2AcceleratorKeyPressedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2AcceleratorKeyPressedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2AcceleratorKeyPressedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2AcceleratorKeyPressedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler,
    ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler_Vtbl,
    0xb99369f3_9b11_47b5_bc6f_8e7895fcea17
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::HRESULT,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler_Impl:
    windows_core::IUnknownImpl
{
    fn Invoke(
        &self,
        errorcode: windows_core::HRESULT,
        result: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            errorcode: windows_core::HRESULT,
            result: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&errorcode),
                    core::mem::transmute(&result),
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
        iid == & < ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler as windows_core::Interface >::IID
    }
}
impl windows_core::RuntimeName
    for ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler
{
}
windows_core::imp::define_interface!(
    ICoreWebView2BytesReceivedChangedEventHandler,
    ICoreWebView2BytesReceivedChangedEventHandler_Vtbl,
    0x828e8ab6_d94c_4264_9cef_5217170d6251
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2BytesReceivedChangedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2BytesReceivedChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2BytesReceivedChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2DownloadOperation>,
        args: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2BytesReceivedChangedEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2BytesReceivedChangedEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2BytesReceivedChangedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2BytesReceivedChangedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2BytesReceivedChangedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2BytesReceivedChangedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2ClearBrowsingDataCompletedHandler,
    ICoreWebView2ClearBrowsingDataCompletedHandler_Vtbl,
    0xe9710a06_1d1d_49b2_8234_226f35846ae5
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2ClearBrowsingDataCompletedHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2ClearBrowsingDataCompletedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::HRESULT,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2ClearBrowsingDataCompletedHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, errorcode: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl ICoreWebView2ClearBrowsingDataCompletedHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2ClearBrowsingDataCompletedHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2ClearBrowsingDataCompletedHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            errorcode: windows_core::HRESULT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2ClearBrowsingDataCompletedHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&errorcode),
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
        iid == &<ICoreWebView2ClearBrowsingDataCompletedHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2ClearBrowsingDataCompletedHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2ContainsFullScreenElementChangedEventHandler,
    ICoreWebView2ContainsFullScreenElementChangedEventHandler_Vtbl,
    0xe45d98b1_afef_45be_8baf_6c7728867f73
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2ContainsFullScreenElementChangedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2ContainsFullScreenElementChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2ContainsFullScreenElementChangedEventHandler_Impl:
    windows_core::IUnknownImpl
{
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2>,
        args: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2ContainsFullScreenElementChangedEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2ContainsFullScreenElementChangedEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2ContainsFullScreenElementChangedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2ContainsFullScreenElementChangedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == & < ICoreWebView2ContainsFullScreenElementChangedEventHandler as windows_core::Interface >::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2ContainsFullScreenElementChangedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2ContentLoadingEventArgs,
    ICoreWebView2ContentLoadingEventArgs_Vtbl,
    0x0c8a1275_9b6b_4901_87ad_70df25bafa6e
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2ContentLoadingEventArgs,
    windows_core::IUnknown
);
impl ICoreWebView2ContentLoadingEventArgs {
    pub(crate) unsafe fn IsErrorPage(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsErrorPage)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn NavigationId(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NavigationId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2ContentLoadingEventArgs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsErrorPage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub NavigationId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2ContentLoadingEventHandler,
    ICoreWebView2ContentLoadingEventHandler_Vtbl,
    0x364471e7_f2be_4910_bdba_d72077d51c4b
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2ContentLoadingEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2ContentLoadingEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2ContentLoadingEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2>,
        args: windows_core::Ref<ICoreWebView2ContentLoadingEventArgs>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2ContentLoadingEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2ContentLoadingEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2ContentLoadingEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2ContentLoadingEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2ContentLoadingEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2ContentLoadingEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2Controller,
    ICoreWebView2Controller_Vtbl,
    0x4d00c0d1_9434_4eb6_8078_8697a560334f
);
windows_core::imp::interface_hierarchy!(ICoreWebView2Controller, windows_core::IUnknown);
impl ICoreWebView2Controller {
    pub(crate) unsafe fn SetIsVisible(&self, isvisible: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsVisible)(
                windows_core::Interface::as_raw(self),
                isvisible.into(),
            )
        }
    }
    pub(crate) unsafe fn SetBounds(&self, bounds: RECT) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetBounds)(
                windows_core::Interface::as_raw(self),
                bounds,
            )
        }
    }
    pub(crate) unsafe fn ZoomFactor(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ZoomFactor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetZoomFactor(&self, zoomfactor: f64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetZoomFactor)(
                windows_core::Interface::as_raw(self),
                zoomfactor,
            )
        }
    }
    pub(crate) unsafe fn MoveFocus(
        &self,
        reason: COREWEBVIEW2_MOVE_FOCUS_REASON,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).MoveFocus)(
                windows_core::Interface::as_raw(self),
                reason,
            )
        }
    }
    pub(crate) unsafe fn add_MoveFocusRequested<P0>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2MoveFocusRequestedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_MoveFocusRequested)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_MoveFocusRequested(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_MoveFocusRequested)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn add_GotFocus<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2FocusChangedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_GotFocus)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_GotFocus(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_GotFocus)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn add_LostFocus<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2FocusChangedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_LostFocus)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_LostFocus(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_LostFocus)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn add_AcceleratorKeyPressed<P0>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2AcceleratorKeyPressedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_AcceleratorKeyPressed)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_AcceleratorKeyPressed(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_AcceleratorKeyPressed)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn NotifyParentWindowPositionChanged(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).NotifyParentWindowPositionChanged)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn CoreWebView2(&self) -> windows_core::Result<ICoreWebView2> {
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
    pub ZoomFactor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetZoomFactor:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    add_ZoomFactorChanged: usize,
    remove_ZoomFactorChanged: usize,
    SetBoundsAndZoomFactor: usize,
    pub MoveFocus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        COREWEBVIEW2_MOVE_FOCUS_REASON,
    ) -> windows_core::HRESULT,
    pub add_MoveFocusRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_MoveFocusRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_GotFocus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_GotFocus:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_LostFocus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_LostFocus:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_AcceleratorKeyPressed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_AcceleratorKeyPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ParentWindow: usize,
    SetParentWindow: usize,
    pub NotifyParentWindowPositionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CoreWebView2: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Controller2,
    ICoreWebView2Controller2_Vtbl,
    0xc979903e_d4ca_4228_92eb_47ee3fa96eab
);
impl core::ops::Deref for ICoreWebView2Controller2 {
    type Target = ICoreWebView2Controller;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Controller2,
    windows_core::IUnknown,
    ICoreWebView2Controller
);
impl ICoreWebView2Controller2 {
    pub(crate) unsafe fn DefaultBackgroundColor(&self) -> windows_core::Result<COREWEBVIEW2_COLOR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultBackgroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetDefaultBackgroundColor(
        &self,
        value: COREWEBVIEW2_COLOR,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultBackgroundColor)(
                windows_core::Interface::as_raw(self),
                value,
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Controller2_Vtbl {
    pub base__: ICoreWebView2Controller_Vtbl,
    pub DefaultBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut COREWEBVIEW2_COLOR,
    ) -> windows_core::HRESULT,
    pub SetDefaultBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        COREWEBVIEW2_COLOR,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Controller3,
    ICoreWebView2Controller3_Vtbl,
    0xf9614724_5d2b_41dc_aef7_73d62b51543b
);
impl core::ops::Deref for ICoreWebView2Controller3 {
    type Target = ICoreWebView2Controller2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Controller3,
    windows_core::IUnknown,
    ICoreWebView2Controller,
    ICoreWebView2Controller2
);
impl ICoreWebView2Controller3 {
    pub(crate) unsafe fn RasterizationScale(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RasterizationScale)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetRasterizationScale(&self, scale: f64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetRasterizationScale)(
                windows_core::Interface::as_raw(self),
                scale,
            )
        }
    }
    pub(crate) unsafe fn ShouldDetectMonitorScaleChanges(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShouldDetectMonitorScaleChanges)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetShouldDetectMonitorScaleChanges(
        &self,
        value: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetShouldDetectMonitorScaleChanges)(
                windows_core::Interface::as_raw(self),
                value.into(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Controller3_Vtbl {
    pub base__: ICoreWebView2Controller2_Vtbl,
    pub RasterizationScale:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRasterizationScale:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ShouldDetectMonitorScaleChanges: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetShouldDetectMonitorScaleChanges: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    add_RasterizationScaleChanged: usize,
    remove_RasterizationScaleChanged: usize,
    BoundsMode: usize,
    SetBoundsMode: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2Controller4,
    ICoreWebView2Controller4_Vtbl,
    0x97d418d5_a426_4e49_a151_e1a10f327d9e
);
impl core::ops::Deref for ICoreWebView2Controller4 {
    type Target = ICoreWebView2Controller3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Controller4,
    windows_core::IUnknown,
    ICoreWebView2Controller,
    ICoreWebView2Controller2,
    ICoreWebView2Controller3
);
impl ICoreWebView2Controller4 {
    pub(crate) unsafe fn AllowExternalDrop(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowExternalDrop)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetAllowExternalDrop(&self, value: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetAllowExternalDrop)(
                windows_core::Interface::as_raw(self),
                value.into(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Controller4_Vtbl {
    pub base__: ICoreWebView2Controller3_Vtbl,
    pub AllowExternalDrop: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetAllowExternalDrop: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2ControllerOptions,
    ICoreWebView2ControllerOptions_Vtbl,
    0x12aae616_8ccb_44ec_bcb3_eb1831881635
);
windows_core::imp::interface_hierarchy!(ICoreWebView2ControllerOptions, windows_core::IUnknown);
impl ICoreWebView2ControllerOptions {
    pub(crate) unsafe fn SetProfileName<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetProfileName)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetIsInPrivateModeEnabled(&self, value: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsInPrivateModeEnabled)(
                windows_core::Interface::as_raw(self),
                value.into(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2ControllerOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    ProfileName: usize,
    pub SetProfileName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    IsInPrivateModeEnabled: usize,
    pub SetIsInPrivateModeEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2ControllerOptions2,
    ICoreWebView2ControllerOptions2_Vtbl,
    0x06c991d8_9e7e_11ed_a8fc_0242ac120002
);
impl core::ops::Deref for ICoreWebView2ControllerOptions2 {
    type Target = ICoreWebView2ControllerOptions;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2ControllerOptions2,
    windows_core::IUnknown,
    ICoreWebView2ControllerOptions
);
#[repr(C)]
pub struct ICoreWebView2ControllerOptions2_Vtbl {
    pub base__: ICoreWebView2ControllerOptions_Vtbl,
    ScriptLocale: usize,
    SetScriptLocale: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2ControllerOptions3,
    ICoreWebView2ControllerOptions3_Vtbl,
    0xb32b191a_8998_57ca_b7cb_e04617e4ce4a
);
impl core::ops::Deref for ICoreWebView2ControllerOptions3 {
    type Target = ICoreWebView2ControllerOptions2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2ControllerOptions3,
    windows_core::IUnknown,
    ICoreWebView2ControllerOptions,
    ICoreWebView2ControllerOptions2
);
impl ICoreWebView2ControllerOptions3 {
    pub(crate) unsafe fn SetDefaultBackgroundColor(
        &self,
        value: COREWEBVIEW2_COLOR,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultBackgroundColor)(
                windows_core::Interface::as_raw(self),
                value,
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2ControllerOptions3_Vtbl {
    pub base__: ICoreWebView2ControllerOptions2_Vtbl,
    DefaultBackgroundColor: usize,
    pub SetDefaultBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        COREWEBVIEW2_COLOR,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Cookie,
    ICoreWebView2Cookie_Vtbl,
    0xad26d6be_1486_43e6_bf87_a2034006ca21
);
windows_core::imp::interface_hierarchy!(ICoreWebView2Cookie, windows_core::IUnknown);
impl ICoreWebView2Cookie {
    pub(crate) unsafe fn Name(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn Value(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn Domain(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Domain)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn Path(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn Expires(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Expires)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetExpires(&self, expires: f64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetExpires)(
                windows_core::Interface::as_raw(self),
                expires,
            )
        }
    }
    pub(crate) unsafe fn IsHttpOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsHttpOnly)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetIsHttpOnly(&self, ishttponly: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsHttpOnly)(
                windows_core::Interface::as_raw(self),
                ishttponly.into(),
            )
        }
    }
    pub(crate) unsafe fn SameSite(
        &self,
    ) -> windows_core::Result<COREWEBVIEW2_COOKIE_SAME_SITE_KIND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SameSite)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetSameSite(
        &self,
        samesite: COREWEBVIEW2_COOKIE_SAME_SITE_KIND,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetSameSite)(
                windows_core::Interface::as_raw(self),
                samesite,
            )
        }
    }
    pub(crate) unsafe fn IsSecure(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSecure)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetIsSecure(&self, issecure: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsSecure)(
                windows_core::Interface::as_raw(self),
                issecure.into(),
            )
        }
    }
    pub(crate) unsafe fn IsSession(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSession)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Cookie_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub Value:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    SetValue: usize,
    pub Domain:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub Path:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub Expires:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetExpires: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub IsHttpOnly: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetIsHttpOnly: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SameSite: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut COREWEBVIEW2_COOKIE_SAME_SITE_KIND,
    ) -> windows_core::HRESULT,
    pub SetSameSite: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        COREWEBVIEW2_COOKIE_SAME_SITE_KIND,
    ) -> windows_core::HRESULT,
    pub IsSecure: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetIsSecure: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub IsSession: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2CookieList,
    ICoreWebView2CookieList_Vtbl,
    0xf7f6f714_5d2a_43c6_9503_346ece02d186
);
windows_core::imp::interface_hierarchy!(ICoreWebView2CookieList, windows_core::IUnknown);
impl ICoreWebView2CookieList {
    pub(crate) unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetValueAtIndex(
        &self,
        index: u32,
    ) -> windows_core::Result<ICoreWebView2Cookie> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValueAtIndex)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2CookieList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetValueAtIndex: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2CookieManager,
    ICoreWebView2CookieManager_Vtbl,
    0x177cd9e7_b6f5_451a_94a0_5d7a3a4c4141
);
windows_core::imp::interface_hierarchy!(ICoreWebView2CookieManager, windows_core::IUnknown);
impl ICoreWebView2CookieManager {
    pub(crate) unsafe fn CreateCookie<P0, P1, P2, P3>(
        &self,
        name: P0,
        value: P1,
        domain: P2,
        path: P3,
    ) -> windows_core::Result<ICoreWebView2Cookie>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCookie)(
                windows_core::Interface::as_raw(self),
                name.param().abi(),
                value.param().abi(),
                domain.param().abi(),
                path.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetCookies<P0, P1>(&self, uri: P0, handler: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICoreWebView2GetCookiesCompletedHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetCookies)(
                windows_core::Interface::as_raw(self),
                uri.param().abi(),
                handler.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn AddOrUpdateCookie<P0>(&self, cookie: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICoreWebView2Cookie>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddOrUpdateCookie)(
                windows_core::Interface::as_raw(self),
                cookie.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn DeleteCookies<P0, P1>(&self, name: P0, uri: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteCookies)(
                windows_core::Interface::as_raw(self),
                name.param().abi(),
                uri.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn DeleteCookiesWithDomainAndPath<P0, P1, P2>(
        &self,
        name: P0,
        domain: P1,
        path: P2,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteCookiesWithDomainAndPath)(
                windows_core::Interface::as_raw(self),
                name.param().abi(),
                domain.param().abi(),
                path.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn DeleteAllCookies(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteAllCookies)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2CookieManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateCookie: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CopyCookie: usize,
    pub GetCookies: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddOrUpdateCookie: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    DeleteCookie: usize,
    pub DeleteCookies: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub DeleteCookiesWithDomainAndPath: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub DeleteAllCookies:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
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
    ICoreWebView2Deferral,
    ICoreWebView2Deferral_Vtbl,
    0xc10e7f7b_b585_46f0_a623_8befbf3e4ee0
);
windows_core::imp::interface_hierarchy!(ICoreWebView2Deferral, windows_core::IUnknown);
impl ICoreWebView2Deferral {
    pub(crate) unsafe fn Complete(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self))
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Deferral_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2DocumentTitleChangedEventHandler,
    ICoreWebView2DocumentTitleChangedEventHandler_Vtbl,
    0xf5f2b923_953e_4042_9f95_f3a118e1afd4
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2DocumentTitleChangedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2DocumentTitleChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2DocumentTitleChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2>,
        args: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2DocumentTitleChangedEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2DocumentTitleChangedEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2DocumentTitleChangedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2DocumentTitleChangedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2DocumentTitleChangedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2DocumentTitleChangedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2DownloadOperation,
    ICoreWebView2DownloadOperation_Vtbl,
    0x3d6b6cf2_afe1_44c7_a995_c65117714336
);
windows_core::imp::interface_hierarchy!(ICoreWebView2DownloadOperation, windows_core::IUnknown);
impl ICoreWebView2DownloadOperation {
    pub(crate) unsafe fn add_BytesReceivedChanged<P0>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2BytesReceivedChangedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_BytesReceivedChanged)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_BytesReceivedChanged(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_BytesReceivedChanged)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn add_StateChanged<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2StateChangedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_StateChanged)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_StateChanged(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_StateChanged)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
    pub(crate) unsafe fn Uri(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn ContentDisposition(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentDisposition)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn MimeType(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MimeType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn TotalBytesToReceive(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalBytesToReceive)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn BytesReceived(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BytesReceived)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn ResultFilePath(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResultFilePath)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn State(&self) -> windows_core::Result<COREWEBVIEW2_DOWNLOAD_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn InterruptReason(
        &self,
    ) -> windows_core::Result<COREWEBVIEW2_DOWNLOAD_INTERRUPT_REASON> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InterruptReason)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn Resume(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn CanResume(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanResume)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2DownloadOperation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub add_BytesReceivedChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_BytesReceivedChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_EstimatedEndTimeChanged: usize,
    remove_EstimatedEndTimeChanged: usize,
    pub add_StateChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_StateChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Uri:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub ContentDisposition:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub MimeType:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub TotalBytesToReceive:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub BytesReceived:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    EstimatedEndTime: usize,
    pub ResultFilePath:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut COREWEBVIEW2_DOWNLOAD_STATE,
    ) -> windows_core::HRESULT,
    pub InterruptReason: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut COREWEBVIEW2_DOWNLOAD_INTERRUPT_REASON,
    ) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanResume: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2DownloadStartingEventArgs,
    ICoreWebView2DownloadStartingEventArgs_Vtbl,
    0xe99bbe21_43e9_4544_a732_282764eafa60
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2DownloadStartingEventArgs,
    windows_core::IUnknown
);
impl ICoreWebView2DownloadStartingEventArgs {
    pub(crate) unsafe fn DownloadOperation(
        &self,
    ) -> windows_core::Result<ICoreWebView2DownloadOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DownloadOperation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn Cancel(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cancel)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetCancel(&self, cancel: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetCancel)(
                windows_core::Interface::as_raw(self),
                cancel.into(),
            )
        }
    }
    pub(crate) unsafe fn ResultFilePath(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResultFilePath)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetResultFilePath<P0>(&self, resultfilepath: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetResultFilePath)(
                windows_core::Interface::as_raw(self),
                resultfilepath.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn Handled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetHandled(&self, handled: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetHandled)(
                windows_core::Interface::as_raw(self),
                handled.into(),
            )
        }
    }
    pub(crate) unsafe fn GetDeferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
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
pub struct ICoreWebView2DownloadStartingEventArgs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DownloadOperation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub ResultFilePath:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub SetResultFilePath: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2DownloadStartingEventHandler,
    ICoreWebView2DownloadStartingEventHandler_Vtbl,
    0xefedc989_c396_41ca_83f7_07f845a55724
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2DownloadStartingEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2DownloadStartingEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2DownloadStartingEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2>,
        args: windows_core::Ref<ICoreWebView2DownloadStartingEventArgs>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2DownloadStartingEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2DownloadStartingEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2DownloadStartingEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2DownloadStartingEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2DownloadStartingEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2DownloadStartingEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2Environment,
    ICoreWebView2Environment_Vtbl,
    0xb96d755e_0319_4e92_a296_23436f46a1fc
);
windows_core::imp::interface_hierarchy!(ICoreWebView2Environment, windows_core::IUnknown);
impl ICoreWebView2Environment {
    pub(crate) unsafe fn CreateCoreWebView2Controller<P1>(
        &self,
        parentwindow: HWND,
        handler: P1,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ICoreWebView2CreateCoreWebView2ControllerCompletedHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateCoreWebView2Controller)(
                windows_core::Interface::as_raw(self),
                parentwindow,
                handler.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn CreateWebResourceResponse<P0, P2, P3>(
        &self,
        content: P0,
        statuscode: i32,
        reasonphrase: P2,
        headers: P3,
    ) -> windows_core::Result<ICoreWebView2WebResourceResponse>
    where
        P0: windows_core::Param<IStream>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateWebResourceResponse)(
                windows_core::Interface::as_raw(self),
                content.param().abi(),
                statuscode,
                reasonphrase.param().abi(),
                headers.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
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
    pub CreateWebResourceResponse: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    BrowserVersionString: usize,
    add_NewBrowserVersionAvailable: usize,
    remove_NewBrowserVersionAvailable: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2Environment10,
    ICoreWebView2Environment10_Vtbl,
    0xee0eb9df_6f12_46ce_b53f_3f47b9c928e0
);
impl core::ops::Deref for ICoreWebView2Environment10 {
    type Target = ICoreWebView2Environment9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Environment10,
    windows_core::IUnknown,
    ICoreWebView2Environment,
    ICoreWebView2Environment2,
    ICoreWebView2Environment3,
    ICoreWebView2Environment4,
    ICoreWebView2Environment5,
    ICoreWebView2Environment6,
    ICoreWebView2Environment7,
    ICoreWebView2Environment8,
    ICoreWebView2Environment9
);
impl ICoreWebView2Environment10 {
    pub(crate) unsafe fn CreateCoreWebView2ControllerOptions(
        &self,
    ) -> windows_core::Result<ICoreWebView2ControllerOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCoreWebView2ControllerOptions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateCoreWebView2ControllerWithOptions<P1, P2>(
        &self,
        parentwindow: HWND,
        options: P1,
        handler: P2,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ICoreWebView2ControllerOptions>,
        P2: windows_core::Param<ICoreWebView2CreateCoreWebView2ControllerCompletedHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateCoreWebView2ControllerWithOptions)(
                windows_core::Interface::as_raw(self),
                parentwindow,
                options.param().abi(),
                handler.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Environment10_Vtbl {
    pub base__: ICoreWebView2Environment9_Vtbl,
    pub CreateCoreWebView2ControllerOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    pub CreateCoreWebView2ControllerWithOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HWND,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    CreateCoreWebView2CompositionControllerWithOptions: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2Environment2,
    ICoreWebView2Environment2_Vtbl,
    0x41f3632b_5ef4_404f_ad82_2d606c5a9a21
);
impl core::ops::Deref for ICoreWebView2Environment2 {
    type Target = ICoreWebView2Environment;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Environment2,
    windows_core::IUnknown,
    ICoreWebView2Environment
);
impl ICoreWebView2Environment2 {
    pub(crate) unsafe fn CreateWebResourceRequest<P0, P1, P2, P3>(
        &self,
        uri: P0,
        method: P1,
        postdata: P2,
        headers: P3,
    ) -> windows_core::Result<ICoreWebView2WebResourceRequest>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IStream>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateWebResourceRequest)(
                windows_core::Interface::as_raw(self),
                uri.param().abi(),
                method.param().abi(),
                postdata.param().abi(),
                headers.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Environment2_Vtbl {
    pub base__: ICoreWebView2Environment_Vtbl,
    pub CreateWebResourceRequest: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Environment3,
    ICoreWebView2Environment3_Vtbl,
    0x80a22ae3_be7c_4ce2_afe1_5a50056cdeeb
);
impl core::ops::Deref for ICoreWebView2Environment3 {
    type Target = ICoreWebView2Environment2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Environment3,
    windows_core::IUnknown,
    ICoreWebView2Environment,
    ICoreWebView2Environment2
);
#[repr(C)]
pub struct ICoreWebView2Environment3_Vtbl {
    pub base__: ICoreWebView2Environment2_Vtbl,
    CreateCoreWebView2CompositionController: usize,
    CreateCoreWebView2PointerInfo: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2Environment4,
    ICoreWebView2Environment4_Vtbl,
    0x20944379_6dcf_41d6_a0a0_abc0fc50de0d
);
impl core::ops::Deref for ICoreWebView2Environment4 {
    type Target = ICoreWebView2Environment3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Environment4,
    windows_core::IUnknown,
    ICoreWebView2Environment,
    ICoreWebView2Environment2,
    ICoreWebView2Environment3
);
#[repr(C)]
pub struct ICoreWebView2Environment4_Vtbl {
    pub base__: ICoreWebView2Environment3_Vtbl,
    GetAutomationProviderForWindow: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2Environment5,
    ICoreWebView2Environment5_Vtbl,
    0x319e423d_e0d7_4b8d_9254_ae9475de9b17
);
impl core::ops::Deref for ICoreWebView2Environment5 {
    type Target = ICoreWebView2Environment4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Environment5,
    windows_core::IUnknown,
    ICoreWebView2Environment,
    ICoreWebView2Environment2,
    ICoreWebView2Environment3,
    ICoreWebView2Environment4
);
#[repr(C)]
pub struct ICoreWebView2Environment5_Vtbl {
    pub base__: ICoreWebView2Environment4_Vtbl,
    add_BrowserProcessExited: usize,
    remove_BrowserProcessExited: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2Environment6,
    ICoreWebView2Environment6_Vtbl,
    0xe59ee362_acbd_4857_9a8e_d3644d9459a9
);
impl core::ops::Deref for ICoreWebView2Environment6 {
    type Target = ICoreWebView2Environment5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Environment6,
    windows_core::IUnknown,
    ICoreWebView2Environment,
    ICoreWebView2Environment2,
    ICoreWebView2Environment3,
    ICoreWebView2Environment4,
    ICoreWebView2Environment5
);
#[repr(C)]
pub struct ICoreWebView2Environment6_Vtbl {
    pub base__: ICoreWebView2Environment5_Vtbl,
    CreatePrintSettings: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2Environment7,
    ICoreWebView2Environment7_Vtbl,
    0x43c22296_3bbd_43a4_9c00_5c0df6dd29a2
);
impl core::ops::Deref for ICoreWebView2Environment7 {
    type Target = ICoreWebView2Environment6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Environment7,
    windows_core::IUnknown,
    ICoreWebView2Environment,
    ICoreWebView2Environment2,
    ICoreWebView2Environment3,
    ICoreWebView2Environment4,
    ICoreWebView2Environment5,
    ICoreWebView2Environment6
);
#[repr(C)]
pub struct ICoreWebView2Environment7_Vtbl {
    pub base__: ICoreWebView2Environment6_Vtbl,
    UserDataFolder: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2Environment8,
    ICoreWebView2Environment8_Vtbl,
    0xd6eb91dd_c3d2_45e5_bd29_6dc2bc4de9cf
);
impl core::ops::Deref for ICoreWebView2Environment8 {
    type Target = ICoreWebView2Environment7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Environment8,
    windows_core::IUnknown,
    ICoreWebView2Environment,
    ICoreWebView2Environment2,
    ICoreWebView2Environment3,
    ICoreWebView2Environment4,
    ICoreWebView2Environment5,
    ICoreWebView2Environment6,
    ICoreWebView2Environment7
);
#[repr(C)]
pub struct ICoreWebView2Environment8_Vtbl {
    pub base__: ICoreWebView2Environment7_Vtbl,
    add_ProcessInfosChanged: usize,
    remove_ProcessInfosChanged: usize,
    GetProcessInfos: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2Environment9,
    ICoreWebView2Environment9_Vtbl,
    0xf06f41bf_4b5a_49d8_b9f6_fa16cd29f274
);
impl core::ops::Deref for ICoreWebView2Environment9 {
    type Target = ICoreWebView2Environment8;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Environment9,
    windows_core::IUnknown,
    ICoreWebView2Environment,
    ICoreWebView2Environment2,
    ICoreWebView2Environment3,
    ICoreWebView2Environment4,
    ICoreWebView2Environment5,
    ICoreWebView2Environment6,
    ICoreWebView2Environment7,
    ICoreWebView2Environment8
);
#[repr(C)]
pub struct ICoreWebView2Environment9_Vtbl {
    pub base__: ICoreWebView2Environment8_Vtbl,
    CreateContextMenuItem: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2EnvironmentOptions,
    ICoreWebView2EnvironmentOptions_Vtbl,
    0x2fde08a8_1e9a_4766_8c05_95a9ceb9d1c5
);
windows_core::imp::interface_hierarchy!(ICoreWebView2EnvironmentOptions, windows_core::IUnknown);
#[repr(C)]
pub struct ICoreWebView2EnvironmentOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdditionalBrowserArguments:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub SetAdditionalBrowserArguments: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub Language:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub TargetCompatibleBrowserVersion:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub SetTargetCompatibleBrowserVersion: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub AllowSingleSignOnUsingOSPrimaryAccount: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    )
        -> windows_core::HRESULT,
    pub SetAllowSingleSignOnUsingOSPrimaryAccount:
        unsafe extern "system" fn(
            *mut core::ffi::c_void,
            windows_core::BOOL,
        ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2EnvironmentOptions_Impl: windows_core::IUnknownImpl {
    fn AdditionalBrowserArguments(&self) -> windows_core::Result<LPWSTR>;
    fn SetAdditionalBrowserArguments(
        &self,
        value: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
    fn Language(&self) -> windows_core::Result<LPWSTR>;
    fn SetLanguage(&self, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn TargetCompatibleBrowserVersion(&self) -> windows_core::Result<LPWSTR>;
    fn SetTargetCompatibleBrowserVersion(
        &self,
        value: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
    fn AllowSingleSignOnUsingOSPrimaryAccount(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetAllowSingleSignOnUsingOSPrimaryAccount(
        &self,
        allow: windows_core::BOOL,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2EnvironmentOptions_Vtbl {
    pub const fn new<Identity: ICoreWebView2EnvironmentOptions_Impl, const OFFSET: isize>() -> Self
    {
        unsafe extern "system" fn AdditionalBrowserArguments<
            Identity: ICoreWebView2EnvironmentOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut LPWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWebView2EnvironmentOptions_Impl::AdditionalBrowserArguments(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAdditionalBrowserArguments<
            Identity: ICoreWebView2EnvironmentOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2EnvironmentOptions_Impl::SetAdditionalBrowserArguments(
                    this,
                    core::mem::transmute(&value),
                )
                .into()
            }
        }
        unsafe extern "system" fn Language<
            Identity: ICoreWebView2EnvironmentOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut LPWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWebView2EnvironmentOptions_Impl::Language(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLanguage<
            Identity: ICoreWebView2EnvironmentOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2EnvironmentOptions_Impl::SetLanguage(
                    this,
                    core::mem::transmute(&value),
                )
                .into()
            }
        }
        unsafe extern "system" fn TargetCompatibleBrowserVersion<
            Identity: ICoreWebView2EnvironmentOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut LPWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWebView2EnvironmentOptions_Impl::TargetCompatibleBrowserVersion(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetCompatibleBrowserVersion<
            Identity: ICoreWebView2EnvironmentOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2EnvironmentOptions_Impl::SetTargetCompatibleBrowserVersion(
                    this,
                    core::mem::transmute(&value),
                )
                .into()
            }
        }
        unsafe extern "system" fn AllowSingleSignOnUsingOSPrimaryAccount<
            Identity: ICoreWebView2EnvironmentOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            allow: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWebView2EnvironmentOptions_Impl::AllowSingleSignOnUsingOSPrimaryAccount(
                    this,
                ) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowSingleSignOnUsingOSPrimaryAccount<
            Identity: ICoreWebView2EnvironmentOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            allow: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2EnvironmentOptions_Impl::SetAllowSingleSignOnUsingOSPrimaryAccount(
                    this,
                    core::mem::transmute_copy(&allow),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdditionalBrowserArguments: AdditionalBrowserArguments::<Identity, OFFSET>,
            SetAdditionalBrowserArguments: SetAdditionalBrowserArguments::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
            SetLanguage: SetLanguage::<Identity, OFFSET>,
            TargetCompatibleBrowserVersion: TargetCompatibleBrowserVersion::<Identity, OFFSET>,
            SetTargetCompatibleBrowserVersion: SetTargetCompatibleBrowserVersion::<Identity, OFFSET>,
            AllowSingleSignOnUsingOSPrimaryAccount: AllowSingleSignOnUsingOSPrimaryAccount::<
                Identity,
                OFFSET,
            >,
            SetAllowSingleSignOnUsingOSPrimaryAccount: SetAllowSingleSignOnUsingOSPrimaryAccount::<
                Identity,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWebView2EnvironmentOptions as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2EnvironmentOptions {}
windows_core::imp::define_interface!(
    ICoreWebView2EnvironmentOptions6,
    ICoreWebView2EnvironmentOptions6_Vtbl,
    0x57d29cc3_c84f_42a0_b0e2_effbd5e179de
);
windows_core::imp::interface_hierarchy!(ICoreWebView2EnvironmentOptions6, windows_core::IUnknown);
#[repr(C)]
pub struct ICoreWebView2EnvironmentOptions6_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AreBrowserExtensionsEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetAreBrowserExtensionsEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2EnvironmentOptions6_Impl: windows_core::IUnknownImpl {
    fn AreBrowserExtensionsEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetAreBrowserExtensionsEnabled(&self, value: windows_core::BOOL)
    -> windows_core::Result<()>;
}
impl ICoreWebView2EnvironmentOptions6_Vtbl {
    pub const fn new<Identity: ICoreWebView2EnvironmentOptions6_Impl, const OFFSET: isize>() -> Self
    {
        unsafe extern "system" fn AreBrowserExtensionsEnabled<
            Identity: ICoreWebView2EnvironmentOptions6_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWebView2EnvironmentOptions6_Impl::AreBrowserExtensionsEnabled(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAreBrowserExtensionsEnabled<
            Identity: ICoreWebView2EnvironmentOptions6_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2EnvironmentOptions6_Impl::SetAreBrowserExtensionsEnabled(
                    this,
                    core::mem::transmute_copy(&value),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AreBrowserExtensionsEnabled: AreBrowserExtensionsEnabled::<Identity, OFFSET>,
            SetAreBrowserExtensionsEnabled: SetAreBrowserExtensionsEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWebView2EnvironmentOptions6 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2EnvironmentOptions6 {}
windows_core::imp::define_interface!(
    ICoreWebView2EnvironmentOptions8,
    ICoreWebView2EnvironmentOptions8_Vtbl,
    0x7c7ecf51_e918_5caf_853c_e9a2bcc27775
);
windows_core::imp::interface_hierarchy!(ICoreWebView2EnvironmentOptions8, windows_core::IUnknown);
#[repr(C)]
pub struct ICoreWebView2EnvironmentOptions8_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ScrollBarStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut COREWEBVIEW2_SCROLLBAR_STYLE,
    ) -> windows_core::HRESULT,
    pub SetScrollBarStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        COREWEBVIEW2_SCROLLBAR_STYLE,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2EnvironmentOptions8_Impl: windows_core::IUnknownImpl {
    fn ScrollBarStyle(&self) -> windows_core::Result<COREWEBVIEW2_SCROLLBAR_STYLE>;
    fn SetScrollBarStyle(&self, value: COREWEBVIEW2_SCROLLBAR_STYLE) -> windows_core::Result<()>;
}
impl ICoreWebView2EnvironmentOptions8_Vtbl {
    pub const fn new<Identity: ICoreWebView2EnvironmentOptions8_Impl, const OFFSET: isize>() -> Self
    {
        unsafe extern "system" fn ScrollBarStyle<
            Identity: ICoreWebView2EnvironmentOptions8_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut COREWEBVIEW2_SCROLLBAR_STYLE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWebView2EnvironmentOptions8_Impl::ScrollBarStyle(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScrollBarStyle<
            Identity: ICoreWebView2EnvironmentOptions8_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: COREWEBVIEW2_SCROLLBAR_STYLE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2EnvironmentOptions8_Impl::SetScrollBarStyle(
                    this,
                    core::mem::transmute_copy(&value),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ScrollBarStyle: ScrollBarStyle::<Identity, OFFSET>,
            SetScrollBarStyle: SetScrollBarStyle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWebView2EnvironmentOptions8 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2EnvironmentOptions8 {}
windows_core::imp::define_interface!(
    ICoreWebView2ExecuteScriptCompletedHandler,
    ICoreWebView2ExecuteScriptCompletedHandler_Vtbl,
    0x49511172_cc67_4bca_9923_137112f4c4cc
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2ExecuteScriptCompletedHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2ExecuteScriptCompletedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::HRESULT,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2ExecuteScriptCompletedHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        errorcode: windows_core::HRESULT,
        result: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
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
            result: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2ExecuteScriptCompletedHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&errorcode),
                    core::mem::transmute(&result),
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
windows_core::imp::define_interface!(
    ICoreWebView2FocusChangedEventHandler,
    ICoreWebView2FocusChangedEventHandler_Vtbl,
    0x05ea24bd_6452_4926_9014_4b82b498135d
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2FocusChangedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2FocusChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2FocusChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2Controller>,
        args: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2FocusChangedEventHandler_Vtbl {
    pub const fn new<Identity: ICoreWebView2FocusChangedEventHandler_Impl, const OFFSET: isize>()
    -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2FocusChangedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2FocusChangedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2FocusChangedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2FocusChangedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2GetCookiesCompletedHandler,
    ICoreWebView2GetCookiesCompletedHandler_Vtbl,
    0x5a4f5069_5c15_47c3_8646_f4de1c116670
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2GetCookiesCompletedHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2GetCookiesCompletedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::HRESULT,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2GetCookiesCompletedHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        errorcode: windows_core::HRESULT,
        result: windows_core::Ref<ICoreWebView2CookieList>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2GetCookiesCompletedHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2GetCookiesCompletedHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2GetCookiesCompletedHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            errorcode: windows_core::HRESULT,
            result: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2GetCookiesCompletedHandler_Impl::Invoke(
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
        iid == &<ICoreWebView2GetCookiesCompletedHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2GetCookiesCompletedHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2HttpHeadersCollectionIterator,
    ICoreWebView2HttpHeadersCollectionIterator_Vtbl,
    0x0702fc30_f43b_47bb_ab52_a42cb552ad9f
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2HttpHeadersCollectionIterator,
    windows_core::IUnknown
);
impl ICoreWebView2HttpHeadersCollectionIterator {
    pub(crate) unsafe fn GetCurrentHeader(
        &self,
        name: *mut LPWSTR,
        value: *mut LPWSTR,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetCurrentHeader)(
                windows_core::Interface::as_raw(self),
                name as _,
                value as _,
            )
        }
    }
    pub(crate) unsafe fn HasCurrentHeader(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasCurrentHeader)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2HttpHeadersCollectionIterator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut LPWSTR,
        *mut LPWSTR,
    ) -> windows_core::HRESULT,
    pub HasCurrentHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2HttpRequestHeaders,
    ICoreWebView2HttpRequestHeaders_Vtbl,
    0xe86cac0e_5523_465c_b536_8fb9fc8c8c60
);
windows_core::imp::interface_hierarchy!(ICoreWebView2HttpRequestHeaders, windows_core::IUnknown);
impl ICoreWebView2HttpRequestHeaders {
    pub(crate) unsafe fn GetIterator(
        &self,
    ) -> windows_core::Result<ICoreWebView2HttpHeadersCollectionIterator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIterator)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2HttpRequestHeaders_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetHeader: usize,
    GetHeaders: usize,
    Contains: usize,
    SetHeader: usize,
    RemoveHeader: usize,
    pub GetIterator: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Interop2,
    ICoreWebView2Interop2_Vtbl,
    0xb151ad7c_cfb0_4ecf_b9b2_afca868581a6
);
windows_core::imp::interface_hierarchy!(ICoreWebView2Interop2, windows_core::IUnknown);
impl ICoreWebView2Interop2 {
    pub(crate) unsafe fn GetComICoreWebView2(&self) -> windows_core::Result<ICoreWebView2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComICoreWebView2)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Interop2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetComICoreWebView2: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2MoveFocusRequestedEventArgs,
    ICoreWebView2MoveFocusRequestedEventArgs_Vtbl,
    0x2d6aa13b_3839_4a15_92fc_d88b3c0d9c9d
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2MoveFocusRequestedEventArgs,
    windows_core::IUnknown
);
impl ICoreWebView2MoveFocusRequestedEventArgs {
    pub(crate) unsafe fn Reason(&self) -> windows_core::Result<COREWEBVIEW2_MOVE_FOCUS_REASON> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reason)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn Handled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetHandled(&self, value: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetHandled)(
                windows_core::Interface::as_raw(self),
                value.into(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2MoveFocusRequestedEventArgs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reason: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut COREWEBVIEW2_MOVE_FOCUS_REASON,
    ) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2MoveFocusRequestedEventHandler,
    ICoreWebView2MoveFocusRequestedEventHandler_Vtbl,
    0x69035451_6dc7_4cb8_9bce_b2bd70ad289f
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2MoveFocusRequestedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2MoveFocusRequestedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2MoveFocusRequestedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2Controller>,
        args: windows_core::Ref<ICoreWebView2MoveFocusRequestedEventArgs>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2MoveFocusRequestedEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2MoveFocusRequestedEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2MoveFocusRequestedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2MoveFocusRequestedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2MoveFocusRequestedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2MoveFocusRequestedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2NavigationCompletedEventArgs,
    ICoreWebView2NavigationCompletedEventArgs_Vtbl,
    0x30d68b7d_20d9_4752_a9ca_ec8448fbb5c1
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2NavigationCompletedEventArgs,
    windows_core::IUnknown
);
impl ICoreWebView2NavigationCompletedEventArgs {
    pub(crate) unsafe fn IsSuccess(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSuccess)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn NavigationId(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NavigationId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2NavigationCompletedEventArgs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsSuccess: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    WebErrorStatus: usize,
    pub NavigationId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2NavigationCompletedEventHandler,
    ICoreWebView2NavigationCompletedEventHandler_Vtbl,
    0xd33a35bf_1c49_4f98_93ab_006e0533fe1c
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2NavigationCompletedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2NavigationCompletedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2NavigationCompletedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2>,
        args: windows_core::Ref<ICoreWebView2NavigationCompletedEventArgs>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2NavigationCompletedEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2NavigationCompletedEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2NavigationCompletedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2NavigationCompletedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2NavigationCompletedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2NavigationCompletedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2NavigationStartingEventArgs,
    ICoreWebView2NavigationStartingEventArgs_Vtbl,
    0x5b495469_e119_438a_9b18_7604f25f2e49
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2NavigationStartingEventArgs,
    windows_core::IUnknown
);
impl ICoreWebView2NavigationStartingEventArgs {
    pub(crate) unsafe fn Uri(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn IsUserInitiated(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUserInitiated)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn IsRedirected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRedirected)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn Cancel(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cancel)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetCancel(&self, cancel: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetCancel)(
                windows_core::Interface::as_raw(self),
                cancel.into(),
            )
        }
    }
    pub(crate) unsafe fn NavigationId(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NavigationId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2NavigationStartingEventArgs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Uri:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub IsRedirected: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    RequestHeaders: usize,
    pub Cancel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub NavigationId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2NavigationStartingEventHandler,
    ICoreWebView2NavigationStartingEventHandler_Vtbl,
    0x9adbe429_f36d_432b_9ddc_f8881fbd76e3
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2NavigationStartingEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2NavigationStartingEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2NavigationStartingEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2>,
        args: windows_core::Ref<ICoreWebView2NavigationStartingEventArgs>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2NavigationStartingEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2NavigationStartingEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2NavigationStartingEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2NavigationStartingEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2NavigationStartingEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2NavigationStartingEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2NewWindowRequestedEventArgs,
    ICoreWebView2NewWindowRequestedEventArgs_Vtbl,
    0x34acb11c_fc37_4418_9132_f9c21d1eafb9
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2NewWindowRequestedEventArgs,
    windows_core::IUnknown
);
impl ICoreWebView2NewWindowRequestedEventArgs {
    pub(crate) unsafe fn Uri(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetNewWindow<P0>(&self, newwindow: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICoreWebView2>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetNewWindow)(
                windows_core::Interface::as_raw(self),
                newwindow.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetHandled(&self, handled: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetHandled)(
                windows_core::Interface::as_raw(self),
                handled.into(),
            )
        }
    }
    pub(crate) unsafe fn Handled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn IsUserInitiated(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUserInitiated)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetDeferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
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
pub struct ICoreWebView2NewWindowRequestedEventArgs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Uri:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub SetNewWindow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    NewWindow: usize,
    pub SetHandled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    WindowFeatures: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2NewWindowRequestedEventHandler,
    ICoreWebView2NewWindowRequestedEventHandler_Vtbl,
    0xd4c185fe_c81c_4989_97af_2d3fa7ab5651
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2NewWindowRequestedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2NewWindowRequestedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2NewWindowRequestedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2>,
        args: windows_core::Ref<ICoreWebView2NewWindowRequestedEventArgs>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2NewWindowRequestedEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2NewWindowRequestedEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2NewWindowRequestedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2NewWindowRequestedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2NewWindowRequestedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2NewWindowRequestedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2PermissionRequestedEventArgs,
    ICoreWebView2PermissionRequestedEventArgs_Vtbl,
    0x973ae2ef_ff18_4894_8fb2_3c758f046810
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2PermissionRequestedEventArgs,
    windows_core::IUnknown
);
impl ICoreWebView2PermissionRequestedEventArgs {
    pub(crate) unsafe fn Uri(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn PermissionKind(
        &self,
    ) -> windows_core::Result<COREWEBVIEW2_PERMISSION_KIND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermissionKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn IsUserInitiated(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUserInitiated)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn State(&self) -> windows_core::Result<COREWEBVIEW2_PERMISSION_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetState(
        &self,
        state: COREWEBVIEW2_PERMISSION_STATE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetState)(
                windows_core::Interface::as_raw(self),
                state,
            )
        }
    }
    pub(crate) unsafe fn GetDeferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
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
pub struct ICoreWebView2PermissionRequestedEventArgs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Uri:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub PermissionKind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut COREWEBVIEW2_PERMISSION_KIND,
    ) -> windows_core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut COREWEBVIEW2_PERMISSION_STATE,
    ) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        COREWEBVIEW2_PERMISSION_STATE,
    ) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2PermissionRequestedEventHandler,
    ICoreWebView2PermissionRequestedEventHandler_Vtbl,
    0x15e1c6a3_c72a_4df3_91d7_d097fbec6bfd
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2PermissionRequestedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2PermissionRequestedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2PermissionRequestedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2>,
        args: windows_core::Ref<ICoreWebView2PermissionRequestedEventArgs>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2PermissionRequestedEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2PermissionRequestedEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2PermissionRequestedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2PermissionRequestedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2PermissionRequestedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2PermissionRequestedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2ProcessFailedEventArgs,
    ICoreWebView2ProcessFailedEventArgs_Vtbl,
    0x8155a9a4_1474_4a86_8cae_151b0fa6b8ca
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2ProcessFailedEventArgs,
    windows_core::IUnknown
);
impl ICoreWebView2ProcessFailedEventArgs {
    pub(crate) unsafe fn ProcessFailedKind(
        &self,
    ) -> windows_core::Result<COREWEBVIEW2_PROCESS_FAILED_KIND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProcessFailedKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2ProcessFailedEventArgs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProcessFailedKind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut COREWEBVIEW2_PROCESS_FAILED_KIND,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2ProcessFailedEventHandler,
    ICoreWebView2ProcessFailedEventHandler_Vtbl,
    0x79e0aea4_990b_42d9_aa1d_0fcc2e5bc7f1
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2ProcessFailedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2ProcessFailedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2ProcessFailedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2>,
        args: windows_core::Ref<ICoreWebView2ProcessFailedEventArgs>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2ProcessFailedEventHandler_Vtbl {
    pub const fn new<Identity: ICoreWebView2ProcessFailedEventHandler_Impl, const OFFSET: isize>()
    -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2ProcessFailedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2ProcessFailedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2ProcessFailedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2ProcessFailedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2Profile,
    ICoreWebView2Profile_Vtbl,
    0x79110ad3_cd5d_4373_8bc3_c60658f17a5f
);
windows_core::imp::interface_hierarchy!(ICoreWebView2Profile, windows_core::IUnknown);
impl ICoreWebView2Profile {
    pub(crate) unsafe fn ProfileName(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProfileName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn IsInPrivateModeEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInPrivateModeEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn ProfilePath(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProfilePath)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn DefaultDownloadFolderPath(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultDownloadFolderPath)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetDefaultDownloadFolderPath<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultDownloadFolderPath)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn PreferredColorScheme(
        &self,
    ) -> windows_core::Result<COREWEBVIEW2_PREFERRED_COLOR_SCHEME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreferredColorScheme)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetPreferredColorScheme(
        &self,
        value: COREWEBVIEW2_PREFERRED_COLOR_SCHEME,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredColorScheme)(
                windows_core::Interface::as_raw(self),
                value,
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Profile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProfileName:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub IsInPrivateModeEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub ProfilePath:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub DefaultDownloadFolderPath:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub SetDefaultDownloadFolderPath: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub PreferredColorScheme: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut COREWEBVIEW2_PREFERRED_COLOR_SCHEME,
    ) -> windows_core::HRESULT,
    pub SetPreferredColorScheme: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        COREWEBVIEW2_PREFERRED_COLOR_SCHEME,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Profile2,
    ICoreWebView2Profile2_Vtbl,
    0xfa740d4b_5eae_4344_a8ad_74be31925397
);
impl core::ops::Deref for ICoreWebView2Profile2 {
    type Target = ICoreWebView2Profile;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Profile2,
    windows_core::IUnknown,
    ICoreWebView2Profile
);
impl ICoreWebView2Profile2 {
    pub(crate) unsafe fn ClearBrowsingDataAll<P0>(&self, handler: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICoreWebView2ClearBrowsingDataCompletedHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearBrowsingDataAll)(
                windows_core::Interface::as_raw(self),
                handler.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Profile2_Vtbl {
    pub base__: ICoreWebView2Profile_Vtbl,
    ClearBrowsingData: usize,
    ClearBrowsingDataInTimeRange: usize,
    pub ClearBrowsingDataAll: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Settings,
    ICoreWebView2Settings_Vtbl,
    0xe562e4f0_d7fa_43ac_8d71_c05150499f00
);
windows_core::imp::interface_hierarchy!(ICoreWebView2Settings, windows_core::IUnknown);
impl ICoreWebView2Settings {
    pub(crate) unsafe fn IsScriptEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsScriptEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetIsScriptEnabled(&self, isscriptenabled: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsScriptEnabled)(
                windows_core::Interface::as_raw(self),
                isscriptenabled.into(),
            )
        }
    }
    pub(crate) unsafe fn IsWebMessageEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsWebMessageEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetIsWebMessageEnabled(
        &self,
        iswebmessageenabled: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsWebMessageEnabled)(
                windows_core::Interface::as_raw(self),
                iswebmessageenabled.into(),
            )
        }
    }
    pub(crate) unsafe fn AreDefaultScriptDialogsEnabled(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreDefaultScriptDialogsEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetAreDefaultScriptDialogsEnabled(
        &self,
        aredefaultscriptdialogsenabled: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetAreDefaultScriptDialogsEnabled)(
                windows_core::Interface::as_raw(self),
                aredefaultscriptdialogsenabled.into(),
            )
        }
    }
    pub(crate) unsafe fn IsStatusBarEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsStatusBarEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetIsStatusBarEnabled(
        &self,
        isstatusbarenabled: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsStatusBarEnabled)(
                windows_core::Interface::as_raw(self),
                isstatusbarenabled.into(),
            )
        }
    }
    pub(crate) unsafe fn AreDevToolsEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreDevToolsEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetAreDevToolsEnabled(
        &self,
        aredevtoolsenabled: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetAreDevToolsEnabled)(
                windows_core::Interface::as_raw(self),
                aredevtoolsenabled.into(),
            )
        }
    }
    pub(crate) unsafe fn AreDefaultContextMenusEnabled(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreDefaultContextMenusEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetAreDefaultContextMenusEnabled(
        &self,
        enabled: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetAreDefaultContextMenusEnabled)(
                windows_core::Interface::as_raw(self),
                enabled.into(),
            )
        }
    }
    pub(crate) unsafe fn AreHostObjectsAllowed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreHostObjectsAllowed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetAreHostObjectsAllowed(&self, allowed: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetAreHostObjectsAllowed)(
                windows_core::Interface::as_raw(self),
                allowed.into(),
            )
        }
    }
    pub(crate) unsafe fn IsZoomControlEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsZoomControlEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetIsZoomControlEnabled(&self, enabled: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsZoomControlEnabled)(
                windows_core::Interface::as_raw(self),
                enabled.into(),
            )
        }
    }
    pub(crate) unsafe fn IsBuiltInErrorPageEnabled(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsBuiltInErrorPageEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetIsBuiltInErrorPageEnabled(
        &self,
        enabled: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsBuiltInErrorPageEnabled)(
                windows_core::Interface::as_raw(self),
                enabled.into(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Settings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsScriptEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetIsScriptEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub IsWebMessageEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetIsWebMessageEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub AreDefaultScriptDialogsEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetAreDefaultScriptDialogsEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub IsStatusBarEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetIsStatusBarEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub AreDevToolsEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetAreDevToolsEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub AreDefaultContextMenusEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetAreDefaultContextMenusEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub AreHostObjectsAllowed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetAreHostObjectsAllowed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub IsZoomControlEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetIsZoomControlEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub IsBuiltInErrorPageEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetIsBuiltInErrorPageEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Settings2,
    ICoreWebView2Settings2_Vtbl,
    0xee9a0f68_f46c_4e32_ac23_ef8cac224d2a
);
impl core::ops::Deref for ICoreWebView2Settings2 {
    type Target = ICoreWebView2Settings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Settings2,
    windows_core::IUnknown,
    ICoreWebView2Settings
);
impl ICoreWebView2Settings2 {
    pub(crate) unsafe fn UserAgent(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserAgent)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetUserAgent<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetUserAgent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Settings2_Vtbl {
    pub base__: ICoreWebView2Settings_Vtbl,
    pub UserAgent:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub SetUserAgent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Settings3,
    ICoreWebView2Settings3_Vtbl,
    0xfdb5ab74_af33_4854_84f0_0a631deb5eba
);
impl core::ops::Deref for ICoreWebView2Settings3 {
    type Target = ICoreWebView2Settings2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Settings3,
    windows_core::IUnknown,
    ICoreWebView2Settings,
    ICoreWebView2Settings2
);
impl ICoreWebView2Settings3 {
    pub(crate) unsafe fn AreBrowserAcceleratorKeysEnabled(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreBrowserAcceleratorKeysEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetAreBrowserAcceleratorKeysEnabled(
        &self,
        value: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetAreBrowserAcceleratorKeysEnabled)(
                windows_core::Interface::as_raw(self),
                value.into(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Settings3_Vtbl {
    pub base__: ICoreWebView2Settings2_Vtbl,
    pub AreBrowserAcceleratorKeysEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetAreBrowserAcceleratorKeysEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    )
        -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Settings4,
    ICoreWebView2Settings4_Vtbl,
    0xcb56846c_4168_4d53_b04f_03b6d6796ff2
);
impl core::ops::Deref for ICoreWebView2Settings4 {
    type Target = ICoreWebView2Settings3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Settings4,
    windows_core::IUnknown,
    ICoreWebView2Settings,
    ICoreWebView2Settings2,
    ICoreWebView2Settings3
);
impl ICoreWebView2Settings4 {
    pub(crate) unsafe fn IsPasswordAutosaveEnabled(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPasswordAutosaveEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetIsPasswordAutosaveEnabled(&self, value: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsPasswordAutosaveEnabled)(
                windows_core::Interface::as_raw(self),
                value.into(),
            )
        }
    }
    pub(crate) unsafe fn IsGeneralAutofillEnabled(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsGeneralAutofillEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetIsGeneralAutofillEnabled(&self, value: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsGeneralAutofillEnabled)(
                windows_core::Interface::as_raw(self),
                value.into(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Settings4_Vtbl {
    pub base__: ICoreWebView2Settings3_Vtbl,
    pub IsPasswordAutosaveEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetIsPasswordAutosaveEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub IsGeneralAutofillEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetIsGeneralAutofillEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Settings5,
    ICoreWebView2Settings5_Vtbl,
    0x183e7052_1d03_43a0_ab99_98e043b66b39
);
impl core::ops::Deref for ICoreWebView2Settings5 {
    type Target = ICoreWebView2Settings4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Settings5,
    windows_core::IUnknown,
    ICoreWebView2Settings,
    ICoreWebView2Settings2,
    ICoreWebView2Settings3,
    ICoreWebView2Settings4
);
impl ICoreWebView2Settings5 {
    pub(crate) unsafe fn IsPinchZoomEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPinchZoomEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetIsPinchZoomEnabled(&self, value: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsPinchZoomEnabled)(
                windows_core::Interface::as_raw(self),
                value.into(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Settings5_Vtbl {
    pub base__: ICoreWebView2Settings4_Vtbl,
    pub IsPinchZoomEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetIsPinchZoomEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Settings6,
    ICoreWebView2Settings6_Vtbl,
    0x11cb3acd_9bc8_43b8_83bf_f40753714f87
);
impl core::ops::Deref for ICoreWebView2Settings6 {
    type Target = ICoreWebView2Settings5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Settings6,
    windows_core::IUnknown,
    ICoreWebView2Settings,
    ICoreWebView2Settings2,
    ICoreWebView2Settings3,
    ICoreWebView2Settings4,
    ICoreWebView2Settings5
);
impl ICoreWebView2Settings6 {
    pub(crate) unsafe fn IsSwipeNavigationEnabled(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSwipeNavigationEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetIsSwipeNavigationEnabled(&self, value: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsSwipeNavigationEnabled)(
                windows_core::Interface::as_raw(self),
                value.into(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Settings6_Vtbl {
    pub base__: ICoreWebView2Settings5_Vtbl,
    pub IsSwipeNavigationEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetIsSwipeNavigationEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Settings7,
    ICoreWebView2Settings7_Vtbl,
    0x488dc902_35ef_42d2_bc7d_94b65c4bc49c
);
impl core::ops::Deref for ICoreWebView2Settings7 {
    type Target = ICoreWebView2Settings6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Settings7,
    windows_core::IUnknown,
    ICoreWebView2Settings,
    ICoreWebView2Settings2,
    ICoreWebView2Settings3,
    ICoreWebView2Settings4,
    ICoreWebView2Settings5,
    ICoreWebView2Settings6
);
#[repr(C)]
pub struct ICoreWebView2Settings7_Vtbl {
    pub base__: ICoreWebView2Settings6_Vtbl,
    HiddenPdfToolbarItems: usize,
    SetHiddenPdfToolbarItems: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2Settings8,
    ICoreWebView2Settings8_Vtbl,
    0x9e6b0e8f_86ad_4e81_8147_a9b5edb68650
);
impl core::ops::Deref for ICoreWebView2Settings8 {
    type Target = ICoreWebView2Settings7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Settings8,
    windows_core::IUnknown,
    ICoreWebView2Settings,
    ICoreWebView2Settings2,
    ICoreWebView2Settings3,
    ICoreWebView2Settings4,
    ICoreWebView2Settings5,
    ICoreWebView2Settings6,
    ICoreWebView2Settings7
);
#[repr(C)]
pub struct ICoreWebView2Settings8_Vtbl {
    pub base__: ICoreWebView2Settings7_Vtbl,
    IsReputationCheckingRequired: usize,
    SetIsReputationCheckingRequired: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2Settings9,
    ICoreWebView2Settings9_Vtbl,
    0x0528a73b_e92d_49f4_927a_e547dddaa37d
);
impl core::ops::Deref for ICoreWebView2Settings9 {
    type Target = ICoreWebView2Settings8;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2Settings9,
    windows_core::IUnknown,
    ICoreWebView2Settings,
    ICoreWebView2Settings2,
    ICoreWebView2Settings3,
    ICoreWebView2Settings4,
    ICoreWebView2Settings5,
    ICoreWebView2Settings6,
    ICoreWebView2Settings7,
    ICoreWebView2Settings8
);
impl ICoreWebView2Settings9 {
    pub(crate) unsafe fn IsNonClientRegionSupportEnabled(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsNonClientRegionSupportEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetIsNonClientRegionSupportEnabled(
        &self,
        value: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsNonClientRegionSupportEnabled)(
                windows_core::Interface::as_raw(self),
                value.into(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Settings9_Vtbl {
    pub base__: ICoreWebView2Settings8_Vtbl,
    pub IsNonClientRegionSupportEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetIsNonClientRegionSupportEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2StateChangedEventHandler,
    ICoreWebView2StateChangedEventHandler_Vtbl,
    0x81336594_7ede_4ba9_bf71_acf0a95b58dd
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2StateChangedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2StateChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2StateChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2DownloadOperation>,
        args: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2StateChangedEventHandler_Vtbl {
    pub const fn new<Identity: ICoreWebView2StateChangedEventHandler_Impl, const OFFSET: isize>()
    -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2StateChangedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2StateChangedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2StateChangedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2StateChangedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2WebMessageReceivedEventArgs,
    ICoreWebView2WebMessageReceivedEventArgs_Vtbl,
    0x0f99a40c_e962_4207_9e92_e3d542eff849
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2WebMessageReceivedEventArgs,
    windows_core::IUnknown
);
impl ICoreWebView2WebMessageReceivedEventArgs {
    pub(crate) unsafe fn Source(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Source)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn WebMessageAsJson(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WebMessageAsJson)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn TryGetWebMessageAsString(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryGetWebMessageAsString)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2WebMessageReceivedEventArgs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Source:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub WebMessageAsJson:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub TryGetWebMessageAsString:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2WebMessageReceivedEventHandler,
    ICoreWebView2WebMessageReceivedEventHandler_Vtbl,
    0x57213f19_00e6_49fa_8e07_898ea01ecbd2
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2WebMessageReceivedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2WebMessageReceivedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2WebMessageReceivedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2>,
        args: windows_core::Ref<ICoreWebView2WebMessageReceivedEventArgs>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2WebMessageReceivedEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2WebMessageReceivedEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2WebMessageReceivedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2WebMessageReceivedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2WebMessageReceivedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2WebMessageReceivedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2WebResourceRequest,
    ICoreWebView2WebResourceRequest_Vtbl,
    0x97055cd4_512c_4264_8b5f_e3f446cea6a5
);
windows_core::imp::interface_hierarchy!(ICoreWebView2WebResourceRequest, windows_core::IUnknown);
impl ICoreWebView2WebResourceRequest {
    pub(crate) unsafe fn Uri(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn Method(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Method)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn Headers(&self) -> windows_core::Result<ICoreWebView2HttpRequestHeaders> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Headers)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2WebResourceRequest_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Uri:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    SetUri: usize,
    pub Method:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    SetMethod: usize,
    Content: usize,
    SetContent: usize,
    pub Headers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2WebResourceRequestedEventArgs,
    ICoreWebView2WebResourceRequestedEventArgs_Vtbl,
    0x453e667f_12c7_49d4_be6d_ddbe7956f57a
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2WebResourceRequestedEventArgs,
    windows_core::IUnknown
);
impl ICoreWebView2WebResourceRequestedEventArgs {
    pub(crate) unsafe fn Request(&self) -> windows_core::Result<ICoreWebView2WebResourceRequest> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Request)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn SetResponse<P0>(&self, response: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICoreWebView2WebResourceResponse>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetResponse)(
                windows_core::Interface::as_raw(self),
                response.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2WebResourceRequestedEventArgs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Request: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Response: usize,
    pub SetResponse: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetDeferral: usize,
    ResourceContext: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2WebResourceRequestedEventHandler,
    ICoreWebView2WebResourceRequestedEventHandler_Vtbl,
    0xab00b74c_15f1_4646_80e8_e76341d25d71
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2WebResourceRequestedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2WebResourceRequestedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2WebResourceRequestedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2>,
        args: windows_core::Ref<ICoreWebView2WebResourceRequestedEventArgs>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2WebResourceRequestedEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2WebResourceRequestedEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2WebResourceRequestedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2WebResourceRequestedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2WebResourceRequestedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2WebResourceRequestedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2WebResourceResponse,
    ICoreWebView2WebResourceResponse_Vtbl,
    0xaafcc94f_fa27_48fd_97df_830ef75aaec9
);
windows_core::imp::interface_hierarchy!(ICoreWebView2WebResourceResponse, windows_core::IUnknown);
#[repr(C)]
pub struct ICoreWebView2WebResourceResponse_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Content: usize,
    SetContent: usize,
    Headers: usize,
    StatusCode: usize,
    SetStatusCode: usize,
    ReasonPhrase: usize,
    SetReasonPhrase: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2WindowCloseRequestedEventHandler,
    ICoreWebView2WindowCloseRequestedEventHandler_Vtbl,
    0x5c19e9e0_092f_486b_affa_ca8231913039
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2WindowCloseRequestedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct ICoreWebView2WindowCloseRequestedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2WindowCloseRequestedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(
        &self,
        sender: windows_core::Ref<ICoreWebView2>,
        args: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
}
impl ICoreWebView2WindowCloseRequestedEventHandler_Vtbl {
    pub const fn new<
        Identity: ICoreWebView2WindowCloseRequestedEventHandler_Impl,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: ICoreWebView2WindowCloseRequestedEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sender: *mut core::ffi::c_void,
            args: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2WindowCloseRequestedEventHandler_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
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
        iid == &<ICoreWebView2WindowCloseRequestedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWebView2WindowCloseRequestedEventHandler {}
windows_core::imp::define_interface!(
    ICoreWebView2_10,
    ICoreWebView2_10_Vtbl,
    0xb1690564_6f5a_4983_8e48_31d1143fecdb
);
impl core::ops::Deref for ICoreWebView2_10 {
    type Target = ICoreWebView2_9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_10,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8,
    ICoreWebView2_9
);
#[repr(C)]
pub struct ICoreWebView2_10_Vtbl {
    pub base__: ICoreWebView2_9_Vtbl,
    add_BasicAuthenticationRequested: usize,
    remove_BasicAuthenticationRequested: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_11,
    ICoreWebView2_11_Vtbl,
    0x0be78e56_c193_4051_b943_23b460c08bdb
);
impl core::ops::Deref for ICoreWebView2_11 {
    type Target = ICoreWebView2_10;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_11,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8,
    ICoreWebView2_9,
    ICoreWebView2_10
);
#[repr(C)]
pub struct ICoreWebView2_11_Vtbl {
    pub base__: ICoreWebView2_10_Vtbl,
    CallDevToolsProtocolMethodForSession: usize,
    add_ContextMenuRequested: usize,
    remove_ContextMenuRequested: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_12,
    ICoreWebView2_12_Vtbl,
    0x35d69927_bcfa_4566_9349_6b3e0d154cac
);
impl core::ops::Deref for ICoreWebView2_12 {
    type Target = ICoreWebView2_11;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_12,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8,
    ICoreWebView2_9,
    ICoreWebView2_10,
    ICoreWebView2_11
);
#[repr(C)]
pub struct ICoreWebView2_12_Vtbl {
    pub base__: ICoreWebView2_11_Vtbl,
    add_StatusBarTextChanged: usize,
    remove_StatusBarTextChanged: usize,
    StatusBarText: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_13,
    ICoreWebView2_13_Vtbl,
    0xf75f09a8_667e_4983_88d6_c8773f315e84
);
impl core::ops::Deref for ICoreWebView2_13 {
    type Target = ICoreWebView2_12;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_13,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8,
    ICoreWebView2_9,
    ICoreWebView2_10,
    ICoreWebView2_11,
    ICoreWebView2_12
);
impl ICoreWebView2_13 {
    pub(crate) unsafe fn Profile(&self) -> windows_core::Result<ICoreWebView2Profile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Profile)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2_13_Vtbl {
    pub base__: ICoreWebView2_12_Vtbl,
    pub Profile: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2_14,
    ICoreWebView2_14_Vtbl,
    0x6daa4f10_4a90_4753_8898_77c5df534165
);
impl core::ops::Deref for ICoreWebView2_14 {
    type Target = ICoreWebView2_13;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_14,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8,
    ICoreWebView2_9,
    ICoreWebView2_10,
    ICoreWebView2_11,
    ICoreWebView2_12,
    ICoreWebView2_13
);
#[repr(C)]
pub struct ICoreWebView2_14_Vtbl {
    pub base__: ICoreWebView2_13_Vtbl,
    add_ServerCertificateErrorDetected: usize,
    remove_ServerCertificateErrorDetected: usize,
    ClearServerCertificateErrorActions: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_15,
    ICoreWebView2_15_Vtbl,
    0x517b2d1d_7dae_4a66_a4f4_10352ffb9518
);
impl core::ops::Deref for ICoreWebView2_15 {
    type Target = ICoreWebView2_14;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_15,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8,
    ICoreWebView2_9,
    ICoreWebView2_10,
    ICoreWebView2_11,
    ICoreWebView2_12,
    ICoreWebView2_13,
    ICoreWebView2_14
);
#[repr(C)]
pub struct ICoreWebView2_15_Vtbl {
    pub base__: ICoreWebView2_14_Vtbl,
    add_FaviconChanged: usize,
    remove_FaviconChanged: usize,
    FaviconUri: usize,
    GetFavicon: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_16,
    ICoreWebView2_16_Vtbl,
    0x0eb34dc9_9f91_41e1_8639_95cd5943906b
);
impl core::ops::Deref for ICoreWebView2_16 {
    type Target = ICoreWebView2_15;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_16,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8,
    ICoreWebView2_9,
    ICoreWebView2_10,
    ICoreWebView2_11,
    ICoreWebView2_12,
    ICoreWebView2_13,
    ICoreWebView2_14,
    ICoreWebView2_15
);
#[repr(C)]
pub struct ICoreWebView2_16_Vtbl {
    pub base__: ICoreWebView2_15_Vtbl,
    Print: usize,
    ShowPrintUI: usize,
    PrintToPdfStream: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_17,
    ICoreWebView2_17_Vtbl,
    0x702e75d4_fd44_434d_9d70_1a68a6b1192a
);
impl core::ops::Deref for ICoreWebView2_17 {
    type Target = ICoreWebView2_16;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_17,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8,
    ICoreWebView2_9,
    ICoreWebView2_10,
    ICoreWebView2_11,
    ICoreWebView2_12,
    ICoreWebView2_13,
    ICoreWebView2_14,
    ICoreWebView2_15,
    ICoreWebView2_16
);
#[repr(C)]
pub struct ICoreWebView2_17_Vtbl {
    pub base__: ICoreWebView2_16_Vtbl,
    PostSharedBufferToScript: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_18,
    ICoreWebView2_18_Vtbl,
    0x7a626017_28be_49b2_b865_3ba2b3522d90
);
impl core::ops::Deref for ICoreWebView2_18 {
    type Target = ICoreWebView2_17;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_18,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8,
    ICoreWebView2_9,
    ICoreWebView2_10,
    ICoreWebView2_11,
    ICoreWebView2_12,
    ICoreWebView2_13,
    ICoreWebView2_14,
    ICoreWebView2_15,
    ICoreWebView2_16,
    ICoreWebView2_17
);
#[repr(C)]
pub struct ICoreWebView2_18_Vtbl {
    pub base__: ICoreWebView2_17_Vtbl,
    add_LaunchingExternalUriScheme: usize,
    remove_LaunchingExternalUriScheme: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_19,
    ICoreWebView2_19_Vtbl,
    0x6921f954_79b0_437f_a997_c85811897c68
);
impl core::ops::Deref for ICoreWebView2_19 {
    type Target = ICoreWebView2_18;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_19,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8,
    ICoreWebView2_9,
    ICoreWebView2_10,
    ICoreWebView2_11,
    ICoreWebView2_12,
    ICoreWebView2_13,
    ICoreWebView2_14,
    ICoreWebView2_15,
    ICoreWebView2_16,
    ICoreWebView2_17,
    ICoreWebView2_18
);
impl ICoreWebView2_19 {
    pub(crate) unsafe fn MemoryUsageTargetLevel(
        &self,
    ) -> windows_core::Result<COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MemoryUsageTargetLevel)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetMemoryUsageTargetLevel(
        &self,
        value: COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetMemoryUsageTargetLevel)(
                windows_core::Interface::as_raw(self),
                value,
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2_19_Vtbl {
    pub base__: ICoreWebView2_18_Vtbl,
    pub MemoryUsageTargetLevel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL,
    ) -> windows_core::HRESULT,
    pub SetMemoryUsageTargetLevel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2_2,
    ICoreWebView2_2_Vtbl,
    0x9e8f0cf8_e670_4b5e_b2bc_73e061e3184c
);
impl core::ops::Deref for ICoreWebView2_2 {
    type Target = ICoreWebView2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreWebView2_2, windows_core::IUnknown, ICoreWebView2);
impl ICoreWebView2_2 {
    pub(crate) unsafe fn NavigateWithWebResourceRequest<P0>(
        &self,
        request: P0,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICoreWebView2WebResourceRequest>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).NavigateWithWebResourceRequest)(
                windows_core::Interface::as_raw(self),
                request.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn CookieManager(&self) -> windows_core::Result<ICoreWebView2CookieManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CookieManager)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn Environment(&self) -> windows_core::Result<ICoreWebView2Environment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Environment)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2_2_Vtbl {
    pub base__: ICoreWebView2_Vtbl,
    add_WebResourceResponseReceived: usize,
    remove_WebResourceResponseReceived: usize,
    pub NavigateWithWebResourceRequest: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    add_DOMContentLoaded: usize,
    remove_DOMContentLoaded: usize,
    pub CookieManager: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Environment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2_20,
    ICoreWebView2_20_Vtbl,
    0xb4bc1926_7305_11ee_b962_0242ac120002
);
impl core::ops::Deref for ICoreWebView2_20 {
    type Target = ICoreWebView2_19;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_20,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8,
    ICoreWebView2_9,
    ICoreWebView2_10,
    ICoreWebView2_11,
    ICoreWebView2_12,
    ICoreWebView2_13,
    ICoreWebView2_14,
    ICoreWebView2_15,
    ICoreWebView2_16,
    ICoreWebView2_17,
    ICoreWebView2_18,
    ICoreWebView2_19
);
#[repr(C)]
pub struct ICoreWebView2_20_Vtbl {
    pub base__: ICoreWebView2_19_Vtbl,
    FrameId: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_21,
    ICoreWebView2_21_Vtbl,
    0xc4980dea_587b_43b9_8143_3ef3bf552d95
);
impl core::ops::Deref for ICoreWebView2_21 {
    type Target = ICoreWebView2_20;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_21,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8,
    ICoreWebView2_9,
    ICoreWebView2_10,
    ICoreWebView2_11,
    ICoreWebView2_12,
    ICoreWebView2_13,
    ICoreWebView2_14,
    ICoreWebView2_15,
    ICoreWebView2_16,
    ICoreWebView2_17,
    ICoreWebView2_18,
    ICoreWebView2_19,
    ICoreWebView2_20
);
#[repr(C)]
pub struct ICoreWebView2_21_Vtbl {
    pub base__: ICoreWebView2_20_Vtbl,
    ExecuteScriptWithResult: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_22,
    ICoreWebView2_22_Vtbl,
    0xdb75dfc7_a857_4632_a398_6969dde26c0a
);
impl core::ops::Deref for ICoreWebView2_22 {
    type Target = ICoreWebView2_21;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_22,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8,
    ICoreWebView2_9,
    ICoreWebView2_10,
    ICoreWebView2_11,
    ICoreWebView2_12,
    ICoreWebView2_13,
    ICoreWebView2_14,
    ICoreWebView2_15,
    ICoreWebView2_16,
    ICoreWebView2_17,
    ICoreWebView2_18,
    ICoreWebView2_19,
    ICoreWebView2_20,
    ICoreWebView2_21
);
impl ICoreWebView2_22 {
    pub(crate) unsafe fn AddWebResourceRequestedFilterWithRequestSourceKinds<P0>(
        &self,
        uri: P0,
        resourcecontext: COREWEBVIEW2_WEB_RESOURCE_CONTEXT,
        requestsourcekinds: COREWEBVIEW2_WEB_RESOURCE_REQUEST_SOURCE_KINDS,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self)
                .AddWebResourceRequestedFilterWithRequestSourceKinds)(
                windows_core::Interface::as_raw(self),
                uri.param().abi(),
                resourcecontext,
                requestsourcekinds,
            )
        }
    }
    pub(crate) unsafe fn RemoveWebResourceRequestedFilterWithRequestSourceKinds<P0>(
        &self,
        uri: P0,
        resourcecontext: COREWEBVIEW2_WEB_RESOURCE_CONTEXT,
        requestsourcekinds: COREWEBVIEW2_WEB_RESOURCE_REQUEST_SOURCE_KINDS,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self)
                .RemoveWebResourceRequestedFilterWithRequestSourceKinds)(
                windows_core::Interface::as_raw(self),
                uri.param().abi(),
                resourcecontext,
                requestsourcekinds,
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2_22_Vtbl {
    pub base__: ICoreWebView2_21_Vtbl,
    pub AddWebResourceRequestedFilterWithRequestSourceKinds:
        unsafe extern "system" fn(
            *mut core::ffi::c_void,
            windows_core::PCWSTR,
            COREWEBVIEW2_WEB_RESOURCE_CONTEXT,
            COREWEBVIEW2_WEB_RESOURCE_REQUEST_SOURCE_KINDS,
        ) -> windows_core::HRESULT,
    pub RemoveWebResourceRequestedFilterWithRequestSourceKinds:
        unsafe extern "system" fn(
            *mut core::ffi::c_void,
            windows_core::PCWSTR,
            COREWEBVIEW2_WEB_RESOURCE_CONTEXT,
            COREWEBVIEW2_WEB_RESOURCE_REQUEST_SOURCE_KINDS,
        ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2_3,
    ICoreWebView2_3_Vtbl,
    0xa0d6df20_3b92_416d_aa0c_437a9c727857
);
impl core::ops::Deref for ICoreWebView2_3 {
    type Target = ICoreWebView2_2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_3,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2
);
impl ICoreWebView2_3 {
    pub(crate) unsafe fn SetVirtualHostNameToFolderMapping<P0, P1>(
        &self,
        hostname: P0,
        folderpath: P1,
        accesskind: COREWEBVIEW2_HOST_RESOURCE_ACCESS_KIND,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetVirtualHostNameToFolderMapping)(
                windows_core::Interface::as_raw(self),
                hostname.param().abi(),
                folderpath.param().abi(),
                accesskind,
            )
        }
    }
    pub(crate) unsafe fn ClearVirtualHostNameToFolderMapping<P0>(
        &self,
        hostname: P0,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearVirtualHostNameToFolderMapping)(
                windows_core::Interface::as_raw(self),
                hostname.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2_3_Vtbl {
    pub base__: ICoreWebView2_2_Vtbl,
    TrySuspend: usize,
    Resume: usize,
    IsSuspended: usize,
    pub SetVirtualHostNameToFolderMapping: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        COREWEBVIEW2_HOST_RESOURCE_ACCESS_KIND,
    ) -> windows_core::HRESULT,
    pub ClearVirtualHostNameToFolderMapping: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    )
        -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2_4,
    ICoreWebView2_4_Vtbl,
    0x20d02d59_6df2_42dc_bd06_f98a694b1302
);
impl core::ops::Deref for ICoreWebView2_4 {
    type Target = ICoreWebView2_3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_4,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3
);
impl ICoreWebView2_4 {
    pub(crate) unsafe fn add_DownloadStarting<P0>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<ICoreWebView2DownloadStartingEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_DownloadStarting)(
                windows_core::Interface::as_raw(self),
                eventhandler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn remove_DownloadStarting(&self, token: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_DownloadStarting)(
                windows_core::Interface::as_raw(self),
                token,
            )
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2_4_Vtbl {
    pub base__: ICoreWebView2_3_Vtbl,
    add_FrameCreated: usize,
    remove_FrameCreated: usize,
    pub add_DownloadStarting: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_DownloadStarting:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2_5,
    ICoreWebView2_5_Vtbl,
    0xbedb11b8_d63c_11eb_b8bc_0242ac130003
);
impl core::ops::Deref for ICoreWebView2_5 {
    type Target = ICoreWebView2_4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_5,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4
);
#[repr(C)]
pub struct ICoreWebView2_5_Vtbl {
    pub base__: ICoreWebView2_4_Vtbl,
    add_ClientCertificateRequested: usize,
    remove_ClientCertificateRequested: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_6,
    ICoreWebView2_6_Vtbl,
    0x499aadac_d92c_4589_8a75_111bfc167795
);
impl core::ops::Deref for ICoreWebView2_6 {
    type Target = ICoreWebView2_5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_6,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5
);
#[repr(C)]
pub struct ICoreWebView2_6_Vtbl {
    pub base__: ICoreWebView2_5_Vtbl,
    OpenTaskManagerWindow: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_7,
    ICoreWebView2_7_Vtbl,
    0x79c24d83_09a3_45ae_9418_487f32a58740
);
impl core::ops::Deref for ICoreWebView2_7 {
    type Target = ICoreWebView2_6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_7,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6
);
#[repr(C)]
pub struct ICoreWebView2_7_Vtbl {
    pub base__: ICoreWebView2_6_Vtbl,
    PrintToPdf: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_8,
    ICoreWebView2_8_Vtbl,
    0xe9632730_6e1e_43ab_b7b8_7b2c9e62e094
);
impl core::ops::Deref for ICoreWebView2_8 {
    type Target = ICoreWebView2_7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_8,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7
);
#[repr(C)]
pub struct ICoreWebView2_8_Vtbl {
    pub base__: ICoreWebView2_7_Vtbl,
    add_IsMutedChanged: usize,
    remove_IsMutedChanged: usize,
    IsMuted: usize,
    SetIsMuted: usize,
    add_IsDocumentPlayingAudioChanged: usize,
    remove_IsDocumentPlayingAudioChanged: usize,
    IsDocumentPlayingAudio: usize,
}
windows_core::imp::define_interface!(
    ICoreWebView2_9,
    ICoreWebView2_9_Vtbl,
    0x4d7b2eab_9fdc_468d_b998_a9260b5ed651
);
impl core::ops::Deref for ICoreWebView2_9 {
    type Target = ICoreWebView2_8;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ICoreWebView2_9,
    windows_core::IUnknown,
    ICoreWebView2,
    ICoreWebView2_2,
    ICoreWebView2_3,
    ICoreWebView2_4,
    ICoreWebView2_5,
    ICoreWebView2_6,
    ICoreWebView2_7,
    ICoreWebView2_8
);
#[repr(C)]
pub struct ICoreWebView2_9_Vtbl {
    pub base__: ICoreWebView2_8_Vtbl,
    add_IsDefaultDownloadDialogOpenChanged: usize,
    remove_IsDefaultDownloadDialogOpenChanged: usize,
    IsDefaultDownloadDialogOpen: usize,
    OpenDefaultDownloadDialog: usize,
    CloseDefaultDownloadDialog: usize,
    DefaultDownloadDialogCornerAlignment: usize,
    SetDefaultDownloadDialogCornerAlignment: usize,
    DefaultDownloadDialogMargin: usize,
    SetDefaultDownloadDialogMargin: usize,
}
windows_core::imp::define_interface!(
    ISequentialStream,
    ISequentialStream_Vtbl,
    0x0c733a30_2a1c_11ce_ade5_00aa0044773d
);
windows_core::imp::interface_hierarchy!(ISequentialStream, windows_core::IUnknown);
#[repr(C)]
pub struct ISequentialStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Read: usize,
    Write: usize,
}
windows_core::imp::define_interface!(
    IStream,
    IStream_Vtbl,
    0x0000000c_0000_0000_c000_000000000046
);
impl core::ops::Deref for IStream {
    type Target = ISequentialStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStream, windows_core::IUnknown, ISequentialStream);
#[repr(C)]
pub struct IStream_Vtbl {
    pub base__: ISequentialStream_Vtbl,
    Seek: usize,
    SetSize: usize,
    CopyTo: usize,
    Commit: usize,
    Revert: usize,
    LockRegion: usize,
    UnlockRegion: usize,
    Stat: usize,
    Clone: usize,
}
pub type LPARAM = isize;
pub type LPWSTR = *mut u16;
pub type LRESULT = isize;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
pub const RPC_E_CHANGED_MODE: windows_core::HRESULT = windows_core::HRESULT(0x80010106_u32 as _);
pub type WPARAM = usize;
