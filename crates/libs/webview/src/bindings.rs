windows_core::link!("combase.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
windows_core::link!("combase.dll" "system" fn CoTaskMemFree(pv : *const core::ffi::c_void));
windows_core::link!("webview2loader.dll" "C" fn CreateCoreWebView2Environment(environmentcreatedhandler : *mut core::ffi::c_void) -> windows_core::HRESULT);
windows_core::link!("webview2loader.dll" "C" fn CreateCoreWebView2EnvironmentWithOptions(browserexecutablefolder : PCWSTR, userdatafolder : PCWSTR, environmentoptions : *mut core::ffi::c_void, environmentcreatedhandler : *mut core::ffi::c_void) -> windows_core::HRESULT);
windows_core::link!("user32.dll" "system" fn DispatchMessageW(lpmsg : *const MSG) -> LRESULT);
windows_core::link!("user32.dll" "system" fn GetMessageW(lpmsg : *mut MSG, hwnd : HWND, wmsgfiltermin : u32, wmsgfiltermax : u32) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn TranslateMessage(lpmsg : *const MSG) -> windows_core::BOOL);
pub type COREWEBVIEW2_PERMISSION_KIND = i32;
pub type COREWEBVIEW2_PERMISSION_STATE = i32;
pub const E_OUTOFMEMORY: windows_core::HRESULT = windows_core::HRESULT(0x8007000E_u32 as _);
pub type HWND = *mut core::ffi::c_void;
windows_core::imp::define_interface!(
    ICoreWebView2,
    ICoreWebView2_Vtbl,
    0x76eceacb_0462_4d94_ac83_423a6793775e
);
windows_core::imp::interface_hierarchy!(ICoreWebView2, windows_core::IUnknown);
impl ICoreWebView2 {
    pub unsafe fn Settings(&self) -> windows_core::Result<ICoreWebView2Settings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Settings)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Source(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Source)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Navigate(&self, uri: LPCWSTR) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Navigate)(
                windows_core::Interface::as_raw(self),
                uri,
            )
            .ok()
        }
    }
    pub unsafe fn NavigateToString(&self, htmlcontent: LPCWSTR) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).NavigateToString)(
                windows_core::Interface::as_raw(self),
                htmlcontent,
            )
            .ok()
        }
    }
    pub unsafe fn add_NavigationStarting<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
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
    pub unsafe fn remove_NavigationStarting(&self, token: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).remove_NavigationStarting)(
                windows_core::Interface::as_raw(self),
                token,
            )
            .ok()
        }
    }
    pub unsafe fn add_ContentLoading<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
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
    pub unsafe fn remove_ContentLoading(&self, token: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).remove_ContentLoading)(
                windows_core::Interface::as_raw(self),
                token,
            )
            .ok()
        }
    }
    pub unsafe fn add_NavigationCompleted<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
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
    pub unsafe fn remove_NavigationCompleted(&self, token: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).remove_NavigationCompleted)(
                windows_core::Interface::as_raw(self),
                token,
            )
            .ok()
        }
    }
    pub unsafe fn add_PermissionRequested<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
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
    pub unsafe fn remove_PermissionRequested(&self, token: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).remove_PermissionRequested)(
                windows_core::Interface::as_raw(self),
                token,
            )
            .ok()
        }
    }
    pub unsafe fn AddScriptToExecuteOnDocumentCreated<P1>(
        &self,
        javascript: LPCWSTR,
        handler: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddScriptToExecuteOnDocumentCreated)(
                windows_core::Interface::as_raw(self),
                javascript,
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn RemoveScriptToExecuteOnDocumentCreated(
        &self,
        id: LPCWSTR,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveScriptToExecuteOnDocumentCreated)(
                windows_core::Interface::as_raw(self),
                id,
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
    pub unsafe fn Reload(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Reload)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub unsafe fn PostWebMessageAsJson(
        &self,
        webmessageasjson: LPCWSTR,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).PostWebMessageAsJson)(
                windows_core::Interface::as_raw(self),
                webmessageasjson,
            )
            .ok()
        }
    }
    pub unsafe fn PostWebMessageAsString(
        &self,
        webmessageasstring: LPCWSTR,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).PostWebMessageAsString)(
                windows_core::Interface::as_raw(self),
                webmessageasstring,
            )
            .ok()
        }
    }
    pub unsafe fn add_WebMessageReceived<P0>(&self, handler: P0) -> windows_core::Result<i64>
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
    pub unsafe fn remove_WebMessageReceived(&self, token: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).remove_WebMessageReceived)(
                windows_core::Interface::as_raw(self),
                token,
            )
            .ok()
        }
    }
    pub unsafe fn GoBack(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GoBack)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub unsafe fn GoForward(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GoForward)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
        }
    }
    pub unsafe fn add_NewWindowRequested<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
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
    pub unsafe fn remove_NewWindowRequested(&self, token: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).remove_NewWindowRequested)(
                windows_core::Interface::as_raw(self),
                token,
            )
            .ok()
        }
    }
    pub unsafe fn add_DocumentTitleChanged<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
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
    pub unsafe fn remove_DocumentTitleChanged(&self, token: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).remove_DocumentTitleChanged)(
                windows_core::Interface::as_raw(self),
                token,
            )
            .ok()
        }
    }
    pub unsafe fn DocumentTitle(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DocumentTitle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn add_WindowCloseRequested<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
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
    pub unsafe fn remove_WindowCloseRequested(&self, token: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).remove_WindowCloseRequested)(
                windows_core::Interface::as_raw(self),
                token,
            )
            .ok()
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
    pub Navigate:
        unsafe extern "system" fn(*mut core::ffi::c_void, LPCWSTR) -> windows_core::HRESULT,
    pub NavigateToString:
        unsafe extern "system" fn(*mut core::ffi::c_void, LPCWSTR) -> windows_core::HRESULT,
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
    add_ProcessFailed: usize,
    remove_ProcessFailed: usize,
    pub AddScriptToExecuteOnDocumentCreated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        LPCWSTR,
        *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    pub RemoveScriptToExecuteOnDocumentCreated:
        unsafe extern "system" fn(*mut core::ffi::c_void, LPCWSTR) -> windows_core::HRESULT,
    pub ExecuteScript: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        LPCWSTR,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CapturePreview: usize,
    pub Reload: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PostWebMessageAsJson:
        unsafe extern "system" fn(*mut core::ffi::c_void, LPCWSTR) -> windows_core::HRESULT,
    pub PostWebMessageAsString:
        unsafe extern "system" fn(*mut core::ffi::c_void, LPCWSTR) -> windows_core::HRESULT,
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
    OpenDevToolsWindow: usize,
    add_ContainsFullScreenElementChanged: usize,
    remove_ContainsFullScreenElementChanged: usize,
    ContainsFullScreenElement: usize,
    add_WebResourceRequested: usize,
    remove_WebResourceRequested: usize,
    AddWebResourceRequestedFilter: usize,
    RemoveWebResourceRequestedFilter: usize,
    pub add_WindowCloseRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_WindowCloseRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler,
    ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler_Vtbl,
    0xb99369f3_9b11_47b5_bc6f_8e7895fcea17
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler,
    windows_core::IUnknown
);
impl ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler {
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
pub struct ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::HRESULT,
        LPCWSTR,
    ) -> windows_core::HRESULT,
}
pub trait ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler_Impl:
    windows_core::IUnknownImpl
{
    fn Invoke(&self, errorcode: windows_core::HRESULT, result: LPCWSTR)
    -> windows_core::Result<()>;
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
            result: LPCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler_Impl::Invoke(
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
        iid == & < ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler as windows_core::Interface >::IID
    }
}
impl windows_core::RuntimeName
    for ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler
{
}
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
    pub unsafe fn IsErrorPage(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsErrorPage)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn NavigationId(&self) -> windows_core::Result<UINT64> {
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
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut UINT64) -> windows_core::HRESULT,
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
impl ICoreWebView2ContentLoadingEventHandler {
    pub unsafe fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICoreWebView2>,
        P1: windows_core::Param<ICoreWebView2ContentLoadingEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                sender.param().abi(),
                args.param().abi(),
            )
            .ok()
        }
    }
}
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
    ICoreWebView2Deferral,
    ICoreWebView2Deferral_Vtbl,
    0xc10e7f7b_b585_46f0_a623_8befbf3e4ee0
);
windows_core::imp::interface_hierarchy!(ICoreWebView2Deferral, windows_core::IUnknown);
impl ICoreWebView2Deferral {
    pub unsafe fn Complete(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self))
                .ok()
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
impl ICoreWebView2DocumentTitleChangedEventHandler {
    pub unsafe fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICoreWebView2>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                sender.param().abi(),
                args.param().abi(),
            )
            .ok()
        }
    }
}
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
    ICoreWebView2EnvironmentOptions,
    ICoreWebView2EnvironmentOptions_Vtbl,
    0x2fde08a8_1e9a_4766_8c05_95a9ceb9d1c5
);
windows_core::imp::interface_hierarchy!(ICoreWebView2EnvironmentOptions, windows_core::IUnknown);
impl ICoreWebView2EnvironmentOptions {
    pub unsafe fn AdditionalBrowserArguments(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdditionalBrowserArguments)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetAdditionalBrowserArguments(&self, value: LPCWSTR) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAdditionalBrowserArguments)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub unsafe fn Language(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Language)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetLanguage(&self, value: LPCWSTR) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetLanguage)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub unsafe fn TargetCompatibleBrowserVersion(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetCompatibleBrowserVersion)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetTargetCompatibleBrowserVersion(
        &self,
        value: LPCWSTR,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTargetCompatibleBrowserVersion)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub unsafe fn AllowSingleSignOnUsingOSPrimaryAccount(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowSingleSignOnUsingOSPrimaryAccount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetAllowSingleSignOnUsingOSPrimaryAccount(
        &self,
        allow: bool,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAllowSingleSignOnUsingOSPrimaryAccount)(
                windows_core::Interface::as_raw(self),
                allow.into(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2EnvironmentOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdditionalBrowserArguments:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub SetAdditionalBrowserArguments:
        unsafe extern "system" fn(*mut core::ffi::c_void, LPCWSTR) -> windows_core::HRESULT,
    pub Language:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub SetLanguage:
        unsafe extern "system" fn(*mut core::ffi::c_void, LPCWSTR) -> windows_core::HRESULT,
    pub TargetCompatibleBrowserVersion:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPWSTR) -> windows_core::HRESULT,
    pub SetTargetCompatibleBrowserVersion:
        unsafe extern "system" fn(*mut core::ffi::c_void, LPCWSTR) -> windows_core::HRESULT,
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
    fn SetAdditionalBrowserArguments(&self, value: LPCWSTR) -> windows_core::Result<()>;
    fn Language(&self) -> windows_core::Result<LPWSTR>;
    fn SetLanguage(&self, value: LPCWSTR) -> windows_core::Result<()>;
    fn TargetCompatibleBrowserVersion(&self) -> windows_core::Result<LPWSTR>;
    fn SetTargetCompatibleBrowserVersion(&self, value: LPCWSTR) -> windows_core::Result<()>;
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
            value: LPCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2EnvironmentOptions_Impl::SetAdditionalBrowserArguments(
                    this,
                    core::mem::transmute_copy(&value),
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
            value: LPCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2EnvironmentOptions_Impl::SetLanguage(
                    this,
                    core::mem::transmute_copy(&value),
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
            value: LPCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWebView2EnvironmentOptions_Impl::SetTargetCompatibleBrowserVersion(
                    this,
                    core::mem::transmute_copy(&value),
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
    pub unsafe fn IsSuccess(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSuccess)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn NavigationId(&self) -> windows_core::Result<UINT64> {
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
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut UINT64) -> windows_core::HRESULT,
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
impl ICoreWebView2NavigationCompletedEventHandler {
    pub unsafe fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICoreWebView2>,
        P1: windows_core::Param<ICoreWebView2NavigationCompletedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                sender.param().abi(),
                args.param().abi(),
            )
            .ok()
        }
    }
}
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
    pub unsafe fn Uri(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn IsUserInitiated(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUserInitiated)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn IsRedirected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRedirected)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cancel)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetCancel(&self, cancel: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCancel)(
                windows_core::Interface::as_raw(self),
                cancel.into(),
            )
            .ok()
        }
    }
    pub unsafe fn NavigationId(&self) -> windows_core::Result<UINT64> {
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
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut UINT64) -> windows_core::HRESULT,
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
impl ICoreWebView2NavigationStartingEventHandler {
    pub unsafe fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICoreWebView2>,
        P1: windows_core::Param<ICoreWebView2NavigationStartingEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                sender.param().abi(),
                args.param().abi(),
            )
            .ok()
        }
    }
}
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
    pub unsafe fn Uri(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetNewWindow<P0>(&self, newwindow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICoreWebView2>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetNewWindow)(
                windows_core::Interface::as_raw(self),
                newwindow.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn NewWindow(&self) -> windows_core::Result<ICoreWebView2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewWindow)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetHandled(&self, handled: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHandled)(
                windows_core::Interface::as_raw(self),
                handled.into(),
            )
            .ok()
        }
    }
    pub unsafe fn Handled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn IsUserInitiated(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUserInitiated)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetDeferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
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
    pub NewWindow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
impl ICoreWebView2NewWindowRequestedEventHandler {
    pub unsafe fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICoreWebView2>,
        P1: windows_core::Param<ICoreWebView2NewWindowRequestedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                sender.param().abi(),
                args.param().abi(),
            )
            .ok()
        }
    }
}
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
    pub unsafe fn Uri(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uri)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn PermissionKind(&self) -> windows_core::Result<COREWEBVIEW2_PERMISSION_KIND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermissionKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn IsUserInitiated(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUserInitiated)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<COREWEBVIEW2_PERMISSION_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetState(
        &self,
        state: COREWEBVIEW2_PERMISSION_STATE,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetState)(
                windows_core::Interface::as_raw(self),
                state,
            )
            .ok()
        }
    }
    pub unsafe fn GetDeferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
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
impl ICoreWebView2PermissionRequestedEventHandler {
    pub unsafe fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICoreWebView2>,
        P1: windows_core::Param<ICoreWebView2PermissionRequestedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                sender.param().abi(),
                args.param().abi(),
            )
            .ok()
        }
    }
}
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
    ICoreWebView2Settings,
    ICoreWebView2Settings_Vtbl,
    0xe562e4f0_d7fa_43ac_8d71_c05150499f00
);
windows_core::imp::interface_hierarchy!(ICoreWebView2Settings, windows_core::IUnknown);
impl ICoreWebView2Settings {
    pub unsafe fn IsScriptEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsScriptEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetIsScriptEnabled(&self, isscriptenabled: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsScriptEnabled)(
                windows_core::Interface::as_raw(self),
                isscriptenabled.into(),
            )
            .ok()
        }
    }
    pub unsafe fn IsWebMessageEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsWebMessageEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetIsWebMessageEnabled(
        &self,
        iswebmessageenabled: bool,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsWebMessageEnabled)(
                windows_core::Interface::as_raw(self),
                iswebmessageenabled.into(),
            )
            .ok()
        }
    }
    pub unsafe fn AreDefaultScriptDialogsEnabled(
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
    pub unsafe fn SetAreDefaultScriptDialogsEnabled(
        &self,
        aredefaultscriptdialogsenabled: bool,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAreDefaultScriptDialogsEnabled)(
                windows_core::Interface::as_raw(self),
                aredefaultscriptdialogsenabled.into(),
            )
            .ok()
        }
    }
    pub unsafe fn IsStatusBarEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsStatusBarEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetIsStatusBarEnabled(
        &self,
        isstatusbarenabled: bool,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsStatusBarEnabled)(
                windows_core::Interface::as_raw(self),
                isstatusbarenabled.into(),
            )
            .ok()
        }
    }
    pub unsafe fn AreDevToolsEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreDevToolsEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetAreDevToolsEnabled(
        &self,
        aredevtoolsenabled: bool,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAreDevToolsEnabled)(
                windows_core::Interface::as_raw(self),
                aredevtoolsenabled.into(),
            )
            .ok()
        }
    }
    pub unsafe fn AreDefaultContextMenusEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreDefaultContextMenusEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetAreDefaultContextMenusEnabled(
        &self,
        enabled: bool,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAreDefaultContextMenusEnabled)(
                windows_core::Interface::as_raw(self),
                enabled.into(),
            )
            .ok()
        }
    }
    pub unsafe fn AreHostObjectsAllowed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreHostObjectsAllowed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetAreHostObjectsAllowed(&self, allowed: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAreHostObjectsAllowed)(
                windows_core::Interface::as_raw(self),
                allowed.into(),
            )
            .ok()
        }
    }
    pub unsafe fn IsZoomControlEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsZoomControlEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetIsZoomControlEnabled(&self, enabled: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsZoomControlEnabled)(
                windows_core::Interface::as_raw(self),
                enabled.into(),
            )
            .ok()
        }
    }
    pub unsafe fn IsBuiltInErrorPageEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsBuiltInErrorPageEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetIsBuiltInErrorPageEnabled(&self, enabled: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsBuiltInErrorPageEnabled)(
                windows_core::Interface::as_raw(self),
                enabled.into(),
            )
            .ok()
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
    ICoreWebView2WebMessageReceivedEventArgs,
    ICoreWebView2WebMessageReceivedEventArgs_Vtbl,
    0x0f99a40c_e962_4207_9e92_e3d542eff849
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2WebMessageReceivedEventArgs,
    windows_core::IUnknown
);
impl ICoreWebView2WebMessageReceivedEventArgs {
    pub unsafe fn Source(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Source)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn WebMessageAsJson(&self) -> windows_core::Result<LPWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WebMessageAsJson)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn TryGetWebMessageAsString(&self) -> windows_core::Result<LPWSTR> {
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
impl ICoreWebView2WebMessageReceivedEventHandler {
    pub unsafe fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICoreWebView2>,
        P1: windows_core::Param<ICoreWebView2WebMessageReceivedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                sender.param().abi(),
                args.param().abi(),
            )
            .ok()
        }
    }
}
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
    ICoreWebView2WindowCloseRequestedEventHandler,
    ICoreWebView2WindowCloseRequestedEventHandler_Vtbl,
    0x5c19e9e0_092f_486b_affa_ca8231913039
);
windows_core::imp::interface_hierarchy!(
    ICoreWebView2WindowCloseRequestedEventHandler,
    windows_core::IUnknown
);
impl ICoreWebView2WindowCloseRequestedEventHandler {
    pub unsafe fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICoreWebView2>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                sender.param().abi(),
                args.param().abi(),
            )
            .ok()
        }
    }
}
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
pub type LPARAM = isize;
pub type LPCWSTR = *const WCHAR;
pub type LPWSTR = *mut WCHAR;
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
pub type PCWSTR = *const WCHAR;
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
pub type UINT64 = u64;
pub type WCHAR = u16;
pub type WPARAM = usize;
