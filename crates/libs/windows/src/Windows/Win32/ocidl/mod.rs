#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HPALETTE_UserFree64(param0: *const u32, param1: *const super::HPALETTE) {
    windows_core::link!("ole32.dll" "system" fn HPALETTE_UserFree64(param0 : *const u32, param1 : *const super::HPALETTE));
    unsafe { HPALETTE_UserFree64(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::HPALETTE) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HPALETTE_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::HPALETTE) -> *mut u8);
    unsafe { HPALETTE_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HPALETTE_UserSize64(param0: *const u32, param1: u32, param2: *const super::HPALETTE) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HPALETTE_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::HPALETTE) -> u32);
    unsafe { HPALETTE_UserSize64(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::HPALETTE) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HPALETTE_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::HPALETTE) -> *mut u8);
    unsafe { HPALETTE_UserUnmarshal64(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HRGN_UserFree(param0: *const u32, param1: *const super::HRGN) {
    windows_core::link!("ole32.dll" "system" fn HRGN_UserFree(param0 : *const u32, param1 : *const super::HRGN));
    unsafe { HRGN_UserFree(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HRGN_UserFree64(param0: *const u32, param1: *const super::HRGN) {
    windows_core::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserFree64(param0 : *const u32, param1 : *const super::HRGN));
    unsafe { HRGN_UserFree64(param0, param1) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HRGN_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::HRGN) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HRGN_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::HRGN) -> *mut u8);
    unsafe { HRGN_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HRGN_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::HRGN) -> *mut u8 {
    windows_core::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::HRGN) -> *mut u8);
    unsafe { HRGN_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HRGN_UserSize(param0: *const u32, param1: u32, param2: *const super::HRGN) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HRGN_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::HRGN) -> u32);
    unsafe { HRGN_UserSize(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HRGN_UserSize64(param0: *const u32, param1: u32, param2: *const super::HRGN) -> u32 {
    windows_core::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::HRGN) -> u32);
    unsafe { HRGN_UserSize64(param0, param1, param2) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HRGN_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::HRGN) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HRGN_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::HRGN) -> *mut u8);
    unsafe { HRGN_UserUnmarshal(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HRGN_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::HRGN) -> *mut u8 {
    windows_core::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::HRGN) -> *mut u8);
    unsafe { HRGN_UserUnmarshal64(param0, param1.as_ptr(), param2 as _) }
}
pub type ACTIVATEFLAGS = i32;
pub const ACTIVATE_WINDOWLESS: ACTIVATEFLAGS = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CADWORD {
    pub cElems: super::ULONG,
    pub pElems: *mut super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CALPOLESTR {
    pub cElems: super::ULONG,
    pub pElems: *mut super::LPOLESTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAUUID {
    pub cElems: super::ULONG,
    pub pElems: *mut windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CONNECTDATA {
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwCookie: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONTROLINFO {
    pub cb: super::ULONG,
    pub hAccel: super::HACCEL,
    pub cAccel: super::USHORT,
    pub dwFlags: super::DWORD,
}
pub type CTRLINFO = i32;
pub const CTRLINFO_EATS_ESCAPE: CTRLINFO = 2;
pub const CTRLINFO_EATS_RETURN: CTRLINFO = 1;
pub type DVASPECT2 = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DVASPECTINFO {
    pub cb: super::ULONG,
    pub dwFlags: super::DWORD,
}
pub type DVASPECTINFOFLAG = i32;
pub const DVASPECTINFOFLAG_CANOPTIMIZE: DVASPECTINFOFLAG = 1;
pub const DVASPECT_OPAQUE: DVASPECT2 = 16;
pub const DVASPECT_TRANSPARENT: DVASPECT2 = 32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DVEXTENTINFO {
    pub cb: super::ULONG,
    pub dwExtentMode: super::DWORD,
    pub sizelProposed: super::SIZEL,
}
pub type DVEXTENTMODE = i32;
pub const DVEXTENT_CONTENT: DVEXTENTMODE = 0;
pub const DVEXTENT_INTEGRAL: DVEXTENTMODE = 1;
pub type GUIDKIND = i32;
pub const GUIDKIND_DEFAULT_SOURCE_DISP_IID: GUIDKIND = 1;
#[cfg(target_arch = "x86")]
pub type HHANDLE = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type HHANDLE = u64;
pub type HITRESULT = i32;
pub const HITRESULT_CLOSE: HITRESULT = 2;
pub const HITRESULT_HIT: HITRESULT = 3;
pub const HITRESULT_OUTSIDE: HITRESULT = 0;
pub const HITRESULT_TRANSPARENT: HITRESULT = 1;
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(IAdviseSinkEx, IAdviseSinkEx_Vtbl, 0x3af24290_0c96_11ce_a0cf_00aa00600ab8);
#[cfg(feature = "objidl")]
impl core::ops::Deref for IAdviseSinkEx {
    type Target = super::IAdviseSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(IAdviseSinkEx, windows_core::IUnknown, super::IAdviseSink);
#[cfg(feature = "objidl")]
impl IAdviseSinkEx {
    #[cfg(feature = "shared")]
    pub unsafe fn OnViewStatusChange(&self, dwviewstatus: super::DWORD) {
        unsafe {
            (windows_core::Interface::vtable(self).OnViewStatusChange)(windows_core::Interface::as_raw(self), dwviewstatus);
        }
    }
}
#[cfg(feature = "objidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSinkEx_Vtbl {
    pub base__: super::IAdviseSink_Vtbl,
    #[cfg(feature = "shared")]
    pub OnViewStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD),
    #[cfg(not(feature = "shared"))]
    OnViewStatusChange: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAdviseSinkEx_Impl: super::IAdviseSink_Impl {
    fn OnViewStatusChange(&self, dwviewstatus: super::DWORD);
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IAdviseSinkEx_Vtbl {
    pub const fn new<Identity: IAdviseSinkEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnViewStatusChange<Identity: IAdviseSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwviewstatus: super::DWORD) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSinkEx_Impl::OnViewStatusChange(this, core::mem::transmute_copy(&dwviewstatus));
            }
        }
        Self { base__: super::IAdviseSink_Vtbl::new::<Identity, OFFSET>(), OnViewStatusChange: OnViewStatusChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdviseSinkEx as windows_core::Interface>::IID || iid == &<super::IAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAdviseSinkEx {}
#[cfg(feature = "unknwnbase")]
windows_core::imp::define_interface!(IClassFactory2, IClassFactory2_Vtbl, 0xb196b28f_bab4_101a_b69c_00aa00341d07);
#[cfg(feature = "unknwnbase")]
impl core::ops::Deref for IClassFactory2 {
    type Target = super::IClassFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "unknwnbase")]
windows_core::imp::interface_hierarchy!(IClassFactory2, windows_core::IUnknown, super::IClassFactory);
#[cfg(feature = "unknwnbase")]
impl IClassFactory2 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetLicInfo(&self, plicinfo: *mut LICINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLicInfo)(windows_core::Interface::as_raw(self), plicinfo as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RequestLicKey(&self, dwreserved: super::DWORD) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestLicKey)(windows_core::Interface::as_raw(self), dwreserved, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "guiddef", feature = "winnt"))]
    pub unsafe fn CreateInstanceLic<P0, P1>(&self, punkouter: P0, punkreserved: P1, riid: *const super::IID, bstrkey: &windows_core::BSTR, ppvobj: *mut super::PVOID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateInstanceLic)(windows_core::Interface::as_raw(self), punkouter.param().abi(), punkreserved.param().abi(), riid, core::mem::transmute_copy(bstrkey), ppvobj as _) }
    }
}
#[cfg(feature = "unknwnbase")]
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory2_Vtbl {
    pub base__: super::IClassFactory_Vtbl,
    #[cfg(feature = "shared")]
    pub GetLicInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LICINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLicInfo: usize,
    #[cfg(feature = "shared")]
    pub RequestLicKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestLicKey: usize,
    #[cfg(all(feature = "guiddef", feature = "winnt"))]
    pub CreateInstanceLic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, *mut core::ffi::c_void, *mut super::PVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "winnt")))]
    CreateInstanceLic: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "unknwnbase", feature = "winnt"))]
pub trait IClassFactory2_Impl: super::IClassFactory_Impl {
    fn GetLicInfo(&self, plicinfo: *mut LICINFO) -> windows_core::Result<()>;
    fn RequestLicKey(&self, dwreserved: super::DWORD) -> windows_core::Result<windows_core::BSTR>;
    fn CreateInstanceLic(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, punkreserved: windows_core::Ref<windows_core::IUnknown>, riid: *const super::IID, bstrkey: &windows_core::BSTR, ppvobj: *mut super::PVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "unknwnbase", feature = "winnt"))]
impl IClassFactory2_Vtbl {
    pub const fn new<Identity: IClassFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLicInfo<Identity: IClassFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plicinfo: *mut LICINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassFactory2_Impl::GetLicInfo(this, core::mem::transmute_copy(&plicinfo)).into()
            }
        }
        unsafe extern "system" fn RequestLicKey<Identity: IClassFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: super::DWORD, pbstrkey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClassFactory2_Impl::RequestLicKey(this, core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        pbstrkey.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInstanceLic<Identity: IClassFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, punkreserved: *mut core::ffi::c_void, riid: *const super::IID, bstrkey: *mut core::ffi::c_void, ppvobj: *mut super::PVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassFactory2_Impl::CreateInstanceLic(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&punkreserved), core::mem::transmute_copy(&riid), core::mem::transmute(&bstrkey), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        Self {
            base__: super::IClassFactory_Vtbl::new::<Identity, OFFSET>(),
            GetLicInfo: GetLicInfo::<Identity, OFFSET>,
            RequestLicKey: RequestLicKey::<Identity, OFFSET>,
            CreateInstanceLic: CreateInstanceLic::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClassFactory2 as windows_core::Interface>::IID || iid == &<super::IClassFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "unknwnbase", feature = "winnt"))]
impl windows_core::RuntimeName for IClassFactory2 {}
windows_core::imp::define_interface!(IConnectionPoint, IConnectionPoint_Vtbl, 0xb196b286_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IConnectionPoint, windows_core::IUnknown);
impl IConnectionPoint {
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetConnectionInterface(&self) -> windows_core::Result<super::IID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectionInterface)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnectionPointContainer(&self) -> windows_core::Result<IConnectionPointContainer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectionPointContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Advise<P0>(&self, punksink: P0) -> windows_core::Result<super::DWORD>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), punksink.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Unadvise(&self, dwcookie: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwcookie) }
    }
    pub unsafe fn EnumConnections(&self) -> windows_core::Result<IEnumConnections> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumConnections)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPoint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "guiddef")]
    pub GetConnectionInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::IID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetConnectionInterface: usize,
    pub GetConnectionPointContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Advise: usize,
    #[cfg(feature = "shared")]
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Unadvise: usize,
    pub EnumConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IConnectionPoint_Impl: windows_core::IUnknownImpl {
    fn GetConnectionInterface(&self) -> windows_core::Result<super::IID>;
    fn GetConnectionPointContainer(&self) -> windows_core::Result<IConnectionPointContainer>;
    fn Advise(&self, punksink: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<super::DWORD>;
    fn Unadvise(&self, dwcookie: super::DWORD) -> windows_core::Result<()>;
    fn EnumConnections(&self) -> windows_core::Result<IEnumConnections>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IConnectionPoint_Vtbl {
    pub const fn new<Identity: IConnectionPoint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConnectionInterface<Identity: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut super::IID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectionPoint_Impl::GetConnectionInterface(this) {
                    Ok(ok__) => {
                        piid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnectionPointContainer<Identity: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcpc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectionPoint_Impl::GetConnectionPointContainer(this) {
                    Ok(ok__) => {
                        ppcpc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Advise<Identity: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksink: *mut core::ffi::c_void, pdwcookie: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectionPoint_Impl::Advise(this, core::mem::transmute_copy(&punksink)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnectionPoint_Impl::Unadvise(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        unsafe extern "system" fn EnumConnections<Identity: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectionPoint_Impl::EnumConnections(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnectionInterface: GetConnectionInterface::<Identity, OFFSET>,
            GetConnectionPointContainer: GetConnectionPointContainer::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            EnumConnections: EnumConnections::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConnectionPoint as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IConnectionPoint {}
windows_core::imp::define_interface!(IConnectionPointContainer, IConnectionPointContainer_Vtbl, 0xb196b284_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IConnectionPointContainer, windows_core::IUnknown);
impl IConnectionPointContainer {
    pub unsafe fn EnumConnectionPoints(&self) -> windows_core::Result<IEnumConnectionPoints> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumConnectionPoints)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn FindConnectionPoint(&self, riid: *const super::IID) -> windows_core::Result<IConnectionPoint> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindConnectionPoint)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPointContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumConnectionPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "guiddef")]
    pub FindConnectionPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    FindConnectionPoint: usize,
}
#[cfg(feature = "guiddef")]
pub trait IConnectionPointContainer_Impl: windows_core::IUnknownImpl {
    fn EnumConnectionPoints(&self) -> windows_core::Result<IEnumConnectionPoints>;
    fn FindConnectionPoint(&self, riid: *const super::IID) -> windows_core::Result<IConnectionPoint>;
}
#[cfg(feature = "guiddef")]
impl IConnectionPointContainer_Vtbl {
    pub const fn new<Identity: IConnectionPointContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumConnectionPoints<Identity: IConnectionPointContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectionPointContainer_Impl::EnumConnectionPoints(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindConnectionPoint<Identity: IConnectionPointContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, ppcp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectionPointContainer_Impl::FindConnectionPoint(this, core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        ppcp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumConnectionPoints: EnumConnectionPoints::<Identity, OFFSET>,
            FindConnectionPoint: FindConnectionPoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConnectionPointContainer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for IConnectionPointContainer {}
windows_core::imp::define_interface!(IEnumConnectionPoints, IEnumConnectionPoints_Vtbl, 0xb196b285_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IEnumConnectionPoints, windows_core::IUnknown);
impl IEnumConnectionPoints {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, cconnections: super::ULONG, ppcp: *mut Option<LPCONNECTIONPOINT>, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cconnections, core::mem::transmute(ppcp), pcfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, cconnections: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cconnections) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnectionPoints_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IEnumConnectionPoints_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cconnections: super::ULONG, ppcp: windows_core::OutRef<LPCONNECTIONPOINT>, pcfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, cconnections: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumConnectionPoints>;
}
#[cfg(feature = "shared")]
impl IEnumConnectionPoints_Vtbl {
    pub const fn new<Identity: IEnumConnectionPoints_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: super::ULONG, ppcp: *mut *mut core::ffi::c_void, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnectionPoints_Impl::Next(this, core::mem::transmute_copy(&cconnections), core::mem::transmute_copy(&ppcp), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnectionPoints_Impl::Skip(this, core::mem::transmute_copy(&cconnections)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnectionPoints_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumConnectionPoints_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumConnectionPoints as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumConnectionPoints {}
windows_core::imp::define_interface!(IEnumConnections, IEnumConnections_Vtbl, 0xb196b287_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IEnumConnections, windows_core::IUnknown);
impl IEnumConnections {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, cconnections: super::ULONG, rgcd: LPCONNECTDATA, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cconnections, rgcd, pcfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, cconnections: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cconnections) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnections_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, LPCONNECTDATA, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IEnumConnections_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cconnections: super::ULONG, rgcd: LPCONNECTDATA, pcfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, cconnections: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumConnections>;
}
#[cfg(feature = "shared")]
impl IEnumConnections_Vtbl {
    pub const fn new<Identity: IEnumConnections_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: super::ULONG, rgcd: LPCONNECTDATA, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnections_Impl::Next(this, core::mem::transmute_copy(&cconnections), core::mem::transmute_copy(&rgcd), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnections_Impl::Skip(this, core::mem::transmute_copy(&cconnections)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnections_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumConnections_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumConnections as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumConnections {}
windows_core::imp::define_interface!(IEnumOleUndoUnits, IEnumOleUndoUnits_Vtbl, 0xb3e7c340_ef97_11ce_9bc9_00aa00608e01);
windows_core::imp::interface_hierarchy!(IEnumOleUndoUnits, windows_core::IUnknown);
impl IEnumOleUndoUnits {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut Option<IOleUndoUnit>, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOleUndoUnits_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IEnumOleUndoUnits_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: windows_core::OutRef<IOleUndoUnit>, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOleUndoUnits>;
}
#[cfg(feature = "shared")]
impl IEnumOleUndoUnits_Vtbl {
    pub const fn new<Identity: IEnumOleUndoUnits_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOleUndoUnits_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOleUndoUnits_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOleUndoUnits_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumOleUndoUnits_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumOleUndoUnits as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumOleUndoUnits {}
windows_core::imp::define_interface!(IFont, IFont_Vtbl, 0xbef6e002_a874_101a_8bba_00aa00300cab);
windows_core::imp::interface_hierarchy!(IFont, windows_core::IUnknown);
impl IFont {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Name)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn get_Size(&self) -> windows_core::Result<super::CY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn put_Size(&self, size: super::CY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Size)(windows_core::Interface::as_raw(self), size) }
    }
    pub unsafe fn get_Bold(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Bold)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Bold(&self, bold: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Bold)(windows_core::Interface::as_raw(self), bold.into()) }
    }
    pub unsafe fn get_Italic(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Italic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Italic(&self, italic: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Italic)(windows_core::Interface::as_raw(self), italic.into()) }
    }
    pub unsafe fn get_Underline(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Underline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Underline(&self, underline: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Underline)(windows_core::Interface::as_raw(self), underline.into()) }
    }
    pub unsafe fn get_Strikethrough(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Strikethrough)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Strikethrough(&self, strikethrough: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Strikethrough)(windows_core::Interface::as_raw(self), strikethrough.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Weight(&self) -> windows_core::Result<super::SHORT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Weight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_Weight(&self, weight: super::SHORT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Weight)(windows_core::Interface::as_raw(self), weight) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Charset(&self) -> windows_core::Result<super::SHORT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Charset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_Charset(&self, charset: super::SHORT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Charset)(windows_core::Interface::as_raw(self), charset) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn get_hFont(&self) -> windows_core::Result<super::HFONT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_hFont)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsEqual<P0>(&self, pfontother: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), pfontother.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetRatio(&self, cylogical: super::LONG, cyhimetric: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRatio)(windows_core::Interface::as_raw(self), cylogical, cyhimetric) }
    }
    #[cfg(all(feature = "shared", feature = "wingdi", feature = "winnt"))]
    pub unsafe fn QueryTextMetrics(&self, ptm: *mut TEXTMETRICOLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryTextMetrics)(windows_core::Interface::as_raw(self), ptm as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn AddRefHfont(&self, hfont: super::HFONT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddRefHfont)(windows_core::Interface::as_raw(self), hfont) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ReleaseHfont(&self, hfont: super::HFONT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseHfont)(windows_core::Interface::as_raw(self), hfont) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetHdc(&self, hdc: super::HDC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHdc)(windows_core::Interface::as_raw(self), hdc) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFont_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub get_Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    get_Size: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub put_Size: unsafe extern "system" fn(*mut core::ffi::c_void, super::CY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    put_Size: usize,
    pub get_Bold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_Bold: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub get_Italic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_Italic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub get_Underline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_Underline: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub get_Strikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_Strikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Weight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SHORT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Weight: usize,
    #[cfg(feature = "shared")]
    pub put_Weight: unsafe extern "system" fn(*mut core::ffi::c_void, super::SHORT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_Weight: usize,
    #[cfg(feature = "shared")]
    pub get_Charset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SHORT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Charset: usize,
    #[cfg(feature = "shared")]
    pub put_Charset: unsafe extern "system" fn(*mut core::ffi::c_void, super::SHORT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_Charset: usize,
    #[cfg(feature = "windef")]
    pub get_hFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    get_hFont: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetRatio: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRatio: usize,
    #[cfg(all(feature = "shared", feature = "wingdi", feature = "winnt"))]
    pub QueryTextMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TEXTMETRICOLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wingdi", feature = "winnt")))]
    QueryTextMetrics: usize,
    #[cfg(feature = "windef")]
    pub AddRefHfont: unsafe extern "system" fn(*mut core::ffi::c_void, super::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AddRefHfont: usize,
    #[cfg(feature = "windef")]
    pub ReleaseHfont: unsafe extern "system" fn(*mut core::ffi::c_void, super::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ReleaseHfont: usize,
    #[cfg(feature = "windef")]
    pub SetHdc: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetHdc: usize,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt", feature = "wtypes"))]
pub trait IFont_Impl: windows_core::IUnknownImpl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Size(&self) -> windows_core::Result<super::CY>;
    fn put_Size(&self, size: &super::CY) -> windows_core::Result<()>;
    fn get_Bold(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_Bold(&self, bold: windows_core::BOOL) -> windows_core::Result<()>;
    fn get_Italic(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_Italic(&self, italic: windows_core::BOOL) -> windows_core::Result<()>;
    fn get_Underline(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_Underline(&self, underline: windows_core::BOOL) -> windows_core::Result<()>;
    fn get_Strikethrough(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_Strikethrough(&self, strikethrough: windows_core::BOOL) -> windows_core::Result<()>;
    fn get_Weight(&self) -> windows_core::Result<super::SHORT>;
    fn put_Weight(&self, weight: super::SHORT) -> windows_core::Result<()>;
    fn get_Charset(&self) -> windows_core::Result<super::SHORT>;
    fn put_Charset(&self, charset: super::SHORT) -> windows_core::Result<()>;
    fn get_hFont(&self) -> windows_core::Result<super::HFONT>;
    fn Clone(&self) -> windows_core::Result<IFont>;
    fn IsEqual(&self, pfontother: windows_core::Ref<IFont>) -> windows_core::Result<()>;
    fn SetRatio(&self, cylogical: super::LONG, cyhimetric: super::LONG) -> windows_core::Result<()>;
    fn QueryTextMetrics(&self, ptm: *mut TEXTMETRICOLE) -> windows_core::Result<()>;
    fn AddRefHfont(&self, hfont: super::HFONT) -> windows_core::Result<()>;
    fn ReleaseHfont(&self, hfont: super::HFONT) -> windows_core::Result<()>;
    fn SetHdc(&self, hdc: super::HDC) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt", feature = "wtypes"))]
impl IFont_Vtbl {
    pub const fn new<Identity: IFont_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFont_Impl::get_Name(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Name<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::put_Name(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_Size<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psize: *mut super::CY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFont_Impl::get_Size(this) {
                    Ok(ok__) => {
                        psize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Size<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: super::CY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::put_Size(this, core::mem::transmute(&size)).into()
            }
        }
        unsafe extern "system" fn get_Bold<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbold: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFont_Impl::get_Bold(this) {
                    Ok(ok__) => {
                        pbold.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Bold<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bold: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::put_Bold(this, core::mem::transmute_copy(&bold)).into()
            }
        }
        unsafe extern "system" fn get_Italic<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitalic: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFont_Impl::get_Italic(this) {
                    Ok(ok__) => {
                        pitalic.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Italic<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, italic: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::put_Italic(this, core::mem::transmute_copy(&italic)).into()
            }
        }
        unsafe extern "system" fn get_Underline<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punderline: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFont_Impl::get_Underline(this) {
                    Ok(ok__) => {
                        punderline.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Underline<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, underline: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::put_Underline(this, core::mem::transmute_copy(&underline)).into()
            }
        }
        unsafe extern "system" fn get_Strikethrough<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrikethrough: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFont_Impl::get_Strikethrough(this) {
                    Ok(ok__) => {
                        pstrikethrough.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Strikethrough<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strikethrough: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::put_Strikethrough(this, core::mem::transmute_copy(&strikethrough)).into()
            }
        }
        unsafe extern "system" fn get_Weight<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pweight: *mut super::SHORT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFont_Impl::get_Weight(this) {
                    Ok(ok__) => {
                        pweight.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Weight<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, weight: super::SHORT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::put_Weight(this, core::mem::transmute_copy(&weight)).into()
            }
        }
        unsafe extern "system" fn get_Charset<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcharset: *mut super::SHORT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFont_Impl::get_Charset(this) {
                    Ok(ok__) => {
                        pcharset.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Charset<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, charset: super::SHORT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::put_Charset(this, core::mem::transmute_copy(&charset)).into()
            }
        }
        unsafe extern "system" fn get_hFont<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phfont: *mut super::HFONT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFont_Impl::get_hFont(this) {
                    Ok(ok__) => {
                        phfont.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFont_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfontother: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::IsEqual(this, core::mem::transmute_copy(&pfontother)).into()
            }
        }
        unsafe extern "system" fn SetRatio<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cylogical: super::LONG, cyhimetric: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::SetRatio(this, core::mem::transmute_copy(&cylogical), core::mem::transmute_copy(&cyhimetric)).into()
            }
        }
        unsafe extern "system" fn QueryTextMetrics<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptm: *mut TEXTMETRICOLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::QueryTextMetrics(this, core::mem::transmute_copy(&ptm)).into()
            }
        }
        unsafe extern "system" fn AddRefHfont<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfont: super::HFONT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::AddRefHfont(this, core::mem::transmute_copy(&hfont)).into()
            }
        }
        unsafe extern "system" fn ReleaseHfont<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfont: super::HFONT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::ReleaseHfont(this, core::mem::transmute_copy(&hfont)).into()
            }
        }
        unsafe extern "system" fn SetHdc<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::SetHdc(this, core::mem::transmute_copy(&hdc)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            put_Name: put_Name::<Identity, OFFSET>,
            get_Size: get_Size::<Identity, OFFSET>,
            put_Size: put_Size::<Identity, OFFSET>,
            get_Bold: get_Bold::<Identity, OFFSET>,
            put_Bold: put_Bold::<Identity, OFFSET>,
            get_Italic: get_Italic::<Identity, OFFSET>,
            put_Italic: put_Italic::<Identity, OFFSET>,
            get_Underline: get_Underline::<Identity, OFFSET>,
            put_Underline: put_Underline::<Identity, OFFSET>,
            get_Strikethrough: get_Strikethrough::<Identity, OFFSET>,
            put_Strikethrough: put_Strikethrough::<Identity, OFFSET>,
            get_Weight: get_Weight::<Identity, OFFSET>,
            put_Weight: put_Weight::<Identity, OFFSET>,
            get_Charset: get_Charset::<Identity, OFFSET>,
            put_Charset: put_Charset::<Identity, OFFSET>,
            get_hFont: get_hFont::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            SetRatio: SetRatio::<Identity, OFFSET>,
            QueryTextMetrics: QueryTextMetrics::<Identity, OFFSET>,
            AddRefHfont: AddRefHfont::<Identity, OFFSET>,
            ReleaseHfont: ReleaseHfont::<Identity, OFFSET>,
            SetHdc: SetHdc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFont as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IFont {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFontDisp, IFontDisp_Vtbl, 0xbef6e003_a874_101a_8bba_00aa00300cab);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFontDisp {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFontDisp, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFontDisp_Vtbl {
    pub base__: super::IDispatch_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFontDisp_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFontDisp_Vtbl {
    pub const fn new<Identity: IFontDisp_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFontDisp as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFontDisp {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFontEventsDisp, IFontEventsDisp_Vtbl, 0x4ef6100a_af88_11d0_9846_00c04fc29993);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFontEventsDisp {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFontEventsDisp, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFontEventsDisp_Vtbl {
    pub base__: super::IDispatch_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFontEventsDisp_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFontEventsDisp_Vtbl {
    pub const fn new<Identity: IFontEventsDisp_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFontEventsDisp as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFontEventsDisp {}
windows_core::imp::define_interface!(IObjectWithSite, IObjectWithSite_Vtbl, 0xfc4801a3_2ba9_11cf_a229_00aa003d7352);
windows_core::imp::interface_hierarchy!(IObjectWithSite, windows_core::IUnknown);
impl IObjectWithSite {
    pub unsafe fn SetSite<P0>(&self, punksite: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSite)(windows_core::Interface::as_raw(self), punksite.param().abi()) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetSite<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetSite)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectWithSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "guiddef")]
    pub GetSite: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetSite: usize,
}
#[cfg(feature = "guiddef")]
pub trait IObjectWithSite_Impl: windows_core::IUnknownImpl {
    fn SetSite(&self, punksite: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetSite(&self, riid: *const super::IID, ppvsite: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "guiddef")]
impl IObjectWithSite_Vtbl {
    pub const fn new<Identity: IObjectWithSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSite<Identity: IObjectWithSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectWithSite_Impl::SetSite(this, core::mem::transmute_copy(&punksite)).into()
            }
        }
        unsafe extern "system" fn GetSite<Identity: IObjectWithSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, ppvsite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectWithSite_Impl::GetSite(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvsite)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetSite: SetSite::<Identity, OFFSET>, GetSite: GetSite::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectWithSite as windows_core::Interface>::IID
    }
}
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for IObjectWithSite {}
windows_core::imp::define_interface!(IOleControl, IOleControl_Vtbl, 0xb196b288_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IOleControl, windows_core::IUnknown);
impl IOleControl {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetControlInfo(&self, pci: *mut CONTROLINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetControlInfo)(windows_core::Interface::as_raw(self), pci as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
    pub unsafe fn OnMnemonic(&self, pmsg: *const super::MSG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnMnemonic)(windows_core::Interface::as_raw(self), pmsg) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnAmbientPropertyChange(&self, dispid: super::DISPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnAmbientPropertyChange)(windows_core::Interface::as_raw(self), dispid) }
    }
    pub unsafe fn FreezeEvents(&self, bfreeze: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreezeEvents)(windows_core::Interface::as_raw(self), bfreeze.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetControlInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CONTROLINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetControlInfo: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
    pub OnMnemonic: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::MSG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser")))]
    OnMnemonic: usize,
    #[cfg(feature = "shared")]
    pub OnAmbientPropertyChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnAmbientPropertyChange: usize,
    pub FreezeEvents: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub trait IOleControl_Impl: windows_core::IUnknownImpl {
    fn GetControlInfo(&self, pci: *mut CONTROLINFO) -> windows_core::Result<()>;
    fn OnMnemonic(&self, pmsg: *const super::MSG) -> windows_core::Result<()>;
    fn OnAmbientPropertyChange(&self, dispid: super::DISPID) -> windows_core::Result<()>;
    fn FreezeEvents(&self, bfreeze: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
impl IOleControl_Vtbl {
    pub const fn new<Identity: IOleControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetControlInfo<Identity: IOleControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pci: *mut CONTROLINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControl_Impl::GetControlInfo(this, core::mem::transmute_copy(&pci)).into()
            }
        }
        unsafe extern "system" fn OnMnemonic<Identity: IOleControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::MSG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControl_Impl::OnMnemonic(this, core::mem::transmute_copy(&pmsg)).into()
            }
        }
        unsafe extern "system" fn OnAmbientPropertyChange<Identity: IOleControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControl_Impl::OnAmbientPropertyChange(this, core::mem::transmute_copy(&dispid)).into()
            }
        }
        unsafe extern "system" fn FreezeEvents<Identity: IOleControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfreeze: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControl_Impl::FreezeEvents(this, core::mem::transmute_copy(&bfreeze)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetControlInfo: GetControlInfo::<Identity, OFFSET>,
            OnMnemonic: OnMnemonic::<Identity, OFFSET>,
            OnAmbientPropertyChange: OnAmbientPropertyChange::<Identity, OFFSET>,
            FreezeEvents: FreezeEvents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
impl windows_core::RuntimeName for IOleControl {}
windows_core::imp::define_interface!(IOleControlSite, IOleControlSite_Vtbl, 0xb196b289_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IOleControlSite, windows_core::IUnknown);
impl IOleControlSite {
    pub unsafe fn OnControlInfoChanged(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnControlInfoChanged)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn LockInPlaceActive(&self, flock: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockInPlaceActive)(windows_core::Interface::as_raw(self), flock.into()) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetExtendedControl(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExtendedControl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
    pub unsafe fn TransformCoords(&self, pptlhimetric: *mut super::POINTL, pptfcontainer: *mut POINTF, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TransformCoords)(windows_core::Interface::as_raw(self), pptlhimetric as _, pptfcontainer as _, dwflags) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
    pub unsafe fn TranslateAcceleratorA(&self, pmsg: *const super::MSG, grfmodifiers: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TranslateAcceleratorA)(windows_core::Interface::as_raw(self), pmsg, grfmodifiers) }
    }
    pub unsafe fn OnFocus(&self, fgotfocus: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnFocus)(windows_core::Interface::as_raw(self), fgotfocus.into()) }
    }
    pub unsafe fn ShowPropertyFrame(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowPropertyFrame)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleControlSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnControlInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockInPlaceActive: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetExtendedControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetExtendedControl: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
    pub TransformCoords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::POINTL, *mut POINTF, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "windef")))]
    TransformCoords: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
    pub TranslateAcceleratorA: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::MSG, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser")))]
    TranslateAcceleratorA: usize,
    pub OnFocus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub ShowPropertyFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winuser"))]
pub trait IOleControlSite_Impl: windows_core::IUnknownImpl {
    fn OnControlInfoChanged(&self) -> windows_core::Result<()>;
    fn LockInPlaceActive(&self, flock: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetExtendedControl(&self) -> windows_core::Result<super::IDispatch>;
    fn TransformCoords(&self, pptlhimetric: *mut super::POINTL, pptfcontainer: *mut POINTF, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn TranslateAcceleratorA(&self, pmsg: *const super::MSG, grfmodifiers: super::DWORD) -> windows_core::Result<()>;
    fn OnFocus(&self, fgotfocus: windows_core::BOOL) -> windows_core::Result<()>;
    fn ShowPropertyFrame(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winuser"))]
impl IOleControlSite_Vtbl {
    pub const fn new<Identity: IOleControlSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnControlInfoChanged<Identity: IOleControlSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControlSite_Impl::OnControlInfoChanged(this).into()
            }
        }
        unsafe extern "system" fn LockInPlaceActive<Identity: IOleControlSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControlSite_Impl::LockInPlaceActive(this, core::mem::transmute_copy(&flock)).into()
            }
        }
        unsafe extern "system" fn GetExtendedControl<Identity: IOleControlSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleControlSite_Impl::GetExtendedControl(this) {
                    Ok(ok__) => {
                        ppdisp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransformCoords<Identity: IOleControlSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptlhimetric: *mut super::POINTL, pptfcontainer: *mut POINTF, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControlSite_Impl::TransformCoords(this, core::mem::transmute_copy(&pptlhimetric), core::mem::transmute_copy(&pptfcontainer), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn TranslateAcceleratorA<Identity: IOleControlSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::MSG, grfmodifiers: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControlSite_Impl::TranslateAcceleratorA(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&grfmodifiers)).into()
            }
        }
        unsafe extern "system" fn OnFocus<Identity: IOleControlSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fgotfocus: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControlSite_Impl::OnFocus(this, core::mem::transmute_copy(&fgotfocus)).into()
            }
        }
        unsafe extern "system" fn ShowPropertyFrame<Identity: IOleControlSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControlSite_Impl::ShowPropertyFrame(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnControlInfoChanged: OnControlInfoChanged::<Identity, OFFSET>,
            LockInPlaceActive: LockInPlaceActive::<Identity, OFFSET>,
            GetExtendedControl: GetExtendedControl::<Identity, OFFSET>,
            TransformCoords: TransformCoords::<Identity, OFFSET>,
            TranslateAcceleratorA: TranslateAcceleratorA::<Identity, OFFSET>,
            OnFocus: OnFocus::<Identity, OFFSET>,
            ShowPropertyFrame: ShowPropertyFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleControlSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winuser"))]
impl windows_core::RuntimeName for IOleControlSite {}
#[cfg(feature = "oleidl")]
windows_core::imp::define_interface!(IOleInPlaceObjectWindowless, IOleInPlaceObjectWindowless_Vtbl, 0x1c2056cc_5ef4_101b_8bc8_00aa003e3b29);
#[cfg(feature = "oleidl")]
impl core::ops::Deref for IOleInPlaceObjectWindowless {
    type Target = super::IOleInPlaceObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oleidl")]
windows_core::imp::interface_hierarchy!(IOleInPlaceObjectWindowless, windows_core::IUnknown, super::IOleWindow, super::IOleInPlaceObject);
#[cfg(feature = "oleidl")]
impl IOleInPlaceObjectWindowless {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn OnWindowMessage(&self, msg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::Result<super::LRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnWindowMessage)(windows_core::Interface::as_raw(self), msg, wparam, lparam, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDropTarget(&self) -> windows_core::Result<super::IDropTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDropTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oleidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceObjectWindowless_Vtbl {
    pub base__: super::IOleInPlaceObject_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub OnWindowMessage: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::WPARAM, super::LPARAM, *mut super::LRESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    OnWindowMessage: usize,
    pub GetDropTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shared", feature = "windef"))]
pub trait IOleInPlaceObjectWindowless_Impl: super::IOleInPlaceObject_Impl {
    fn OnWindowMessage(&self, msg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::Result<super::LRESULT>;
    fn GetDropTarget(&self) -> windows_core::Result<super::IDropTarget>;
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shared", feature = "windef"))]
impl IOleInPlaceObjectWindowless_Vtbl {
    pub const fn new<Identity: IOleInPlaceObjectWindowless_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnWindowMessage<Identity: IOleInPlaceObjectWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM, plresult: *mut super::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleInPlaceObjectWindowless_Impl::OnWindowMessage(this, core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        plresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDropTarget<Identity: IOleInPlaceObjectWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdroptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleInPlaceObjectWindowless_Impl::GetDropTarget(this) {
                    Ok(ok__) => {
                        ppdroptarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IOleInPlaceObject_Vtbl::new::<Identity, OFFSET>(),
            OnWindowMessage: OnWindowMessage::<Identity, OFFSET>,
            GetDropTarget: GetDropTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceObjectWindowless as windows_core::Interface>::IID || iid == &<super::IOleWindow as windows_core::Interface>::IID || iid == &<super::IOleInPlaceObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IOleInPlaceObjectWindowless {}
#[cfg(feature = "oleidl")]
windows_core::imp::define_interface!(IOleInPlaceSiteEx, IOleInPlaceSiteEx_Vtbl, 0x9c2cad80_3424_11cf_b670_00aa004cd6d8);
#[cfg(feature = "oleidl")]
impl core::ops::Deref for IOleInPlaceSiteEx {
    type Target = super::IOleInPlaceSite;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oleidl")]
windows_core::imp::interface_hierarchy!(IOleInPlaceSiteEx, windows_core::IUnknown, super::IOleWindow, super::IOleInPlaceSite);
#[cfg(feature = "oleidl")]
impl IOleInPlaceSiteEx {
    #[cfg(feature = "shared")]
    pub unsafe fn OnInPlaceActivateEx(&self, pfnoredraw: *mut windows_core::BOOL, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnInPlaceActivateEx)(windows_core::Interface::as_raw(self), pfnoredraw as _, dwflags) }
    }
    pub unsafe fn OnInPlaceDeactivateEx(&self, fnoredraw: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnInPlaceDeactivateEx)(windows_core::Interface::as_raw(self), fnoredraw.into()) }
    }
    pub unsafe fn RequestUIActivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestUIActivate)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oleidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceSiteEx_Vtbl {
    pub base__: super::IOleInPlaceSite_Vtbl,
    #[cfg(feature = "shared")]
    pub OnInPlaceActivateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnInPlaceActivateEx: usize,
    pub OnInPlaceDeactivateEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub RequestUIActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oleidl", feature = "shared", feature = "windef"))]
pub trait IOleInPlaceSiteEx_Impl: super::IOleInPlaceSite_Impl {
    fn OnInPlaceActivateEx(&self, pfnoredraw: *mut windows_core::BOOL, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn OnInPlaceDeactivateEx(&self, fnoredraw: windows_core::BOOL) -> windows_core::Result<()>;
    fn RequestUIActivate(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oleidl", feature = "shared", feature = "windef"))]
impl IOleInPlaceSiteEx_Vtbl {
    pub const fn new<Identity: IOleInPlaceSiteEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnInPlaceActivateEx<Identity: IOleInPlaceSiteEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfnoredraw: *mut windows_core::BOOL, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteEx_Impl::OnInPlaceActivateEx(this, core::mem::transmute_copy(&pfnoredraw), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OnInPlaceDeactivateEx<Identity: IOleInPlaceSiteEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnoredraw: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteEx_Impl::OnInPlaceDeactivateEx(this, core::mem::transmute_copy(&fnoredraw)).into()
            }
        }
        unsafe extern "system" fn RequestUIActivate<Identity: IOleInPlaceSiteEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteEx_Impl::RequestUIActivate(this).into()
            }
        }
        Self {
            base__: super::IOleInPlaceSite_Vtbl::new::<Identity, OFFSET>(),
            OnInPlaceActivateEx: OnInPlaceActivateEx::<Identity, OFFSET>,
            OnInPlaceDeactivateEx: OnInPlaceDeactivateEx::<Identity, OFFSET>,
            RequestUIActivate: RequestUIActivate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceSiteEx as windows_core::Interface>::IID || iid == &<super::IOleWindow as windows_core::Interface>::IID || iid == &<super::IOleInPlaceSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oleidl", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IOleInPlaceSiteEx {}
#[cfg(feature = "oleidl")]
windows_core::imp::define_interface!(IOleInPlaceSiteWindowless, IOleInPlaceSiteWindowless_Vtbl, 0x922eada0_3424_11cf_b670_00aa004cd6d8);
#[cfg(feature = "oleidl")]
impl core::ops::Deref for IOleInPlaceSiteWindowless {
    type Target = IOleInPlaceSiteEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oleidl")]
windows_core::imp::interface_hierarchy!(IOleInPlaceSiteWindowless, windows_core::IUnknown, super::IOleWindow, super::IOleInPlaceSite, IOleInPlaceSiteEx);
#[cfg(feature = "oleidl")]
impl IOleInPlaceSiteWindowless {
    pub unsafe fn CanWindowlessActivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CanWindowlessActivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCapture(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCapture)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetCapture(&self, fcapture: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCapture)(windows_core::Interface::as_raw(self), fcapture.into()) }
    }
    pub unsafe fn GetFocus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFocus)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetFocus(&self, ffocus: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self), ffocus.into()) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetDC(&self, prect: Option<super::LPCRECT>, grfflags: super::DWORD) -> windows_core::Result<super::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), prect.unwrap_or(core::mem::zeroed()) as _, grfflags, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ReleaseDC(&self, hdc: super::HDC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), hdc) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn InvalidateRect(&self, prect: Option<super::LPCRECT>, ferase: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvalidateRect)(windows_core::Interface::as_raw(self), prect.unwrap_or(core::mem::zeroed()) as _, ferase.into()) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn InvalidateRgn(&self, hrgn: super::HRGN, ferase: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvalidateRgn)(windows_core::Interface::as_raw(self), hrgn, ferase.into()) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn ScrollRect(&self, dx: super::INT, dy: super::INT, prectscroll: super::LPCRECT, prectclip: super::LPCRECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ScrollRect)(windows_core::Interface::as_raw(self), dx, dy, prectscroll, prectclip) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn AdjustRect(&self, prc: super::LPRECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AdjustRect)(windows_core::Interface::as_raw(self), prc as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn OnDefWindowMessage(&self, msg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::Result<super::LRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnDefWindowMessage)(windows_core::Interface::as_raw(self), msg, wparam, lparam, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oleidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceSiteWindowless_Vtbl {
    pub base__: IOleInPlaceSiteEx_Vtbl,
    pub CanWindowlessActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCapture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCapture: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCRECT, super::DWORD, *mut super::HDC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetDC: usize,
    #[cfg(feature = "windef")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ReleaseDC: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub InvalidateRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCRECT, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    InvalidateRect: usize,
    #[cfg(feature = "minwindef")]
    pub InvalidateRgn: unsafe extern "system" fn(*mut core::ffi::c_void, super::HRGN, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    InvalidateRgn: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub ScrollRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::INT, super::INT, super::LPCRECT, super::LPCRECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    ScrollRect: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub AdjustRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPRECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    AdjustRect: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub OnDefWindowMessage: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::WPARAM, super::LPARAM, *mut super::LRESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    OnDefWindowMessage: usize,
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shared", feature = "windef"))]
pub trait IOleInPlaceSiteWindowless_Impl: IOleInPlaceSiteEx_Impl {
    fn CanWindowlessActivate(&self) -> windows_core::Result<()>;
    fn GetCapture(&self) -> windows_core::Result<()>;
    fn SetCapture(&self, fcapture: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetFocus(&self) -> windows_core::Result<()>;
    fn SetFocus(&self, ffocus: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetDC(&self, prect: super::LPCRECT, grfflags: super::DWORD) -> windows_core::Result<super::HDC>;
    fn ReleaseDC(&self, hdc: super::HDC) -> windows_core::Result<()>;
    fn InvalidateRect(&self, prect: super::LPCRECT, ferase: windows_core::BOOL) -> windows_core::Result<()>;
    fn InvalidateRgn(&self, hrgn: super::HRGN, ferase: windows_core::BOOL) -> windows_core::Result<()>;
    fn ScrollRect(&self, dx: super::INT, dy: super::INT, prectscroll: super::LPCRECT, prectclip: super::LPCRECT) -> windows_core::Result<()>;
    fn AdjustRect(&self, prc: super::LPRECT) -> windows_core::Result<()>;
    fn OnDefWindowMessage(&self, msg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::Result<super::LRESULT>;
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shared", feature = "windef"))]
impl IOleInPlaceSiteWindowless_Vtbl {
    pub const fn new<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanWindowlessActivate<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::CanWindowlessActivate(this).into()
            }
        }
        unsafe extern "system" fn GetCapture<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::GetCapture(this).into()
            }
        }
        unsafe extern "system" fn SetCapture<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcapture: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::SetCapture(this, core::mem::transmute_copy(&fcapture)).into()
            }
        }
        unsafe extern "system" fn GetFocus<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::GetFocus(this).into()
            }
        }
        unsafe extern "system" fn SetFocus<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffocus: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::SetFocus(this, core::mem::transmute_copy(&ffocus)).into()
            }
        }
        unsafe extern "system" fn GetDC<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: super::LPCRECT, grfflags: super::DWORD, phdc: *mut super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleInPlaceSiteWindowless_Impl::GetDC(this, core::mem::transmute_copy(&prect), core::mem::transmute_copy(&grfflags)) {
                    Ok(ok__) => {
                        phdc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::ReleaseDC(this, core::mem::transmute_copy(&hdc)).into()
            }
        }
        unsafe extern "system" fn InvalidateRect<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: super::LPCRECT, ferase: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::InvalidateRect(this, core::mem::transmute_copy(&prect), core::mem::transmute_copy(&ferase)).into()
            }
        }
        unsafe extern "system" fn InvalidateRgn<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrgn: super::HRGN, ferase: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::InvalidateRgn(this, core::mem::transmute_copy(&hrgn), core::mem::transmute_copy(&ferase)).into()
            }
        }
        unsafe extern "system" fn ScrollRect<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dx: super::INT, dy: super::INT, prectscroll: super::LPCRECT, prectclip: super::LPCRECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::ScrollRect(this, core::mem::transmute_copy(&dx), core::mem::transmute_copy(&dy), core::mem::transmute_copy(&prectscroll), core::mem::transmute_copy(&prectclip)).into()
            }
        }
        unsafe extern "system" fn AdjustRect<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: super::LPRECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::AdjustRect(this, core::mem::transmute_copy(&prc)).into()
            }
        }
        unsafe extern "system" fn OnDefWindowMessage<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM, plresult: *mut super::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleInPlaceSiteWindowless_Impl::OnDefWindowMessage(this, core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        plresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IOleInPlaceSiteEx_Vtbl::new::<Identity, OFFSET>(),
            CanWindowlessActivate: CanWindowlessActivate::<Identity, OFFSET>,
            GetCapture: GetCapture::<Identity, OFFSET>,
            SetCapture: SetCapture::<Identity, OFFSET>,
            GetFocus: GetFocus::<Identity, OFFSET>,
            SetFocus: SetFocus::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
            InvalidateRect: InvalidateRect::<Identity, OFFSET>,
            InvalidateRgn: InvalidateRgn::<Identity, OFFSET>,
            ScrollRect: ScrollRect::<Identity, OFFSET>,
            AdjustRect: AdjustRect::<Identity, OFFSET>,
            OnDefWindowMessage: OnDefWindowMessage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceSiteWindowless as windows_core::Interface>::IID || iid == &<super::IOleWindow as windows_core::Interface>::IID || iid == &<super::IOleInPlaceSite as windows_core::Interface>::IID || iid == &<IOleInPlaceSiteEx as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IOleInPlaceSiteWindowless {}
windows_core::imp::define_interface!(IOleParentUndoUnit, IOleParentUndoUnit_Vtbl, 0xa1faf330_ef97_11ce_9bc9_00aa00608e01);
impl core::ops::Deref for IOleParentUndoUnit {
    type Target = IOleUndoUnit;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleParentUndoUnit, windows_core::IUnknown, IOleUndoUnit);
impl IOleParentUndoUnit {
    pub unsafe fn Open<P0>(&self, ppuu: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), ppuu.param().abi()) }
    }
    pub unsafe fn Close<P0>(&self, ppuu: P0, fcommit: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), ppuu.param().abi(), fcommit.into()) }
    }
    pub unsafe fn Add<P0>(&self, puu: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IOleUndoUnit>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), puu.param().abi()) }
    }
    pub unsafe fn FindUnit<P0>(&self, puu: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IOleUndoUnit>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindUnit)(windows_core::Interface::as_raw(self), puu.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetParentState(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleParentUndoUnit_Vtbl {
    pub base__: IOleUndoUnit_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetParentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetParentState: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IOleParentUndoUnit_Impl: IOleUndoUnit_Impl {
    fn Open(&self, ppuu: windows_core::Ref<IOleParentUndoUnit>) -> windows_core::Result<()>;
    fn Close(&self, ppuu: windows_core::Ref<IOleParentUndoUnit>, fcommit: windows_core::BOOL) -> windows_core::Result<()>;
    fn Add(&self, puu: windows_core::Ref<IOleUndoUnit>) -> windows_core::Result<()>;
    fn FindUnit(&self, puu: windows_core::Ref<IOleUndoUnit>) -> windows_core::Result<()>;
    fn GetParentState(&self) -> windows_core::Result<super::DWORD>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IOleParentUndoUnit_Vtbl {
    pub const fn new<Identity: IOleParentUndoUnit_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppuu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleParentUndoUnit_Impl::Open(this, core::mem::transmute_copy(&ppuu)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppuu: *mut core::ffi::c_void, fcommit: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleParentUndoUnit_Impl::Close(this, core::mem::transmute_copy(&ppuu), core::mem::transmute_copy(&fcommit)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleParentUndoUnit_Impl::Add(this, core::mem::transmute_copy(&puu)).into()
            }
        }
        unsafe extern "system" fn FindUnit<Identity: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleParentUndoUnit_Impl::FindUnit(this, core::mem::transmute_copy(&puu)).into()
            }
        }
        unsafe extern "system" fn GetParentState<Identity: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleParentUndoUnit_Impl::GetParentState(this) {
                    Ok(ok__) => {
                        pdwstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IOleUndoUnit_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            FindUnit: FindUnit::<Identity, OFFSET>,
            GetParentState: GetParentState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleParentUndoUnit as windows_core::Interface>::IID || iid == &<IOleUndoUnit as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IOleParentUndoUnit {}
windows_core::imp::define_interface!(IOleUndoManager, IOleUndoManager_Vtbl, 0xd001f200_ef97_11ce_9bc9_00aa00608e01);
windows_core::imp::interface_hierarchy!(IOleUndoManager, windows_core::IUnknown);
impl IOleUndoManager {
    pub unsafe fn Open<P0>(&self, ppuu: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IOleParentUndoUnit>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), ppuu.param().abi()) }
    }
    pub unsafe fn Close<P0>(&self, ppuu: P0, fcommit: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IOleParentUndoUnit>,
    {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), ppuu.param().abi(), fcommit.into()) }
    }
    pub unsafe fn Add<P0>(&self, puu: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IOleUndoUnit>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), puu.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOpenParentState(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOpenParentState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DiscardFrom<P0>(&self, puu: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IOleUndoUnit>,
    {
        unsafe { (windows_core::Interface::vtable(self).DiscardFrom)(windows_core::Interface::as_raw(self), puu.param().abi()) }
    }
    pub unsafe fn UndoTo<P0>(&self, puu: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IOleUndoUnit>,
    {
        unsafe { (windows_core::Interface::vtable(self).UndoTo)(windows_core::Interface::as_raw(self), puu.param().abi()) }
    }
    pub unsafe fn RedoTo<P0>(&self, puu: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IOleUndoUnit>,
    {
        unsafe { (windows_core::Interface::vtable(self).RedoTo)(windows_core::Interface::as_raw(self), puu.param().abi()) }
    }
    pub unsafe fn EnumUndoable(&self) -> windows_core::Result<IEnumOleUndoUnits> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumUndoable)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumRedoable(&self) -> windows_core::Result<IEnumOleUndoUnits> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRedoable)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLastUndoDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastUndoDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetLastRedoDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastRedoDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Enable(&self, fenable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), fenable.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUndoManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetOpenParentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOpenParentState: usize,
    pub DiscardFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UndoTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RedoTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumUndoable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumRedoable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastUndoDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastRedoDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IOleUndoManager_Impl: windows_core::IUnknownImpl {
    fn Open(&self, ppuu: windows_core::Ref<IOleParentUndoUnit>) -> windows_core::Result<()>;
    fn Close(&self, ppuu: windows_core::Ref<IOleParentUndoUnit>, fcommit: windows_core::BOOL) -> windows_core::Result<()>;
    fn Add(&self, puu: windows_core::Ref<IOleUndoUnit>) -> windows_core::Result<()>;
    fn GetOpenParentState(&self) -> windows_core::Result<super::DWORD>;
    fn DiscardFrom(&self, puu: windows_core::Ref<IOleUndoUnit>) -> windows_core::Result<()>;
    fn UndoTo(&self, puu: windows_core::Ref<IOleUndoUnit>) -> windows_core::Result<()>;
    fn RedoTo(&self, puu: windows_core::Ref<IOleUndoUnit>) -> windows_core::Result<()>;
    fn EnumUndoable(&self) -> windows_core::Result<IEnumOleUndoUnits>;
    fn EnumRedoable(&self) -> windows_core::Result<IEnumOleUndoUnits>;
    fn GetLastUndoDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetLastRedoDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Enable(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IOleUndoManager_Vtbl {
    pub const fn new<Identity: IOleUndoManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppuu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleUndoManager_Impl::Open(this, core::mem::transmute_copy(&ppuu)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppuu: *mut core::ffi::c_void, fcommit: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleUndoManager_Impl::Close(this, core::mem::transmute_copy(&ppuu), core::mem::transmute_copy(&fcommit)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleUndoManager_Impl::Add(this, core::mem::transmute_copy(&puu)).into()
            }
        }
        unsafe extern "system" fn GetOpenParentState<Identity: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleUndoManager_Impl::GetOpenParentState(this) {
                    Ok(ok__) => {
                        pdwstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DiscardFrom<Identity: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleUndoManager_Impl::DiscardFrom(this, core::mem::transmute_copy(&puu)).into()
            }
        }
        unsafe extern "system" fn UndoTo<Identity: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleUndoManager_Impl::UndoTo(this, core::mem::transmute_copy(&puu)).into()
            }
        }
        unsafe extern "system" fn RedoTo<Identity: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleUndoManager_Impl::RedoTo(this, core::mem::transmute_copy(&puu)).into()
            }
        }
        unsafe extern "system" fn EnumUndoable<Identity: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleUndoManager_Impl::EnumUndoable(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumRedoable<Identity: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleUndoManager_Impl::EnumRedoable(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastUndoDescription<Identity: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleUndoManager_Impl::GetLastUndoDescription(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastRedoDescription<Identity: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleUndoManager_Impl::GetLastRedoDescription(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enable<Identity: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleUndoManager_Impl::Enable(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            GetOpenParentState: GetOpenParentState::<Identity, OFFSET>,
            DiscardFrom: DiscardFrom::<Identity, OFFSET>,
            UndoTo: UndoTo::<Identity, OFFSET>,
            RedoTo: RedoTo::<Identity, OFFSET>,
            EnumUndoable: EnumUndoable::<Identity, OFFSET>,
            EnumRedoable: EnumRedoable::<Identity, OFFSET>,
            GetLastUndoDescription: GetLastUndoDescription::<Identity, OFFSET>,
            GetLastRedoDescription: GetLastRedoDescription::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleUndoManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IOleUndoManager {}
windows_core::imp::define_interface!(IOleUndoUnit, IOleUndoUnit_Vtbl, 0x894ad3b0_ef97_11ce_9bc9_00aa00608e01);
windows_core::imp::interface_hierarchy!(IOleUndoUnit, windows_core::IUnknown);
impl IOleUndoUnit {
    pub unsafe fn Do<P0>(&self, pundomanager: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IOleUndoManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).Do)(windows_core::Interface::as_raw(self), pundomanager.param().abi()) }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetUnitType(&self, pclsid: *mut super::CLSID, plid: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetUnitType)(windows_core::Interface::as_raw(self), pclsid as _, plid as _) }
    }
    pub unsafe fn OnNextAdd(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnNextAdd)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUndoUnit_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Do: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetUnitType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CLSID, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetUnitType: usize,
    pub OnNextAdd: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IOleUndoUnit_Impl: windows_core::IUnknownImpl {
    fn Do(&self, pundomanager: windows_core::Ref<IOleUndoManager>) -> windows_core::Result<()>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetUnitType(&self, pclsid: *mut super::CLSID, plid: *mut super::LONG) -> windows_core::Result<()>;
    fn OnNextAdd(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IOleUndoUnit_Vtbl {
    pub const fn new<Identity: IOleUndoUnit_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Do<Identity: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pundomanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleUndoUnit_Impl::Do(this, core::mem::transmute_copy(&pundomanager)).into()
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleUndoUnit_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUnitType<Identity: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut super::CLSID, plid: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleUndoUnit_Impl::GetUnitType(this, core::mem::transmute_copy(&pclsid), core::mem::transmute_copy(&plid)).into()
            }
        }
        unsafe extern "system" fn OnNextAdd<Identity: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleUndoUnit_Impl::OnNextAdd(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Do: Do::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetUnitType: GetUnitType::<Identity, OFFSET>,
            OnNextAdd: OnNextAdd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleUndoUnit as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IOleUndoUnit {}
windows_core::imp::define_interface!(IPerPropertyBrowsing, IPerPropertyBrowsing_Vtbl, 0x376bd3aa_3845_101b_84ed_08002b2ec713);
windows_core::imp::interface_hierarchy!(IPerPropertyBrowsing, windows_core::IUnknown);
impl IPerPropertyBrowsing {
    #[cfg(feature = "shared")]
    pub unsafe fn GetDisplayString(&self, dispid: super::DISPID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayString)(windows_core::Interface::as_raw(self), dispid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn MapPropertyToPage(&self, dispid: super::DISPID) -> windows_core::Result<super::CLSID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MapPropertyToPage)(windows_core::Interface::as_raw(self), dispid, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn GetPredefinedStrings(&self, dispid: super::DISPID, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPredefinedStrings)(windows_core::Interface::as_raw(self), dispid, pcastringsout as _, pcacookiesout as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPredefinedValue(&self, dispid: super::DISPID, dwcookie: super::DWORD) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPredefinedValue)(windows_core::Interface::as_raw(self), dispid, dwcookie, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerPropertyBrowsing_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetDisplayString: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDisplayString: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub MapPropertyToPage: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID, *mut super::CLSID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    MapPropertyToPage: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub GetPredefinedStrings: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID, *mut CALPOLESTR, *mut CADWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    GetPredefinedStrings: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPredefinedValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID, super::DWORD, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetPredefinedValue: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPerPropertyBrowsing_Impl: windows_core::IUnknownImpl {
    fn GetDisplayString(&self, dispid: super::DISPID) -> windows_core::Result<windows_core::BSTR>;
    fn MapPropertyToPage(&self, dispid: super::DISPID) -> windows_core::Result<super::CLSID>;
    fn GetPredefinedStrings(&self, dispid: super::DISPID, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> windows_core::Result<()>;
    fn GetPredefinedValue(&self, dispid: super::DISPID, dwcookie: super::DWORD) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPerPropertyBrowsing_Vtbl {
    pub const fn new<Identity: IPerPropertyBrowsing_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDisplayString<Identity: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::DISPID, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerPropertyBrowsing_Impl::GetDisplayString(this, core::mem::transmute_copy(&dispid)) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MapPropertyToPage<Identity: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::DISPID, pclsid: *mut super::CLSID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerPropertyBrowsing_Impl::MapPropertyToPage(this, core::mem::transmute_copy(&dispid)) {
                    Ok(ok__) => {
                        pclsid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPredefinedStrings<Identity: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::DISPID, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerPropertyBrowsing_Impl::GetPredefinedStrings(this, core::mem::transmute_copy(&dispid), core::mem::transmute_copy(&pcastringsout), core::mem::transmute_copy(&pcacookiesout)).into()
            }
        }
        unsafe extern "system" fn GetPredefinedValue<Identity: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::DISPID, dwcookie: super::DWORD, pvarout: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerPropertyBrowsing_Impl::GetPredefinedValue(this, core::mem::transmute_copy(&dispid), core::mem::transmute_copy(&dwcookie)) {
                    Ok(ok__) => {
                        pvarout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayString: GetDisplayString::<Identity, OFFSET>,
            MapPropertyToPage: MapPropertyToPage::<Identity, OFFSET>,
            GetPredefinedStrings: GetPredefinedStrings::<Identity, OFFSET>,
            GetPredefinedValue: GetPredefinedValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPerPropertyBrowsing as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPerPropertyBrowsing {}
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(IPersistMemory, IPersistMemory_Vtbl, 0xbd1ae5e0_a6ae_11ce_bd37_504200c10000);
#[cfg(feature = "objidl")]
impl core::ops::Deref for IPersistMemory {
    type Target = super::IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(IPersistMemory, windows_core::IUnknown, super::IPersist);
#[cfg(feature = "objidl")]
impl IPersistMemory {
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Load(&self, pmem: super::LPVOID, cbsize: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pmem, cbsize) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Save(&self, pmem: super::LPVOID, fcleardirty: bool, cbsize: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pmem as _, fcleardirty.into(), cbsize) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSizeMax(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSizeMax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InitNew(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "objidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPersistMemory_Vtbl {
    pub base__: super::IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPVOID, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Load: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPVOID, windows_core::BOOL, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Save: usize,
    #[cfg(feature = "shared")]
    pub GetSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSizeMax: usize,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "shared"))]
pub trait IPersistMemory_Impl: super::IPersist_Impl {
    fn IsDirty(&self) -> windows_core::Result<()>;
    fn Load(&self, pmem: super::LPVOID, cbsize: super::ULONG) -> windows_core::Result<()>;
    fn Save(&self, pmem: super::LPVOID, fcleardirty: windows_core::BOOL, cbsize: super::ULONG) -> windows_core::Result<()>;
    fn GetSizeMax(&self) -> windows_core::Result<super::ULONG>;
    fn InitNew(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "shared"))]
impl IPersistMemory_Vtbl {
    pub const fn new<Identity: IPersistMemory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDirty<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMemory_Impl::IsDirty(this).into()
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: super::LPVOID, cbsize: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMemory_Impl::Load(this, core::mem::transmute_copy(&pmem), core::mem::transmute_copy(&cbsize)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: super::LPVOID, fcleardirty: windows_core::BOOL, cbsize: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMemory_Impl::Save(this, core::mem::transmute_copy(&pmem), core::mem::transmute_copy(&fcleardirty), core::mem::transmute_copy(&cbsize)).into()
            }
        }
        unsafe extern "system" fn GetSizeMax<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersistMemory_Impl::GetSizeMax(this) {
                    Ok(ok__) => {
                        pcbsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitNew<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMemory_Impl::InitNew(this).into()
            }
        }
        Self {
            base__: super::IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, OFFSET>,
            InitNew: InitNew::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistMemory as windows_core::Interface>::IID || iid == &<super::IPersist as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "shared"))]
impl windows_core::RuntimeName for IPersistMemory {}
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(IPersistPropertyBag, IPersistPropertyBag_Vtbl, 0x37d84f60_42cb_11ce_8135_00aa004bb851);
#[cfg(feature = "objidl")]
impl core::ops::Deref for IPersistPropertyBag {
    type Target = super::IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(IPersistPropertyBag, windows_core::IUnknown, super::IPersist);
#[cfg(feature = "objidl")]
impl IPersistPropertyBag {
    pub unsafe fn InitNew(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn Load<P0, P1>(&self, ppropbag: P0, perrorlog: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IPropertyBag>,
        P1: windows_core::Param<super::IErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), ppropbag.param().abi(), perrorlog.param().abi()) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn Save<P0>(&self, ppropbag: P0, fcleardirty: bool, fsaveallproperties: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IPropertyBag>,
    {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), ppropbag.param().abi(), fcleardirty.into(), fsaveallproperties.into()) }
    }
}
#[cfg(feature = "objidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPersistPropertyBag_Vtbl {
    pub base__: super::IPersist_Vtbl,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    Load: usize,
    #[cfg(feature = "oaidl")]
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    Save: usize,
}
#[cfg(all(feature = "guiddef", feature = "oaidl", feature = "objidl"))]
pub trait IPersistPropertyBag_Impl: super::IPersist_Impl {
    fn InitNew(&self) -> windows_core::Result<()>;
    fn Load(&self, ppropbag: windows_core::Ref<super::IPropertyBag>, perrorlog: windows_core::Ref<super::IErrorLog>) -> windows_core::Result<()>;
    fn Save(&self, ppropbag: windows_core::Ref<super::IPropertyBag>, fcleardirty: windows_core::BOOL, fsaveallproperties: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "oaidl", feature = "objidl"))]
impl IPersistPropertyBag_Vtbl {
    pub const fn new<Identity: IPersistPropertyBag_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitNew<Identity: IPersistPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistPropertyBag_Impl::InitNew(this).into()
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropbag: *mut core::ffi::c_void, perrorlog: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistPropertyBag_Impl::Load(this, core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&perrorlog)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropbag: *mut core::ffi::c_void, fcleardirty: windows_core::BOOL, fsaveallproperties: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistPropertyBag_Impl::Save(this, core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&fcleardirty), core::mem::transmute_copy(&fsaveallproperties)).into()
            }
        }
        Self {
            base__: super::IPersist_Vtbl::new::<Identity, OFFSET>(),
            InitNew: InitNew::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistPropertyBag as windows_core::Interface>::IID || iid == &<super::IPersist as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "oaidl", feature = "objidl"))]
impl windows_core::RuntimeName for IPersistPropertyBag {}
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(IPersistPropertyBag2, IPersistPropertyBag2_Vtbl, 0x22f55881_280b_11d0_a8a9_00a0c90c2004);
#[cfg(feature = "objidl")]
impl core::ops::Deref for IPersistPropertyBag2 {
    type Target = super::IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(IPersistPropertyBag2, windows_core::IUnknown, super::IPersist);
#[cfg(feature = "objidl")]
impl IPersistPropertyBag2 {
    pub unsafe fn InitNew(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn Load<P0, P1>(&self, ppropbag: P0, perrlog: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IPropertyBag2>,
        P1: windows_core::Param<super::IErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), ppropbag.param().abi(), perrlog.param().abi()) }
    }
    pub unsafe fn Save<P0>(&self, ppropbag: P0, fcleardirty: bool, fsaveallproperties: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IPropertyBag2>,
    {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), ppropbag.param().abi(), fcleardirty.into(), fsaveallproperties.into()) }
    }
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "objidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPersistPropertyBag2_Vtbl {
    pub base__: super::IPersist_Vtbl,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    Load: usize,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "oaidl", feature = "objidl"))]
pub trait IPersistPropertyBag2_Impl: super::IPersist_Impl {
    fn InitNew(&self) -> windows_core::Result<()>;
    fn Load(&self, ppropbag: windows_core::Ref<IPropertyBag2>, perrlog: windows_core::Ref<super::IErrorLog>) -> windows_core::Result<()>;
    fn Save(&self, ppropbag: windows_core::Ref<IPropertyBag2>, fcleardirty: windows_core::BOOL, fsaveallproperties: windows_core::BOOL) -> windows_core::Result<()>;
    fn IsDirty(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "oaidl", feature = "objidl"))]
impl IPersistPropertyBag2_Vtbl {
    pub const fn new<Identity: IPersistPropertyBag2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitNew<Identity: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistPropertyBag2_Impl::InitNew(this).into()
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropbag: *mut core::ffi::c_void, perrlog: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistPropertyBag2_Impl::Load(this, core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&perrlog)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropbag: *mut core::ffi::c_void, fcleardirty: windows_core::BOOL, fsaveallproperties: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistPropertyBag2_Impl::Save(this, core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&fcleardirty), core::mem::transmute_copy(&fsaveallproperties)).into()
            }
        }
        unsafe extern "system" fn IsDirty<Identity: IPersistPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistPropertyBag2_Impl::IsDirty(this).into()
            }
        }
        Self {
            base__: super::IPersist_Vtbl::new::<Identity, OFFSET>(),
            InitNew: InitNew::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            IsDirty: IsDirty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistPropertyBag2 as windows_core::Interface>::IID || iid == &<super::IPersist as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "oaidl", feature = "objidl"))]
impl windows_core::RuntimeName for IPersistPropertyBag2 {}
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(IPersistStreamInit, IPersistStreamInit_Vtbl, 0x7fd52380_4e07_101b_ae2d_08002b2ec713);
#[cfg(feature = "objidl")]
impl core::ops::Deref for IPersistStreamInit {
    type Target = super::IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(IPersistStreamInit, windows_core::IUnknown, super::IPersist);
#[cfg(feature = "objidl")]
impl IPersistStreamInit {
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Load<P0>(&self, pstm: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::LPSTREAM>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pstm.param().abi()) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Save<P0>(&self, pstm: P0, fcleardirty: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::LPSTREAM>,
    {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pstm.param().abi(), fcleardirty.into()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetSizeMax(&self) -> windows_core::Result<super::ULARGE_INTEGER> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSizeMax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InitNew(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "objidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStreamInit_Vtbl {
    pub base__: super::IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Load: usize,
    #[cfg(feature = "objidlbase")]
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Save: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULARGE_INTEGER) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetSizeMax: usize,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt"))]
pub trait IPersistStreamInit_Impl: super::IPersist_Impl {
    fn IsDirty(&self) -> windows_core::Result<()>;
    fn Load(&self, pstm: windows_core::Ref<super::LPSTREAM>) -> windows_core::Result<()>;
    fn Save(&self, pstm: windows_core::Ref<super::LPSTREAM>, fcleardirty: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSizeMax(&self) -> windows_core::Result<super::ULARGE_INTEGER>;
    fn InitNew(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt"))]
impl IPersistStreamInit_Vtbl {
    pub const fn new<Identity: IPersistStreamInit_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDirty<Identity: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStreamInit_Impl::IsDirty(this).into()
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStreamInit_Impl::Load(this, core::mem::transmute_copy(&pstm)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, fcleardirty: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStreamInit_Impl::Save(this, core::mem::transmute_copy(&pstm), core::mem::transmute_copy(&fcleardirty)).into()
            }
        }
        unsafe extern "system" fn GetSizeMax<Identity: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut super::ULARGE_INTEGER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersistStreamInit_Impl::GetSizeMax(this) {
                    Ok(ok__) => {
                        pcbsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitNew<Identity: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStreamInit_Impl::InitNew(this).into()
            }
        }
        Self {
            base__: super::IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, OFFSET>,
            InitNew: InitNew::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistStreamInit as windows_core::Interface>::IID || iid == &<super::IPersist as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IPersistStreamInit {}
windows_core::imp::define_interface!(IPicture, IPicture_Vtbl, 0x7bf80980_bf32_101a_8bbb_00aa00300cab);
windows_core::imp::interface_hierarchy!(IPicture, windows_core::IUnknown);
impl IPicture {
    pub unsafe fn get_Handle(&self) -> windows_core::Result<OLE_HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_hPal(&self) -> windows_core::Result<OLE_HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_hPal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Type(&self) -> windows_core::Result<super::SHORT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Width(&self) -> windows_core::Result<OLE_XSIZE_HIMETRIC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Height(&self) -> windows_core::Result<OLE_YSIZE_HIMETRIC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn Render(&self, hdc: super::HDC, x: super::LONG, y: super::LONG, cx: super::LONG, cy: super::LONG, xsrc: OLE_XPOS_HIMETRIC, ysrc: OLE_YPOS_HIMETRIC, cxsrc: OLE_XSIZE_HIMETRIC, cysrc: OLE_YSIZE_HIMETRIC, prcwbounds: super::LPCRECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Render)(windows_core::Interface::as_raw(self), hdc, x, y, cx, cy, xsrc, ysrc, cxsrc, cysrc, prcwbounds) }
    }
    pub unsafe fn set_hPal(&self, hpal: OLE_HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).set_hPal)(windows_core::Interface::as_raw(self), hpal) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn get_CurDC(&self) -> windows_core::Result<super::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SelectPicture(&self, hdcin: super::HDC, phdcout: *mut super::HDC, phbmpout: *mut OLE_HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SelectPicture)(windows_core::Interface::as_raw(self), hdcin, phdcout as _, phbmpout as _) }
    }
    pub unsafe fn get_KeepOriginalFormat(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KeepOriginalFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_KeepOriginalFormat(&self, keep: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_KeepOriginalFormat)(windows_core::Interface::as_raw(self), keep.into()) }
    }
    pub unsafe fn PictureChanged(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PictureChanged)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub unsafe fn SaveAsFile<P0>(&self, pstream: P0, fsavememcopy: bool) -> windows_core::Result<super::LONG>
    where
        P0: windows_core::Param<super::LPSTREAM>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SaveAsFile)(windows_core::Interface::as_raw(self), pstream.param().abi(), fsavememcopy.into(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Attributes(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Attributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPicture_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OLE_HANDLE) -> windows_core::HRESULT,
    pub get_hPal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OLE_HANDLE) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SHORT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Type: usize,
    pub get_Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OLE_XSIZE_HIMETRIC) -> windows_core::HRESULT,
    pub get_Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OLE_YSIZE_HIMETRIC) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub Render: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC, super::LONG, super::LONG, super::LONG, super::LONG, OLE_XPOS_HIMETRIC, OLE_YPOS_HIMETRIC, OLE_XSIZE_HIMETRIC, OLE_YSIZE_HIMETRIC, super::LPCRECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    Render: usize,
    pub set_hPal: unsafe extern "system" fn(*mut core::ffi::c_void, OLE_HANDLE) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub get_CurDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    get_CurDC: usize,
    #[cfg(feature = "windef")]
    pub SelectPicture: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC, *mut super::HDC, *mut OLE_HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SelectPicture: usize,
    pub get_KeepOriginalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_KeepOriginalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub PictureChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub SaveAsFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "shared")))]
    SaveAsFile: usize,
    #[cfg(feature = "shared")]
    pub get_Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Attributes: usize,
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "windef"))]
pub trait IPicture_Impl: windows_core::IUnknownImpl {
    fn get_Handle(&self) -> windows_core::Result<OLE_HANDLE>;
    fn get_hPal(&self) -> windows_core::Result<OLE_HANDLE>;
    fn get_Type(&self) -> windows_core::Result<super::SHORT>;
    fn get_Width(&self) -> windows_core::Result<OLE_XSIZE_HIMETRIC>;
    fn get_Height(&self) -> windows_core::Result<OLE_YSIZE_HIMETRIC>;
    fn Render(&self, hdc: super::HDC, x: super::LONG, y: super::LONG, cx: super::LONG, cy: super::LONG, xsrc: OLE_XPOS_HIMETRIC, ysrc: OLE_YPOS_HIMETRIC, cxsrc: OLE_XSIZE_HIMETRIC, cysrc: OLE_YSIZE_HIMETRIC, prcwbounds: super::LPCRECT) -> windows_core::Result<()>;
    fn set_hPal(&self, hpal: OLE_HANDLE) -> windows_core::Result<()>;
    fn get_CurDC(&self) -> windows_core::Result<super::HDC>;
    fn SelectPicture(&self, hdcin: super::HDC, phdcout: *mut super::HDC, phbmpout: *mut OLE_HANDLE) -> windows_core::Result<()>;
    fn get_KeepOriginalFormat(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_KeepOriginalFormat(&self, keep: windows_core::BOOL) -> windows_core::Result<()>;
    fn PictureChanged(&self) -> windows_core::Result<()>;
    fn SaveAsFile(&self, pstream: windows_core::Ref<super::LPSTREAM>, fsavememcopy: windows_core::BOOL) -> windows_core::Result<super::LONG>;
    fn get_Attributes(&self) -> windows_core::Result<super::DWORD>;
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "windef"))]
impl IPicture_Vtbl {
    pub const fn new<Identity: IPicture_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Handle<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut OLE_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture_Impl::get_Handle(this) {
                    Ok(ok__) => {
                        phandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_hPal<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phpal: *mut OLE_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture_Impl::get_hPal(this) {
                    Ok(ok__) => {
                        phpal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Type<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut super::SHORT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture_Impl::get_Type(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Width<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut OLE_XSIZE_HIMETRIC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture_Impl::get_Width(this) {
                    Ok(ok__) => {
                        pwidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Height<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheight: *mut OLE_YSIZE_HIMETRIC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture_Impl::get_Height(this) {
                    Ok(ok__) => {
                        pheight.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Render<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::HDC, x: super::LONG, y: super::LONG, cx: super::LONG, cy: super::LONG, xsrc: OLE_XPOS_HIMETRIC, ysrc: OLE_YPOS_HIMETRIC, cxsrc: OLE_XSIZE_HIMETRIC, cysrc: OLE_YSIZE_HIMETRIC, prcwbounds: super::LPCRECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPicture_Impl::Render(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy), core::mem::transmute_copy(&xsrc), core::mem::transmute_copy(&ysrc), core::mem::transmute_copy(&cxsrc), core::mem::transmute_copy(&cysrc), core::mem::transmute_copy(&prcwbounds)).into()
            }
        }
        unsafe extern "system" fn set_hPal<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpal: OLE_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPicture_Impl::set_hPal(this, core::mem::transmute_copy(&hpal)).into()
            }
        }
        unsafe extern "system" fn get_CurDC<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdc: *mut super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture_Impl::get_CurDC(this) {
                    Ok(ok__) => {
                        phdc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectPicture<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdcin: super::HDC, phdcout: *mut super::HDC, phbmpout: *mut OLE_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPicture_Impl::SelectPicture(this, core::mem::transmute_copy(&hdcin), core::mem::transmute_copy(&phdcout), core::mem::transmute_copy(&phbmpout)).into()
            }
        }
        unsafe extern "system" fn get_KeepOriginalFormat<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeep: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture_Impl::get_KeepOriginalFormat(this) {
                    Ok(ok__) => {
                        pkeep.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_KeepOriginalFormat<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keep: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPicture_Impl::put_KeepOriginalFormat(this, core::mem::transmute_copy(&keep)).into()
            }
        }
        unsafe extern "system" fn PictureChanged<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPicture_Impl::PictureChanged(this).into()
            }
        }
        unsafe extern "system" fn SaveAsFile<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, fsavememcopy: windows_core::BOOL, pcbsize: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture_Impl::SaveAsFile(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&fsavememcopy)) {
                    Ok(ok__) => {
                        pcbsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Attributes<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattr: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture_Impl::get_Attributes(this) {
                    Ok(ok__) => {
                        pdwattr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Handle: get_Handle::<Identity, OFFSET>,
            get_hPal: get_hPal::<Identity, OFFSET>,
            get_Type: get_Type::<Identity, OFFSET>,
            get_Width: get_Width::<Identity, OFFSET>,
            get_Height: get_Height::<Identity, OFFSET>,
            Render: Render::<Identity, OFFSET>,
            set_hPal: set_hPal::<Identity, OFFSET>,
            get_CurDC: get_CurDC::<Identity, OFFSET>,
            SelectPicture: SelectPicture::<Identity, OFFSET>,
            get_KeepOriginalFormat: get_KeepOriginalFormat::<Identity, OFFSET>,
            put_KeepOriginalFormat: put_KeepOriginalFormat::<Identity, OFFSET>,
            PictureChanged: PictureChanged::<Identity, OFFSET>,
            SaveAsFile: SaveAsFile::<Identity, OFFSET>,
            get_Attributes: get_Attributes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPicture as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IPicture {}
windows_core::imp::define_interface!(IPicture2, IPicture2_Vtbl, 0xf5185dd8_2012_4b0b_aad9_f052c6bd482b);
windows_core::imp::interface_hierarchy!(IPicture2, windows_core::IUnknown);
impl IPicture2 {
    pub unsafe fn get_Handle(&self) -> windows_core::Result<HHANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_hPal(&self) -> windows_core::Result<HHANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_hPal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Type(&self) -> windows_core::Result<super::SHORT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Width(&self) -> windows_core::Result<OLE_XSIZE_HIMETRIC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Height(&self) -> windows_core::Result<OLE_YSIZE_HIMETRIC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn Render(&self, hdc: super::HDC, x: super::LONG, y: super::LONG, cx: super::LONG, cy: super::LONG, xsrc: OLE_XPOS_HIMETRIC, ysrc: OLE_YPOS_HIMETRIC, cxsrc: OLE_XSIZE_HIMETRIC, cysrc: OLE_YSIZE_HIMETRIC, prcwbounds: super::LPCRECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Render)(windows_core::Interface::as_raw(self), hdc, x, y, cx, cy, xsrc, ysrc, cxsrc, cysrc, prcwbounds) }
    }
    pub unsafe fn set_hPal(&self, hpal: HHANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).set_hPal)(windows_core::Interface::as_raw(self), hpal) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn get_CurDC(&self) -> windows_core::Result<super::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SelectPicture(&self, hdcin: super::HDC, phdcout: *mut super::HDC, phbmpout: *mut HHANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SelectPicture)(windows_core::Interface::as_raw(self), hdcin, phdcout as _, phbmpout as _) }
    }
    pub unsafe fn get_KeepOriginalFormat(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KeepOriginalFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_KeepOriginalFormat(&self, keep: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_KeepOriginalFormat)(windows_core::Interface::as_raw(self), keep.into()) }
    }
    pub unsafe fn PictureChanged(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PictureChanged)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub unsafe fn SaveAsFile<P0>(&self, pstream: P0, fsavememcopy: bool) -> windows_core::Result<super::LONG>
    where
        P0: windows_core::Param<super::LPSTREAM>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SaveAsFile)(windows_core::Interface::as_raw(self), pstream.param().abi(), fsavememcopy.into(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Attributes(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Attributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPicture2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HHANDLE) -> windows_core::HRESULT,
    pub get_hPal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HHANDLE) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SHORT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Type: usize,
    pub get_Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OLE_XSIZE_HIMETRIC) -> windows_core::HRESULT,
    pub get_Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OLE_YSIZE_HIMETRIC) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub Render: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC, super::LONG, super::LONG, super::LONG, super::LONG, OLE_XPOS_HIMETRIC, OLE_YPOS_HIMETRIC, OLE_XSIZE_HIMETRIC, OLE_YSIZE_HIMETRIC, super::LPCRECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    Render: usize,
    pub set_hPal: unsafe extern "system" fn(*mut core::ffi::c_void, HHANDLE) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub get_CurDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    get_CurDC: usize,
    #[cfg(feature = "windef")]
    pub SelectPicture: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC, *mut super::HDC, *mut HHANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SelectPicture: usize,
    pub get_KeepOriginalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_KeepOriginalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub PictureChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub SaveAsFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "shared")))]
    SaveAsFile: usize,
    #[cfg(feature = "shared")]
    pub get_Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Attributes: usize,
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "windef"))]
pub trait IPicture2_Impl: windows_core::IUnknownImpl {
    fn get_Handle(&self) -> windows_core::Result<HHANDLE>;
    fn get_hPal(&self) -> windows_core::Result<HHANDLE>;
    fn get_Type(&self) -> windows_core::Result<super::SHORT>;
    fn get_Width(&self) -> windows_core::Result<OLE_XSIZE_HIMETRIC>;
    fn get_Height(&self) -> windows_core::Result<OLE_YSIZE_HIMETRIC>;
    fn Render(&self, hdc: super::HDC, x: super::LONG, y: super::LONG, cx: super::LONG, cy: super::LONG, xsrc: OLE_XPOS_HIMETRIC, ysrc: OLE_YPOS_HIMETRIC, cxsrc: OLE_XSIZE_HIMETRIC, cysrc: OLE_YSIZE_HIMETRIC, prcwbounds: super::LPCRECT) -> windows_core::Result<()>;
    fn set_hPal(&self, hpal: HHANDLE) -> windows_core::Result<()>;
    fn get_CurDC(&self) -> windows_core::Result<super::HDC>;
    fn SelectPicture(&self, hdcin: super::HDC, phdcout: *mut super::HDC, phbmpout: *mut HHANDLE) -> windows_core::Result<()>;
    fn get_KeepOriginalFormat(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_KeepOriginalFormat(&self, keep: windows_core::BOOL) -> windows_core::Result<()>;
    fn PictureChanged(&self) -> windows_core::Result<()>;
    fn SaveAsFile(&self, pstream: windows_core::Ref<super::LPSTREAM>, fsavememcopy: windows_core::BOOL) -> windows_core::Result<super::LONG>;
    fn get_Attributes(&self) -> windows_core::Result<super::DWORD>;
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "windef"))]
impl IPicture2_Vtbl {
    pub const fn new<Identity: IPicture2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Handle<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut HHANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture2_Impl::get_Handle(this) {
                    Ok(ok__) => {
                        phandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_hPal<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phpal: *mut HHANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture2_Impl::get_hPal(this) {
                    Ok(ok__) => {
                        phpal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Type<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut super::SHORT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture2_Impl::get_Type(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Width<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut OLE_XSIZE_HIMETRIC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture2_Impl::get_Width(this) {
                    Ok(ok__) => {
                        pwidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Height<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheight: *mut OLE_YSIZE_HIMETRIC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture2_Impl::get_Height(this) {
                    Ok(ok__) => {
                        pheight.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Render<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::HDC, x: super::LONG, y: super::LONG, cx: super::LONG, cy: super::LONG, xsrc: OLE_XPOS_HIMETRIC, ysrc: OLE_YPOS_HIMETRIC, cxsrc: OLE_XSIZE_HIMETRIC, cysrc: OLE_YSIZE_HIMETRIC, prcwbounds: super::LPCRECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPicture2_Impl::Render(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy), core::mem::transmute_copy(&xsrc), core::mem::transmute_copy(&ysrc), core::mem::transmute_copy(&cxsrc), core::mem::transmute_copy(&cysrc), core::mem::transmute_copy(&prcwbounds)).into()
            }
        }
        unsafe extern "system" fn set_hPal<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpal: HHANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPicture2_Impl::set_hPal(this, core::mem::transmute_copy(&hpal)).into()
            }
        }
        unsafe extern "system" fn get_CurDC<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdc: *mut super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture2_Impl::get_CurDC(this) {
                    Ok(ok__) => {
                        phdc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectPicture<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdcin: super::HDC, phdcout: *mut super::HDC, phbmpout: *mut HHANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPicture2_Impl::SelectPicture(this, core::mem::transmute_copy(&hdcin), core::mem::transmute_copy(&phdcout), core::mem::transmute_copy(&phbmpout)).into()
            }
        }
        unsafe extern "system" fn get_KeepOriginalFormat<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeep: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture2_Impl::get_KeepOriginalFormat(this) {
                    Ok(ok__) => {
                        pkeep.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_KeepOriginalFormat<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keep: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPicture2_Impl::put_KeepOriginalFormat(this, core::mem::transmute_copy(&keep)).into()
            }
        }
        unsafe extern "system" fn PictureChanged<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPicture2_Impl::PictureChanged(this).into()
            }
        }
        unsafe extern "system" fn SaveAsFile<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, fsavememcopy: windows_core::BOOL, pcbsize: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture2_Impl::SaveAsFile(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&fsavememcopy)) {
                    Ok(ok__) => {
                        pcbsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Attributes<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattr: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPicture2_Impl::get_Attributes(this) {
                    Ok(ok__) => {
                        pdwattr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Handle: get_Handle::<Identity, OFFSET>,
            get_hPal: get_hPal::<Identity, OFFSET>,
            get_Type: get_Type::<Identity, OFFSET>,
            get_Width: get_Width::<Identity, OFFSET>,
            get_Height: get_Height::<Identity, OFFSET>,
            Render: Render::<Identity, OFFSET>,
            set_hPal: set_hPal::<Identity, OFFSET>,
            get_CurDC: get_CurDC::<Identity, OFFSET>,
            SelectPicture: SelectPicture::<Identity, OFFSET>,
            get_KeepOriginalFormat: get_KeepOriginalFormat::<Identity, OFFSET>,
            put_KeepOriginalFormat: put_KeepOriginalFormat::<Identity, OFFSET>,
            PictureChanged: PictureChanged::<Identity, OFFSET>,
            SaveAsFile: SaveAsFile::<Identity, OFFSET>,
            get_Attributes: get_Attributes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPicture2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IPicture2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IPictureDisp, IPictureDisp_Vtbl, 0x7bf80981_bf32_101a_8bbb_00aa00300cab);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IPictureDisp {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IPictureDisp, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPictureDisp_Vtbl {
    pub base__: super::IDispatch_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPictureDisp_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPictureDisp_Vtbl {
    pub const fn new<Identity: IPictureDisp_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPictureDisp as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPictureDisp {}
windows_core::imp::define_interface!(IPointerInactive, IPointerInactive_Vtbl, 0x55980ba0_35aa_11cf_b671_00aa004cd6d8);
windows_core::imp::interface_hierarchy!(IPointerInactive, windows_core::IUnknown);
impl IPointerInactive {
    #[cfg(feature = "shared")]
    pub unsafe fn GetActivationPolicy(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActivationPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn OnInactiveMouseMove(&self, prectbounds: super::LPCRECT, x: super::LONG, y: super::LONG, grfkeystate: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnInactiveMouseMove)(windows_core::Interface::as_raw(self), prectbounds, x, y, grfkeystate) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn OnInactiveSetCursor(&self, prectbounds: super::LPCRECT, x: super::LONG, y: super::LONG, dwmousemsg: super::DWORD, fsetalways: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnInactiveSetCursor)(windows_core::Interface::as_raw(self), prectbounds, x, y, dwmousemsg, fsetalways.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerInactive_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetActivationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetActivationPolicy: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub OnInactiveMouseMove: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCRECT, super::LONG, super::LONG, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    OnInactiveMouseMove: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub OnInactiveSetCursor: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCRECT, super::LONG, super::LONG, super::DWORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    OnInactiveSetCursor: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait IPointerInactive_Impl: windows_core::IUnknownImpl {
    fn GetActivationPolicy(&self) -> windows_core::Result<super::DWORD>;
    fn OnInactiveMouseMove(&self, prectbounds: super::LPCRECT, x: super::LONG, y: super::LONG, grfkeystate: super::DWORD) -> windows_core::Result<()>;
    fn OnInactiveSetCursor(&self, prectbounds: super::LPCRECT, x: super::LONG, y: super::LONG, dwmousemsg: super::DWORD, fsetalways: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl IPointerInactive_Vtbl {
    pub const fn new<Identity: IPointerInactive_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetActivationPolicy<Identity: IPointerInactive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpolicy: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerInactive_Impl::GetActivationPolicy(this) {
                    Ok(ok__) => {
                        pdwpolicy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnInactiveMouseMove<Identity: IPointerInactive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prectbounds: super::LPCRECT, x: super::LONG, y: super::LONG, grfkeystate: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPointerInactive_Impl::OnInactiveMouseMove(this, core::mem::transmute_copy(&prectbounds), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&grfkeystate)).into()
            }
        }
        unsafe extern "system" fn OnInactiveSetCursor<Identity: IPointerInactive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prectbounds: super::LPCRECT, x: super::LONG, y: super::LONG, dwmousemsg: super::DWORD, fsetalways: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPointerInactive_Impl::OnInactiveSetCursor(this, core::mem::transmute_copy(&prectbounds), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&dwmousemsg), core::mem::transmute_copy(&fsetalways)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetActivationPolicy: GetActivationPolicy::<Identity, OFFSET>,
            OnInactiveMouseMove: OnInactiveMouseMove::<Identity, OFFSET>,
            OnInactiveSetCursor: OnInactiveSetCursor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerInactive as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IPointerInactive {}
windows_core::imp::define_interface!(IPropertyBag2, IPropertyBag2_Vtbl, 0x22f55882_280b_11d0_a8a9_00a0c90c2004);
windows_core::imp::interface_hierarchy!(IPropertyBag2, windows_core::IUnknown);
impl IPropertyBag2 {
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Read<P2>(&self, cproperties: super::ULONG, ppropbag: *const PROPBAG2, perrlog: P2, pvarvalue: *mut super::VARIANT, phrerror: Option<*mut windows_core::HRESULT>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::IErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), cproperties, ppropbag, perrlog.param().abi(), pvarvalue, phrerror.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Write(&self, cproperties: super::ULONG, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), cproperties, ppropbag, pvarvalue) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CountProperties(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CountProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPropertyInfo(&self, iproperty: super::ULONG, cproperties: super::ULONG, ppropbag: *mut PROPBAG2, pcproperties: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyInfo)(windows_core::Interface::as_raw(self), iproperty, cproperties, ppropbag as _, pcproperties as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn LoadObject<P2, P3>(&self, pstrname: super::LPCOLESTR, dwhint: super::DWORD, punkobject: P2, perrlog: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<super::IErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadObject)(windows_core::Interface::as_raw(self), pstrname, dwhint, punkobject.param().abi(), perrlog.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyBag2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const PROPBAG2, *mut core::ffi::c_void, *mut super::VARIANT, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Read: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const PROPBAG2, *const super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Write: usize,
    #[cfg(feature = "shared")]
    pub CountProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CountProperties: usize,
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPropertyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, super::ULONG, *mut PROPBAG2, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared", feature = "wtypes", feature = "wtypesbase")))]
    GetPropertyInfo: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypesbase"))]
    pub LoadObject: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCOLESTR, super::DWORD, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "wtypesbase")))]
    LoadObject: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyBag2_Impl: windows_core::IUnknownImpl {
    fn Read(&self, cproperties: super::ULONG, ppropbag: *const PROPBAG2, perrlog: windows_core::Ref<super::IErrorLog>, pvarvalue: *mut super::VARIANT, phrerror: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn Write(&self, cproperties: super::ULONG, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> windows_core::Result<()>;
    fn CountProperties(&self) -> windows_core::Result<super::ULONG>;
    fn GetPropertyInfo(&self, iproperty: super::ULONG, cproperties: super::ULONG, ppropbag: *mut PROPBAG2, pcproperties: *mut super::ULONG) -> windows_core::Result<()>;
    fn LoadObject(&self, pstrname: super::LPCOLESTR, dwhint: super::DWORD, punkobject: windows_core::Ref<windows_core::IUnknown>, perrlog: windows_core::Ref<super::IErrorLog>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyBag2_Vtbl {
    pub const fn new<Identity: IPropertyBag2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: super::ULONG, ppropbag: *const PROPBAG2, perrlog: *mut core::ffi::c_void, pvarvalue: *mut super::VARIANT, phrerror: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag2_Impl::Read(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&perrlog), core::mem::transmute_copy(&pvarvalue), core::mem::transmute_copy(&phrerror)).into()
            }
        }
        unsafe extern "system" fn Write<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: super::ULONG, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag2_Impl::Write(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn CountProperties<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcproperties: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyBag2_Impl::CountProperties(this) {
                    Ok(ok__) => {
                        pcproperties.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iproperty: super::ULONG, cproperties: super::ULONG, ppropbag: *mut PROPBAG2, pcproperties: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag2_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&iproperty), core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&pcproperties)).into()
            }
        }
        unsafe extern "system" fn LoadObject<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrname: super::LPCOLESTR, dwhint: super::DWORD, punkobject: *mut core::ffi::c_void, perrlog: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag2_Impl::LoadObject(this, core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&dwhint), core::mem::transmute_copy(&punkobject), core::mem::transmute_copy(&perrlog)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Read: Read::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            CountProperties: CountProperties::<Identity, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, OFFSET>,
            LoadObject: LoadObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyBag2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyBag2 {}
windows_core::imp::define_interface!(IPropertyNotifySink, IPropertyNotifySink_Vtbl, 0x9bfbbc02_eff1_101a_84ed_00aa00341d07);
windows_core::imp::interface_hierarchy!(IPropertyNotifySink, windows_core::IUnknown);
impl IPropertyNotifySink {
    #[cfg(feature = "shared")]
    pub unsafe fn OnChanged(&self, dispid: super::DISPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnChanged)(windows_core::Interface::as_raw(self), dispid) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnRequestEdit(&self, dispid: super::DISPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnRequestEdit)(windows_core::Interface::as_raw(self), dispid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub OnChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnChanged: usize,
    #[cfg(feature = "shared")]
    pub OnRequestEdit: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnRequestEdit: usize,
}
#[cfg(feature = "shared")]
pub trait IPropertyNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnChanged(&self, dispid: super::DISPID) -> windows_core::Result<()>;
    fn OnRequestEdit(&self, dispid: super::DISPID) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IPropertyNotifySink_Vtbl {
    pub const fn new<Identity: IPropertyNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnChanged<Identity: IPropertyNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyNotifySink_Impl::OnChanged(this, core::mem::transmute_copy(&dispid)).into()
            }
        }
        unsafe extern "system" fn OnRequestEdit<Identity: IPropertyNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyNotifySink_Impl::OnRequestEdit(this, core::mem::transmute_copy(&dispid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnChanged: OnChanged::<Identity, OFFSET>,
            OnRequestEdit: OnRequestEdit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyNotifySink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IPropertyNotifySink {}
windows_core::imp::define_interface!(IPropertyPage, IPropertyPage_Vtbl, 0xb196b28d_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IPropertyPage, windows_core::IUnknown);
impl IPropertyPage {
    pub unsafe fn SetPageSite<P0>(&self, ppagesite: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IPropertyPageSite>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPageSite)(windows_core::Interface::as_raw(self), ppagesite.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn Activate(&self, hwndparent: super::HWND, prect: super::LPCRECT, bmodal: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), hwndparent, prect, bmodal.into()) }
    }
    pub unsafe fn Deactivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "windef", feature = "wtypesbase"))]
    pub unsafe fn GetPageInfo(&self, ppageinfo: *mut PROPPAGEINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPageInfo)(windows_core::Interface::as_raw(self), ppageinfo as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetObjects(&self, ppunk: &[Option<windows_core::IUnknown>]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetObjects)(windows_core::Interface::as_raw(self), ppunk.len().try_into().unwrap(), core::mem::transmute(ppunk.as_ptr())) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Show(&self, ncmdshow: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), ncmdshow) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn Move(&self, prect: super::LPCRECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), prect) }
    }
    pub unsafe fn IsPageDirty(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsPageDirty)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Apply(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn Help(&self, pszhelpdir: super::LPCOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Help)(windows_core::Interface::as_raw(self), pszhelpdir) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
    pub unsafe fn TranslateAcceleratorA(&self, pmsg: *const super::MSG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TranslateAcceleratorA)(windows_core::Interface::as_raw(self), pmsg) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPageSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::LPCRECT, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    Activate: usize,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef", feature = "wtypesbase"))]
    pub GetPageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPPAGEINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef", feature = "wtypesbase")))]
    GetPageInfo: usize,
    #[cfg(feature = "shared")]
    pub SetObjects: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetObjects: usize,
    #[cfg(feature = "shared")]
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Show: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCRECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    Move: usize,
    pub IsPageDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypesbase")]
    pub Help: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    Help: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
    pub TranslateAcceleratorA: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::MSG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser")))]
    TranslateAcceleratorA: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser", feature = "wtypesbase"))]
pub trait IPropertyPage_Impl: windows_core::IUnknownImpl {
    fn SetPageSite(&self, ppagesite: windows_core::Ref<IPropertyPageSite>) -> windows_core::Result<()>;
    fn Activate(&self, hwndparent: super::HWND, prect: super::LPCRECT, bmodal: windows_core::BOOL) -> windows_core::Result<()>;
    fn Deactivate(&self) -> windows_core::Result<()>;
    fn GetPageInfo(&self, ppageinfo: *mut PROPPAGEINFO) -> windows_core::Result<()>;
    fn SetObjects(&self, cobjects: super::ULONG, ppunk: *const Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Show(&self, ncmdshow: super::UINT) -> windows_core::Result<()>;
    fn Move(&self, prect: super::LPCRECT) -> windows_core::Result<()>;
    fn IsPageDirty(&self) -> windows_core::Result<()>;
    fn Apply(&self) -> windows_core::Result<()>;
    fn Help(&self, pszhelpdir: super::LPCOLESTR) -> windows_core::Result<()>;
    fn TranslateAcceleratorA(&self, pmsg: *const super::MSG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser", feature = "wtypesbase"))]
impl IPropertyPage_Vtbl {
    pub const fn new<Identity: IPropertyPage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPageSite<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppagesite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::SetPageSite(this, core::mem::transmute_copy(&ppagesite)).into()
            }
        }
        unsafe extern "system" fn Activate<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::HWND, prect: super::LPCRECT, bmodal: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::Activate(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&bmodal)).into()
            }
        }
        unsafe extern "system" fn Deactivate<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::Deactivate(this).into()
            }
        }
        unsafe extern "system" fn GetPageInfo<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppageinfo: *mut PROPPAGEINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::GetPageInfo(this, core::mem::transmute_copy(&ppageinfo)).into()
            }
        }
        unsafe extern "system" fn SetObjects<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjects: super::ULONG, ppunk: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::SetObjects(this, core::mem::transmute_copy(&cobjects), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn Show<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncmdshow: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::Show(this, core::mem::transmute_copy(&ncmdshow)).into()
            }
        }
        unsafe extern "system" fn Move<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: super::LPCRECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::Move(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn IsPageDirty<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::IsPageDirty(this).into()
            }
        }
        unsafe extern "system" fn Apply<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::Apply(this).into()
            }
        }
        unsafe extern "system" fn Help<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhelpdir: super::LPCOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::Help(this, core::mem::transmute_copy(&pszhelpdir)).into()
            }
        }
        unsafe extern "system" fn TranslateAcceleratorA<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::MSG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::TranslateAcceleratorA(this, core::mem::transmute_copy(&pmsg)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPageSite: SetPageSite::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            GetPageInfo: GetPageInfo::<Identity, OFFSET>,
            SetObjects: SetObjects::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            IsPageDirty: IsPageDirty::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
            Help: Help::<Identity, OFFSET>,
            TranslateAcceleratorA: TranslateAcceleratorA::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyPage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyPage {}
windows_core::imp::define_interface!(IPropertyPage2, IPropertyPage2_Vtbl, 0x01e44665_24ac_101b_84ed_08002b2ec713);
impl core::ops::Deref for IPropertyPage2 {
    type Target = IPropertyPage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyPage2, windows_core::IUnknown, IPropertyPage);
impl IPropertyPage2 {
    #[cfg(feature = "shared")]
    pub unsafe fn EditProperty(&self, dispid: super::DISPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EditProperty)(windows_core::Interface::as_raw(self), dispid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPage2_Vtbl {
    pub base__: IPropertyPage_Vtbl,
    #[cfg(feature = "shared")]
    pub EditProperty: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EditProperty: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser", feature = "wtypesbase"))]
pub trait IPropertyPage2_Impl: IPropertyPage_Impl {
    fn EditProperty(&self, dispid: super::DISPID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser", feature = "wtypesbase"))]
impl IPropertyPage2_Vtbl {
    pub const fn new<Identity: IPropertyPage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EditProperty<Identity: IPropertyPage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage2_Impl::EditProperty(this, core::mem::transmute_copy(&dispid)).into()
            }
        }
        Self { base__: IPropertyPage_Vtbl::new::<Identity, OFFSET>(), EditProperty: EditProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyPage2 as windows_core::Interface>::IID || iid == &<IPropertyPage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyPage2 {}
windows_core::imp::define_interface!(IPropertyPageSite, IPropertyPageSite_Vtbl, 0xb196b28c_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IPropertyPageSite, windows_core::IUnknown);
impl IPropertyPageSite {
    #[cfg(feature = "shared")]
    pub unsafe fn OnStatusChange(&self, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLocaleID(&self) -> windows_core::Result<super::LCID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocaleID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPageContainer(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPageContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
    pub unsafe fn TranslateAcceleratorA(&self, pmsg: *const super::MSG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TranslateAcceleratorA)(windows_core::Interface::as_raw(self), pmsg) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPageSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnStatusChange: usize,
    #[cfg(feature = "shared")]
    pub GetLocaleID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLocaleID: usize,
    pub GetPageContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
    pub TranslateAcceleratorA: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::MSG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser")))]
    TranslateAcceleratorA: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub trait IPropertyPageSite_Impl: windows_core::IUnknownImpl {
    fn OnStatusChange(&self, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn GetLocaleID(&self) -> windows_core::Result<super::LCID>;
    fn GetPageContainer(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn TranslateAcceleratorA(&self, pmsg: *const super::MSG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
impl IPropertyPageSite_Vtbl {
    pub const fn new<Identity: IPropertyPageSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStatusChange<Identity: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPageSite_Impl::OnStatusChange(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetLocaleID<Identity: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocaleid: *mut super::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyPageSite_Impl::GetLocaleID(this) {
                    Ok(ok__) => {
                        plocaleid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPageContainer<Identity: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyPageSite_Impl::GetPageContainer(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TranslateAcceleratorA<Identity: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::MSG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPageSite_Impl::TranslateAcceleratorA(this, core::mem::transmute_copy(&pmsg)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStatusChange: OnStatusChange::<Identity, OFFSET>,
            GetLocaleID: GetLocaleID::<Identity, OFFSET>,
            GetPageContainer: GetPageContainer::<Identity, OFFSET>,
            TranslateAcceleratorA: TranslateAcceleratorA::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyPageSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
impl windows_core::RuntimeName for IPropertyPageSite {}
windows_core::imp::define_interface!(IProvideClassInfo, IProvideClassInfo_Vtbl, 0xb196b283_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IProvideClassInfo, windows_core::IUnknown);
impl IProvideClassInfo {
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetClassInfoA(&self) -> windows_core::Result<super::ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClassInfoA)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideClassInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub GetClassInfoA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetClassInfoA: usize,
}
#[cfg(feature = "oaidl")]
pub trait IProvideClassInfo_Impl: windows_core::IUnknownImpl {
    fn GetClassInfoA(&self) -> windows_core::Result<super::ITypeInfo>;
}
#[cfg(feature = "oaidl")]
impl IProvideClassInfo_Vtbl {
    pub const fn new<Identity: IProvideClassInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassInfoA<Identity: IProvideClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppti: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideClassInfo_Impl::GetClassInfoA(this) {
                    Ok(ok__) => {
                        ppti.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClassInfoA: GetClassInfoA::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideClassInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IProvideClassInfo {}
windows_core::imp::define_interface!(IProvideClassInfo2, IProvideClassInfo2_Vtbl, 0xa6bc3ac0_dbaa_11ce_9de3_00aa004bb851);
impl core::ops::Deref for IProvideClassInfo2 {
    type Target = IProvideClassInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProvideClassInfo2, windows_core::IUnknown, IProvideClassInfo);
impl IProvideClassInfo2 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetGUID(&self, dwguidkind: super::DWORD) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGUID)(windows_core::Interface::as_raw(self), dwguidkind, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideClassInfo2_Vtbl {
    pub base__: IProvideClassInfo_Vtbl,
    #[cfg(feature = "shared")]
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetGUID: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
pub trait IProvideClassInfo2_Impl: IProvideClassInfo_Impl {
    fn GetGUID(&self, dwguidkind: super::DWORD) -> windows_core::Result<windows_core::GUID>;
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
impl IProvideClassInfo2_Vtbl {
    pub const fn new<Identity: IProvideClassInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGUID<Identity: IProvideClassInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwguidkind: super::DWORD, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideClassInfo2_Impl::GetGUID(this, core::mem::transmute_copy(&dwguidkind)) {
                    Ok(ok__) => {
                        pguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IProvideClassInfo_Vtbl::new::<Identity, OFFSET>(), GetGUID: GetGUID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideClassInfo2 as windows_core::Interface>::IID || iid == &<IProvideClassInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
impl windows_core::RuntimeName for IProvideClassInfo2 {}
windows_core::imp::define_interface!(IProvideMultipleClassInfo, IProvideMultipleClassInfo_Vtbl, 0xa7aba9c1_8983_11cf_8f20_00805f2cd064);
impl core::ops::Deref for IProvideMultipleClassInfo {
    type Target = IProvideClassInfo2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProvideMultipleClassInfo, windows_core::IUnknown, IProvideClassInfo, IProvideClassInfo2);
impl IProvideMultipleClassInfo {
    #[cfg(feature = "shared")]
    pub unsafe fn GetMultiTypeInfoCount(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMultiTypeInfoCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "guiddef", feature = "oaidl", feature = "shared"))]
    pub unsafe fn GetInfoOfIndex(&self, iti: super::ULONG, dwflags: super::DWORD, ppticoclass: *mut Option<super::ITypeInfo>, pdwtiflags: *mut super::DWORD, pcdispidreserved: *mut super::ULONG, piidprimary: *mut super::IID, piidsource: *mut super::IID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInfoOfIndex)(windows_core::Interface::as_raw(self), iti, dwflags, core::mem::transmute(ppticoclass), pdwtiflags as _, pcdispidreserved as _, piidprimary as _, piidsource as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideMultipleClassInfo_Vtbl {
    pub base__: IProvideClassInfo2_Vtbl,
    #[cfg(feature = "shared")]
    pub GetMultiTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMultiTypeInfoCount: usize,
    #[cfg(all(feature = "guiddef", feature = "oaidl", feature = "shared"))]
    pub GetInfoOfIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, super::DWORD, *mut *mut core::ffi::c_void, *mut super::DWORD, *mut super::ULONG, *mut super::IID, *mut super::IID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "oaidl", feature = "shared")))]
    GetInfoOfIndex: usize,
}
#[cfg(all(feature = "guiddef", feature = "oaidl", feature = "shared"))]
pub trait IProvideMultipleClassInfo_Impl: IProvideClassInfo2_Impl {
    fn GetMultiTypeInfoCount(&self) -> windows_core::Result<super::ULONG>;
    fn GetInfoOfIndex(&self, iti: super::ULONG, dwflags: super::DWORD, ppticoclass: windows_core::OutRef<super::ITypeInfo>, pdwtiflags: *mut super::DWORD, pcdispidreserved: *mut super::ULONG, piidprimary: *mut super::IID, piidsource: *mut super::IID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "oaidl", feature = "shared"))]
impl IProvideMultipleClassInfo_Vtbl {
    pub const fn new<Identity: IProvideMultipleClassInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMultiTypeInfoCount<Identity: IProvideMultipleClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcti: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideMultipleClassInfo_Impl::GetMultiTypeInfoCount(this) {
                    Ok(ok__) => {
                        pcti.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInfoOfIndex<Identity: IProvideMultipleClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iti: super::ULONG, dwflags: super::DWORD, ppticoclass: *mut *mut core::ffi::c_void, pdwtiflags: *mut super::DWORD, pcdispidreserved: *mut super::ULONG, piidprimary: *mut super::IID, piidsource: *mut super::IID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProvideMultipleClassInfo_Impl::GetInfoOfIndex(this, core::mem::transmute_copy(&iti), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppticoclass), core::mem::transmute_copy(&pdwtiflags), core::mem::transmute_copy(&pcdispidreserved), core::mem::transmute_copy(&piidprimary), core::mem::transmute_copy(&piidsource)).into()
            }
        }
        Self {
            base__: IProvideClassInfo2_Vtbl::new::<Identity, OFFSET>(),
            GetMultiTypeInfoCount: GetMultiTypeInfoCount::<Identity, OFFSET>,
            GetInfoOfIndex: GetInfoOfIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideMultipleClassInfo as windows_core::Interface>::IID || iid == &<IProvideClassInfo as windows_core::Interface>::IID || iid == &<IProvideClassInfo2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "oaidl", feature = "shared"))]
impl windows_core::RuntimeName for IProvideMultipleClassInfo {}
windows_core::imp::define_interface!(IQuickActivate, IQuickActivate_Vtbl, 0xcf51ed10_62fe_11cf_bf86_00a0c9034836);
windows_core::imp::interface_hierarchy!(IQuickActivate, windows_core::IUnknown);
impl IQuickActivate {
    #[cfg(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "shared", feature = "urlmon", feature = "windef"))]
    pub unsafe fn QuickActivate(&self, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QuickActivate)(windows_core::Interface::as_raw(self), pqacontainer, pqacontrol as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn SetContentExtent(&self, psizel: super::LPSIZEL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContentExtent)(windows_core::Interface::as_raw(self), psizel) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetContentExtent(&self) -> windows_core::Result<super::SIZE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContentExtent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuickActivate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "shared", feature = "urlmon", feature = "windef"))]
    pub QuickActivate: unsafe extern "system" fn(*mut core::ffi::c_void, *const QACONTAINER, *mut QACONTROL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "shared", feature = "urlmon", feature = "windef")))]
    QuickActivate: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub SetContentExtent: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPSIZEL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    SetContentExtent: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetContentExtent: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPSIZEL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetContentExtent: usize,
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "shared", feature = "urlmon", feature = "windef"))]
pub trait IQuickActivate_Impl: windows_core::IUnknownImpl {
    fn QuickActivate(&self, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> windows_core::Result<()>;
    fn SetContentExtent(&self, psizel: super::LPSIZEL) -> windows_core::Result<()>;
    fn GetContentExtent(&self) -> windows_core::Result<super::SIZE>;
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "shared", feature = "urlmon", feature = "windef"))]
impl IQuickActivate_Vtbl {
    pub const fn new<Identity: IQuickActivate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QuickActivate<Identity: IQuickActivate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQuickActivate_Impl::QuickActivate(this, core::mem::transmute_copy(&pqacontainer), core::mem::transmute_copy(&pqacontrol)).into()
            }
        }
        unsafe extern "system" fn SetContentExtent<Identity: IQuickActivate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psizel: super::LPSIZEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQuickActivate_Impl::SetContentExtent(this, core::mem::transmute_copy(&psizel)).into()
            }
        }
        unsafe extern "system" fn GetContentExtent<Identity: IQuickActivate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psizel: super::LPSIZEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQuickActivate_Impl::GetContentExtent(this) {
                    Ok(ok__) => {
                        psizel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QuickActivate: QuickActivate::<Identity, OFFSET>,
            SetContentExtent: SetContentExtent::<Identity, OFFSET>,
            GetContentExtent: GetContentExtent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQuickActivate as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "shared", feature = "urlmon", feature = "windef"))]
impl windows_core::RuntimeName for IQuickActivate {}
windows_core::imp::define_interface!(ISimpleFrameSite, ISimpleFrameSite_Vtbl, 0x742b0e01_14e6_101b_914e_00aa00300cab);
windows_core::imp::interface_hierarchy!(ISimpleFrameSite, windows_core::IUnknown);
impl ISimpleFrameSite {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
    pub unsafe fn PreMessageFilter(&self, hwnd: super::HWND, msg: super::UINT, wp: super::WPARAM, lp: super::LPARAM, plresult: *mut super::LRESULT, pdwcookie: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PreMessageFilter)(windows_core::Interface::as_raw(self), hwnd, msg, wp, lp, plresult as _, pdwcookie as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
    pub unsafe fn PostMessageFilter(&self, hwnd: super::HWND, msg: super::UINT, wp: super::WPARAM, lp: super::LPARAM, plresult: *mut super::LRESULT, dwcookie: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PostMessageFilter)(windows_core::Interface::as_raw(self), hwnd, msg, wp, lp, plresult as _, dwcookie) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleFrameSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
    pub PreMessageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::UINT, super::WPARAM, super::LPARAM, *mut super::LRESULT, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "windef")))]
    PreMessageFilter: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
    pub PostMessageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::UINT, super::WPARAM, super::LPARAM, *mut super::LRESULT, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "windef")))]
    PostMessageFilter: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub trait ISimpleFrameSite_Impl: windows_core::IUnknownImpl {
    fn PreMessageFilter(&self, hwnd: super::HWND, msg: super::UINT, wp: super::WPARAM, lp: super::LPARAM, plresult: *mut super::LRESULT, pdwcookie: *mut super::DWORD) -> windows_core::Result<()>;
    fn PostMessageFilter(&self, hwnd: super::HWND, msg: super::UINT, wp: super::WPARAM, lp: super::LPARAM, plresult: *mut super::LRESULT, dwcookie: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
impl ISimpleFrameSite_Vtbl {
    pub const fn new<Identity: ISimpleFrameSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PreMessageFilter<Identity: ISimpleFrameSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::HWND, msg: super::UINT, wp: super::WPARAM, lp: super::LPARAM, plresult: *mut super::LRESULT, pdwcookie: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimpleFrameSite_Impl::PreMessageFilter(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wp), core::mem::transmute_copy(&lp), core::mem::transmute_copy(&plresult), core::mem::transmute_copy(&pdwcookie)).into()
            }
        }
        unsafe extern "system" fn PostMessageFilter<Identity: ISimpleFrameSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::HWND, msg: super::UINT, wp: super::WPARAM, lp: super::LPARAM, plresult: *mut super::LRESULT, dwcookie: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimpleFrameSite_Impl::PostMessageFilter(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wp), core::mem::transmute_copy(&lp), core::mem::transmute_copy(&plresult), core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreMessageFilter: PreMessageFilter::<Identity, OFFSET>,
            PostMessageFilter: PostMessageFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimpleFrameSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for ISimpleFrameSite {}
windows_core::imp::define_interface!(ISpecifyPropertyPages, ISpecifyPropertyPages_Vtbl, 0xb196b28b_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(ISpecifyPropertyPages, windows_core::IUnknown);
impl ISpecifyPropertyPages {
    #[cfg(feature = "shared")]
    pub unsafe fn GetPages(&self) -> windows_core::Result<CAUUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpecifyPropertyPages_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CAUUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPages: usize,
}
#[cfg(feature = "shared")]
pub trait ISpecifyPropertyPages_Impl: windows_core::IUnknownImpl {
    fn GetPages(&self) -> windows_core::Result<CAUUID>;
}
#[cfg(feature = "shared")]
impl ISpecifyPropertyPages_Vtbl {
    pub const fn new<Identity: ISpecifyPropertyPages_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPages<Identity: ISpecifyPropertyPages_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppages: *mut CAUUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpecifyPropertyPages_Impl::GetPages(this) {
                    Ok(ok__) => {
                        ppages.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPages: GetPages::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpecifyPropertyPages as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ISpecifyPropertyPages {}
#[cfg(feature = "oleidl")]
windows_core::imp::define_interface!(IViewObjectEx, IViewObjectEx_Vtbl, 0x3af24292_0c96_11ce_a0cf_00aa00600ab8);
#[cfg(feature = "oleidl")]
impl core::ops::Deref for IViewObjectEx {
    type Target = super::IViewObject2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oleidl")]
windows_core::imp::interface_hierarchy!(IViewObjectEx, windows_core::IUnknown, super::IViewObject, super::IViewObject2);
#[cfg(feature = "oleidl")]
impl IViewObjectEx {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetRect(&self, dwaspect: super::DWORD) -> windows_core::Result<super::RECTL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRect)(windows_core::Interface::as_raw(self), dwaspect, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetViewStatus(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn QueryHitPoint(&self, dwaspect: super::DWORD, prectbounds: super::LPCRECT, ptlloc: super::POINT, lclosehint: super::LONG) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryHitPoint)(windows_core::Interface::as_raw(self), dwaspect, prectbounds, ptlloc, lclosehint, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn QueryHitRect(&self, dwaspect: super::DWORD, prectbounds: super::LPCRECT, prectloc: super::LPCRECT, lclosehint: super::LONG) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryHitRect)(windows_core::Interface::as_raw(self), dwaspect, prectbounds, prectloc, lclosehint, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "objidl", feature = "shared", feature = "windef"))]
    pub unsafe fn GetNaturalExtent(&self, dwaspect: super::DWORD, lindex: super::LONG, ptd: *const super::DVTARGETDEVICE, hictargetdev: super::HDC, pextentinfo: *const DVEXTENTINFO) -> windows_core::Result<super::SIZE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNaturalExtent)(windows_core::Interface::as_raw(self), dwaspect, lindex, ptd, hictargetdev, pextentinfo, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oleidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IViewObjectEx_Vtbl {
    pub base__: super::IViewObject2_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LPRECTL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetRect: usize,
    #[cfg(feature = "shared")]
    pub GetViewStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetViewStatus: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub QueryHitPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LPCRECT, super::POINT, super::LONG, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    QueryHitPoint: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub QueryHitRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LPCRECT, super::LPCRECT, super::LONG, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    QueryHitRect: usize,
    #[cfg(all(feature = "objidl", feature = "shared", feature = "windef"))]
    pub GetNaturalExtent: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LONG, *const super::DVTARGETDEVICE, super::HDC, *const DVEXTENTINFO, super::LPSIZEL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared", feature = "windef")))]
    GetNaturalExtent: usize,
}
#[cfg(all(feature = "httpext", feature = "objidl", feature = "oleidl", feature = "shared", feature = "windef", feature = "wingdi"))]
pub trait IViewObjectEx_Impl: super::IViewObject2_Impl {
    fn GetRect(&self, dwaspect: super::DWORD) -> windows_core::Result<super::RECTL>;
    fn GetViewStatus(&self) -> windows_core::Result<super::DWORD>;
    fn QueryHitPoint(&self, dwaspect: super::DWORD, prectbounds: super::LPCRECT, ptlloc: &super::POINT, lclosehint: super::LONG) -> windows_core::Result<super::DWORD>;
    fn QueryHitRect(&self, dwaspect: super::DWORD, prectbounds: super::LPCRECT, prectloc: super::LPCRECT, lclosehint: super::LONG) -> windows_core::Result<super::DWORD>;
    fn GetNaturalExtent(&self, dwaspect: super::DWORD, lindex: super::LONG, ptd: *const super::DVTARGETDEVICE, hictargetdev: super::HDC, pextentinfo: *const DVEXTENTINFO) -> windows_core::Result<super::SIZE>;
}
#[cfg(all(feature = "httpext", feature = "objidl", feature = "oleidl", feature = "shared", feature = "windef", feature = "wingdi"))]
impl IViewObjectEx_Vtbl {
    pub const fn new<Identity: IViewObjectEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRect<Identity: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: super::DWORD, prect: super::LPRECTL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewObjectEx_Impl::GetRect(this, core::mem::transmute_copy(&dwaspect)) {
                    Ok(ok__) => {
                        prect.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetViewStatus<Identity: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewObjectEx_Impl::GetViewStatus(this) {
                    Ok(ok__) => {
                        pdwstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryHitPoint<Identity: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: super::DWORD, prectbounds: super::LPCRECT, ptlloc: super::POINT, lclosehint: super::LONG, phitresult: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewObjectEx_Impl::QueryHitPoint(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&prectbounds), core::mem::transmute(&ptlloc), core::mem::transmute_copy(&lclosehint)) {
                    Ok(ok__) => {
                        phitresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryHitRect<Identity: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: super::DWORD, prectbounds: super::LPCRECT, prectloc: super::LPCRECT, lclosehint: super::LONG, phitresult: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewObjectEx_Impl::QueryHitRect(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&prectbounds), core::mem::transmute_copy(&prectloc), core::mem::transmute_copy(&lclosehint)) {
                    Ok(ok__) => {
                        phitresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNaturalExtent<Identity: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: super::DWORD, lindex: super::LONG, ptd: *const super::DVTARGETDEVICE, hictargetdev: super::HDC, pextentinfo: *const DVEXTENTINFO, psizel: super::LPSIZEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewObjectEx_Impl::GetNaturalExtent(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&pextentinfo)) {
                    Ok(ok__) => {
                        psizel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IViewObject2_Vtbl::new::<Identity, OFFSET>(),
            GetRect: GetRect::<Identity, OFFSET>,
            GetViewStatus: GetViewStatus::<Identity, OFFSET>,
            QueryHitPoint: QueryHitPoint::<Identity, OFFSET>,
            QueryHitRect: QueryHitRect::<Identity, OFFSET>,
            GetNaturalExtent: GetNaturalExtent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObjectEx as windows_core::Interface>::IID || iid == &<super::IViewObject as windows_core::Interface>::IID || iid == &<super::IViewObject2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "httpext", feature = "objidl", feature = "oleidl", feature = "shared", feature = "windef", feature = "wingdi"))]
impl windows_core::RuntimeName for IViewObjectEx {}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LICINFO {
    pub cbLicInfo: super::LONG,
    pub fRuntimeKeyAvail: windows_core::BOOL,
    pub fLicVerified: windows_core::BOOL,
}
#[cfg(feature = "objidl")]
pub type LPADVISESINKEX = IAdviseSinkEx;
#[cfg(feature = "shared")]
pub type LPCADWORD = *mut CADWORD;
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub type LPCALPOLESTR = *mut CALPOLESTR;
#[cfg(feature = "shared")]
pub type LPCAUUID = *mut CAUUID;
#[cfg(feature = "unknwnbase")]
pub type LPCLASSFACTORY2 = IClassFactory2;
#[cfg(feature = "shared")]
pub type LPCONNECTDATA = *mut CONNECTDATA;
pub type LPCONNECTIONPOINT = IConnectionPoint;
pub type LPCONNECTIONPOINTCONTAINER = IConnectionPointContainer;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPCONTROLINFO = *mut CONTROLINFO;
pub type LPENUMCONNECTIONPOINTS = IEnumConnectionPoints;
pub type LPENUMCONNECTIONS = IEnumConnections;
pub type LPENUMOLEUNDOUNITS = IEnumOleUndoUnits;
pub type LPFONT = IFont;
#[cfg(feature = "oaidl")]
pub type LPFONTDISP = IFontDisp;
#[cfg(feature = "oaidl")]
pub type LPFONTEVENTS = IFontEventsDisp;
#[cfg(feature = "shared")]
pub type LPLICINFO = *mut LICINFO;
pub type LPOBJECTWITHSITE = IObjectWithSite;
pub type LPOLECONTROL = IOleControl;
pub type LPOLECONTROLSITE = IOleControlSite;
#[cfg(feature = "oleidl")]
pub type LPOLEINPLACEOBJECTWINDOWLESS = IOleInPlaceObjectWindowless;
#[cfg(feature = "oleidl")]
pub type LPOLEINPLACESITEEX = IOleInPlaceSiteEx;
#[cfg(feature = "oleidl")]
pub type LPOLEINPLACESITEWINDOWLESS = IOleInPlaceSiteWindowless;
pub type LPOLEPARENTUNDOUNIT = IOleParentUndoUnit;
pub type LPOLEUNDOMANAGER = IOleUndoManager;
pub type LPOLEUNDOUNIT = IOleUndoUnit;
pub type LPPERPROPERTYBROWSING = IPerPropertyBrowsing;
#[cfg(feature = "objidl")]
pub type LPPERSISTMEMORY = IPersistMemory;
#[cfg(feature = "objidl")]
pub type LPPERSISTPROPERTYBAG = IPersistPropertyBag;
#[cfg(feature = "objidl")]
pub type LPPERSISTPROPERTYBAG2 = IPersistPropertyBag2;
#[cfg(feature = "objidl")]
pub type LPPERSISTSTREAMINIT = IPersistStreamInit;
pub type LPPICTURE = IPicture;
pub type LPPICTURE2 = IPicture2;
#[cfg(feature = "oaidl")]
pub type LPPICTUREDISP = IPictureDisp;
pub type LPPOINTERINACTIVE = IPointerInactive;
#[cfg(feature = "minwindef")]
pub type LPPOINTF = *mut POINTF;
pub type LPPROPERTYBAG2 = IPropertyBag2;
pub type LPPROPERTYNOTIFYSINK = IPropertyNotifySink;
pub type LPPROPERTYPAGE = IPropertyPage;
pub type LPPROPERTYPAGE2 = IPropertyPage2;
pub type LPPROPERTYPAGESITE = IPropertyPageSite;
#[cfg(all(feature = "shared", feature = "windef", feature = "wtypesbase"))]
pub type LPPROPPAGEINFO = *mut PROPPAGEINFO;
pub type LPPROVIDECLASSINFO = IProvideClassInfo;
pub type LPPROVIDECLASSINFO2 = IProvideClassInfo2;
pub type LPPROVIDEMULTIPLECLASSINFO = IProvideMultipleClassInfo;
pub type LPQUICKACTIVATE = IQuickActivate;
pub type LPSIMPLEFRAMESITE = ISimpleFrameSite;
pub type LPSPECIFYPROPERTYPAGES = ISpecifyPropertyPages;
#[cfg(all(feature = "shared", feature = "wingdi", feature = "winnt"))]
pub type LPTEXTMETRICOLE = *mut super::TEXTMETRICW;
#[cfg(feature = "oleidl")]
pub type LPVIEWOBJECTEX = IViewObjectEx;
pub const MULTICLASSINFO_GETIIDPRIMARY: i32 = 4;
pub const MULTICLASSINFO_GETIIDSOURCE: i32 = 8;
pub const MULTICLASSINFO_GETNUMRESERVEDDISPIDS: i32 = 2;
pub const MULTICLASSINFO_GETTYPEINFO: i32 = 1;
pub type OLEDCFLAGS = i32;
pub const OLEDC_NODRAW: OLEDCFLAGS = 1;
pub const OLEDC_OFFSCREEN: OLEDCFLAGS = 4;
pub const OLEDC_PAINTBKGND: OLEDCFLAGS = 2;
pub type OLE_COLOR = u32;
pub type OLE_HANDLE = u32;
pub type OLE_XPOS_HIMETRIC = i32;
pub type OLE_XSIZE_HIMETRIC = i32;
pub type OLE_YPOS_HIMETRIC = i32;
pub type OLE_YSIZE_HIMETRIC = i32;
#[cfg(feature = "shared")]
pub type PCONNECTDATA = *mut CONNECTDATA;
pub type PCONNECTIONPOINT = IConnectionPoint;
pub type PCONNECTIONPOINTCONTAINER = IConnectionPointContainer;
pub type PENUMCONNECTIONPOINTS = IEnumConnectionPoints;
pub type PENUMCONNECTIONS = IEnumConnections;
pub type PICTUREATTRIBUTES = i32;
pub const PICTURE_SCALABLE: PICTUREATTRIBUTES = 1;
pub const PICTURE_TRANSPARENT: PICTUREATTRIBUTES = 2;
pub type POINTERINACTIVE = i32;
pub const POINTERINACTIVE_ACTIVATEONDRAG: POINTERINACTIVE = 4;
pub const POINTERINACTIVE_ACTIVATEONENTRY: POINTERINACTIVE = 1;
pub const POINTERINACTIVE_DEACTIVATEONLEAVE: POINTERINACTIVE = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINTF {
    pub x: super::FLOAT,
    pub y: super::FLOAT,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPBAG2 {
    pub dwType: super::DWORD,
    pub vt: super::VARTYPE,
    pub cfType: super::CLIPFORMAT,
    pub dwHint: super::DWORD,
    pub pstrName: super::LPOLESTR,
    pub clsid: super::CLSID,
}
pub type PROPBAG2_TYPE = i32;
pub const PROPBAG2_TYPE_DATA: PROPBAG2_TYPE = 1;
pub const PROPBAG2_TYPE_MONIKER: PROPBAG2_TYPE = 6;
pub const PROPBAG2_TYPE_OBJECT: PROPBAG2_TYPE = 3;
pub const PROPBAG2_TYPE_STORAGE: PROPBAG2_TYPE = 5;
pub const PROPBAG2_TYPE_STREAM: PROPBAG2_TYPE = 4;
pub const PROPBAG2_TYPE_UNDEFINED: PROPBAG2_TYPE = 0;
pub const PROPBAG2_TYPE_URL: PROPBAG2_TYPE = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPPAGEINFO {
    pub cb: super::ULONG,
    pub pszTitle: super::LPOLESTR,
    pub size: super::SIZE,
    pub pszDocString: super::LPOLESTR,
    pub pszHelpFile: super::LPOLESTR,
    pub dwHelpContext: super::DWORD,
}
pub type PROPPAGESTATUS = i32;
pub const PROPPAGESTATUS_CLEAN: PROPPAGESTATUS = 4;
pub const PROPPAGESTATUS_DIRTY: PROPPAGESTATUS = 1;
pub const PROPPAGESTATUS_VALIDATE: PROPPAGESTATUS = 2;
#[repr(C)]
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "shared", feature = "urlmon", feature = "windef"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QACONTAINER {
    pub cbSize: super::ULONG,
    pub pClientSite: core::mem::ManuallyDrop<Option<super::IOleClientSite>>,
    pub pAdviseSink: core::mem::ManuallyDrop<Option<IAdviseSinkEx>>,
    pub pPropertyNotifySink: core::mem::ManuallyDrop<Option<IPropertyNotifySink>>,
    pub pUnkEventSink: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwAmbientFlags: super::DWORD,
    pub colorFore: OLE_COLOR,
    pub colorBack: OLE_COLOR,
    pub pFont: core::mem::ManuallyDrop<Option<IFont>>,
    pub pUndoMgr: core::mem::ManuallyDrop<Option<IOleUndoManager>>,
    pub dwAppearance: super::DWORD,
    pub lcid: super::LONG,
    pub hpal: super::HPALETTE,
    pub pBindHost: core::mem::ManuallyDrop<Option<super::IBindHost>>,
    pub pOleControlSite: core::mem::ManuallyDrop<Option<IOleControlSite>>,
    pub pServiceProvider: core::mem::ManuallyDrop<Option<super::IServiceProvider>>,
}
pub type QACONTAINERFLAGS = i32;
pub const QACONTAINER_AUTOCLIP: QACONTAINERFLAGS = 32;
pub const QACONTAINER_DISPLAYASDEFAULT: QACONTAINERFLAGS = 8;
pub const QACONTAINER_MESSAGEREFLECT: QACONTAINERFLAGS = 64;
pub const QACONTAINER_SHOWGRABHANDLES: QACONTAINERFLAGS = 2;
pub const QACONTAINER_SHOWHATCHING: QACONTAINERFLAGS = 1;
pub const QACONTAINER_SUPPORTSMNEMONICS: QACONTAINERFLAGS = 128;
pub const QACONTAINER_UIDEAD: QACONTAINERFLAGS = 16;
pub const QACONTAINER_USERMODE: QACONTAINERFLAGS = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QACONTROL {
    pub cbSize: super::ULONG,
    pub dwMiscStatus: super::DWORD,
    pub dwViewStatus: super::DWORD,
    pub dwEventCookie: super::DWORD,
    pub dwPropNotifyCookie: super::DWORD,
    pub dwPointerActivationPolicy: super::DWORD,
}
pub type READYSTATE = i32;
pub const READYSTATE_COMPLETE: READYSTATE = 4;
pub const READYSTATE_INTERACTIVE: READYSTATE = 3;
pub const READYSTATE_LOADED: READYSTATE = 2;
pub const READYSTATE_LOADING: READYSTATE = 1;
pub const READYSTATE_UNINITIALIZED: READYSTATE = 0;
#[cfg(all(feature = "shared", feature = "wingdi", feature = "winnt"))]
pub type TEXTMETRICOLE = super::TEXTMETRICW;
pub const TIFLAGS_EXTENDDISPATCHONLY: i32 = 1;
pub type UASFLAGS = i32;
pub const UAS_BLOCKED: UASFLAGS = 1;
pub const UAS_MASK: UASFLAGS = 3;
pub const UAS_NOPARENTENABLE: UASFLAGS = 2;
pub const UAS_NORMAL: UASFLAGS = 0;
pub type VIEWSTATUS = i32;
pub const VIEWSTATUS_3DSURFACE: VIEWSTATUS = 32;
pub const VIEWSTATUS_DVASPECTOPAQUE: VIEWSTATUS = 4;
pub const VIEWSTATUS_DVASPECTTRANSPARENT: VIEWSTATUS = 8;
pub const VIEWSTATUS_OPAQUE: VIEWSTATUS = 1;
pub const VIEWSTATUS_SOLIDBKGND: VIEWSTATUS = 2;
pub const VIEWSTATUS_SURFACE: VIEWSTATUS = 16;
pub type XFORMCOORDS = i32;
pub const XFORMCOORDS_CONTAINERTOHIMETRIC: XFORMCOORDS = 8;
pub const XFORMCOORDS_EVENTCOMPAT: XFORMCOORDS = 16;
pub const XFORMCOORDS_HIMETRICTOCONTAINER: XFORMCOORDS = 4;
pub const XFORMCOORDS_POSITION: XFORMCOORDS = 1;
pub const XFORMCOORDS_SIZE: XFORMCOORDS = 2;
