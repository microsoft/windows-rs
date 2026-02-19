windows_core::imp::define_interface!(IMcpMessageFilterExperimental, IMcpMessageFilterExperimental_Vtbl, 0xc5f8f821_895c_5241_b45a_92e249a7d873);
impl windows_core::RuntimeType for IMcpMessageFilterExperimental {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IMcpMessageFilterExperimental, windows_core::IUnknown, windows_core::IInspectable);
impl IMcpMessageFilterExperimental {
    pub fn Initialize(&self, clientappusermodelid: &windows_core::HSTRING, clientprocessid: u32, serveridentity: &windows_core::HSTRING, servername: &windows_core::HSTRING, serverprocessid: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Initialize)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(clientappusermodelid), clientprocessid, core::mem::transmute_copy(serveridentity), core::mem::transmute_copy(servername), serverprocessid).ok() }
    }
    pub fn OnMessage<P2>(&self, message: &windows_core::HSTRING, direction: McpMessageDirection, filterresponse: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<McpMessageFilterResponse>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(message), direction, filterresponse.param().abi()).ok() }
    }
}
impl windows_core::RuntimeName for IMcpMessageFilterExperimental {
    const NAME: &'static str = "Windows.AI.Agents.Mcp.IMcpMessageFilterExperimental";
}
pub trait IMcpMessageFilterExperimental_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, clientAppUserModelId: &windows_core::HSTRING, clientProcessId: u32, serverIdentity: &windows_core::HSTRING, serverName: &windows_core::HSTRING, serverProcessId: u32) -> windows_core::Result<()>;
    fn OnMessage(&self, message: &windows_core::HSTRING, direction: McpMessageDirection, filterResponse: windows_core::Ref<McpMessageFilterResponse>) -> windows_core::Result<()>;
}
impl IMcpMessageFilterExperimental_Vtbl {
    pub const fn new<Identity: IMcpMessageFilterExperimental_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IMcpMessageFilterExperimental_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientappusermodelid: *mut core::ffi::c_void, clientprocessid: u32, serveridentity: *mut core::ffi::c_void, servername: *mut core::ffi::c_void, serverprocessid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMcpMessageFilterExperimental_Impl::Initialize(this, core::mem::transmute(&clientappusermodelid), clientprocessid, core::mem::transmute(&serveridentity), core::mem::transmute(&servername), serverprocessid).into()
            }
        }
        unsafe extern "system" fn OnMessage<Identity: IMcpMessageFilterExperimental_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut core::ffi::c_void, direction: McpMessageDirection, filterresponse: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMcpMessageFilterExperimental_Impl::OnMessage(this, core::mem::transmute(&message), direction, core::mem::transmute_copy(&filterresponse)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMcpMessageFilterExperimental, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            OnMessage: OnMessage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMcpMessageFilterExperimental as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMcpMessageFilterExperimental_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, McpMessageDirection, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMcpMessageFilterResponse, IMcpMessageFilterResponse_Vtbl, 0x363ce02c_7098_5e13_a408_7b43e1f452ac);
impl windows_core::RuntimeType for IMcpMessageFilterResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMcpMessageFilterResponse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMcpMessageFilterResponseExperimental, IMcpMessageFilterResponseExperimental_Vtbl, 0xe215b5f2_cb02_56cf_aab0_84aef65d1665);
impl windows_core::RuntimeType for IMcpMessageFilterResponseExperimental {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IMcpMessageFilterResponseExperimental, windows_core::IUnknown, windows_core::IInspectable);
impl IMcpMessageFilterResponseExperimental {
    pub fn IsAllowed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAllowed(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsAllowed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MessageIfNotAllowed(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageIfNotAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetMessageIfNotAllowed(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMessageIfNotAllowed)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeName for IMcpMessageFilterResponseExperimental {
    const NAME: &'static str = "Windows.AI.Agents.Mcp.IMcpMessageFilterResponseExperimental";
}
pub trait IMcpMessageFilterResponseExperimental_Impl: windows_core::IUnknownImpl {
    fn IsAllowed(&self) -> windows_core::Result<bool>;
    fn SetIsAllowed(&self, value: bool) -> windows_core::Result<()>;
    fn MessageIfNotAllowed(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetMessageIfNotAllowed(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl IMcpMessageFilterResponseExperimental_Vtbl {
    pub const fn new<Identity: IMcpMessageFilterResponseExperimental_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsAllowed<Identity: IMcpMessageFilterResponseExperimental_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMcpMessageFilterResponseExperimental_Impl::IsAllowed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsAllowed<Identity: IMcpMessageFilterResponseExperimental_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMcpMessageFilterResponseExperimental_Impl::SetIsAllowed(this, value).into()
            }
        }
        unsafe extern "system" fn MessageIfNotAllowed<Identity: IMcpMessageFilterResponseExperimental_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMcpMessageFilterResponseExperimental_Impl::MessageIfNotAllowed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMessageIfNotAllowed<Identity: IMcpMessageFilterResponseExperimental_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMcpMessageFilterResponseExperimental_Impl::SetMessageIfNotAllowed(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMcpMessageFilterResponseExperimental, OFFSET>(),
            IsAllowed: IsAllowed::<Identity, OFFSET>,
            SetIsAllowed: SetIsAllowed::<Identity, OFFSET>,
            MessageIfNotAllowed: MessageIfNotAllowed::<Identity, OFFSET>,
            SetMessageIfNotAllowed: SetMessageIfNotAllowed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMcpMessageFilterResponseExperimental as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMcpMessageFilterResponseExperimental_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub MessageIfNotAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMessageIfNotAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMcpMessageFilterResponseExperimental2, IMcpMessageFilterResponseExperimental2_Vtbl, 0x10f4b099_6632_505a_a638_e704c7e47abf);
impl windows_core::RuntimeType for IMcpMessageFilterResponseExperimental2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IMcpMessageFilterResponseExperimental2, windows_core::IUnknown, windows_core::IInspectable);
impl IMcpMessageFilterResponseExperimental2 {
    pub fn Allow(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Allow)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reject(&self, reason: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Reject)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(reason)).ok() }
    }
}
impl windows_core::RuntimeName for IMcpMessageFilterResponseExperimental2 {
    const NAME: &'static str = "Windows.AI.Agents.Mcp.IMcpMessageFilterResponseExperimental2";
}
pub trait IMcpMessageFilterResponseExperimental2_Impl: windows_core::IUnknownImpl {
    fn Allow(&self) -> windows_core::Result<()>;
    fn Reject(&self, reason: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl IMcpMessageFilterResponseExperimental2_Vtbl {
    pub const fn new<Identity: IMcpMessageFilterResponseExperimental2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Allow<Identity: IMcpMessageFilterResponseExperimental2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMcpMessageFilterResponseExperimental2_Impl::Allow(this).into()
            }
        }
        unsafe extern "system" fn Reject<Identity: IMcpMessageFilterResponseExperimental2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reason: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMcpMessageFilterResponseExperimental2_Impl::Reject(this, core::mem::transmute(&reason)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMcpMessageFilterResponseExperimental2, OFFSET>(),
            Allow: Allow::<Identity, OFFSET>,
            Reject: Reject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMcpMessageFilterResponseExperimental2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMcpMessageFilterResponseExperimental2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Allow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct McpMessageDirection(pub i32);
impl McpMessageDirection {
    pub const ClientToServer: Self = Self(0i32);
    pub const ServerToClient: Self = Self(1i32);
}
impl windows_core::TypeKind for McpMessageDirection {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for McpMessageDirection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.Agents.Mcp.McpMessageDirection;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpMessageFilterResponse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(McpMessageFilterResponse, windows_core::IUnknown, windows_core::IInspectable);
impl McpMessageFilterResponse {}
impl windows_core::RuntimeType for McpMessageFilterResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMcpMessageFilterResponse>();
}
unsafe impl windows_core::Interface for McpMessageFilterResponse {
    type Vtable = <IMcpMessageFilterResponse as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMcpMessageFilterResponse as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for McpMessageFilterResponse {
    const NAME: &'static str = "Windows.AI.Agents.Mcp.McpMessageFilterResponse";
}
unsafe impl Send for McpMessageFilterResponse {}
unsafe impl Sync for McpMessageFilterResponse {}
