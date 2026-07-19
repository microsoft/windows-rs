#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AccNotifyTouchInteraction(hwndapp: super::HWND, hwndtarget: super::HWND, pttarget: super::POINT) -> windows_core::HRESULT {
    windows_core::link!("oleacc.dll" "system" fn AccNotifyTouchInteraction(hwndapp : super::HWND, hwndtarget : super::HWND, pttarget : super::POINT) -> windows_core::HRESULT);
    unsafe { AccNotifyTouchInteraction(hwndapp, hwndtarget, pttarget) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AccSetRunningUtilityState(hwndapp: super::HWND, dwutilitystatemask: u32, dwutilitystate: u32) -> windows_core::HRESULT {
    windows_core::link!("oleacc.dll" "system" fn AccSetRunningUtilityState(hwndapp : super::HWND, dwutilitystatemask : u32, dwutilitystate : u32) -> windows_core::HRESULT);
    unsafe { AccSetRunningUtilityState(hwndapp, dwutilitystatemask, dwutilitystate) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn AccessibleChildren<P0>(pacccontainer: P0, ichildstart: i32, rgvarchildren: &mut [super::VARIANT], pcobtained: *mut i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAccessible>,
{
    windows_core::link!("oleacc.dll" "system" fn AccessibleChildren(pacccontainer : *mut core::ffi::c_void, ichildstart : i32, cchildren : i32, rgvarchildren : *mut super::VARIANT, pcobtained : *mut i32) -> windows_core::HRESULT);
    unsafe { AccessibleChildren(pacccontainer.param().abi(), ichildstart, rgvarchildren.len().try_into().unwrap(), rgvarchildren.as_mut_ptr(), pcobtained as _) }
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn AccessibleObjectFromEvent(hwnd: super::HWND, dwid: u32, dwchildid: u32, ppacc: *mut Option<IAccessible>, pvarchild: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleacc.dll" "system" fn AccessibleObjectFromEvent(hwnd : super::HWND, dwid : u32, dwchildid : u32, ppacc : *mut *mut core::ffi::c_void, pvarchild : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { AccessibleObjectFromEvent(hwnd, dwid, dwchildid, core::mem::transmute(ppacc), pvarchild) }
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn AccessibleObjectFromPoint(ptscreen: super::POINT, ppacc: *mut Option<IAccessible>, pvarchild: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleacc.dll" "system" fn AccessibleObjectFromPoint(ptscreen : super::POINT, ppacc : *mut *mut core::ffi::c_void, pvarchild : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { AccessibleObjectFromPoint(ptscreen, core::mem::transmute(ppacc), pvarchild) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AccessibleObjectFromWindow<T>(hwnd: super::HWND, dwid: u32) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("oleacc.dll" "system" fn AccessibleObjectFromWindow(hwnd : super::HWND, dwid : u32, riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { AccessibleObjectFromWindow(hwnd, dwid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateStdAccessibleObject<T>(hwnd: super::HWND, idobject: i32) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("oleacc.dll" "system" fn CreateStdAccessibleObject(hwnd : super::HWND, idobject : i32, riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateStdAccessibleObject(hwnd, idobject, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateStdAccessibleProxyA<P1, T>(hwnd: super::HWND, pclassname: P1, idobject: i32) -> windows_core::Result<T>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    T: windows_core::Interface,
{
    windows_core::link!("oleacc.dll" "system" fn CreateStdAccessibleProxyA(hwnd : super::HWND, pclassname : windows_core::PCSTR, idobject : i32, riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateStdAccessibleProxyA(hwnd, pclassname.param().abi(), idobject, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateStdAccessibleProxyW<P1, T>(hwnd: super::HWND, pclassname: P1, idobject: i32) -> windows_core::Result<T>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    T: windows_core::Interface,
{
    windows_core::link!("oleacc.dll" "system" fn CreateStdAccessibleProxyW(hwnd : super::HWND, pclassname : windows_core::PCWSTR, idobject : i32, riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateStdAccessibleProxyW(hwnd, pclassname.param().abi(), idobject, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn GetOleaccVersionInfo(pver: *mut u32, pbuild: *mut u32) {
    windows_core::link!("oleacc.dll" "system" fn GetOleaccVersionInfo(pver : *mut u32, pbuild : *mut u32));
    unsafe { GetOleaccVersionInfo(pver as _, pbuild as _) }
}
#[inline]
pub unsafe fn GetRoleTextA(lrole: u32, lpszrole: Option<&mut [u8]>) -> u32 {
    windows_core::link!("oleacc.dll" "system" fn GetRoleTextA(lrole : u32, lpszrole : windows_core::PSTR, cchrolemax : u32) -> u32);
    unsafe { GetRoleTextA(lrole, core::mem::transmute(lpszrole.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpszrole.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetRoleTextW(lrole: u32, lpszrole: Option<&mut [u16]>) -> u32 {
    windows_core::link!("oleacc.dll" "system" fn GetRoleTextW(lrole : u32, lpszrole : windows_core::PWSTR, cchrolemax : u32) -> u32);
    unsafe { GetRoleTextW(lrole, core::mem::transmute(lpszrole.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpszrole.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetStateTextA(lstatebit: u32, lpszstate: Option<&mut [u8]>) -> u32 {
    windows_core::link!("oleacc.dll" "system" fn GetStateTextA(lstatebit : u32, lpszstate : windows_core::PSTR, cchstate : u32) -> u32);
    unsafe { GetStateTextA(lstatebit, core::mem::transmute(lpszstate.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpszstate.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetStateTextW(lstatebit: u32, lpszstate: Option<&mut [u16]>) -> u32 {
    windows_core::link!("oleacc.dll" "system" fn GetStateTextW(lstatebit : u32, lpszstate : windows_core::PWSTR, cchstate : u32) -> u32);
    unsafe { GetStateTextW(lstatebit, core::mem::transmute(lpszstate.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpszstate.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LresultFromObject<P2>(riid: *const windows_core::GUID, wparam: super::WPARAM, punk: P2) -> super::LRESULT
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("oleacc.dll" "system" fn LresultFromObject(riid : *const windows_core::GUID, wparam : super::WPARAM, punk : *mut core::ffi::c_void) -> super::LRESULT);
    unsafe { LresultFromObject(riid, wparam, punk.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ObjectFromLresult<T>(lresult: super::LRESULT, wparam: super::WPARAM) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("oleacc.dll" "system" fn ObjectFromLresult(lresult : super::LRESULT, riid : *const windows_core::GUID, wparam : super::WPARAM, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { ObjectFromLresult(lresult, &T::IID, wparam, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "oaidl", feature = "windef"))]
#[inline]
pub unsafe fn WindowFromAccessibleObject<P0>(param0: P0, phwnd: Option<*mut super::HWND>) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAccessible>,
{
    windows_core::link!("oleacc.dll" "system" fn WindowFromAccessibleObject(param0 : *mut core::ffi::c_void, phwnd : *mut super::HWND) -> windows_core::HRESULT);
    unsafe { WindowFromAccessibleObject(param0.param().abi(), phwnd.unwrap_or(core::mem::zeroed()) as _) }
}
pub const ANNO_CONTAINER: AnnoScope = 1;
pub const ANNO_THIS: AnnoScope = 0;
pub const ANRUS_ON_SCREEN_KEYBOARD_ACTIVE: u32 = 1;
pub const ANRUS_PRIORITY_AUDIO_ACTIVE: u32 = 4;
pub const ANRUS_PRIORITY_AUDIO_ACTIVE_NODUCK: u32 = 8;
pub const ANRUS_PRIORITY_AUDIO_DYNAMIC_DUCK: u32 = 16;
pub const ANRUS_TOUCH_MODIFICATION_ACTIVE: u32 = 2;
pub type AnnoScope = i32;
pub const CAccPropServices: windows_core::GUID = windows_core::GUID::from_u128(0xb5f8350b_0548_48b1_a6ee_88bd00b4a5e7);
pub const CLSID_AccPropServices: windows_core::GUID = windows_core::GUID::from_u128(0xb5f8350b_0548_48b1_a6ee_88bd00b4a5e7);
pub const DISPID_ACC_CHILD: i32 = -5002;
pub const DISPID_ACC_CHILDCOUNT: i32 = -5001;
pub const DISPID_ACC_DEFAULTACTION: i32 = -5013;
pub const DISPID_ACC_DESCRIPTION: i32 = -5005;
pub const DISPID_ACC_DODEFAULTACTION: i32 = -5018;
pub const DISPID_ACC_FOCUS: i32 = -5011;
pub const DISPID_ACC_HELP: i32 = -5008;
pub const DISPID_ACC_HELPTOPIC: i32 = -5009;
pub const DISPID_ACC_HITTEST: i32 = -5017;
pub const DISPID_ACC_KEYBOARDSHORTCUT: i32 = -5010;
pub const DISPID_ACC_LOCATION: i32 = -5015;
pub const DISPID_ACC_NAME: i32 = -5003;
pub const DISPID_ACC_NAVIGATE: i32 = -5016;
pub const DISPID_ACC_PARENT: i32 = -5000;
pub const DISPID_ACC_ROLE: i32 = -5006;
pub const DISPID_ACC_SELECT: i32 = -5014;
pub const DISPID_ACC_SELECTION: i32 = -5012;
pub const DISPID_ACC_STATE: i32 = -5007;
pub const DISPID_ACC_VALUE: i32 = -5004;
windows_core::imp::define_interface!(IAccIdentity, IAccIdentity_Vtbl, 0x7852b78d_1cfd_41c1_a615_9c0c85960b5f);
windows_core::imp::interface_hierarchy!(IAccIdentity, windows_core::IUnknown);
impl IAccIdentity {
    pub unsafe fn GetIdentityString(&self, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIdentityString)(windows_core::Interface::as_raw(self), dwidchild, ppidstring as _, pdwidstringlen as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccIdentity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIdentityString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IAccIdentity_Impl: windows_core::IUnknownImpl {
    fn GetIdentityString(&self, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> windows_core::Result<()>;
}
impl IAccIdentity_Vtbl {
    pub const fn new<Identity: IAccIdentity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIdentityString<Identity: IAccIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccIdentity_Impl::GetIdentityString(this, core::mem::transmute_copy(&dwidchild), core::mem::transmute_copy(&ppidstring), core::mem::transmute_copy(&pdwidstringlen)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIdentityString: GetIdentityString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccIdentity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAccIdentity {}
windows_core::imp::define_interface!(IAccPropServer, IAccPropServer_Vtbl, 0x76c0dbbb_15e0_4e7b_b61b_20eeea2001e0);
windows_core::imp::interface_hierarchy!(IAccPropServer, windows_core::IUnknown);
impl IAccPropServer {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPropValue(&self, pidstring: *const u8, dwidstringlen: u32, idprop: MSAAPROPID, pvarvalue: *mut super::VARIANT, pfhasprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropValue)(windows_core::Interface::as_raw(self), pidstring, dwidstringlen, idprop, pvarvalue, pfhasprop as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccPropServer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPropValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, MSAAPROPID, *mut super::VARIANT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetPropValue: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAccPropServer_Impl: windows_core::IUnknownImpl {
    fn GetPropValue(&self, pidstring: *const u8, dwidstringlen: u32, idprop: &MSAAPROPID, pvarvalue: *mut super::VARIANT, pfhasprop: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IAccPropServer_Vtbl {
    pub const fn new<Identity: IAccPropServer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropValue<Identity: IAccPropServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: MSAAPROPID, pvarvalue: *mut super::VARIANT, pfhasprop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServer_Impl::GetPropValue(this, core::mem::transmute_copy(&pidstring), core::mem::transmute_copy(&dwidstringlen), core::mem::transmute(&idprop), core::mem::transmute_copy(&pvarvalue), core::mem::transmute_copy(&pfhasprop)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPropValue: GetPropValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccPropServer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAccPropServer {}
windows_core::imp::define_interface!(IAccPropServices, IAccPropServices_Vtbl, 0x6e26e776_04f0_495d_80e4_3330352e3169);
windows_core::imp::interface_hierarchy!(IAccPropServices, windows_core::IUnknown);
impl IAccPropServices {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetPropValue(&self, pidstring: *const u8, dwidstringlen: u32, idprop: MSAAPROPID, var: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPropValue)(windows_core::Interface::as_raw(self), pidstring, dwidstringlen, idprop, core::mem::transmute_copy(var)) }
    }
    pub unsafe fn SetPropServer<P4>(&self, pidstring: *const u8, dwidstringlen: u32, paprops: *const MSAAPROPID, cprops: i32, pserver: P4, annoscope: AnnoScope) -> windows_core::HRESULT
    where
        P4: windows_core::Param<IAccPropServer>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPropServer)(windows_core::Interface::as_raw(self), pidstring, dwidstringlen, paprops, cprops, pserver.param().abi(), annoscope) }
    }
    pub unsafe fn ClearProps(&self, pidstring: *const u8, dwidstringlen: u32, paprops: *const MSAAPROPID, cprops: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearProps)(windows_core::Interface::as_raw(self), pidstring, dwidstringlen, paprops, cprops) }
    }
    #[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetHwndProp(&self, hwnd: super::HWND, idobject: u32, idchild: u32, idprop: MSAAPROPID, var: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHwndProp)(windows_core::Interface::as_raw(self), hwnd, idobject, idchild, idprop, core::mem::transmute_copy(var)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetHwndPropStr<P4>(&self, hwnd: super::HWND, idobject: u32, idchild: u32, idprop: MSAAPROPID, str: P4) -> windows_core::HRESULT
    where
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHwndPropStr)(windows_core::Interface::as_raw(self), hwnd, idobject, idchild, idprop, str.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetHwndPropServer<P5>(&self, hwnd: super::HWND, idobject: u32, idchild: u32, paprops: *const MSAAPROPID, cprops: i32, pserver: P5, annoscope: AnnoScope) -> windows_core::HRESULT
    where
        P5: windows_core::Param<IAccPropServer>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHwndPropServer)(windows_core::Interface::as_raw(self), hwnd, idobject, idchild, paprops, cprops, pserver.param().abi(), annoscope) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ClearHwndProps(&self, hwnd: super::HWND, idobject: u32, idchild: u32, paprops: *const MSAAPROPID, cprops: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearHwndProps)(windows_core::Interface::as_raw(self), hwnd, idobject, idchild, paprops, cprops) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ComposeHwndIdentityString(&self, hwnd: super::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ComposeHwndIdentityString)(windows_core::Interface::as_raw(self), hwnd, idobject, idchild, ppidstring as _, pdwidstringlen as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn DecomposeHwndIdentityString(&self, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::HWND, pidobject: *mut u32, pidchild: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DecomposeHwndIdentityString)(windows_core::Interface::as_raw(self), pidstring, dwidstringlen, phwnd as _, pidobject as _, pidchild as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetHmenuProp(&self, hmenu: super::HMENU, idchild: u32, idprop: MSAAPROPID, var: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHmenuProp)(windows_core::Interface::as_raw(self), hmenu, idchild, idprop, core::mem::transmute_copy(var)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetHmenuPropStr<P3>(&self, hmenu: super::HMENU, idchild: u32, idprop: MSAAPROPID, str: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHmenuPropStr)(windows_core::Interface::as_raw(self), hmenu, idchild, idprop, str.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetHmenuPropServer<P4>(&self, hmenu: super::HMENU, idchild: u32, paprops: *const MSAAPROPID, cprops: i32, pserver: P4, annoscope: AnnoScope) -> windows_core::HRESULT
    where
        P4: windows_core::Param<IAccPropServer>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHmenuPropServer)(windows_core::Interface::as_raw(self), hmenu, idchild, paprops, cprops, pserver.param().abi(), annoscope) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ClearHmenuProps(&self, hmenu: super::HMENU, idchild: u32, paprops: *const MSAAPROPID, cprops: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearHmenuProps)(windows_core::Interface::as_raw(self), hmenu, idchild, paprops, cprops) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ComposeHmenuIdentityString(&self, hmenu: super::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ComposeHmenuIdentityString)(windows_core::Interface::as_raw(self), hmenu, idchild, ppidstring as _, pdwidstringlen as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn DecomposeHmenuIdentityString(&self, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::HMENU, pidchild: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DecomposeHmenuIdentityString)(windows_core::Interface::as_raw(self), pidstring, dwidstringlen, phmenu as _, pidchild as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccPropServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub SetPropValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, MSAAPROPID, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    SetPropValue: usize,
    pub SetPropServer: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const MSAAPROPID, i32, *mut core::ffi::c_void, AnnoScope) -> windows_core::HRESULT,
    pub ClearProps: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const MSAAPROPID, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
    pub SetHwndProp: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, u32, u32, MSAAPROPID, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase")))]
    SetHwndProp: usize,
    #[cfg(feature = "windef")]
    pub SetHwndPropStr: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, u32, u32, MSAAPROPID, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetHwndPropStr: usize,
    #[cfg(feature = "windef")]
    pub SetHwndPropServer: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, u32, u32, *const MSAAPROPID, i32, *mut core::ffi::c_void, AnnoScope) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetHwndPropServer: usize,
    #[cfg(feature = "windef")]
    pub ClearHwndProps: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, u32, u32, *const MSAAPROPID, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ClearHwndProps: usize,
    #[cfg(feature = "windef")]
    pub ComposeHwndIdentityString: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, u32, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ComposeHwndIdentityString: usize,
    #[cfg(feature = "windef")]
    pub DecomposeHwndIdentityString: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut super::HWND, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    DecomposeHwndIdentityString: usize,
    #[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
    pub SetHmenuProp: unsafe extern "system" fn(*mut core::ffi::c_void, super::HMENU, u32, MSAAPROPID, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase")))]
    SetHmenuProp: usize,
    #[cfg(feature = "windef")]
    pub SetHmenuPropStr: unsafe extern "system" fn(*mut core::ffi::c_void, super::HMENU, u32, MSAAPROPID, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetHmenuPropStr: usize,
    #[cfg(feature = "windef")]
    pub SetHmenuPropServer: unsafe extern "system" fn(*mut core::ffi::c_void, super::HMENU, u32, *const MSAAPROPID, i32, *mut core::ffi::c_void, AnnoScope) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetHmenuPropServer: usize,
    #[cfg(feature = "windef")]
    pub ClearHmenuProps: unsafe extern "system" fn(*mut core::ffi::c_void, super::HMENU, u32, *const MSAAPROPID, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ClearHmenuProps: usize,
    #[cfg(feature = "windef")]
    pub ComposeHmenuIdentityString: unsafe extern "system" fn(*mut core::ffi::c_void, super::HMENU, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ComposeHmenuIdentityString: usize,
    #[cfg(feature = "windef")]
    pub DecomposeHmenuIdentityString: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut super::HMENU, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    DecomposeHmenuIdentityString: usize,
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAccPropServices_Impl: windows_core::IUnknownImpl {
    fn SetPropValue(&self, pidstring: *const u8, dwidstringlen: u32, idprop: &MSAAPROPID, var: &super::VARIANT) -> windows_core::Result<()>;
    fn SetPropServer(&self, pidstring: *const u8, dwidstringlen: u32, paprops: *const MSAAPROPID, cprops: i32, pserver: windows_core::Ref<IAccPropServer>, annoscope: AnnoScope) -> windows_core::Result<()>;
    fn ClearProps(&self, pidstring: *const u8, dwidstringlen: u32, paprops: *const MSAAPROPID, cprops: i32) -> windows_core::Result<()>;
    fn SetHwndProp(&self, hwnd: super::HWND, idobject: u32, idchild: u32, idprop: &MSAAPROPID, var: &super::VARIANT) -> windows_core::Result<()>;
    fn SetHwndPropStr(&self, hwnd: super::HWND, idobject: u32, idchild: u32, idprop: &MSAAPROPID, str: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHwndPropServer(&self, hwnd: super::HWND, idobject: u32, idchild: u32, paprops: *const MSAAPROPID, cprops: i32, pserver: windows_core::Ref<IAccPropServer>, annoscope: AnnoScope) -> windows_core::Result<()>;
    fn ClearHwndProps(&self, hwnd: super::HWND, idobject: u32, idchild: u32, paprops: *const MSAAPROPID, cprops: i32) -> windows_core::Result<()>;
    fn ComposeHwndIdentityString(&self, hwnd: super::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> windows_core::Result<()>;
    fn DecomposeHwndIdentityString(&self, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::HWND, pidobject: *mut u32, pidchild: *mut u32) -> windows_core::Result<()>;
    fn SetHmenuProp(&self, hmenu: super::HMENU, idchild: u32, idprop: &MSAAPROPID, var: &super::VARIANT) -> windows_core::Result<()>;
    fn SetHmenuPropStr(&self, hmenu: super::HMENU, idchild: u32, idprop: &MSAAPROPID, str: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHmenuPropServer(&self, hmenu: super::HMENU, idchild: u32, paprops: *const MSAAPROPID, cprops: i32, pserver: windows_core::Ref<IAccPropServer>, annoscope: AnnoScope) -> windows_core::Result<()>;
    fn ClearHmenuProps(&self, hmenu: super::HMENU, idchild: u32, paprops: *const MSAAPROPID, cprops: i32) -> windows_core::Result<()>;
    fn ComposeHmenuIdentityString(&self, hmenu: super::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> windows_core::Result<()>;
    fn DecomposeHmenuIdentityString(&self, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::HMENU, pidchild: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IAccPropServices_Vtbl {
    pub const fn new<Identity: IAccPropServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPropValue<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: MSAAPROPID, var: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::SetPropValue(this, core::mem::transmute_copy(&pidstring), core::mem::transmute_copy(&dwidstringlen), core::mem::transmute(&idprop), core::mem::transmute(&var)).into()
            }
        }
        unsafe extern "system" fn SetPropServer<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const MSAAPROPID, cprops: i32, pserver: *mut core::ffi::c_void, annoscope: AnnoScope) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::SetPropServer(this, core::mem::transmute_copy(&pidstring), core::mem::transmute_copy(&dwidstringlen), core::mem::transmute_copy(&paprops), core::mem::transmute_copy(&cprops), core::mem::transmute_copy(&pserver), core::mem::transmute_copy(&annoscope)).into()
            }
        }
        unsafe extern "system" fn ClearProps<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const MSAAPROPID, cprops: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::ClearProps(this, core::mem::transmute_copy(&pidstring), core::mem::transmute_copy(&dwidstringlen), core::mem::transmute_copy(&paprops), core::mem::transmute_copy(&cprops)).into()
            }
        }
        unsafe extern "system" fn SetHwndProp<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::HWND, idobject: u32, idchild: u32, idprop: MSAAPROPID, var: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::SetHwndProp(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&idobject), core::mem::transmute_copy(&idchild), core::mem::transmute(&idprop), core::mem::transmute(&var)).into()
            }
        }
        unsafe extern "system" fn SetHwndPropStr<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::HWND, idobject: u32, idchild: u32, idprop: MSAAPROPID, str: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::SetHwndPropStr(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&idobject), core::mem::transmute_copy(&idchild), core::mem::transmute(&idprop), core::mem::transmute(&str)).into()
            }
        }
        unsafe extern "system" fn SetHwndPropServer<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::HWND, idobject: u32, idchild: u32, paprops: *const MSAAPROPID, cprops: i32, pserver: *mut core::ffi::c_void, annoscope: AnnoScope) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::SetHwndPropServer(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&idobject), core::mem::transmute_copy(&idchild), core::mem::transmute_copy(&paprops), core::mem::transmute_copy(&cprops), core::mem::transmute_copy(&pserver), core::mem::transmute_copy(&annoscope)).into()
            }
        }
        unsafe extern "system" fn ClearHwndProps<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::HWND, idobject: u32, idchild: u32, paprops: *const MSAAPROPID, cprops: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::ClearHwndProps(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&idobject), core::mem::transmute_copy(&idchild), core::mem::transmute_copy(&paprops), core::mem::transmute_copy(&cprops)).into()
            }
        }
        unsafe extern "system" fn ComposeHwndIdentityString<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::ComposeHwndIdentityString(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&idobject), core::mem::transmute_copy(&idchild), core::mem::transmute_copy(&ppidstring), core::mem::transmute_copy(&pdwidstringlen)).into()
            }
        }
        unsafe extern "system" fn DecomposeHwndIdentityString<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::HWND, pidobject: *mut u32, pidchild: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::DecomposeHwndIdentityString(this, core::mem::transmute_copy(&pidstring), core::mem::transmute_copy(&dwidstringlen), core::mem::transmute_copy(&phwnd), core::mem::transmute_copy(&pidobject), core::mem::transmute_copy(&pidchild)).into()
            }
        }
        unsafe extern "system" fn SetHmenuProp<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmenu: super::HMENU, idchild: u32, idprop: MSAAPROPID, var: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::SetHmenuProp(this, core::mem::transmute_copy(&hmenu), core::mem::transmute_copy(&idchild), core::mem::transmute(&idprop), core::mem::transmute(&var)).into()
            }
        }
        unsafe extern "system" fn SetHmenuPropStr<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmenu: super::HMENU, idchild: u32, idprop: MSAAPROPID, str: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::SetHmenuPropStr(this, core::mem::transmute_copy(&hmenu), core::mem::transmute_copy(&idchild), core::mem::transmute(&idprop), core::mem::transmute(&str)).into()
            }
        }
        unsafe extern "system" fn SetHmenuPropServer<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmenu: super::HMENU, idchild: u32, paprops: *const MSAAPROPID, cprops: i32, pserver: *mut core::ffi::c_void, annoscope: AnnoScope) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::SetHmenuPropServer(this, core::mem::transmute_copy(&hmenu), core::mem::transmute_copy(&idchild), core::mem::transmute_copy(&paprops), core::mem::transmute_copy(&cprops), core::mem::transmute_copy(&pserver), core::mem::transmute_copy(&annoscope)).into()
            }
        }
        unsafe extern "system" fn ClearHmenuProps<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmenu: super::HMENU, idchild: u32, paprops: *const MSAAPROPID, cprops: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::ClearHmenuProps(this, core::mem::transmute_copy(&hmenu), core::mem::transmute_copy(&idchild), core::mem::transmute_copy(&paprops), core::mem::transmute_copy(&cprops)).into()
            }
        }
        unsafe extern "system" fn ComposeHmenuIdentityString<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmenu: super::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::ComposeHmenuIdentityString(this, core::mem::transmute_copy(&hmenu), core::mem::transmute_copy(&idchild), core::mem::transmute_copy(&ppidstring), core::mem::transmute_copy(&pdwidstringlen)).into()
            }
        }
        unsafe extern "system" fn DecomposeHmenuIdentityString<Identity: IAccPropServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::HMENU, pidchild: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccPropServices_Impl::DecomposeHmenuIdentityString(this, core::mem::transmute_copy(&pidstring), core::mem::transmute_copy(&dwidstringlen), core::mem::transmute_copy(&phmenu), core::mem::transmute_copy(&pidchild)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPropValue: SetPropValue::<Identity, OFFSET>,
            SetPropServer: SetPropServer::<Identity, OFFSET>,
            ClearProps: ClearProps::<Identity, OFFSET>,
            SetHwndProp: SetHwndProp::<Identity, OFFSET>,
            SetHwndPropStr: SetHwndPropStr::<Identity, OFFSET>,
            SetHwndPropServer: SetHwndPropServer::<Identity, OFFSET>,
            ClearHwndProps: ClearHwndProps::<Identity, OFFSET>,
            ComposeHwndIdentityString: ComposeHwndIdentityString::<Identity, OFFSET>,
            DecomposeHwndIdentityString: DecomposeHwndIdentityString::<Identity, OFFSET>,
            SetHmenuProp: SetHmenuProp::<Identity, OFFSET>,
            SetHmenuPropStr: SetHmenuPropStr::<Identity, OFFSET>,
            SetHmenuPropServer: SetHmenuPropServer::<Identity, OFFSET>,
            ClearHmenuProps: ClearHmenuProps::<Identity, OFFSET>,
            ComposeHmenuIdentityString: ComposeHmenuIdentityString::<Identity, OFFSET>,
            DecomposeHmenuIdentityString: DecomposeHmenuIdentityString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccPropServices as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAccPropServices {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IAccessible, IAccessible_Vtbl, 0x618736e0_3c3d_11cf_810c_00aa00389b71);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IAccessible {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IAccessible, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IAccessible {
    pub unsafe fn accParent(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn accChildCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accChildCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accChild(&self, varchild: &super::VARIANT) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accChild)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varchild), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accName(&self, varchild: &super::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varchild), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accValue(&self, varchild: &super::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varchild), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accDescription(&self, varchild: &super::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varchild), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accRole(&self, varchild: &super::VARIANT) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accRole)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varchild), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accState(&self, varchild: &super::VARIANT) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accState)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varchild), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accHelp(&self, varchild: &super::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accHelp)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varchild), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accHelpTopic(&self, pszhelpfile: *mut windows_core::BSTR, varchild: &super::VARIANT) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accHelpTopic)(windows_core::Interface::as_raw(self), core::mem::transmute(pszhelpfile), core::mem::transmute_copy(varchild), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accKeyboardShortcut(&self, varchild: &super::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accKeyboardShortcut)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varchild), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accFocus(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accSelection(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accDefaultAction(&self, varchild: &super::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accDefaultAction)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varchild), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accSelect(&self, flagsselect: i32, varchild: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).accSelect)(windows_core::Interface::as_raw(self), flagsselect, core::mem::transmute_copy(varchild)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accLocation(&self, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).accLocation)(windows_core::Interface::as_raw(self), pxleft as _, pytop as _, pcxwidth as _, pcyheight as _, core::mem::transmute_copy(varchild)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accNavigate(&self, navdir: i32, varstart: &super::VARIANT) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accNavigate)(windows_core::Interface::as_raw(self), navdir, core::mem::transmute_copy(varstart), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accHitTest(&self, xleft: i32, ytop: i32) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).accHitTest)(windows_core::Interface::as_raw(self), xleft, ytop, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn accDoDefaultAction(&self, varchild: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).accDoDefaultAction)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varchild)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetaccName(&self, varchild: &super::VARIANT, szname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetaccName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varchild), core::mem::transmute_copy(szname)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetaccValue(&self, varchild: &super::VARIANT, szvalue: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetaccValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varchild), core::mem::transmute_copy(szvalue)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAccessible_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub accParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub accChildCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accChild: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accChild: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accName: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accName: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accValue: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accDescription: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accDescription: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accRole: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accRole: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accState: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accState: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accHelp: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accHelp: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accHelpTopic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, super::VARIANT, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accHelpTopic: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accKeyboardShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accKeyboardShortcut: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accFocus: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accSelection: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accDefaultAction: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accSelect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accSelect: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accLocation: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accNavigate: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::VARIANT, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accNavigate: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accHitTest: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accHitTest: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub accDoDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    accDoDefaultAction: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetaccName: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetaccName: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetaccValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetaccValue: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAccessible_Impl: super::IDispatch_Impl {
    fn accParent(&self) -> windows_core::Result<super::IDispatch>;
    fn accChildCount(&self) -> windows_core::Result<i32>;
    fn accChild(&self, varchild: &super::VARIANT) -> windows_core::Result<super::IDispatch>;
    fn accName(&self, varchild: &super::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn accValue(&self, varchild: &super::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn accDescription(&self, varchild: &super::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn accRole(&self, varchild: &super::VARIANT) -> windows_core::Result<super::VARIANT>;
    fn accState(&self, varchild: &super::VARIANT) -> windows_core::Result<super::VARIANT>;
    fn accHelp(&self, varchild: &super::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn accHelpTopic(&self, pszhelpfile: *mut windows_core::BSTR, varchild: &super::VARIANT) -> windows_core::Result<i32>;
    fn accKeyboardShortcut(&self, varchild: &super::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn accFocus(&self) -> windows_core::Result<super::VARIANT>;
    fn accSelection(&self) -> windows_core::Result<super::VARIANT>;
    fn accDefaultAction(&self, varchild: &super::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn accSelect(&self, flagsselect: i32, varchild: &super::VARIANT) -> windows_core::Result<()>;
    fn accLocation(&self, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: &super::VARIANT) -> windows_core::Result<()>;
    fn accNavigate(&self, navdir: i32, varstart: &super::VARIANT) -> windows_core::Result<super::VARIANT>;
    fn accHitTest(&self, xleft: i32, ytop: i32) -> windows_core::Result<super::VARIANT>;
    fn accDoDefaultAction(&self, varchild: &super::VARIANT) -> windows_core::Result<()>;
    fn SetaccName(&self, varchild: &super::VARIANT, szname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetaccValue(&self, varchild: &super::VARIANT, szvalue: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAccessible_Vtbl {
    pub const fn new<Identity: IAccessible_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn accParent<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdispparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accParent(this) {
                    Ok(ok__) => {
                        ppdispparent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accChildCount<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcountchildren: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accChildCount(this) {
                    Ok(ok__) => {
                        pcountchildren.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accChild<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varchild: super::VARIANT, ppdispchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accChild(this, core::mem::transmute(&varchild)) {
                    Ok(ok__) => {
                        ppdispchild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accName<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varchild: super::VARIANT, pszname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accName(this, core::mem::transmute(&varchild)) {
                    Ok(ok__) => {
                        pszname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accValue<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varchild: super::VARIANT, pszvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accValue(this, core::mem::transmute(&varchild)) {
                    Ok(ok__) => {
                        pszvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accDescription<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varchild: super::VARIANT, pszdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accDescription(this, core::mem::transmute(&varchild)) {
                    Ok(ok__) => {
                        pszdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accRole<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varchild: super::VARIANT, pvarrole: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accRole(this, core::mem::transmute(&varchild)) {
                    Ok(ok__) => {
                        pvarrole.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accState<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varchild: super::VARIANT, pvarstate: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accState(this, core::mem::transmute(&varchild)) {
                    Ok(ok__) => {
                        pvarstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accHelp<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varchild: super::VARIANT, pszhelp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accHelp(this, core::mem::transmute(&varchild)) {
                    Ok(ok__) => {
                        pszhelp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accHelpTopic<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhelpfile: *mut *mut core::ffi::c_void, varchild: super::VARIANT, pidtopic: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accHelpTopic(this, core::mem::transmute_copy(&pszhelpfile), core::mem::transmute(&varchild)) {
                    Ok(ok__) => {
                        pidtopic.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accKeyboardShortcut<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varchild: super::VARIANT, pszkeyboardshortcut: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accKeyboardShortcut(this, core::mem::transmute(&varchild)) {
                    Ok(ok__) => {
                        pszkeyboardshortcut.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accFocus<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarchild: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accFocus(this) {
                    Ok(ok__) => {
                        pvarchild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accSelection<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarchildren: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accSelection(this) {
                    Ok(ok__) => {
                        pvarchildren.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accDefaultAction<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varchild: super::VARIANT, pszdefaultaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accDefaultAction(this, core::mem::transmute(&varchild)) {
                    Ok(ok__) => {
                        pszdefaultaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accSelect<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flagsselect: i32, varchild: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessible_Impl::accSelect(this, core::mem::transmute_copy(&flagsselect), core::mem::transmute(&varchild)).into()
            }
        }
        unsafe extern "system" fn accLocation<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessible_Impl::accLocation(this, core::mem::transmute_copy(&pxleft), core::mem::transmute_copy(&pytop), core::mem::transmute_copy(&pcxwidth), core::mem::transmute_copy(&pcyheight), core::mem::transmute(&varchild)).into()
            }
        }
        unsafe extern "system" fn accNavigate<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, navdir: i32, varstart: super::VARIANT, pvarendupat: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accNavigate(this, core::mem::transmute_copy(&navdir), core::mem::transmute(&varstart)) {
                    Ok(ok__) => {
                        pvarendupat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accHitTest<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xleft: i32, ytop: i32, pvarchild: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessible_Impl::accHitTest(this, core::mem::transmute_copy(&xleft), core::mem::transmute_copy(&ytop)) {
                    Ok(ok__) => {
                        pvarchild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn accDoDefaultAction<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varchild: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessible_Impl::accDoDefaultAction(this, core::mem::transmute(&varchild)).into()
            }
        }
        unsafe extern "system" fn SetaccName<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varchild: super::VARIANT, szname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessible_Impl::SetaccName(this, core::mem::transmute(&varchild), core::mem::transmute(&szname)).into()
            }
        }
        unsafe extern "system" fn SetaccValue<Identity: IAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varchild: super::VARIANT, szvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessible_Impl::SetaccValue(this, core::mem::transmute(&varchild), core::mem::transmute(&szvalue)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            accParent: accParent::<Identity, OFFSET>,
            accChildCount: accChildCount::<Identity, OFFSET>,
            accChild: accChild::<Identity, OFFSET>,
            accName: accName::<Identity, OFFSET>,
            accValue: accValue::<Identity, OFFSET>,
            accDescription: accDescription::<Identity, OFFSET>,
            accRole: accRole::<Identity, OFFSET>,
            accState: accState::<Identity, OFFSET>,
            accHelp: accHelp::<Identity, OFFSET>,
            accHelpTopic: accHelpTopic::<Identity, OFFSET>,
            accKeyboardShortcut: accKeyboardShortcut::<Identity, OFFSET>,
            accFocus: accFocus::<Identity, OFFSET>,
            accSelection: accSelection::<Identity, OFFSET>,
            accDefaultAction: accDefaultAction::<Identity, OFFSET>,
            accSelect: accSelect::<Identity, OFFSET>,
            accLocation: accLocation::<Identity, OFFSET>,
            accNavigate: accNavigate::<Identity, OFFSET>,
            accHitTest: accHitTest::<Identity, OFFSET>,
            accDoDefaultAction: accDoDefaultAction::<Identity, OFFSET>,
            SetaccName: SetaccName::<Identity, OFFSET>,
            SetaccValue: SetaccValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessible as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAccessible {}
windows_core::imp::define_interface!(IAccessibleHandler, IAccessibleHandler_Vtbl, 0x03022430_abc4_11d0_bde2_00aa001a1953);
windows_core::imp::interface_hierarchy!(IAccessibleHandler, windows_core::IUnknown);
impl IAccessibleHandler {
    #[cfg(feature = "oaidl")]
    pub unsafe fn AccessibleObjectFromID(&self, hwnd: i32, lobjectid: i32) -> windows_core::Result<IAccessible> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AccessibleObjectFromID)(windows_core::Interface::as_raw(self), hwnd, lobjectid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub AccessibleObjectFromID: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    AccessibleObjectFromID: usize,
}
#[cfg(feature = "oaidl")]
pub trait IAccessibleHandler_Impl: windows_core::IUnknownImpl {
    fn AccessibleObjectFromID(&self, hwnd: i32, lobjectid: i32) -> windows_core::Result<IAccessible>;
}
#[cfg(feature = "oaidl")]
impl IAccessibleHandler_Vtbl {
    pub const fn new<Identity: IAccessibleHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AccessibleObjectFromID<Identity: IAccessibleHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: i32, lobjectid: i32, piaccessible: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessibleHandler_Impl::AccessibleObjectFromID(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&lobjectid)) {
                    Ok(ok__) => {
                        piaccessible.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AccessibleObjectFromID: AccessibleObjectFromID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessibleHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IAccessibleHandler {}
windows_core::imp::define_interface!(IAccessibleWindowlessSite, IAccessibleWindowlessSite_Vtbl, 0xbf3abd9c_76da_4389_9eb6_1427d25abab7);
windows_core::imp::interface_hierarchy!(IAccessibleWindowlessSite, windows_core::IUnknown);
impl IAccessibleWindowlessSite {
    pub unsafe fn AcquireObjectIdRange<P1>(&self, rangesize: i32, prangeowner: P1) -> windows_core::Result<i32>
    where
        P1: windows_core::Param<IAccessibleHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AcquireObjectIdRange)(windows_core::Interface::as_raw(self), rangesize, prangeowner.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReleaseObjectIdRange<P1>(&self, rangebase: i32, prangeowner: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IAccessibleHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReleaseObjectIdRange)(windows_core::Interface::as_raw(self), rangebase, prangeowner.param().abi()) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn QueryObjectIdRanges<P0>(&self, prangesowner: P0) -> windows_core::Result<*mut super::SAFEARRAY>
    where
        P0: windows_core::Param<IAccessibleHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryObjectIdRanges)(windows_core::Interface::as_raw(self), prangesowner.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetParentAccessible(&self) -> windows_core::Result<IAccessible> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentAccessible)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleWindowlessSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AcquireObjectIdRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ReleaseObjectIdRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub QueryObjectIdRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    QueryObjectIdRanges: usize,
    #[cfg(feature = "oaidl")]
    pub GetParentAccessible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetParentAccessible: usize,
}
#[cfg(feature = "oaidl")]
pub trait IAccessibleWindowlessSite_Impl: windows_core::IUnknownImpl {
    fn AcquireObjectIdRange(&self, rangesize: i32, prangeowner: windows_core::Ref<IAccessibleHandler>) -> windows_core::Result<i32>;
    fn ReleaseObjectIdRange(&self, rangebase: i32, prangeowner: windows_core::Ref<IAccessibleHandler>) -> windows_core::Result<()>;
    fn QueryObjectIdRanges(&self, prangesowner: windows_core::Ref<IAccessibleHandler>) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn GetParentAccessible(&self) -> windows_core::Result<IAccessible>;
}
#[cfg(feature = "oaidl")]
impl IAccessibleWindowlessSite_Vtbl {
    pub const fn new<Identity: IAccessibleWindowlessSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AcquireObjectIdRange<Identity: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangesize: i32, prangeowner: *mut core::ffi::c_void, prangebase: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessibleWindowlessSite_Impl::AcquireObjectIdRange(this, core::mem::transmute_copy(&rangesize), core::mem::transmute_copy(&prangeowner)) {
                    Ok(ok__) => {
                        prangebase.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseObjectIdRange<Identity: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangebase: i32, prangeowner: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessibleWindowlessSite_Impl::ReleaseObjectIdRange(this, core::mem::transmute_copy(&rangebase), core::mem::transmute_copy(&prangeowner)).into()
            }
        }
        unsafe extern "system" fn QueryObjectIdRanges<Identity: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangesowner: *mut core::ffi::c_void, psaranges: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessibleWindowlessSite_Impl::QueryObjectIdRanges(this, core::mem::transmute_copy(&prangesowner)) {
                    Ok(ok__) => {
                        psaranges.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParentAccessible<Identity: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessibleWindowlessSite_Impl::GetParentAccessible(this) {
                    Ok(ok__) => {
                        ppparent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AcquireObjectIdRange: AcquireObjectIdRange::<Identity, OFFSET>,
            ReleaseObjectIdRange: ReleaseObjectIdRange::<Identity, OFFSET>,
            QueryObjectIdRanges: QueryObjectIdRanges::<Identity, OFFSET>,
            GetParentAccessible: GetParentAccessible::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessibleWindowlessSite as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IAccessibleWindowlessSite {}
pub const IID_IAccPropMgrInternal: windows_core::GUID = windows_core::GUID::from_u128(0x2bd370a9_3e7f_4edd_8a85_f8fed1f8e51f);
pub const IIS_ControlAccessible: windows_core::GUID = windows_core::GUID::from_u128(0x38c682a6_9731_43f2_9fae_e901e641b101);
pub const IIS_IsOleaccProxy: windows_core::GUID = windows_core::GUID::from_u128(0x902697fa_80e4_4560_802a_a13f22a64709);
pub const LIBID_Accessibility: windows_core::GUID = windows_core::GUID::from_u128(0x1ea4dbf0_3c3b_11cf_810c_00aa00389b71);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub type LPFNACCESSIBLECHILDREN = Option<unsafe extern "system" fn(pacccontainer: windows_core::Ref<IAccessible>, ichildstart: i32, cchildren: i32, rgvarchildren: *mut super::VARIANT, pcobtained: *mut i32) -> windows_core::HRESULT>;
#[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub type LPFNACCESSIBLEOBJECTFROMPOINT = Option<unsafe extern "system" fn(ptscreen: super::POINT, ppacc: windows_core::OutRef<IAccessible>, pvarchild: *mut super::VARIANT) -> windows_core::HRESULT>;
#[cfg(feature = "windef")]
pub type LPFNACCESSIBLEOBJECTFROMWINDOW = Option<unsafe extern "system" fn(hwnd: super::HWND, dwid: u32, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "windef")]
pub type LPFNCREATESTDACCESSIBLEOBJECT = Option<unsafe extern "system" fn(hwnd: super::HWND, idobject: i32, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "minwindef")]
pub type LPFNLRESULTFROMOBJECT = Option<unsafe extern "system" fn(riid: *const windows_core::GUID, wparam: super::WPARAM, punk: windows_core::Ref<windows_core::IUnknown>) -> super::LRESULT>;
#[cfg(feature = "minwindef")]
pub type LPFNOBJECTFROMLRESULT = Option<unsafe extern "system" fn(lresult: super::LRESULT, riid: *const windows_core::GUID, wparam: super::WPARAM, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type LPMSAAMENUINFO = *mut MSAAMENUINFO;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSAAMENUINFO {
    pub dwMSAASignature: u32,
    pub cchWText: u32,
    pub pszWText: windows_core::PWSTR,
}
pub type MSAAPROPID = windows_core::GUID;
pub const MSAA_MENU_SIG: u32 = 2853040141;
pub const NAVDIR_DOWN: u32 = 2;
pub const NAVDIR_FIRSTCHILD: u32 = 7;
pub const NAVDIR_LASTCHILD: u32 = 8;
pub const NAVDIR_LEFT: u32 = 3;
pub const NAVDIR_MAX: u32 = 9;
pub const NAVDIR_MIN: u32 = 0;
pub const NAVDIR_NEXT: u32 = 5;
pub const NAVDIR_PREVIOUS: u32 = 6;
pub const NAVDIR_RIGHT: u32 = 4;
pub const NAVDIR_UP: u32 = 1;
pub const PROPID_ACC_DEFAULTACTION: windows_core::GUID = windows_core::GUID::from_u128(0x180c072b_c27f_43c7_9922_f63562a4632b);
pub const PROPID_ACC_DESCRIPTION: windows_core::GUID = windows_core::GUID::from_u128(0x4d48dfe4_bd3f_491f_a648_492d6f20c588);
pub const PROPID_ACC_DESCRIPTIONMAP: windows_core::GUID = windows_core::GUID::from_u128(0x1ff1435f_8a14_477b_b226_a0abe279975d);
pub const PROPID_ACC_DODEFAULTACTION: windows_core::GUID = windows_core::GUID::from_u128(0x1ba09523_2e3b_49a6_a059_59682a3c48fd);
pub const PROPID_ACC_FOCUS: windows_core::GUID = windows_core::GUID::from_u128(0x6eb335df_1c29_4127_b12c_dee9fd157f2b);
pub const PROPID_ACC_HELP: windows_core::GUID = windows_core::GUID::from_u128(0xc831e11f_44db_4a99_9768_cb8f978b7231);
pub const PROPID_ACC_HELPTOPIC: windows_core::GUID = windows_core::GUID::from_u128(0x787d1379_8ede_440b_8aec_11f7bf9030b3);
pub const PROPID_ACC_KEYBOARDSHORTCUT: windows_core::GUID = windows_core::GUID::from_u128(0x7d9bceee_7d1e_4979_9382_5180f4172c34);
pub const PROPID_ACC_NAME: windows_core::GUID = windows_core::GUID::from_u128(0x608d3df8_8128_4aa7_a428_f55e49267291);
pub const PROPID_ACC_NAV_DOWN: windows_core::GUID = windows_core::GUID::from_u128(0x031670ed_3cdf_48d2_9613_138f2dd8a668);
pub const PROPID_ACC_NAV_FIRSTCHILD: windows_core::GUID = windows_core::GUID::from_u128(0xcfd02558_557b_4c67_84f9_2a09fce40749);
pub const PROPID_ACC_NAV_LASTCHILD: windows_core::GUID = windows_core::GUID::from_u128(0x302ecaa5_48d5_4f8d_b671_1a8d20a77832);
pub const PROPID_ACC_NAV_LEFT: windows_core::GUID = windows_core::GUID::from_u128(0x228086cb_82f1_4a39_8705_dcdc0fff92f5);
pub const PROPID_ACC_NAV_NEXT: windows_core::GUID = windows_core::GUID::from_u128(0x1cdc5455_8cd9_4c92_a371_3939a2fe3eee);
pub const PROPID_ACC_NAV_PREV: windows_core::GUID = windows_core::GUID::from_u128(0x776d3891_c73b_4480_b3f6_076a16a15af6);
pub const PROPID_ACC_NAV_RIGHT: windows_core::GUID = windows_core::GUID::from_u128(0xcd211d9f_e1cb_4fe5_a77c_920b884d095b);
pub const PROPID_ACC_NAV_UP: windows_core::GUID = windows_core::GUID::from_u128(0x016e1a2b_1a4e_4767_8612_3386f66935ec);
pub const PROPID_ACC_PARENT: windows_core::GUID = windows_core::GUID::from_u128(0x474c22b6_ffc2_467a_b1b5_e958b4657330);
pub const PROPID_ACC_ROLE: windows_core::GUID = windows_core::GUID::from_u128(0xcb905ff2_7bd1_4c05_b3c8_e6c241364d70);
pub const PROPID_ACC_ROLEMAP: windows_core::GUID = windows_core::GUID::from_u128(0xf79acda2_140d_4fe6_8914_208476328269);
pub const PROPID_ACC_SELECTION: windows_core::GUID = windows_core::GUID::from_u128(0xb99d073c_d731_405b_9061_d95e8f842984);
pub const PROPID_ACC_STATE: windows_core::GUID = windows_core::GUID::from_u128(0xa8d4d5b0_0a21_42d0_a5c0_514e984f457b);
pub const PROPID_ACC_STATEMAP: windows_core::GUID = windows_core::GUID::from_u128(0x43946c5e_0ac0_4042_b525_07bbdbe17fa7);
pub const PROPID_ACC_VALUE: windows_core::GUID = windows_core::GUID::from_u128(0x123fe443_211a_4615_9527_c45a7e93717a);
pub const PROPID_ACC_VALUEMAP: windows_core::GUID = windows_core::GUID::from_u128(0xda1c3d79_fc5c_420e_b399_9d1533549e75);
pub const ROLE_SYSTEM_ALERT: u32 = 8;
pub const ROLE_SYSTEM_ANIMATION: u32 = 54;
pub const ROLE_SYSTEM_APPLICATION: u32 = 14;
pub const ROLE_SYSTEM_BORDER: u32 = 19;
pub const ROLE_SYSTEM_BUTTONDROPDOWN: u32 = 56;
pub const ROLE_SYSTEM_BUTTONDROPDOWNGRID: u32 = 58;
pub const ROLE_SYSTEM_BUTTONMENU: u32 = 57;
pub const ROLE_SYSTEM_CARET: u32 = 7;
pub const ROLE_SYSTEM_CELL: u32 = 29;
pub const ROLE_SYSTEM_CHARACTER: u32 = 32;
pub const ROLE_SYSTEM_CHART: u32 = 17;
pub const ROLE_SYSTEM_CHECKBUTTON: u32 = 44;
pub const ROLE_SYSTEM_CLIENT: u32 = 10;
pub const ROLE_SYSTEM_CLOCK: u32 = 61;
pub const ROLE_SYSTEM_COLUMN: u32 = 27;
pub const ROLE_SYSTEM_COLUMNHEADER: u32 = 25;
pub const ROLE_SYSTEM_COMBOBOX: u32 = 46;
pub const ROLE_SYSTEM_CURSOR: u32 = 6;
pub const ROLE_SYSTEM_DIAGRAM: u32 = 53;
pub const ROLE_SYSTEM_DIAL: u32 = 49;
pub const ROLE_SYSTEM_DIALOG: u32 = 18;
pub const ROLE_SYSTEM_DOCUMENT: u32 = 15;
pub const ROLE_SYSTEM_DROPLIST: u32 = 47;
pub const ROLE_SYSTEM_EQUATION: u32 = 55;
pub const ROLE_SYSTEM_GRAPHIC: u32 = 40;
pub const ROLE_SYSTEM_GRIP: u32 = 4;
pub const ROLE_SYSTEM_GROUPING: u32 = 20;
pub const ROLE_SYSTEM_HELPBALLOON: u32 = 31;
pub const ROLE_SYSTEM_HOTKEYFIELD: u32 = 50;
pub const ROLE_SYSTEM_INDICATOR: u32 = 39;
pub const ROLE_SYSTEM_IPADDRESS: u32 = 63;
pub const ROLE_SYSTEM_LINK: u32 = 30;
pub const ROLE_SYSTEM_LIST: u32 = 33;
pub const ROLE_SYSTEM_LISTITEM: u32 = 34;
pub const ROLE_SYSTEM_MENUBAR: u32 = 2;
pub const ROLE_SYSTEM_MENUITEM: u32 = 12;
pub const ROLE_SYSTEM_MENUPOPUP: u32 = 11;
pub const ROLE_SYSTEM_OUTLINE: u32 = 35;
pub const ROLE_SYSTEM_OUTLINEBUTTON: u32 = 64;
pub const ROLE_SYSTEM_OUTLINEITEM: u32 = 36;
pub const ROLE_SYSTEM_PAGETAB: u32 = 37;
pub const ROLE_SYSTEM_PAGETABLIST: u32 = 60;
pub const ROLE_SYSTEM_PANE: u32 = 16;
pub const ROLE_SYSTEM_PROGRESSBAR: u32 = 48;
pub const ROLE_SYSTEM_PROPERTYPAGE: u32 = 38;
pub const ROLE_SYSTEM_PUSHBUTTON: u32 = 43;
pub const ROLE_SYSTEM_RADIOBUTTON: u32 = 45;
pub const ROLE_SYSTEM_ROW: u32 = 28;
pub const ROLE_SYSTEM_ROWHEADER: u32 = 26;
pub const ROLE_SYSTEM_SCROLLBAR: u32 = 3;
pub const ROLE_SYSTEM_SEPARATOR: u32 = 21;
pub const ROLE_SYSTEM_SLIDER: u32 = 51;
pub const ROLE_SYSTEM_SOUND: u32 = 5;
pub const ROLE_SYSTEM_SPINBUTTON: u32 = 52;
pub const ROLE_SYSTEM_SPLITBUTTON: u32 = 62;
pub const ROLE_SYSTEM_STATICTEXT: u32 = 41;
pub const ROLE_SYSTEM_STATUSBAR: u32 = 23;
pub const ROLE_SYSTEM_TABLE: u32 = 24;
pub const ROLE_SYSTEM_TEXT: u32 = 42;
pub const ROLE_SYSTEM_TITLEBAR: u32 = 1;
pub const ROLE_SYSTEM_TOOLBAR: u32 = 22;
pub const ROLE_SYSTEM_TOOLTIP: u32 = 13;
pub const ROLE_SYSTEM_WHITESPACE: u32 = 59;
pub const ROLE_SYSTEM_WINDOW: u32 = 9;
pub const SELFLAG_ADDSELECTION: u32 = 8;
pub const SELFLAG_EXTENDSELECTION: u32 = 4;
pub const SELFLAG_NONE: u32 = 0;
pub const SELFLAG_REMOVESELECTION: u32 = 16;
pub const SELFLAG_TAKEFOCUS: u32 = 1;
pub const SELFLAG_TAKESELECTION: u32 = 2;
pub const SELFLAG_VALID: u32 = 31;
pub const STATE_SYSTEM_HASPOPUP: u32 = 1073741824;
