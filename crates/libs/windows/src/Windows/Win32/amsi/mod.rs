#[inline]
pub unsafe fn AmsiCloseSession(amsicontext: HAMSICONTEXT, amsisession: HAMSISESSION) {
    windows_core::link!("amsi.dll" "system" fn AmsiCloseSession(amsicontext : HAMSICONTEXT, amsisession : HAMSISESSION));
    unsafe { AmsiCloseSession(amsicontext, amsisession) }
}
#[inline]
pub unsafe fn AmsiInitialize<P0>(appname: P0, amsicontext: *mut HAMSICONTEXT) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("amsi.dll" "system" fn AmsiInitialize(appname : windows_core::PCWSTR, amsicontext : *mut HAMSICONTEXT) -> windows_core::HRESULT);
    unsafe { AmsiInitialize(appname.param().abi(), amsicontext as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AmsiNotifyOperation<P3>(amsicontext: HAMSICONTEXT, buffer: super::PVOID, length: super::ULONG, contentname: P3, result: *mut AMSI_RESULT) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("amsi.dll" "system" fn AmsiNotifyOperation(amsicontext : HAMSICONTEXT, buffer : super::PVOID, length : super::ULONG, contentname : windows_core::PCWSTR, result : *mut AMSI_RESULT) -> windows_core::HRESULT);
    unsafe { AmsiNotifyOperation(amsicontext, buffer, length, contentname.param().abi(), result as _) }
}
#[inline]
pub unsafe fn AmsiOpenSession(amsicontext: HAMSICONTEXT, amsisession: *mut HAMSISESSION) -> windows_core::HRESULT {
    windows_core::link!("amsi.dll" "system" fn AmsiOpenSession(amsicontext : HAMSICONTEXT, amsisession : *mut HAMSISESSION) -> windows_core::HRESULT);
    unsafe { AmsiOpenSession(amsicontext, amsisession as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AmsiScanBuffer<P3>(amsicontext: HAMSICONTEXT, buffer: super::PVOID, length: super::ULONG, contentname: P3, amsisession: Option<HAMSISESSION>, result: *mut AMSI_RESULT) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("amsi.dll" "system" fn AmsiScanBuffer(amsicontext : HAMSICONTEXT, buffer : super::PVOID, length : super::ULONG, contentname : windows_core::PCWSTR, amsisession : HAMSISESSION, result : *mut AMSI_RESULT) -> windows_core::HRESULT);
    unsafe { AmsiScanBuffer(amsicontext, buffer, length, contentname.param().abi(), amsisession.unwrap_or(core::mem::zeroed()) as _, result as _) }
}
#[inline]
pub unsafe fn AmsiScanString<P1, P2>(amsicontext: HAMSICONTEXT, string: P1, contentname: P2, amsisession: Option<HAMSISESSION>, result: *mut AMSI_RESULT) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("amsi.dll" "system" fn AmsiScanString(amsicontext : HAMSICONTEXT, string : windows_core::PCWSTR, contentname : windows_core::PCWSTR, amsisession : HAMSISESSION, result : *mut AMSI_RESULT) -> windows_core::HRESULT);
    unsafe { AmsiScanString(amsicontext, string.param().abi(), contentname.param().abi(), amsisession.unwrap_or(core::mem::zeroed()) as _, result as _) }
}
#[inline]
pub unsafe fn AmsiUninitialize(amsicontext: HAMSICONTEXT) {
    windows_core::link!("amsi.dll" "system" fn AmsiUninitialize(amsicontext : HAMSICONTEXT));
    unsafe { AmsiUninitialize(amsicontext) }
}
pub type AMSI_ATTRIBUTE = i32;
pub const AMSI_ATTRIBUTE_ALL_ADDRESS: AMSI_ATTRIBUTE = 8;
pub const AMSI_ATTRIBUTE_ALL_SIZE: AMSI_ATTRIBUTE = 7;
pub const AMSI_ATTRIBUTE_APP_NAME: AMSI_ATTRIBUTE = 0;
pub const AMSI_ATTRIBUTE_CONTENT_ADDRESS: AMSI_ATTRIBUTE = 3;
pub const AMSI_ATTRIBUTE_CONTENT_NAME: AMSI_ATTRIBUTE = 1;
pub const AMSI_ATTRIBUTE_CONTENT_SIZE: AMSI_ATTRIBUTE = 2;
pub const AMSI_ATTRIBUTE_QUIET: AMSI_ATTRIBUTE = 9;
pub const AMSI_ATTRIBUTE_REDIRECT_CHAIN_ADDRESS: AMSI_ATTRIBUTE = 6;
pub const AMSI_ATTRIBUTE_REDIRECT_CHAIN_SIZE: AMSI_ATTRIBUTE = 5;
pub const AMSI_ATTRIBUTE_SESSION: AMSI_ATTRIBUTE = 4;
pub type AMSI_RESULT = i32;
pub const AMSI_RESULT_BLOCKED_BY_ADMIN_END: AMSI_RESULT = 20479;
pub const AMSI_RESULT_BLOCKED_BY_ADMIN_START: AMSI_RESULT = 16384;
pub const AMSI_RESULT_CLEAN: AMSI_RESULT = 0;
pub const AMSI_RESULT_DETECTED: AMSI_RESULT = 32768;
pub const AMSI_RESULT_NOT_DETECTED: AMSI_RESULT = 1;
pub type AMSI_UAC_MSI_ACTION = i32;
pub const AMSI_UAC_MSI_ACTION_INSTALL: AMSI_UAC_MSI_ACTION = 0;
pub const AMSI_UAC_MSI_ACTION_MAINTENANCE: AMSI_UAC_MSI_ACTION = 3;
pub const AMSI_UAC_MSI_ACTION_MAX: AMSI_UAC_MSI_ACTION = 4;
pub const AMSI_UAC_MSI_ACTION_UNINSTALL: AMSI_UAC_MSI_ACTION = 1;
pub const AMSI_UAC_MSI_ACTION_UPDATE: AMSI_UAC_MSI_ACTION = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AMSI_UAC_REQUEST_AX_INFO {
    pub ulLength: super::ULONG,
    pub lpwszLocalInstallPath: windows_core::PWSTR,
    pub lpwszSourceURL: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AMSI_UAC_REQUEST_COM_INFO {
    pub ulLength: super::ULONG,
    pub lpwszServerBinary: windows_core::PWSTR,
    pub lpwszRequestor: windows_core::PWSTR,
    pub Clsid: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct AMSI_UAC_REQUEST_CONTEXT {
    pub ulLength: super::ULONG,
    pub ulRequestorProcessId: super::ULONG,
    pub UACTrustState: AMSI_UAC_TRUST_STATE,
    pub Type: AMSI_UAC_REQUEST_TYPE,
    pub RequestType: AMSI_UAC_REQUEST_CONTEXT_0,
    pub bAutoElevateRequest: windows_core::BOOL,
}
#[cfg(feature = "shared")]
impl Default for AMSI_UAC_REQUEST_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union AMSI_UAC_REQUEST_CONTEXT_0 {
    pub ExeInfo: AMSI_UAC_REQUEST_EXE_INFO,
    pub ComInfo: AMSI_UAC_REQUEST_COM_INFO,
    pub MsiInfo: AMSI_UAC_REQUEST_MSI_INFO,
    pub ActiveXInfo: AMSI_UAC_REQUEST_AX_INFO,
    pub PackagedAppInfo: AMSI_UAC_REQUEST_PACKAGED_APP_INFO,
}
#[cfg(feature = "shared")]
impl Default for AMSI_UAC_REQUEST_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AMSI_UAC_REQUEST_EXE_INFO {
    pub ulLength: super::ULONG,
    pub lpwszApplicationName: windows_core::PWSTR,
    pub lpwszCommandLine: windows_core::PWSTR,
    pub lpwszDLLParameter: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AMSI_UAC_REQUEST_MSI_INFO {
    pub ulLength: super::ULONG,
    pub MsiAction: AMSI_UAC_MSI_ACTION,
    pub lpwszProductName: windows_core::PWSTR,
    pub lpwszVersion: windows_core::PWSTR,
    pub lpwszLanguage: windows_core::PWSTR,
    pub lpwszManufacturer: windows_core::PWSTR,
    pub lpwszPackagePath: windows_core::PWSTR,
    pub lpwszPackageSource: windows_core::PWSTR,
    pub ulUpdates: super::ULONG,
    pub ppwszUpdates: *mut windows_core::PWSTR,
    pub ppwszUpdateSources: *mut windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AMSI_UAC_REQUEST_PACKAGED_APP_INFO {
    pub ulLength: super::ULONG,
    pub lpwszApplicationName: windows_core::PWSTR,
    pub lpwszCommandLine: windows_core::PWSTR,
    pub lpPackageFamilyName: windows_core::PWSTR,
    pub lpApplicationId: windows_core::PWSTR,
}
pub type AMSI_UAC_REQUEST_TYPE = i32;
pub const AMSI_UAC_REQUEST_TYPE_AX: AMSI_UAC_REQUEST_TYPE = 3;
pub const AMSI_UAC_REQUEST_TYPE_COM: AMSI_UAC_REQUEST_TYPE = 1;
pub const AMSI_UAC_REQUEST_TYPE_EXE: AMSI_UAC_REQUEST_TYPE = 0;
pub const AMSI_UAC_REQUEST_TYPE_MAX: AMSI_UAC_REQUEST_TYPE = 5;
pub const AMSI_UAC_REQUEST_TYPE_MSI: AMSI_UAC_REQUEST_TYPE = 2;
pub const AMSI_UAC_REQUEST_TYPE_PACKAGED_APP: AMSI_UAC_REQUEST_TYPE = 4;
pub type AMSI_UAC_TRUST_STATE = i32;
pub const AMSI_UAC_TRUST_STATE_BLOCKED: AMSI_UAC_TRUST_STATE = 2;
pub const AMSI_UAC_TRUST_STATE_MAX: AMSI_UAC_TRUST_STATE = 3;
pub const AMSI_UAC_TRUST_STATE_TRUSTED: AMSI_UAC_TRUST_STATE = 0;
pub const AMSI_UAC_TRUST_STATE_UNTRUSTED: AMSI_UAC_TRUST_STATE = 1;
pub const CAntimalware: windows_core::GUID = windows_core::GUID::from_u128(0xfdb00e52_a214_4aa1_8fba_4357bb0072ec);
pub type HAMSICONTEXT = *mut HAMSICONTEXT__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HAMSICONTEXT__ {
    pub unused: i32,
}
pub type HAMSISESSION = *mut HAMSISESSION__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HAMSISESSION__ {
    pub unused: i32,
}
windows_core::imp::define_interface!(IAmsiStream, IAmsiStream_Vtbl, 0x3e47f2e5_81d4_4d3b_897f_545096770373);
windows_core::imp::interface_hierarchy!(IAmsiStream, windows_core::IUnknown);
impl IAmsiStream {
    #[cfg(feature = "shared")]
    pub unsafe fn GetAttribute(&self, attribute: AMSI_ATTRIBUTE, datasize: super::ULONG, data: *mut u8, retdata: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAttribute)(windows_core::Interface::as_raw(self), attribute, datasize, data as _, retdata as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Read(&self, position: super::ULONGLONG, size: super::ULONG, buffer: *mut u8, readsize: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), position, size, buffer as _, readsize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAmsiStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, AMSI_ATTRIBUTE, super::ULONG, *mut u8, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAttribute: usize,
    #[cfg(feature = "shared")]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONGLONG, super::ULONG, *mut u8, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Read: usize,
}
#[cfg(feature = "shared")]
pub trait IAmsiStream_Impl: windows_core::IUnknownImpl {
    fn GetAttribute(&self, attribute: AMSI_ATTRIBUTE, datasize: super::ULONG, data: *mut u8, retdata: *mut super::ULONG) -> windows_core::Result<()>;
    fn Read(&self, position: super::ULONGLONG, size: super::ULONG, buffer: *mut u8, readsize: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IAmsiStream_Vtbl {
    pub const fn new<Identity: IAmsiStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAttribute<Identity: IAmsiStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attribute: AMSI_ATTRIBUTE, datasize: super::ULONG, data: *mut u8, retdata: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAmsiStream_Impl::GetAttribute(this, core::mem::transmute_copy(&attribute), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&data), core::mem::transmute_copy(&retdata)).into()
            }
        }
        unsafe extern "system" fn Read<Identity: IAmsiStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: super::ULONGLONG, size: super::ULONG, buffer: *mut u8, readsize: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAmsiStream_Impl::Read(this, core::mem::transmute_copy(&position), core::mem::transmute_copy(&size), core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&readsize)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetAttribute: GetAttribute::<Identity, OFFSET>, Read: Read::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAmsiStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IAmsiStream {}
windows_core::imp::define_interface!(IAntimalware, IAntimalware_Vtbl, 0x82d29c2e_f062_44e6_b5c9_3d9a2f24a2df);
windows_core::imp::interface_hierarchy!(IAntimalware, windows_core::IUnknown);
impl IAntimalware {
    pub unsafe fn Scan<P0>(&self, stream: P0, result: *mut AMSI_RESULT, provider: *mut Option<IAntimalwareProvider>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAmsiStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Scan)(windows_core::Interface::as_raw(self), stream.param().abi(), result as _, core::mem::transmute(provider)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CloseSession(&self, session: super::ULONGLONG) {
        unsafe {
            (windows_core::Interface::vtable(self).CloseSession)(windows_core::Interface::as_raw(self), session);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAntimalware_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Scan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut AMSI_RESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub CloseSession: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONGLONG),
    #[cfg(not(feature = "shared"))]
    CloseSession: usize,
}
#[cfg(feature = "shared")]
pub trait IAntimalware_Impl: windows_core::IUnknownImpl {
    fn Scan(&self, stream: windows_core::Ref<IAmsiStream>, result: *mut AMSI_RESULT, provider: windows_core::OutRef<IAntimalwareProvider>) -> windows_core::Result<()>;
    fn CloseSession(&self, session: super::ULONGLONG);
}
#[cfg(feature = "shared")]
impl IAntimalware_Vtbl {
    pub const fn new<Identity: IAntimalware_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Scan<Identity: IAntimalware_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, result: *mut AMSI_RESULT, provider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAntimalware_Impl::Scan(this, core::mem::transmute_copy(&stream), core::mem::transmute_copy(&result), core::mem::transmute_copy(&provider)).into()
            }
        }
        unsafe extern "system" fn CloseSession<Identity: IAntimalware_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, session: super::ULONGLONG) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAntimalware_Impl::CloseSession(this, core::mem::transmute_copy(&session));
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Scan: Scan::<Identity, OFFSET>, CloseSession: CloseSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAntimalware as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IAntimalware {}
windows_core::imp::define_interface!(IAntimalware2, IAntimalware2_Vtbl, 0x301035b5_2d42_4f56_8c65_2dcaa7fb3cdc);
impl core::ops::Deref for IAntimalware2 {
    type Target = IAntimalware;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAntimalware2, windows_core::IUnknown, IAntimalware);
impl IAntimalware2 {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn Notify<P2, P3>(&self, buffer: super::PVOID, length: super::ULONG, contentname: P2, appname: P3, presult: *mut AMSI_RESULT) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), buffer, length, contentname.param().abi(), appname.param().abi(), presult as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAntimalware2_Vtbl {
    pub base__: IAntimalware_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID, super::ULONG, windows_core::PCWSTR, windows_core::PCWSTR, *mut AMSI_RESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    Notify: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IAntimalware2_Impl: IAntimalware_Impl {
    fn Notify(&self, buffer: super::PVOID, length: super::ULONG, contentname: &windows_core::PCWSTR, appname: &windows_core::PCWSTR, presult: *mut AMSI_RESULT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IAntimalware2_Vtbl {
    pub const fn new<Identity: IAntimalware2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Notify<Identity: IAntimalware2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: super::PVOID, length: super::ULONG, contentname: windows_core::PCWSTR, appname: windows_core::PCWSTR, presult: *mut AMSI_RESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAntimalware2_Impl::Notify(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&length), core::mem::transmute(&contentname), core::mem::transmute(&appname), core::mem::transmute_copy(&presult)).into()
            }
        }
        Self { base__: IAntimalware_Vtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAntimalware2 as windows_core::Interface>::IID || iid == &<IAntimalware as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IAntimalware2 {}
windows_core::imp::define_interface!(IAntimalwareProvider, IAntimalwareProvider_Vtbl, 0xb2cabfe3_fe04_42b1_a5df_08d483d4d125);
windows_core::imp::interface_hierarchy!(IAntimalwareProvider, windows_core::IUnknown);
impl IAntimalwareProvider {
    pub unsafe fn Scan<P0>(&self, stream: P0, result: *mut AMSI_RESULT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAmsiStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Scan)(windows_core::Interface::as_raw(self), stream.param().abi(), result as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CloseSession(&self, session: super::ULONGLONG) {
        unsafe {
            (windows_core::Interface::vtable(self).CloseSession)(windows_core::Interface::as_raw(self), session);
        }
    }
    pub unsafe fn DisplayName(&self, displayname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), displayname as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAntimalwareProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Scan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut AMSI_RESULT) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub CloseSession: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONGLONG),
    #[cfg(not(feature = "shared"))]
    CloseSession: usize,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IAntimalwareProvider_Impl: windows_core::IUnknownImpl {
    fn Scan(&self, stream: windows_core::Ref<IAmsiStream>, result: *mut AMSI_RESULT) -> windows_core::Result<()>;
    fn CloseSession(&self, session: super::ULONGLONG);
    fn DisplayName(&self, displayname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IAntimalwareProvider_Vtbl {
    pub const fn new<Identity: IAntimalwareProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Scan<Identity: IAntimalwareProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, result: *mut AMSI_RESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAntimalwareProvider_Impl::Scan(this, core::mem::transmute_copy(&stream), core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn CloseSession<Identity: IAntimalwareProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, session: super::ULONGLONG) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAntimalwareProvider_Impl::CloseSession(this, core::mem::transmute_copy(&session));
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IAntimalwareProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAntimalwareProvider_Impl::DisplayName(this, core::mem::transmute_copy(&displayname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Scan: Scan::<Identity, OFFSET>,
            CloseSession: CloseSession::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAntimalwareProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IAntimalwareProvider {}
windows_core::imp::define_interface!(IAntimalwareProvider2, IAntimalwareProvider2_Vtbl, 0x7c1e6570_3f73_4e0f_8ad4_98b94cd3290f);
impl core::ops::Deref for IAntimalwareProvider2 {
    type Target = IAntimalwareProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAntimalwareProvider2, windows_core::IUnknown, IAntimalwareProvider);
impl IAntimalwareProvider2 {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn Notify<P2, P3>(&self, buffer: super::PVOID, length: super::ULONG, contentname: P2, appname: P3, presult: *mut AMSI_RESULT) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), buffer, length, contentname.param().abi(), appname.param().abi(), presult as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAntimalwareProvider2_Vtbl {
    pub base__: IAntimalwareProvider_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, super::PVOID, super::ULONG, windows_core::PCWSTR, windows_core::PCWSTR, *mut AMSI_RESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    Notify: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IAntimalwareProvider2_Impl: IAntimalwareProvider_Impl {
    fn Notify(&self, buffer: super::PVOID, length: super::ULONG, contentname: &windows_core::PCWSTR, appname: &windows_core::PCWSTR, presult: *mut AMSI_RESULT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IAntimalwareProvider2_Vtbl {
    pub const fn new<Identity: IAntimalwareProvider2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Notify<Identity: IAntimalwareProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: super::PVOID, length: super::ULONG, contentname: windows_core::PCWSTR, appname: windows_core::PCWSTR, presult: *mut AMSI_RESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAntimalwareProvider2_Impl::Notify(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&length), core::mem::transmute(&contentname), core::mem::transmute(&appname), core::mem::transmute_copy(&presult)).into()
            }
        }
        Self { base__: IAntimalwareProvider_Vtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAntimalwareProvider2 as windows_core::Interface>::IID || iid == &<IAntimalwareProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IAntimalwareProvider2 {}
windows_core::imp::define_interface!(IAntimalwareUacProvider, IAntimalwareUacProvider_Vtbl, 0xb2cabfe4_fe04_42b1_a5df_08d483d4d125);
windows_core::imp::interface_hierarchy!(IAntimalwareUacProvider, windows_core::IUnknown);
impl IAntimalwareUacProvider {
    #[cfg(feature = "shared")]
    pub unsafe fn UacScan(&self, context: LPAMSI_UAC_REQUEST_CONTEXT, result: *mut AMSI_RESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UacScan)(windows_core::Interface::as_raw(self), context, result as _) }
    }
    pub unsafe fn DisplayName(&self, displayname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), displayname as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAntimalwareUacProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub UacScan: unsafe extern "system" fn(*mut core::ffi::c_void, LPAMSI_UAC_REQUEST_CONTEXT, *mut AMSI_RESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UacScan: usize,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IAntimalwareUacProvider_Impl: windows_core::IUnknownImpl {
    fn UacScan(&self, context: LPAMSI_UAC_REQUEST_CONTEXT, result: *mut AMSI_RESULT) -> windows_core::Result<()>;
    fn DisplayName(&self, displayname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IAntimalwareUacProvider_Vtbl {
    pub const fn new<Identity: IAntimalwareUacProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UacScan<Identity: IAntimalwareUacProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: LPAMSI_UAC_REQUEST_CONTEXT, result: *mut AMSI_RESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAntimalwareUacProvider_Impl::UacScan(this, core::mem::transmute_copy(&context), core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IAntimalwareUacProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAntimalwareUacProvider_Impl::DisplayName(this, core::mem::transmute_copy(&displayname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UacScan: UacScan::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAntimalwareUacProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IAntimalwareUacProvider {}
#[cfg(feature = "shared")]
pub type LPAMSI_UAC_REQUEST_AX_INFO = *mut AMSI_UAC_REQUEST_AX_INFO;
#[cfg(feature = "shared")]
pub type LPAMSI_UAC_REQUEST_COM_INFO = *mut AMSI_UAC_REQUEST_COM_INFO;
#[cfg(feature = "shared")]
pub type LPAMSI_UAC_REQUEST_CONTEXT = *mut AMSI_UAC_REQUEST_CONTEXT;
#[cfg(feature = "shared")]
pub type LPAMSI_UAC_REQUEST_EXE_INFO = *mut AMSI_UAC_REQUEST_EXE_INFO;
#[cfg(feature = "shared")]
pub type LPAMSI_UAC_REQUEST_MSI_INFO = *mut AMSI_UAC_REQUEST_MSI_INFO;
#[cfg(feature = "shared")]
pub type LPAMSI_UAC_REQUEST_PACKAGED_APP_INFO = *mut AMSI_UAC_REQUEST_PACKAGED_APP_INFO;
