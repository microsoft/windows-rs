pub type ACTIVATEFLAGS = i32;
pub const ACTIVATE_WINDOWLESS: ACTIVATEFLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CADWORD {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl Default for CADWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CALPOLESTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::PWSTR,
}
impl Default for CALPOLESTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CAUUID {
    pub cElems: u32,
    pub pElems: *mut windows_core::GUID,
}
impl Default for CAUUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CONNECTDATA {
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwCookie: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONTROLINFO {
    pub cb: u32,
    pub hAccel: super::windef::HACCEL,
    pub cAccel: u16,
    pub dwFlags: u32,
}
pub type CTRLINFO = i32;
pub const CTRLINFO_EATS_ESCAPE: CTRLINFO = 2;
pub const CTRLINFO_EATS_RETURN: CTRLINFO = 1;
pub type DVASPECT2 = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DVASPECTINFO {
    pub cb: u32,
    pub dwFlags: u32,
}
pub type DVASPECTINFOFLAG = i32;
pub const DVASPECTINFOFLAG_CANOPTIMIZE: DVASPECTINFOFLAG = 1;
pub const DVASPECT_OPAQUE: DVASPECT2 = 16;
pub const DVASPECT_TRANSPARENT: DVASPECT2 = 32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DVEXTENTINFO {
    pub cb: u32,
    pub dwExtentMode: u32,
    pub sizelProposed: super::windef::SIZEL,
}
pub type DVEXTENTMODE = i32;
pub const DVEXTENT_CONTENT: DVEXTENTMODE = 0;
pub const DVEXTENT_INTEGRAL: DVEXTENTMODE = 1;
pub type GUIDKIND = i32;
pub const GUIDKIND_DEFAULT_SOURCE_DISP_IID: GUIDKIND = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HHANDLE(pub usize);
pub type HITRESULT = i32;
pub const HITRESULT_CLOSE: HITRESULT = 2;
pub const HITRESULT_HIT: HITRESULT = 3;
pub const HITRESULT_OUTSIDE: HITRESULT = 0;
pub const HITRESULT_TRANSPARENT: HITRESULT = 1;
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(IAdviseSinkEx, IAdviseSinkEx_Vtbl, 0x3af24290_0c96_11ce_a0cf_00aa00600ab8);
#[cfg(feature = "objidl")]
impl core::ops::Deref for IAdviseSinkEx {
    type Target = super::objidl::IAdviseSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(IAdviseSinkEx, windows_core::IUnknown, super::objidl::IAdviseSink);
#[cfg(feature = "objidl")]
impl IAdviseSinkEx {
    pub unsafe fn OnViewStatusChange(&self, dwviewstatus: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).OnViewStatusChange)(windows_core::Interface::as_raw(self), dwviewstatus);
        }
    }
}
#[cfg(feature = "objidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSinkEx_Vtbl {
    pub base__: super::objidl::IAdviseSink_Vtbl,
    pub OnViewStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait IAdviseSinkEx_Impl: super::objidl::IAdviseSink_Impl {
    fn OnViewStatusChange(&self, dwviewstatus: u32);
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl IAdviseSinkEx_Vtbl {
    pub const fn new<Identity: IAdviseSinkEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnViewStatusChange<Identity: IAdviseSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwviewstatus: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSinkEx_Impl::OnViewStatusChange(this, core::mem::transmute_copy(&dwviewstatus));
            }
        }
        Self { base__: super::objidl::IAdviseSink_Vtbl::new::<Identity, OFFSET>(), OnViewStatusChange: OnViewStatusChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdviseSinkEx as windows_core::Interface>::IID || iid == &<super::objidl::IAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IAdviseSinkEx {}
#[cfg(feature = "unknwnbase")]
windows_core::imp::define_interface!(IClassFactory2, IClassFactory2_Vtbl, 0xb196b28f_bab4_101a_b69c_00aa00341d07);
#[cfg(feature = "unknwnbase")]
impl core::ops::Deref for IClassFactory2 {
    type Target = super::unknwnbase::IClassFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "unknwnbase")]
windows_core::imp::interface_hierarchy!(IClassFactory2, windows_core::IUnknown, super::unknwnbase::IClassFactory);
#[cfg(feature = "unknwnbase")]
impl IClassFactory2 {
    pub unsafe fn GetLicInfo(&self, plicinfo: *mut LICINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLicInfo)(windows_core::Interface::as_raw(self), plicinfo as _) }
    }
    pub unsafe fn RequestLicKey(&self, dwreserved: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestLicKey)(windows_core::Interface::as_raw(self), dwreserved, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreateInstanceLic<P0, P1, T>(&self, punkouter: P0, punkreserved: P1, bstrkey: &windows_core::BSTR) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateInstanceLic)(windows_core::Interface::as_raw(self), punkouter.param().abi(), punkreserved.param().abi(), &T::IID, core::mem::transmute_copy(bstrkey), &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[cfg(feature = "unknwnbase")]
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory2_Vtbl {
    pub base__: super::unknwnbase::IClassFactory_Vtbl,
    pub GetLicInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LICINFO) -> windows_core::HRESULT,
    pub RequestLicKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceLic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "unknwnbase")]
pub trait IClassFactory2_Impl: super::unknwnbase::IClassFactory_Impl {
    fn GetLicInfo(&self, plicinfo: *mut LICINFO) -> windows_core::Result<()>;
    fn RequestLicKey(&self, dwreserved: u32) -> windows_core::Result<windows_core::BSTR>;
    fn CreateInstanceLic(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, punkreserved: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, bstrkey: &windows_core::BSTR, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "unknwnbase")]
impl IClassFactory2_Vtbl {
    pub const fn new<Identity: IClassFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLicInfo<Identity: IClassFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plicinfo: *mut LICINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassFactory2_Impl::GetLicInfo(this, core::mem::transmute_copy(&plicinfo)).into()
            }
        }
        unsafe extern "system" fn RequestLicKey<Identity: IClassFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32, pbstrkey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceLic<Identity: IClassFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, punkreserved: *mut core::ffi::c_void, riid: *const windows_core::GUID, bstrkey: *mut core::ffi::c_void, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassFactory2_Impl::CreateInstanceLic(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&punkreserved), core::mem::transmute_copy(&riid), core::mem::transmute(&bstrkey), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        Self {
            base__: super::unknwnbase::IClassFactory_Vtbl::new::<Identity, OFFSET>(),
            GetLicInfo: GetLicInfo::<Identity, OFFSET>,
            RequestLicKey: RequestLicKey::<Identity, OFFSET>,
            CreateInstanceLic: CreateInstanceLic::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClassFactory2 as windows_core::Interface>::IID || iid == &<super::unknwnbase::IClassFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "unknwnbase")]
impl windows_core::RuntimeName for IClassFactory2 {}
windows_core::imp::define_interface!(IConnectionPoint, IConnectionPoint_Vtbl, 0xb196b286_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IConnectionPoint, windows_core::IUnknown);
impl IConnectionPoint {
    pub unsafe fn GetConnectionInterface(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectionInterface)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnectionPointContainer(&self) -> windows_core::Result<IConnectionPointContainer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectionPointContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Advise<P0>(&self, punksink: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), punksink.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwcookie) }
    }
    pub unsafe fn EnumConnections(&self) -> windows_core::Result<IEnumConnections> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumConnections)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPoint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetConnectionInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetConnectionPointContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IConnectionPoint_Impl: windows_core::IUnknownImpl {
    fn GetConnectionInterface(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetConnectionPointContainer(&self) -> windows_core::Result<IConnectionPointContainer>;
    fn Advise(&self, punksink: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn EnumConnections(&self) -> windows_core::Result<IEnumConnections>;
}
impl IConnectionPoint_Vtbl {
    pub const fn new<Identity: IConnectionPoint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConnectionInterface<Identity: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Advise<Identity: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Unadvise<Identity: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IConnectionPoint {}
windows_core::imp::define_interface!(IConnectionPointContainer, IConnectionPointContainer_Vtbl, 0xb196b284_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IConnectionPointContainer, windows_core::IUnknown);
impl IConnectionPointContainer {
    pub unsafe fn EnumConnectionPoints(&self) -> windows_core::Result<IEnumConnectionPoints> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumConnectionPoints)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindConnectionPoint(&self, riid: *const windows_core::GUID) -> windows_core::Result<IConnectionPoint> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindConnectionPoint)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPointContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumConnectionPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindConnectionPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IConnectionPointContainer_Impl: windows_core::IUnknownImpl {
    fn EnumConnectionPoints(&self) -> windows_core::Result<IEnumConnectionPoints>;
    fn FindConnectionPoint(&self, riid: *const windows_core::GUID) -> windows_core::Result<IConnectionPoint>;
}
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
        unsafe extern "system" fn FindConnectionPoint<Identity: IConnectionPointContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppcp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IConnectionPointContainer {}
windows_core::imp::define_interface!(IEnumConnectionPoints, IEnumConnectionPoints_Vtbl, 0xb196b285_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IEnumConnectionPoints, windows_core::IUnknown);
impl IEnumConnectionPoints {
    pub unsafe fn Next(&self, cconnections: u32, ppcp: *mut Option<IConnectionPoint>, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cconnections, core::mem::transmute(ppcp), pcfetched as _) }
    }
    pub unsafe fn Skip(&self, cconnections: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cconnections) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnectionPoints_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumConnectionPoints_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cconnections: u32, ppcp: windows_core::OutRef<IConnectionPoint>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cconnections: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumConnectionPoints>;
}
impl IEnumConnectionPoints_Vtbl {
    pub const fn new<Identity: IEnumConnectionPoints_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: u32, ppcp: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnectionPoints_Impl::Next(this, core::mem::transmute_copy(&cconnections), core::mem::transmute_copy(&ppcp), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: u32) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IEnumConnectionPoints {}
windows_core::imp::define_interface!(IEnumConnections, IEnumConnections_Vtbl, 0xb196b287_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IEnumConnections, windows_core::IUnknown);
impl IEnumConnections {
    pub unsafe fn Next(&self, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cconnections, rgcd, pcfetched as _) }
    }
    pub unsafe fn Skip(&self, cconnections: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cconnections) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnections_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CONNECTDATA, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumConnections_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cconnections: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumConnections>;
}
impl IEnumConnections_Vtbl {
    pub const fn new<Identity: IEnumConnections_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnections_Impl::Next(this, core::mem::transmute_copy(&cconnections), core::mem::transmute_copy(&rgcd), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: u32) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IEnumConnections {}
windows_core::imp::define_interface!(IEnumOleUndoUnits, IEnumOleUndoUnits_Vtbl, 0xb3e7c340_ef97_11ce_9bc9_00aa00608e01);
windows_core::imp::interface_hierarchy!(IEnumOleUndoUnits, windows_core::IUnknown);
impl IEnumOleUndoUnits {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut Option<IOleUndoUnit>, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOleUndoUnits_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumOleUndoUnits_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: windows_core::OutRef<IOleUndoUnit>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOleUndoUnits>;
}
impl IEnumOleUndoUnits_Vtbl {
    pub const fn new<Identity: IEnumOleUndoUnits_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOleUndoUnits_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumOleUndoUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
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
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Size(&self) -> windows_core::Result<super::wtypes::CY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Size(&self, size: super::wtypes::CY) -> windows_core::HRESULT {
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
    pub unsafe fn get_Weight(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Weight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Weight(&self, weight: i16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Weight)(windows_core::Interface::as_raw(self), weight) }
    }
    pub unsafe fn get_Charset(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Charset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Charset(&self, charset: i16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Charset)(windows_core::Interface::as_raw(self), charset) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn get_hFont(&self) -> windows_core::Result<super::windef::HFONT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_hFont)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsEqual<P0>(&self, pfontother: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), pfontother.param().abi()) }
    }
    pub unsafe fn SetRatio(&self, cylogical: i32, cyhimetric: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRatio)(windows_core::Interface::as_raw(self), cylogical, cyhimetric) }
    }
    #[cfg(feature = "wingdi")]
    pub unsafe fn QueryTextMetrics(&self, ptm: *mut TEXTMETRICOLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryTextMetrics)(windows_core::Interface::as_raw(self), ptm as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn AddRefHfont(&self, hfont: super::windef::HFONT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddRefHfont)(windows_core::Interface::as_raw(self), hfont) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ReleaseHfont(&self, hfont: super::windef::HFONT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseHfont)(windows_core::Interface::as_raw(self), hfont) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetHdc(&self, hdc: super::windef::HDC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHdc)(windows_core::Interface::as_raw(self), hdc) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFont_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::CY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Size: usize,
    #[cfg(feature = "wtypes")]
    pub put_Size: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::CY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Size: usize,
    pub get_Bold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_Bold: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub get_Italic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_Italic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub get_Underline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_Underline: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub get_Strikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_Strikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub get_Weight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub put_Weight: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub get_Charset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub put_Charset: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub get_hFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    get_hFont: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRatio: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wingdi")]
    pub QueryTextMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TEXTMETRICOLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wingdi"))]
    QueryTextMetrics: usize,
    #[cfg(feature = "windef")]
    pub AddRefHfont: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AddRefHfont: usize,
    #[cfg(feature = "windef")]
    pub ReleaseHfont: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ReleaseHfont: usize,
    #[cfg(feature = "windef")]
    pub SetHdc: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetHdc: usize,
}
#[cfg(all(feature = "windef", feature = "wingdi", feature = "wtypes"))]
pub trait IFont_Impl: windows_core::IUnknownImpl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Size(&self) -> windows_core::Result<super::wtypes::CY>;
    fn put_Size(&self, size: &super::wtypes::CY) -> windows_core::Result<()>;
    fn get_Bold(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_Bold(&self, bold: windows_core::BOOL) -> windows_core::Result<()>;
    fn get_Italic(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_Italic(&self, italic: windows_core::BOOL) -> windows_core::Result<()>;
    fn get_Underline(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_Underline(&self, underline: windows_core::BOOL) -> windows_core::Result<()>;
    fn get_Strikethrough(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_Strikethrough(&self, strikethrough: windows_core::BOOL) -> windows_core::Result<()>;
    fn get_Weight(&self) -> windows_core::Result<i16>;
    fn put_Weight(&self, weight: i16) -> windows_core::Result<()>;
    fn get_Charset(&self) -> windows_core::Result<i16>;
    fn put_Charset(&self, charset: i16) -> windows_core::Result<()>;
    fn get_hFont(&self) -> windows_core::Result<super::windef::HFONT>;
    fn Clone(&self) -> windows_core::Result<IFont>;
    fn IsEqual(&self, pfontother: windows_core::Ref<IFont>) -> windows_core::Result<()>;
    fn SetRatio(&self, cylogical: i32, cyhimetric: i32) -> windows_core::Result<()>;
    fn QueryTextMetrics(&self, ptm: *mut TEXTMETRICOLE) -> windows_core::Result<()>;
    fn AddRefHfont(&self, hfont: super::windef::HFONT) -> windows_core::Result<()>;
    fn ReleaseHfont(&self, hfont: super::windef::HFONT) -> windows_core::Result<()>;
    fn SetHdc(&self, hdc: super::windef::HDC) -> windows_core::Result<()>;
}
#[cfg(all(feature = "windef", feature = "wingdi", feature = "wtypes"))]
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
        unsafe extern "system" fn get_Size<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psize: *mut super::wtypes::CY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn put_Size<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: super::wtypes::CY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_Weight<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pweight: *mut i16) -> windows_core::HRESULT {
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
        unsafe extern "system" fn put_Weight<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, weight: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::put_Weight(this, core::mem::transmute_copy(&weight)).into()
            }
        }
        unsafe extern "system" fn get_Charset<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcharset: *mut i16) -> windows_core::HRESULT {
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
        unsafe extern "system" fn put_Charset<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, charset: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::put_Charset(this, core::mem::transmute_copy(&charset)).into()
            }
        }
        unsafe extern "system" fn get_hFont<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phfont: *mut super::windef::HFONT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetRatio<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cylogical: i32, cyhimetric: i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn AddRefHfont<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfont: super::windef::HFONT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::AddRefHfont(this, core::mem::transmute_copy(&hfont)).into()
            }
        }
        unsafe extern "system" fn ReleaseHfont<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfont: super::windef::HFONT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFont_Impl::ReleaseHfont(this, core::mem::transmute_copy(&hfont)).into()
            }
        }
        unsafe extern "system" fn SetHdc<Identity: IFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC) -> windows_core::HRESULT {
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
#[cfg(all(feature = "windef", feature = "wingdi", feature = "wtypes"))]
impl windows_core::RuntimeName for IFont {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFontDisp, IFontDisp_Vtbl, 0xbef6e003_a874_101a_8bba_00aa00300cab);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFontDisp {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFontDisp, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFontDisp_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFontDisp_Impl: super::oaidl::IDispatch_Impl {}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFontDisp_Vtbl {
    pub const fn new<Identity: IFontDisp_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFontDisp as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFontDisp {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFontEventsDisp, IFontEventsDisp_Vtbl, 0x4ef6100a_af88_11d0_9846_00c04fc29993);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFontEventsDisp {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFontEventsDisp, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFontEventsDisp_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFontEventsDisp_Impl: super::oaidl::IDispatch_Impl {}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFontEventsDisp_Vtbl {
    pub const fn new<Identity: IFontEventsDisp_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFontEventsDisp as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn GetSite<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetSite)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectWithSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSite: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IObjectWithSite_Impl: windows_core::IUnknownImpl {
    fn SetSite(&self, punksite: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetSite(&self, riid: *const windows_core::GUID, ppvsite: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IObjectWithSite_Vtbl {
    pub const fn new<Identity: IObjectWithSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSite<Identity: IObjectWithSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectWithSite_Impl::SetSite(this, core::mem::transmute_copy(&punksite)).into()
            }
        }
        unsafe extern "system" fn GetSite<Identity: IObjectWithSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvsite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IObjectWithSite {}
windows_core::imp::define_interface!(IOleControl, IOleControl_Vtbl, 0xb196b288_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IOleControl, windows_core::IUnknown);
impl IOleControl {
    #[cfg(feature = "windef")]
    pub unsafe fn GetControlInfo(&self, pci: *mut CONTROLINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetControlInfo)(windows_core::Interface::as_raw(self), pci as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn OnMnemonic(&self, pmsg: *const super::winuser::MSG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnMnemonic)(windows_core::Interface::as_raw(self), pmsg) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn OnAmbientPropertyChange(&self, dispid: super::oaidl::DISPID) -> windows_core::HRESULT {
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
    #[cfg(feature = "windef")]
    pub GetControlInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CONTROLINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetControlInfo: usize,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub OnMnemonic: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::winuser::MSG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    OnMnemonic: usize,
    #[cfg(feature = "oaidl")]
    pub OnAmbientPropertyChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    OnAmbientPropertyChange: usize,
    pub FreezeEvents: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "windef", feature = "winuser"))]
pub trait IOleControl_Impl: windows_core::IUnknownImpl {
    fn GetControlInfo(&self, pci: *mut CONTROLINFO) -> windows_core::Result<()>;
    fn OnMnemonic(&self, pmsg: *const super::winuser::MSG) -> windows_core::Result<()>;
    fn OnAmbientPropertyChange(&self, dispid: super::oaidl::DISPID) -> windows_core::Result<()>;
    fn FreezeEvents(&self, bfreeze: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "windef", feature = "winuser"))]
impl IOleControl_Vtbl {
    pub const fn new<Identity: IOleControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetControlInfo<Identity: IOleControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pci: *mut CONTROLINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControl_Impl::GetControlInfo(this, core::mem::transmute_copy(&pci)).into()
            }
        }
        unsafe extern "system" fn OnMnemonic<Identity: IOleControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::winuser::MSG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControl_Impl::OnMnemonic(this, core::mem::transmute_copy(&pmsg)).into()
            }
        }
        unsafe extern "system" fn OnAmbientPropertyChange<Identity: IOleControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::oaidl::DISPID) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "windef", feature = "winuser"))]
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
    pub unsafe fn GetExtendedControl(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExtendedControl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TransformCoords(&self, pptlhimetric: *mut super::windef::POINTL, pptfcontainer: *mut POINTF, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TransformCoords)(windows_core::Interface::as_raw(self), pptlhimetric as _, pptfcontainer as _, dwflags) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::winuser::MSG, grfmodifiers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TranslateAccelerator)(windows_core::Interface::as_raw(self), pmsg, grfmodifiers) }
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
    #[cfg(feature = "windef")]
    pub TransformCoords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::POINTL, *mut POINTF, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    TransformCoords: usize,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub TranslateAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::winuser::MSG, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    TranslateAccelerator: usize,
    pub OnFocus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub ShowPropertyFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "windef", feature = "winuser"))]
pub trait IOleControlSite_Impl: windows_core::IUnknownImpl {
    fn OnControlInfoChanged(&self) -> windows_core::Result<()>;
    fn LockInPlaceActive(&self, flock: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetExtendedControl(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn TransformCoords(&self, pptlhimetric: *mut super::windef::POINTL, pptfcontainer: *mut POINTF, dwflags: u32) -> windows_core::Result<()>;
    fn TranslateAccelerator(&self, pmsg: *const super::winuser::MSG, grfmodifiers: u32) -> windows_core::Result<()>;
    fn OnFocus(&self, fgotfocus: windows_core::BOOL) -> windows_core::Result<()>;
    fn ShowPropertyFrame(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "windef", feature = "winuser"))]
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
        unsafe extern "system" fn TransformCoords<Identity: IOleControlSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptlhimetric: *mut super::windef::POINTL, pptfcontainer: *mut POINTF, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControlSite_Impl::TransformCoords(this, core::mem::transmute_copy(&pptlhimetric), core::mem::transmute_copy(&pptfcontainer), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: IOleControlSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::winuser::MSG, grfmodifiers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleControlSite_Impl::TranslateAccelerator(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&grfmodifiers)).into()
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
            TranslateAccelerator: TranslateAccelerator::<Identity, OFFSET>,
            OnFocus: OnFocus::<Identity, OFFSET>,
            ShowPropertyFrame: ShowPropertyFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleControlSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "windef", feature = "winuser"))]
impl windows_core::RuntimeName for IOleControlSite {}
#[cfg(feature = "oleidl")]
windows_core::imp::define_interface!(IOleInPlaceObjectWindowless, IOleInPlaceObjectWindowless_Vtbl, 0x1c2056cc_5ef4_101b_8bc8_00aa003e3b29);
#[cfg(feature = "oleidl")]
impl core::ops::Deref for IOleInPlaceObjectWindowless {
    type Target = super::oleidl::IOleInPlaceObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oleidl")]
windows_core::imp::interface_hierarchy!(IOleInPlaceObjectWindowless, windows_core::IUnknown, super::oleidl::IOleWindow, super::oleidl::IOleInPlaceObject);
#[cfg(feature = "oleidl")]
impl IOleInPlaceObjectWindowless {
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnWindowMessage(&self, msg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<super::minwindef::LRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnWindowMessage)(windows_core::Interface::as_raw(self), msg, wparam, lparam, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDropTarget(&self) -> windows_core::Result<super::oleidl::IDropTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDropTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oleidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceObjectWindowless_Vtbl {
    pub base__: super::oleidl::IOleInPlaceObject_Vtbl,
    #[cfg(feature = "minwindef")]
    pub OnWindowMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut super::minwindef::LRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnWindowMessage: usize,
    pub GetDropTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef"))]
pub trait IOleInPlaceObjectWindowless_Impl: super::oleidl::IOleInPlaceObject_Impl {
    fn OnWindowMessage(&self, msg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<super::minwindef::LRESULT>;
    fn GetDropTarget(&self) -> windows_core::Result<super::oleidl::IDropTarget>;
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef"))]
impl IOleInPlaceObjectWindowless_Vtbl {
    pub const fn new<Identity: IOleInPlaceObjectWindowless_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnWindowMessage<Identity: IOleInPlaceObjectWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, plresult: *mut super::minwindef::LRESULT) -> windows_core::HRESULT {
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
            base__: super::oleidl::IOleInPlaceObject_Vtbl::new::<Identity, OFFSET>(),
            OnWindowMessage: OnWindowMessage::<Identity, OFFSET>,
            GetDropTarget: GetDropTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceObjectWindowless as windows_core::Interface>::IID || iid == &<super::oleidl::IOleWindow as windows_core::Interface>::IID || iid == &<super::oleidl::IOleInPlaceObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef"))]
impl windows_core::RuntimeName for IOleInPlaceObjectWindowless {}
#[cfg(feature = "oleidl")]
windows_core::imp::define_interface!(IOleInPlaceSiteEx, IOleInPlaceSiteEx_Vtbl, 0x9c2cad80_3424_11cf_b670_00aa004cd6d8);
#[cfg(feature = "oleidl")]
impl core::ops::Deref for IOleInPlaceSiteEx {
    type Target = super::oleidl::IOleInPlaceSite;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oleidl")]
windows_core::imp::interface_hierarchy!(IOleInPlaceSiteEx, windows_core::IUnknown, super::oleidl::IOleWindow, super::oleidl::IOleInPlaceSite);
#[cfg(feature = "oleidl")]
impl IOleInPlaceSiteEx {
    pub unsafe fn OnInPlaceActivateEx(&self, pfnoredraw: *mut windows_core::BOOL, dwflags: u32) -> windows_core::HRESULT {
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
    pub base__: super::oleidl::IOleInPlaceSite_Vtbl,
    pub OnInPlaceActivateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub OnInPlaceDeactivateEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub RequestUIActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
pub trait IOleInPlaceSiteEx_Impl: super::oleidl::IOleInPlaceSite_Impl {
    fn OnInPlaceActivateEx(&self, pfnoredraw: *mut windows_core::BOOL, dwflags: u32) -> windows_core::Result<()>;
    fn OnInPlaceDeactivateEx(&self, fnoredraw: windows_core::BOOL) -> windows_core::Result<()>;
    fn RequestUIActivate(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
impl IOleInPlaceSiteEx_Vtbl {
    pub const fn new<Identity: IOleInPlaceSiteEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnInPlaceActivateEx<Identity: IOleInPlaceSiteEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfnoredraw: *mut windows_core::BOOL, dwflags: u32) -> windows_core::HRESULT {
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
            base__: super::oleidl::IOleInPlaceSite_Vtbl::new::<Identity, OFFSET>(),
            OnInPlaceActivateEx: OnInPlaceActivateEx::<Identity, OFFSET>,
            OnInPlaceDeactivateEx: OnInPlaceDeactivateEx::<Identity, OFFSET>,
            RequestUIActivate: RequestUIActivate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceSiteEx as windows_core::Interface>::IID || iid == &<super::oleidl::IOleWindow as windows_core::Interface>::IID || iid == &<super::oleidl::IOleInPlaceSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
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
windows_core::imp::interface_hierarchy!(IOleInPlaceSiteWindowless, windows_core::IUnknown, super::oleidl::IOleWindow, super::oleidl::IOleInPlaceSite, IOleInPlaceSiteEx);
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
    #[cfg(feature = "windef")]
    pub unsafe fn GetDC(&self, prect: *const super::windef::RECT, grfflags: u32) -> windows_core::Result<super::windef::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), prect, grfflags, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ReleaseDC(&self, hdc: super::windef::HDC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), hdc) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn InvalidateRect(&self, prect: *const super::windef::RECT, ferase: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvalidateRect)(windows_core::Interface::as_raw(self), prect, ferase.into()) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn InvalidateRgn(&self, hrgn: super::minwindef::HRGN, ferase: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvalidateRgn)(windows_core::Interface::as_raw(self), hrgn, ferase.into()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ScrollRect(&self, dx: i32, dy: i32, prectscroll: *const super::windef::RECT, prectclip: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ScrollRect)(windows_core::Interface::as_raw(self), dx, dy, prectscroll, prectclip) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn AdjustRect(&self, prc: *mut super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AdjustRect)(windows_core::Interface::as_raw(self), prc as _) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnDefWindowMessage(&self, msg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<super::minwindef::LRESULT> {
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
    #[cfg(feature = "windef")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT, u32, *mut super::windef::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetDC: usize,
    #[cfg(feature = "windef")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ReleaseDC: usize,
    #[cfg(feature = "windef")]
    pub InvalidateRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    InvalidateRect: usize,
    #[cfg(feature = "minwindef")]
    pub InvalidateRgn: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::HRGN, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    InvalidateRgn: usize,
    #[cfg(feature = "windef")]
    pub ScrollRect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *const super::windef::RECT, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ScrollRect: usize,
    #[cfg(feature = "windef")]
    pub AdjustRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AdjustRect: usize,
    #[cfg(feature = "minwindef")]
    pub OnDefWindowMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut super::minwindef::LRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnDefWindowMessage: usize,
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef"))]
pub trait IOleInPlaceSiteWindowless_Impl: IOleInPlaceSiteEx_Impl {
    fn CanWindowlessActivate(&self) -> windows_core::Result<()>;
    fn GetCapture(&self) -> windows_core::Result<()>;
    fn SetCapture(&self, fcapture: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetFocus(&self) -> windows_core::Result<()>;
    fn SetFocus(&self, ffocus: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetDC(&self, prect: *const super::windef::RECT, grfflags: u32) -> windows_core::Result<super::windef::HDC>;
    fn ReleaseDC(&self, hdc: super::windef::HDC) -> windows_core::Result<()>;
    fn InvalidateRect(&self, prect: *const super::windef::RECT, ferase: windows_core::BOOL) -> windows_core::Result<()>;
    fn InvalidateRgn(&self, hrgn: super::minwindef::HRGN, ferase: windows_core::BOOL) -> windows_core::Result<()>;
    fn ScrollRect(&self, dx: i32, dy: i32, prectscroll: *const super::windef::RECT, prectclip: *const super::windef::RECT) -> windows_core::Result<()>;
    fn AdjustRect(&self, prc: *mut super::windef::RECT) -> windows_core::Result<()>;
    fn OnDefWindowMessage(&self, msg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<super::minwindef::LRESULT>;
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef"))]
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
        unsafe extern "system" fn GetDC<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::windef::RECT, grfflags: u32, phdc: *mut super::windef::HDC) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ReleaseDC<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::ReleaseDC(this, core::mem::transmute_copy(&hdc)).into()
            }
        }
        unsafe extern "system" fn InvalidateRect<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::windef::RECT, ferase: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::InvalidateRect(this, core::mem::transmute_copy(&prect), core::mem::transmute_copy(&ferase)).into()
            }
        }
        unsafe extern "system" fn InvalidateRgn<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrgn: super::minwindef::HRGN, ferase: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::InvalidateRgn(this, core::mem::transmute_copy(&hrgn), core::mem::transmute_copy(&ferase)).into()
            }
        }
        unsafe extern "system" fn ScrollRect<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dx: i32, dy: i32, prectscroll: *const super::windef::RECT, prectclip: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::ScrollRect(this, core::mem::transmute_copy(&dx), core::mem::transmute_copy(&dy), core::mem::transmute_copy(&prectscroll), core::mem::transmute_copy(&prectclip)).into()
            }
        }
        unsafe extern "system" fn AdjustRect<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSiteWindowless_Impl::AdjustRect(this, core::mem::transmute_copy(&prc)).into()
            }
        }
        unsafe extern "system" fn OnDefWindowMessage<Identity: IOleInPlaceSiteWindowless_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, plresult: *mut super::minwindef::LRESULT) -> windows_core::HRESULT {
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
        iid == &<IOleInPlaceSiteWindowless as windows_core::Interface>::IID || iid == &<super::oleidl::IOleWindow as windows_core::Interface>::IID || iid == &<super::oleidl::IOleInPlaceSite as windows_core::Interface>::IID || iid == &<IOleInPlaceSiteEx as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef"))]
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
    pub unsafe fn GetParentState(&self) -> windows_core::Result<u32> {
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
    pub GetParentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IOleParentUndoUnit_Impl: IOleUndoUnit_Impl {
    fn Open(&self, ppuu: windows_core::Ref<IOleParentUndoUnit>) -> windows_core::Result<()>;
    fn Close(&self, ppuu: windows_core::Ref<IOleParentUndoUnit>, fcommit: windows_core::BOOL) -> windows_core::Result<()>;
    fn Add(&self, puu: windows_core::Ref<IOleUndoUnit>) -> windows_core::Result<()>;
    fn FindUnit(&self, puu: windows_core::Ref<IOleUndoUnit>) -> windows_core::Result<()>;
    fn GetParentState(&self) -> windows_core::Result<u32>;
}
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
        unsafe extern "system" fn GetParentState<Identity: IOleParentUndoUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut u32) -> windows_core::HRESULT {
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
    pub unsafe fn GetOpenParentState(&self) -> windows_core::Result<u32> {
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
            (windows_core::Interface::vtable(self).EnumUndoable)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumRedoable(&self) -> windows_core::Result<IEnumOleUndoUnits> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRedoable)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub GetOpenParentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DiscardFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UndoTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RedoTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumUndoable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumRedoable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastUndoDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastRedoDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IOleUndoManager_Impl: windows_core::IUnknownImpl {
    fn Open(&self, ppuu: windows_core::Ref<IOleParentUndoUnit>) -> windows_core::Result<()>;
    fn Close(&self, ppuu: windows_core::Ref<IOleParentUndoUnit>, fcommit: windows_core::BOOL) -> windows_core::Result<()>;
    fn Add(&self, puu: windows_core::Ref<IOleUndoUnit>) -> windows_core::Result<()>;
    fn GetOpenParentState(&self) -> windows_core::Result<u32>;
    fn DiscardFrom(&self, puu: windows_core::Ref<IOleUndoUnit>) -> windows_core::Result<()>;
    fn UndoTo(&self, puu: windows_core::Ref<IOleUndoUnit>) -> windows_core::Result<()>;
    fn RedoTo(&self, puu: windows_core::Ref<IOleUndoUnit>) -> windows_core::Result<()>;
    fn EnumUndoable(&self) -> windows_core::Result<IEnumOleUndoUnits>;
    fn EnumRedoable(&self) -> windows_core::Result<IEnumOleUndoUnits>;
    fn GetLastUndoDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetLastRedoDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Enable(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
}
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
        unsafe extern "system" fn GetOpenParentState<Identity: IOleUndoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut u32) -> windows_core::HRESULT {
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
    pub unsafe fn GetUnitType(&self, pclsid: *mut windows_core::GUID, plid: *mut i32) -> windows_core::HRESULT {
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
    pub GetUnitType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut i32) -> windows_core::HRESULT,
    pub OnNextAdd: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOleUndoUnit_Impl: windows_core::IUnknownImpl {
    fn Do(&self, pundomanager: windows_core::Ref<IOleUndoManager>) -> windows_core::Result<()>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetUnitType(&self, pclsid: *mut windows_core::GUID, plid: *mut i32) -> windows_core::Result<()>;
    fn OnNextAdd(&self) -> windows_core::Result<()>;
}
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
        unsafe extern "system" fn GetUnitType<Identity: IOleUndoUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID, plid: *mut i32) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IOleUndoUnit {}
windows_core::imp::define_interface!(IPerPropertyBrowsing, IPerPropertyBrowsing_Vtbl, 0x376bd3aa_3845_101b_84ed_08002b2ec713);
windows_core::imp::interface_hierarchy!(IPerPropertyBrowsing, windows_core::IUnknown);
impl IPerPropertyBrowsing {
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetDisplayString(&self, dispid: super::oaidl::DISPID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayString)(windows_core::Interface::as_raw(self), dispid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn MapPropertyToPage(&self, dispid: super::oaidl::DISPID) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MapPropertyToPage)(windows_core::Interface::as_raw(self), dispid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetPredefinedStrings(&self, dispid: super::oaidl::DISPID, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPredefinedStrings)(windows_core::Interface::as_raw(self), dispid, pcastringsout as _, pcacookiesout as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPredefinedValue(&self, dispid: super::oaidl::DISPID, dwcookie: u32) -> windows_core::Result<super::oaidl::VARIANT> {
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
    #[cfg(feature = "oaidl")]
    pub GetDisplayString: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetDisplayString: usize,
    #[cfg(feature = "oaidl")]
    pub MapPropertyToPage: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    MapPropertyToPage: usize,
    #[cfg(feature = "oaidl")]
    pub GetPredefinedStrings: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, *mut CALPOLESTR, *mut CADWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetPredefinedStrings: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPredefinedValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, u32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetPredefinedValue: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPerPropertyBrowsing_Impl: windows_core::IUnknownImpl {
    fn GetDisplayString(&self, dispid: super::oaidl::DISPID) -> windows_core::Result<windows_core::BSTR>;
    fn MapPropertyToPage(&self, dispid: super::oaidl::DISPID) -> windows_core::Result<windows_core::GUID>;
    fn GetPredefinedStrings(&self, dispid: super::oaidl::DISPID, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> windows_core::Result<()>;
    fn GetPredefinedValue(&self, dispid: super::oaidl::DISPID, dwcookie: u32) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IPerPropertyBrowsing_Vtbl {
    pub const fn new<Identity: IPerPropertyBrowsing_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDisplayString<Identity: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::oaidl::DISPID, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn MapPropertyToPage<Identity: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::oaidl::DISPID, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetPredefinedStrings<Identity: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::oaidl::DISPID, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerPropertyBrowsing_Impl::GetPredefinedStrings(this, core::mem::transmute_copy(&dispid), core::mem::transmute_copy(&pcastringsout), core::mem::transmute_copy(&pcacookiesout)).into()
            }
        }
        unsafe extern "system" fn GetPredefinedValue<Identity: IPerPropertyBrowsing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::oaidl::DISPID, dwcookie: u32, pvarout: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPerPropertyBrowsing {}
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(IPersistMemory, IPersistMemory_Vtbl, 0xbd1ae5e0_a6ae_11ce_bd37_504200c10000);
#[cfg(feature = "objidl")]
impl core::ops::Deref for IPersistMemory {
    type Target = super::objidl::IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(IPersistMemory, windows_core::IUnknown, super::objidl::IPersist);
#[cfg(feature = "objidl")]
impl IPersistMemory {
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Load(&self, pmem: *const core::ffi::c_void, cbsize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pmem, cbsize) }
    }
    pub unsafe fn Save(&self, pmem: *mut core::ffi::c_void, fcleardirty: bool, cbsize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pmem as _, fcleardirty.into(), cbsize) }
    }
    pub unsafe fn GetSizeMax(&self) -> windows_core::Result<u32> {
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
    pub base__: super::objidl::IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub GetSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "objidl")]
pub trait IPersistMemory_Impl: super::objidl::IPersist_Impl {
    fn IsDirty(&self) -> windows_core::Result<()>;
    fn Load(&self, pmem: *const core::ffi::c_void, cbsize: u32) -> windows_core::Result<()>;
    fn Save(&self, pmem: *mut core::ffi::c_void, fcleardirty: windows_core::BOOL, cbsize: u32) -> windows_core::Result<()>;
    fn GetSizeMax(&self) -> windows_core::Result<u32>;
    fn InitNew(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "objidl")]
impl IPersistMemory_Vtbl {
    pub const fn new<Identity: IPersistMemory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDirty<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMemory_Impl::IsDirty(this).into()
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: *const core::ffi::c_void, cbsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMemory_Impl::Load(this, core::mem::transmute_copy(&pmem), core::mem::transmute_copy(&cbsize)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: *mut core::ffi::c_void, fcleardirty: windows_core::BOOL, cbsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMemory_Impl::Save(this, core::mem::transmute_copy(&pmem), core::mem::transmute_copy(&fcleardirty), core::mem::transmute_copy(&cbsize)).into()
            }
        }
        unsafe extern "system" fn GetSizeMax<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u32) -> windows_core::HRESULT {
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
            base__: super::objidl::IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, OFFSET>,
            InitNew: InitNew::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistMemory as windows_core::Interface>::IID || iid == &<super::objidl::IPersist as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for IPersistMemory {}
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(IPersistPropertyBag, IPersistPropertyBag_Vtbl, 0x37d84f60_42cb_11ce_8135_00aa004bb851);
#[cfg(feature = "objidl")]
impl core::ops::Deref for IPersistPropertyBag {
    type Target = super::objidl::IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(IPersistPropertyBag, windows_core::IUnknown, super::objidl::IPersist);
#[cfg(feature = "objidl")]
impl IPersistPropertyBag {
    pub unsafe fn InitNew(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn Load<P0, P1>(&self, ppropbag: P0, perrorlog: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IPropertyBag>,
        P1: windows_core::Param<super::oaidl::IErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), ppropbag.param().abi(), perrorlog.param().abi()) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn Save<P0>(&self, ppropbag: P0, fcleardirty: bool, fsaveallproperties: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IPropertyBag>,
    {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), ppropbag.param().abi(), fcleardirty.into(), fsaveallproperties.into()) }
    }
}
#[cfg(feature = "objidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPersistPropertyBag_Vtbl {
    pub base__: super::objidl::IPersist_Vtbl,
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
#[cfg(all(feature = "oaidl", feature = "objidl"))]
pub trait IPersistPropertyBag_Impl: super::objidl::IPersist_Impl {
    fn InitNew(&self) -> windows_core::Result<()>;
    fn Load(&self, ppropbag: windows_core::Ref<super::oaidl::IPropertyBag>, perrorlog: windows_core::Ref<super::oaidl::IErrorLog>) -> windows_core::Result<()>;
    fn Save(&self, ppropbag: windows_core::Ref<super::oaidl::IPropertyBag>, fcleardirty: windows_core::BOOL, fsaveallproperties: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "objidl"))]
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
            base__: super::objidl::IPersist_Vtbl::new::<Identity, OFFSET>(),
            InitNew: InitNew::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistPropertyBag as windows_core::Interface>::IID || iid == &<super::objidl::IPersist as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "objidl"))]
impl windows_core::RuntimeName for IPersistPropertyBag {}
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(IPersistPropertyBag2, IPersistPropertyBag2_Vtbl, 0x22f55881_280b_11d0_a8a9_00a0c90c2004);
#[cfg(feature = "objidl")]
impl core::ops::Deref for IPersistPropertyBag2 {
    type Target = super::objidl::IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(IPersistPropertyBag2, windows_core::IUnknown, super::objidl::IPersist);
#[cfg(feature = "objidl")]
impl IPersistPropertyBag2 {
    pub unsafe fn InitNew(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn Load<P0, P1>(&self, ppropbag: P0, perrlog: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IPropertyBag2>,
        P1: windows_core::Param<super::oaidl::IErrorLog>,
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
    pub base__: super::objidl::IPersist_Vtbl,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    Load: usize,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "objidl"))]
pub trait IPersistPropertyBag2_Impl: super::objidl::IPersist_Impl {
    fn InitNew(&self) -> windows_core::Result<()>;
    fn Load(&self, ppropbag: windows_core::Ref<IPropertyBag2>, perrlog: windows_core::Ref<super::oaidl::IErrorLog>) -> windows_core::Result<()>;
    fn Save(&self, ppropbag: windows_core::Ref<IPropertyBag2>, fcleardirty: windows_core::BOOL, fsaveallproperties: windows_core::BOOL) -> windows_core::Result<()>;
    fn IsDirty(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "objidl"))]
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
            base__: super::objidl::IPersist_Vtbl::new::<Identity, OFFSET>(),
            InitNew: InitNew::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            IsDirty: IsDirty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistPropertyBag2 as windows_core::Interface>::IID || iid == &<super::objidl::IPersist as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "objidl"))]
impl windows_core::RuntimeName for IPersistPropertyBag2 {}
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(IPersistStreamInit, IPersistStreamInit_Vtbl, 0x7fd52380_4e07_101b_ae2d_08002b2ec713);
#[cfg(feature = "objidl")]
impl core::ops::Deref for IPersistStreamInit {
    type Target = super::objidl::IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(IPersistStreamInit, windows_core::IUnknown, super::objidl::IPersist);
#[cfg(feature = "objidl")]
impl IPersistStreamInit {
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Load<P0>(&self, pstm: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pstm.param().abi()) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Save<P0>(&self, pstm: P0, fcleardirty: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pstm.param().abi(), fcleardirty.into()) }
    }
    pub unsafe fn GetSizeMax(&self) -> windows_core::Result<u64> {
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
    pub base__: super::objidl::IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Load: usize,
    #[cfg(feature = "objidlbase")]
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Save: usize,
    pub GetSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
pub trait IPersistStreamInit_Impl: super::objidl::IPersist_Impl {
    fn IsDirty(&self) -> windows_core::Result<()>;
    fn Load(&self, pstm: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn Save(&self, pstm: windows_core::Ref<super::objidlbase::IStream>, fcleardirty: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSizeMax(&self) -> windows_core::Result<u64>;
    fn InitNew(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
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
        unsafe extern "system" fn GetSizeMax<Identity: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u64) -> windows_core::HRESULT {
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
            base__: super::objidl::IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, OFFSET>,
            InitNew: InitNew::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistStreamInit as windows_core::Interface>::IID || iid == &<super::objidl::IPersist as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
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
    pub unsafe fn get_Type(&self) -> windows_core::Result<i16> {
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
    #[cfg(feature = "windef")]
    pub unsafe fn Render(&self, hdc: super::windef::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: OLE_XPOS_HIMETRIC, ysrc: OLE_YPOS_HIMETRIC, cxsrc: OLE_XSIZE_HIMETRIC, cysrc: OLE_YSIZE_HIMETRIC, prcwbounds: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Render)(windows_core::Interface::as_raw(self), hdc, x, y, cx, cy, xsrc, ysrc, cxsrc, cysrc, prcwbounds) }
    }
    pub unsafe fn set_hPal(&self, hpal: OLE_HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).set_hPal)(windows_core::Interface::as_raw(self), hpal) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn get_CurDC(&self) -> windows_core::Result<super::windef::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SelectPicture(&self, hdcin: super::windef::HDC, phdcout: *mut super::windef::HDC, phbmpout: *mut OLE_HANDLE) -> windows_core::HRESULT {
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
    #[cfg(feature = "objidlbase")]
    pub unsafe fn SaveAsFile<P0>(&self, pstream: P0, fsavememcopy: bool) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SaveAsFile)(windows_core::Interface::as_raw(self), pstream.param().abi(), fsavememcopy.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Attributes(&self) -> windows_core::Result<u32> {
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
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub get_Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OLE_XSIZE_HIMETRIC) -> windows_core::HRESULT,
    pub get_Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OLE_YSIZE_HIMETRIC) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub Render: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, i32, i32, i32, i32, OLE_XPOS_HIMETRIC, OLE_YPOS_HIMETRIC, OLE_XSIZE_HIMETRIC, OLE_YSIZE_HIMETRIC, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Render: usize,
    pub set_hPal: unsafe extern "system" fn(*mut core::ffi::c_void, OLE_HANDLE) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub get_CurDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    get_CurDC: usize,
    #[cfg(feature = "windef")]
    pub SelectPicture: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, *mut super::windef::HDC, *mut OLE_HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SelectPicture: usize,
    pub get_KeepOriginalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_KeepOriginalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub PictureChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub SaveAsFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    SaveAsFile: usize,
    pub get_Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidlbase", feature = "windef"))]
pub trait IPicture_Impl: windows_core::IUnknownImpl {
    fn get_Handle(&self) -> windows_core::Result<OLE_HANDLE>;
    fn get_hPal(&self) -> windows_core::Result<OLE_HANDLE>;
    fn get_Type(&self) -> windows_core::Result<i16>;
    fn get_Width(&self) -> windows_core::Result<OLE_XSIZE_HIMETRIC>;
    fn get_Height(&self) -> windows_core::Result<OLE_YSIZE_HIMETRIC>;
    fn Render(&self, hdc: super::windef::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: OLE_XPOS_HIMETRIC, ysrc: OLE_YPOS_HIMETRIC, cxsrc: OLE_XSIZE_HIMETRIC, cysrc: OLE_YSIZE_HIMETRIC, prcwbounds: *const super::windef::RECT) -> windows_core::Result<()>;
    fn set_hPal(&self, hpal: OLE_HANDLE) -> windows_core::Result<()>;
    fn get_CurDC(&self) -> windows_core::Result<super::windef::HDC>;
    fn SelectPicture(&self, hdcin: super::windef::HDC, phdcout: *mut super::windef::HDC, phbmpout: *mut OLE_HANDLE) -> windows_core::Result<()>;
    fn get_KeepOriginalFormat(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_KeepOriginalFormat(&self, keep: windows_core::BOOL) -> windows_core::Result<()>;
    fn PictureChanged(&self) -> windows_core::Result<()>;
    fn SaveAsFile(&self, pstream: windows_core::Ref<super::objidlbase::IStream>, fsavememcopy: windows_core::BOOL) -> windows_core::Result<i32>;
    fn get_Attributes(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "objidlbase", feature = "windef"))]
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
        unsafe extern "system" fn get_Type<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut i16) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Render<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: OLE_XPOS_HIMETRIC, ysrc: OLE_YPOS_HIMETRIC, cxsrc: OLE_XSIZE_HIMETRIC, cysrc: OLE_YSIZE_HIMETRIC, prcwbounds: *const super::windef::RECT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_CurDC<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdc: *mut super::windef::HDC) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SelectPicture<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdcin: super::windef::HDC, phdcout: *mut super::windef::HDC, phbmpout: *mut OLE_HANDLE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SaveAsFile<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, fsavememcopy: windows_core::BOOL, pcbsize: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_Attributes<Identity: IPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattr: *mut u32) -> windows_core::HRESULT {
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
#[cfg(all(feature = "objidlbase", feature = "windef"))]
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
    pub unsafe fn get_Type(&self) -> windows_core::Result<i16> {
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
    #[cfg(feature = "windef")]
    pub unsafe fn Render(&self, hdc: super::windef::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: OLE_XPOS_HIMETRIC, ysrc: OLE_YPOS_HIMETRIC, cxsrc: OLE_XSIZE_HIMETRIC, cysrc: OLE_YSIZE_HIMETRIC, prcwbounds: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Render)(windows_core::Interface::as_raw(self), hdc, x, y, cx, cy, xsrc, ysrc, cxsrc, cysrc, prcwbounds) }
    }
    pub unsafe fn set_hPal(&self, hpal: HHANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).set_hPal)(windows_core::Interface::as_raw(self), hpal) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn get_CurDC(&self) -> windows_core::Result<super::windef::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SelectPicture(&self, hdcin: super::windef::HDC, phdcout: *mut super::windef::HDC, phbmpout: *mut HHANDLE) -> windows_core::HRESULT {
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
    #[cfg(feature = "objidlbase")]
    pub unsafe fn SaveAsFile<P0>(&self, pstream: P0, fsavememcopy: bool) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SaveAsFile)(windows_core::Interface::as_raw(self), pstream.param().abi(), fsavememcopy.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Attributes(&self) -> windows_core::Result<u32> {
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
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub get_Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OLE_XSIZE_HIMETRIC) -> windows_core::HRESULT,
    pub get_Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OLE_YSIZE_HIMETRIC) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub Render: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, i32, i32, i32, i32, OLE_XPOS_HIMETRIC, OLE_YPOS_HIMETRIC, OLE_XSIZE_HIMETRIC, OLE_YSIZE_HIMETRIC, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Render: usize,
    pub set_hPal: unsafe extern "system" fn(*mut core::ffi::c_void, HHANDLE) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub get_CurDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    get_CurDC: usize,
    #[cfg(feature = "windef")]
    pub SelectPicture: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, *mut super::windef::HDC, *mut HHANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SelectPicture: usize,
    pub get_KeepOriginalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_KeepOriginalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub PictureChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub SaveAsFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    SaveAsFile: usize,
    pub get_Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidlbase", feature = "windef"))]
pub trait IPicture2_Impl: windows_core::IUnknownImpl {
    fn get_Handle(&self) -> windows_core::Result<HHANDLE>;
    fn get_hPal(&self) -> windows_core::Result<HHANDLE>;
    fn get_Type(&self) -> windows_core::Result<i16>;
    fn get_Width(&self) -> windows_core::Result<OLE_XSIZE_HIMETRIC>;
    fn get_Height(&self) -> windows_core::Result<OLE_YSIZE_HIMETRIC>;
    fn Render(&self, hdc: super::windef::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: OLE_XPOS_HIMETRIC, ysrc: OLE_YPOS_HIMETRIC, cxsrc: OLE_XSIZE_HIMETRIC, cysrc: OLE_YSIZE_HIMETRIC, prcwbounds: *const super::windef::RECT) -> windows_core::Result<()>;
    fn set_hPal(&self, hpal: HHANDLE) -> windows_core::Result<()>;
    fn get_CurDC(&self) -> windows_core::Result<super::windef::HDC>;
    fn SelectPicture(&self, hdcin: super::windef::HDC, phdcout: *mut super::windef::HDC, phbmpout: *mut HHANDLE) -> windows_core::Result<()>;
    fn get_KeepOriginalFormat(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_KeepOriginalFormat(&self, keep: windows_core::BOOL) -> windows_core::Result<()>;
    fn PictureChanged(&self) -> windows_core::Result<()>;
    fn SaveAsFile(&self, pstream: windows_core::Ref<super::objidlbase::IStream>, fsavememcopy: windows_core::BOOL) -> windows_core::Result<i32>;
    fn get_Attributes(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "objidlbase", feature = "windef"))]
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
        unsafe extern "system" fn get_Type<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut i16) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Render<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: OLE_XPOS_HIMETRIC, ysrc: OLE_YPOS_HIMETRIC, cxsrc: OLE_XSIZE_HIMETRIC, cysrc: OLE_YSIZE_HIMETRIC, prcwbounds: *const super::windef::RECT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_CurDC<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdc: *mut super::windef::HDC) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SelectPicture<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdcin: super::windef::HDC, phdcout: *mut super::windef::HDC, phbmpout: *mut HHANDLE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SaveAsFile<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, fsavememcopy: windows_core::BOOL, pcbsize: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_Attributes<Identity: IPicture2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattr: *mut u32) -> windows_core::HRESULT {
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
#[cfg(all(feature = "objidlbase", feature = "windef"))]
impl windows_core::RuntimeName for IPicture2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IPictureDisp, IPictureDisp_Vtbl, 0x7bf80981_bf32_101a_8bbb_00aa00300cab);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IPictureDisp {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IPictureDisp, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPictureDisp_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPictureDisp_Impl: super::oaidl::IDispatch_Impl {}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPictureDisp_Vtbl {
    pub const fn new<Identity: IPictureDisp_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPictureDisp as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPictureDisp {}
windows_core::imp::define_interface!(IPointerInactive, IPointerInactive_Vtbl, 0x55980ba0_35aa_11cf_b671_00aa004cd6d8);
windows_core::imp::interface_hierarchy!(IPointerInactive, windows_core::IUnknown);
impl IPointerInactive {
    pub unsafe fn GetActivationPolicy(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActivationPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn OnInactiveMouseMove(&self, prectbounds: *const super::windef::RECT, x: i32, y: i32, grfkeystate: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnInactiveMouseMove)(windows_core::Interface::as_raw(self), prectbounds, x, y, grfkeystate) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn OnInactiveSetCursor(&self, prectbounds: *const super::windef::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnInactiveSetCursor)(windows_core::Interface::as_raw(self), prectbounds, x, y, dwmousemsg, fsetalways.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerInactive_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetActivationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub OnInactiveMouseMove: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT, i32, i32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    OnInactiveMouseMove: usize,
    #[cfg(feature = "windef")]
    pub OnInactiveSetCursor: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT, i32, i32, u32, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    OnInactiveSetCursor: usize,
}
#[cfg(feature = "windef")]
pub trait IPointerInactive_Impl: windows_core::IUnknownImpl {
    fn GetActivationPolicy(&self) -> windows_core::Result<u32>;
    fn OnInactiveMouseMove(&self, prectbounds: *const super::windef::RECT, x: i32, y: i32, grfkeystate: u32) -> windows_core::Result<()>;
    fn OnInactiveSetCursor(&self, prectbounds: *const super::windef::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IPointerInactive_Vtbl {
    pub const fn new<Identity: IPointerInactive_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetActivationPolicy<Identity: IPointerInactive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpolicy: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn OnInactiveMouseMove<Identity: IPointerInactive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prectbounds: *const super::windef::RECT, x: i32, y: i32, grfkeystate: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPointerInactive_Impl::OnInactiveMouseMove(this, core::mem::transmute_copy(&prectbounds), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&grfkeystate)).into()
            }
        }
        unsafe extern "system" fn OnInactiveSetCursor<Identity: IPointerInactive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prectbounds: *const super::windef::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: windows_core::BOOL) -> windows_core::HRESULT {
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
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IPointerInactive {}
windows_core::imp::define_interface!(IPropertyBag2, IPropertyBag2_Vtbl, 0x22f55882_280b_11d0_a8a9_00a0c90c2004);
windows_core::imp::interface_hierarchy!(IPropertyBag2, windows_core::IUnknown);
impl IPropertyBag2 {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Read<P2>(&self, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: P2, pvarvalue: *mut super::oaidl::VARIANT, phrerror: *mut windows_core::HRESULT) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::oaidl::IErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), cproperties, ppropbag, perrlog.param().abi(), pvarvalue, phrerror as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Write(&self, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), cproperties, ppropbag, pvarvalue) }
    }
    pub unsafe fn CountProperties(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CountProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetPropertyInfo(&self, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyInfo)(windows_core::Interface::as_raw(self), iproperty, cproperties, ppropbag as _, pcproperties as _) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn LoadObject<P0, P2, P3>(&self, pstrname: P0, dwhint: u32, punkobject: P2, perrlog: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<super::oaidl::IErrorLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadObject)(windows_core::Interface::as_raw(self), pstrname.param().abi(), dwhint, punkobject.param().abi(), perrlog.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyBag2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PROPBAG2, *mut core::ffi::c_void, *mut super::oaidl::VARIANT, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    Read: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PROPBAG2, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    Write: usize,
    pub CountProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub GetPropertyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut PROPBAG2, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetPropertyInfo: usize,
    #[cfg(feature = "oaidl")]
    pub LoadObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    LoadObject: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyBag2_Impl: windows_core::IUnknownImpl {
    fn Read(&self, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: windows_core::Ref<super::oaidl::IErrorLog>, pvarvalue: *mut super::oaidl::VARIANT, phrerror: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn Write(&self, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn CountProperties(&self) -> windows_core::Result<u32>;
    fn GetPropertyInfo(&self, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> windows_core::Result<()>;
    fn LoadObject(&self, pstrname: &windows_core::PCWSTR, dwhint: u32, punkobject: windows_core::Ref<windows_core::IUnknown>, perrlog: windows_core::Ref<super::oaidl::IErrorLog>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyBag2_Vtbl {
    pub const fn new<Identity: IPropertyBag2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: *mut core::ffi::c_void, pvarvalue: *mut super::oaidl::VARIANT, phrerror: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag2_Impl::Read(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&perrlog), core::mem::transmute_copy(&pvarvalue), core::mem::transmute_copy(&phrerror)).into()
            }
        }
        unsafe extern "system" fn Write<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag2_Impl::Write(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn CountProperties<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcproperties: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetPropertyInfo<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag2_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&iproperty), core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppropbag), core::mem::transmute_copy(&pcproperties)).into()
            }
        }
        unsafe extern "system" fn LoadObject<Identity: IPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrname: windows_core::PCWSTR, dwhint: u32, punkobject: *mut core::ffi::c_void, perrlog: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyBag2_Impl::LoadObject(this, core::mem::transmute(&pstrname), core::mem::transmute_copy(&dwhint), core::mem::transmute_copy(&punkobject), core::mem::transmute_copy(&perrlog)).into()
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
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyBag2 {}
windows_core::imp::define_interface!(IPropertyNotifySink, IPropertyNotifySink_Vtbl, 0x9bfbbc02_eff1_101a_84ed_00aa00341d07);
windows_core::imp::interface_hierarchy!(IPropertyNotifySink, windows_core::IUnknown);
impl IPropertyNotifySink {
    #[cfg(feature = "oaidl")]
    pub unsafe fn OnChanged(&self, dispid: super::oaidl::DISPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnChanged)(windows_core::Interface::as_raw(self), dispid) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn OnRequestEdit(&self, dispid: super::oaidl::DISPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnRequestEdit)(windows_core::Interface::as_raw(self), dispid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub OnChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    OnChanged: usize,
    #[cfg(feature = "oaidl")]
    pub OnRequestEdit: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    OnRequestEdit: usize,
}
#[cfg(feature = "oaidl")]
pub trait IPropertyNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnChanged(&self, dispid: super::oaidl::DISPID) -> windows_core::Result<()>;
    fn OnRequestEdit(&self, dispid: super::oaidl::DISPID) -> windows_core::Result<()>;
}
#[cfg(feature = "oaidl")]
impl IPropertyNotifySink_Vtbl {
    pub const fn new<Identity: IPropertyNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnChanged<Identity: IPropertyNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::oaidl::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyNotifySink_Impl::OnChanged(this, core::mem::transmute_copy(&dispid)).into()
            }
        }
        unsafe extern "system" fn OnRequestEdit<Identity: IPropertyNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::oaidl::DISPID) -> windows_core::HRESULT {
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
#[cfg(feature = "oaidl")]
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
    #[cfg(feature = "windef")]
    pub unsafe fn Activate(&self, hwndparent: super::windef::HWND, prect: *const super::windef::RECT, bmodal: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), hwndparent, prect, bmodal.into()) }
    }
    pub unsafe fn Deactivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetPageInfo(&self, ppageinfo: *mut PROPPAGEINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPageInfo)(windows_core::Interface::as_raw(self), ppageinfo as _) }
    }
    pub unsafe fn SetObjects(&self, cobjects: u32, ppunk: *const Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetObjects)(windows_core::Interface::as_raw(self), cobjects, core::mem::transmute(ppunk)) }
    }
    pub unsafe fn Show(&self, ncmdshow: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), ncmdshow) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Move(&self, prect: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), prect) }
    }
    pub unsafe fn IsPageDirty(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsPageDirty)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Apply(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Help<P0>(&self, pszhelpdir: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Help)(windows_core::Interface::as_raw(self), pszhelpdir.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::winuser::MSG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TranslateAccelerator)(windows_core::Interface::as_raw(self), pmsg) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPageSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const super::windef::RECT, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Activate: usize,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetPageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPPAGEINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetPageInfo: usize,
    pub SetObjects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Move: usize,
    pub IsPageDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Help: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub TranslateAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::winuser::MSG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    TranslateAccelerator: usize,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub trait IPropertyPage_Impl: windows_core::IUnknownImpl {
    fn SetPageSite(&self, ppagesite: windows_core::Ref<IPropertyPageSite>) -> windows_core::Result<()>;
    fn Activate(&self, hwndparent: super::windef::HWND, prect: *const super::windef::RECT, bmodal: windows_core::BOOL) -> windows_core::Result<()>;
    fn Deactivate(&self) -> windows_core::Result<()>;
    fn GetPageInfo(&self, ppageinfo: *mut PROPPAGEINFO) -> windows_core::Result<()>;
    fn SetObjects(&self, cobjects: u32, ppunk: *const Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Show(&self, ncmdshow: u32) -> windows_core::Result<()>;
    fn Move(&self, prect: *const super::windef::RECT) -> windows_core::Result<()>;
    fn IsPageDirty(&self) -> windows_core::Result<()>;
    fn Apply(&self) -> windows_core::Result<()>;
    fn Help(&self, pszhelpdir: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn TranslateAccelerator(&self, pmsg: *const super::winuser::MSG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl IPropertyPage_Vtbl {
    pub const fn new<Identity: IPropertyPage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPageSite<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppagesite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::SetPageSite(this, core::mem::transmute_copy(&ppagesite)).into()
            }
        }
        unsafe extern "system" fn Activate<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::windef::HWND, prect: *const super::windef::RECT, bmodal: windows_core::BOOL) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetObjects<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjects: u32, ppunk: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::SetObjects(this, core::mem::transmute_copy(&cobjects), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn Show<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncmdshow: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::Show(this, core::mem::transmute_copy(&ncmdshow)).into()
            }
        }
        unsafe extern "system" fn Move<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::windef::RECT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Help<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhelpdir: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::Help(this, core::mem::transmute(&pszhelpdir)).into()
            }
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: IPropertyPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::winuser::MSG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPage_Impl::TranslateAccelerator(this, core::mem::transmute_copy(&pmsg)).into()
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
            TranslateAccelerator: TranslateAccelerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyPage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
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
    #[cfg(feature = "oaidl")]
    pub unsafe fn EditProperty(&self, dispid: super::oaidl::DISPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EditProperty)(windows_core::Interface::as_raw(self), dispid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPage2_Vtbl {
    pub base__: IPropertyPage_Vtbl,
    #[cfg(feature = "oaidl")]
    pub EditProperty: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    EditProperty: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "windef", feature = "winuser"))]
pub trait IPropertyPage2_Impl: IPropertyPage_Impl {
    fn EditProperty(&self, dispid: super::oaidl::DISPID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "windef", feature = "winuser"))]
impl IPropertyPage2_Vtbl {
    pub const fn new<Identity: IPropertyPage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EditProperty<Identity: IPropertyPage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::oaidl::DISPID) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "windef", feature = "winuser"))]
impl windows_core::RuntimeName for IPropertyPage2 {}
windows_core::imp::define_interface!(IPropertyPageSite, IPropertyPageSite_Vtbl, 0xb196b28c_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IPropertyPageSite, windows_core::IUnknown);
impl IPropertyPageSite {
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), dwflags) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetLocaleID(&self) -> windows_core::Result<super::winnt::LCID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocaleID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPageContainer(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPageContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::winuser::MSG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TranslateAccelerator)(windows_core::Interface::as_raw(self), pmsg) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPageSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub GetLocaleID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetLocaleID: usize,
    pub GetPageContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub TranslateAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::winuser::MSG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    TranslateAccelerator: usize,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub trait IPropertyPageSite_Impl: windows_core::IUnknownImpl {
    fn OnStatusChange(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetLocaleID(&self) -> windows_core::Result<super::winnt::LCID>;
    fn GetPageContainer(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn TranslateAccelerator(&self, pmsg: *const super::winuser::MSG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl IPropertyPageSite_Vtbl {
    pub const fn new<Identity: IPropertyPageSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStatusChange<Identity: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPageSite_Impl::OnStatusChange(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetLocaleID<Identity: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocaleid: *mut super::winnt::LCID) -> windows_core::HRESULT {
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
        unsafe extern "system" fn TranslateAccelerator<Identity: IPropertyPageSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::winuser::MSG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyPageSite_Impl::TranslateAccelerator(this, core::mem::transmute_copy(&pmsg)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStatusChange: OnStatusChange::<Identity, OFFSET>,
            GetLocaleID: GetLocaleID::<Identity, OFFSET>,
            GetPageContainer: GetPageContainer::<Identity, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyPageSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl windows_core::RuntimeName for IPropertyPageSite {}
windows_core::imp::define_interface!(IProvideClassInfo, IProvideClassInfo_Vtbl, 0xb196b283_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IProvideClassInfo, windows_core::IUnknown);
impl IProvideClassInfo {
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetClassInfo(&self) -> windows_core::Result<super::oaidl::ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClassInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideClassInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub GetClassInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetClassInfo: usize,
}
#[cfg(feature = "oaidl")]
pub trait IProvideClassInfo_Impl: windows_core::IUnknownImpl {
    fn GetClassInfo(&self) -> windows_core::Result<super::oaidl::ITypeInfo>;
}
#[cfg(feature = "oaidl")]
impl IProvideClassInfo_Vtbl {
    pub const fn new<Identity: IProvideClassInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassInfo<Identity: IProvideClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppti: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideClassInfo_Impl::GetClassInfo(this) {
                    Ok(ok__) => {
                        ppti.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClassInfo: GetClassInfo::<Identity, OFFSET> }
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
    pub unsafe fn GetGUID(&self, dwguidkind: u32) -> windows_core::Result<windows_core::GUID> {
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
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait IProvideClassInfo2_Impl: IProvideClassInfo_Impl {
    fn GetGUID(&self, dwguidkind: u32) -> windows_core::Result<windows_core::GUID>;
}
#[cfg(feature = "oaidl")]
impl IProvideClassInfo2_Vtbl {
    pub const fn new<Identity: IProvideClassInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGUID<Identity: IProvideClassInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwguidkind: u32, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
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
#[cfg(feature = "oaidl")]
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
    pub unsafe fn GetMultiTypeInfoCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMultiTypeInfoCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetInfoOfIndex(&self, iti: u32, dwflags: u32, ppticoclass: *mut Option<super::oaidl::ITypeInfo>, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut windows_core::GUID, piidsource: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInfoOfIndex)(windows_core::Interface::as_raw(self), iti, dwflags, core::mem::transmute(ppticoclass), pdwtiflags as _, pcdispidreserved as _, piidprimary as _, piidsource as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideMultipleClassInfo_Vtbl {
    pub base__: IProvideClassInfo2_Vtbl,
    pub GetMultiTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetInfoOfIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32, *mut windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetInfoOfIndex: usize,
}
#[cfg(feature = "oaidl")]
pub trait IProvideMultipleClassInfo_Impl: IProvideClassInfo2_Impl {
    fn GetMultiTypeInfoCount(&self) -> windows_core::Result<u32>;
    fn GetInfoOfIndex(&self, iti: u32, dwflags: u32, ppticoclass: windows_core::OutRef<super::oaidl::ITypeInfo>, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut windows_core::GUID, piidsource: *mut windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "oaidl")]
impl IProvideMultipleClassInfo_Vtbl {
    pub const fn new<Identity: IProvideMultipleClassInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMultiTypeInfoCount<Identity: IProvideMultipleClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcti: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetInfoOfIndex<Identity: IProvideMultipleClassInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iti: u32, dwflags: u32, ppticoclass: *mut *mut core::ffi::c_void, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut windows_core::GUID, piidsource: *mut windows_core::GUID) -> windows_core::HRESULT {
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
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IProvideMultipleClassInfo {}
windows_core::imp::define_interface!(IQuickActivate, IQuickActivate_Vtbl, 0xcf51ed10_62fe_11cf_bf86_00a0c9034836);
windows_core::imp::interface_hierarchy!(IQuickActivate, windows_core::IUnknown);
impl IQuickActivate {
    #[cfg(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "urlmon", feature = "windef"))]
    pub unsafe fn QuickActivate(&self, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QuickActivate)(windows_core::Interface::as_raw(self), pqacontainer, pqacontrol as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetContentExtent(&self, psizel: *const super::windef::SIZE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContentExtent)(windows_core::Interface::as_raw(self), psizel) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetContentExtent(&self) -> windows_core::Result<super::windef::SIZE> {
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
    #[cfg(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "urlmon", feature = "windef"))]
    pub QuickActivate: unsafe extern "system" fn(*mut core::ffi::c_void, *const QACONTAINER, *mut QACONTROL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "urlmon", feature = "windef")))]
    QuickActivate: usize,
    #[cfg(feature = "windef")]
    pub SetContentExtent: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::SIZE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetContentExtent: usize,
    #[cfg(feature = "windef")]
    pub GetContentExtent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::SIZE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetContentExtent: usize,
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "urlmon", feature = "windef"))]
pub trait IQuickActivate_Impl: windows_core::IUnknownImpl {
    fn QuickActivate(&self, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> windows_core::Result<()>;
    fn SetContentExtent(&self, psizel: *const super::windef::SIZE) -> windows_core::Result<()>;
    fn GetContentExtent(&self) -> windows_core::Result<super::windef::SIZE>;
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "urlmon", feature = "windef"))]
impl IQuickActivate_Vtbl {
    pub const fn new<Identity: IQuickActivate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QuickActivate<Identity: IQuickActivate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQuickActivate_Impl::QuickActivate(this, core::mem::transmute_copy(&pqacontainer), core::mem::transmute_copy(&pqacontrol)).into()
            }
        }
        unsafe extern "system" fn SetContentExtent<Identity: IQuickActivate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psizel: *const super::windef::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQuickActivate_Impl::SetContentExtent(this, core::mem::transmute_copy(&psizel)).into()
            }
        }
        unsafe extern "system" fn GetContentExtent<Identity: IQuickActivate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psizel: *mut super::windef::SIZE) -> windows_core::HRESULT {
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
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "urlmon", feature = "windef"))]
impl windows_core::RuntimeName for IQuickActivate {}
windows_core::imp::define_interface!(ISimpleFrameSite, ISimpleFrameSite_Vtbl, 0x742b0e01_14e6_101b_914e_00aa00300cab);
windows_core::imp::interface_hierarchy!(ISimpleFrameSite, windows_core::IUnknown);
impl ISimpleFrameSite {
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub unsafe fn PreMessageFilter(&self, hwnd: super::windef::HWND, msg: u32, wp: super::minwindef::WPARAM, lp: super::minwindef::LPARAM, plresult: *mut super::minwindef::LRESULT, pdwcookie: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PreMessageFilter)(windows_core::Interface::as_raw(self), hwnd, msg, wp, lp, plresult as _, pdwcookie as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub unsafe fn PostMessageFilter(&self, hwnd: super::windef::HWND, msg: u32, wp: super::minwindef::WPARAM, lp: super::minwindef::LPARAM, plresult: *mut super::minwindef::LRESULT, dwcookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PostMessageFilter)(windows_core::Interface::as_raw(self), hwnd, msg, wp, lp, plresult as _, dwcookie) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleFrameSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub PreMessageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut super::minwindef::LRESULT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef")))]
    PreMessageFilter: usize,
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub PostMessageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut super::minwindef::LRESULT, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef")))]
    PostMessageFilter: usize,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub trait ISimpleFrameSite_Impl: windows_core::IUnknownImpl {
    fn PreMessageFilter(&self, hwnd: super::windef::HWND, msg: u32, wp: super::minwindef::WPARAM, lp: super::minwindef::LPARAM, plresult: *mut super::minwindef::LRESULT, pdwcookie: *mut u32) -> windows_core::Result<()>;
    fn PostMessageFilter(&self, hwnd: super::windef::HWND, msg: u32, wp: super::minwindef::WPARAM, lp: super::minwindef::LPARAM, plresult: *mut super::minwindef::LRESULT, dwcookie: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl ISimpleFrameSite_Vtbl {
    pub const fn new<Identity: ISimpleFrameSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PreMessageFilter<Identity: ISimpleFrameSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, msg: u32, wp: super::minwindef::WPARAM, lp: super::minwindef::LPARAM, plresult: *mut super::minwindef::LRESULT, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimpleFrameSite_Impl::PreMessageFilter(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wp), core::mem::transmute_copy(&lp), core::mem::transmute_copy(&plresult), core::mem::transmute_copy(&pdwcookie)).into()
            }
        }
        unsafe extern "system" fn PostMessageFilter<Identity: ISimpleFrameSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, msg: u32, wp: super::minwindef::WPARAM, lp: super::minwindef::LPARAM, plresult: *mut super::minwindef::LRESULT, dwcookie: u32) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl windows_core::RuntimeName for ISimpleFrameSite {}
windows_core::imp::define_interface!(ISpecifyPropertyPages, ISpecifyPropertyPages_Vtbl, 0xb196b28b_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(ISpecifyPropertyPages, windows_core::IUnknown);
impl ISpecifyPropertyPages {
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
    pub GetPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CAUUID) -> windows_core::HRESULT,
}
pub trait ISpecifyPropertyPages_Impl: windows_core::IUnknownImpl {
    fn GetPages(&self) -> windows_core::Result<CAUUID>;
}
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
impl windows_core::RuntimeName for ISpecifyPropertyPages {}
#[cfg(feature = "oleidl")]
windows_core::imp::define_interface!(IViewObjectEx, IViewObjectEx_Vtbl, 0x3af24292_0c96_11ce_a0cf_00aa00600ab8);
#[cfg(feature = "oleidl")]
impl core::ops::Deref for IViewObjectEx {
    type Target = super::oleidl::IViewObject2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oleidl")]
windows_core::imp::interface_hierarchy!(IViewObjectEx, windows_core::IUnknown, super::oleidl::IViewObject, super::oleidl::IViewObject2);
#[cfg(feature = "oleidl")]
impl IViewObjectEx {
    #[cfg(feature = "windef")]
    pub unsafe fn GetRect(&self, dwaspect: u32) -> windows_core::Result<super::windef::RECTL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRect)(windows_core::Interface::as_raw(self), dwaspect, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetViewStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn QueryHitPoint(&self, dwaspect: u32, prectbounds: *const super::windef::RECT, ptlloc: super::windef::POINT, lclosehint: i32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryHitPoint)(windows_core::Interface::as_raw(self), dwaspect, prectbounds, ptlloc, lclosehint, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn QueryHitRect(&self, dwaspect: u32, prectbounds: *const super::windef::RECT, prectloc: *const super::windef::RECT, lclosehint: i32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryHitRect)(windows_core::Interface::as_raw(self), dwaspect, prectbounds, prectloc, lclosehint, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub unsafe fn GetNaturalExtent(&self, dwaspect: u32, lindex: i32, ptd: *const super::objidl::DVTARGETDEVICE, hictargetdev: super::windef::HDC, pextentinfo: *const DVEXTENTINFO) -> windows_core::Result<super::windef::SIZE> {
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
    pub base__: super::oleidl::IViewObject2_Vtbl,
    #[cfg(feature = "windef")]
    pub GetRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::windef::RECTL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetRect: usize,
    pub GetViewStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub QueryHitPoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::windef::RECT, super::windef::POINT, i32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    QueryHitPoint: usize,
    #[cfg(feature = "windef")]
    pub QueryHitRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::windef::RECT, *const super::windef::RECT, i32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    QueryHitRect: usize,
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub GetNaturalExtent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *const super::objidl::DVTARGETDEVICE, super::windef::HDC, *const DVEXTENTINFO, *mut super::windef::SIZE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "windef")))]
    GetNaturalExtent: usize,
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "windef", feature = "wingdi"))]
pub trait IViewObjectEx_Impl: super::oleidl::IViewObject2_Impl {
    fn GetRect(&self, dwaspect: u32) -> windows_core::Result<super::windef::RECTL>;
    fn GetViewStatus(&self) -> windows_core::Result<u32>;
    fn QueryHitPoint(&self, dwaspect: u32, prectbounds: *const super::windef::RECT, ptlloc: &super::windef::POINT, lclosehint: i32) -> windows_core::Result<u32>;
    fn QueryHitRect(&self, dwaspect: u32, prectbounds: *const super::windef::RECT, prectloc: *const super::windef::RECT, lclosehint: i32) -> windows_core::Result<u32>;
    fn GetNaturalExtent(&self, dwaspect: u32, lindex: i32, ptd: *const super::objidl::DVTARGETDEVICE, hictargetdev: super::windef::HDC, pextentinfo: *const DVEXTENTINFO) -> windows_core::Result<super::windef::SIZE>;
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "windef", feature = "wingdi"))]
impl IViewObjectEx_Vtbl {
    pub const fn new<Identity: IViewObjectEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRect<Identity: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, prect: *mut super::windef::RECTL) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetViewStatus<Identity: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn QueryHitPoint<Identity: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, prectbounds: *const super::windef::RECT, ptlloc: super::windef::POINT, lclosehint: i32, phitresult: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn QueryHitRect<Identity: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, prectbounds: *const super::windef::RECT, prectloc: *const super::windef::RECT, lclosehint: i32, phitresult: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetNaturalExtent<Identity: IViewObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, lindex: i32, ptd: *const super::objidl::DVTARGETDEVICE, hictargetdev: super::windef::HDC, pextentinfo: *const DVEXTENTINFO, psizel: *mut super::windef::SIZE) -> windows_core::HRESULT {
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
            base__: super::oleidl::IViewObject2_Vtbl::new::<Identity, OFFSET>(),
            GetRect: GetRect::<Identity, OFFSET>,
            GetViewStatus: GetViewStatus::<Identity, OFFSET>,
            QueryHitPoint: QueryHitPoint::<Identity, OFFSET>,
            QueryHitRect: QueryHitRect::<Identity, OFFSET>,
            GetNaturalExtent: GetNaturalExtent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObjectEx as windows_core::Interface>::IID || iid == &<super::oleidl::IViewObject as windows_core::Interface>::IID || iid == &<super::oleidl::IViewObject2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "windef", feature = "wingdi"))]
impl windows_core::RuntimeName for IViewObjectEx {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LICINFO {
    pub cbLicInfo: i32,
    pub fRuntimeKeyAvail: windows_core::BOOL,
    pub fLicVerified: windows_core::BOOL,
}
pub type LPCADWORD = *mut CADWORD;
pub type LPCALPOLESTR = *mut CALPOLESTR;
pub type LPCAUUID = *mut CAUUID;
pub type LPCONNECTDATA = *mut CONNECTDATA;
#[cfg(feature = "windef")]
pub type LPCONTROLINFO = *mut CONTROLINFO;
pub type LPLICINFO = *mut LICINFO;
pub type LPPOINTF = *mut POINTF;
#[cfg(feature = "windef")]
pub type LPPROPPAGEINFO = *mut PROPPAGEINFO;
#[cfg(feature = "wingdi")]
pub type LPTEXTMETRICOLE = *mut TEXTMETRICOLE;
pub const MULTICLASSINFO_GETIIDPRIMARY: u32 = 4;
pub const MULTICLASSINFO_GETIIDSOURCE: u32 = 8;
pub const MULTICLASSINFO_GETNUMRESERVEDDISPIDS: u32 = 2;
pub const MULTICLASSINFO_GETTYPEINFO: u32 = 1;
pub type OLEDCFLAGS = i32;
pub const OLEDC_NODRAW: OLEDCFLAGS = 1;
pub const OLEDC_OFFSCREEN: OLEDCFLAGS = 4;
pub const OLEDC_PAINTBKGND: OLEDCFLAGS = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OLE_COLOR(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OLE_HANDLE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OLE_XPOS_HIMETRIC(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OLE_XSIZE_HIMETRIC(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OLE_YPOS_HIMETRIC(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OLE_YSIZE_HIMETRIC(pub i32);
pub type PCONNECTDATA = *mut CONNECTDATA;
pub type PICTUREATTRIBUTES = i32;
pub const PICTURE_SCALABLE: PICTUREATTRIBUTES = 1;
pub const PICTURE_TRANSPARENT: PICTUREATTRIBUTES = 2;
pub type POINTERINACTIVE = i32;
pub const POINTERINACTIVE_ACTIVATEONDRAG: POINTERINACTIVE = 4;
pub const POINTERINACTIVE_ACTIVATEONENTRY: POINTERINACTIVE = 1;
pub const POINTERINACTIVE_DEACTIVATEONLEAVE: POINTERINACTIVE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINTF {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPBAG2 {
    pub dwType: u32,
    pub vt: super::wtypes::VARTYPE,
    pub cfType: super::wtypes::CLIPFORMAT,
    pub dwHint: u32,
    pub pstrName: windows_core::PWSTR,
    pub clsid: windows_core::GUID,
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
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPPAGEINFO {
    pub cb: u32,
    pub pszTitle: windows_core::PWSTR,
    pub size: super::windef::SIZE,
    pub pszDocString: windows_core::PWSTR,
    pub pszHelpFile: windows_core::PWSTR,
    pub dwHelpContext: u32,
}
pub type PROPPAGESTATUS = i32;
pub const PROPPAGESTATUS_CLEAN: PROPPAGESTATUS = 4;
pub const PROPPAGESTATUS_DIRTY: PROPPAGESTATUS = 1;
pub const PROPPAGESTATUS_VALIDATE: PROPPAGESTATUS = 2;
#[repr(C)]
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "urlmon", feature = "windef"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QACONTAINER {
    pub cbSize: u32,
    pub pClientSite: core::mem::ManuallyDrop<Option<super::oleidl::IOleClientSite>>,
    pub pAdviseSink: core::mem::ManuallyDrop<Option<IAdviseSinkEx>>,
    pub pPropertyNotifySink: core::mem::ManuallyDrop<Option<IPropertyNotifySink>>,
    pub pUnkEventSink: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwAmbientFlags: u32,
    pub colorFore: OLE_COLOR,
    pub colorBack: OLE_COLOR,
    pub pFont: core::mem::ManuallyDrop<Option<IFont>>,
    pub pUndoMgr: core::mem::ManuallyDrop<Option<IOleUndoManager>>,
    pub dwAppearance: u32,
    pub lcid: i32,
    pub hpal: super::windef::HPALETTE,
    pub pBindHost: core::mem::ManuallyDrop<Option<super::urlmon::IBindHost>>,
    pub pOleControlSite: core::mem::ManuallyDrop<Option<IOleControlSite>>,
    pub pServiceProvider: core::mem::ManuallyDrop<Option<super::servprov::IServiceProvider>>,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QACONTROL {
    pub cbSize: u32,
    pub dwMiscStatus: u32,
    pub dwViewStatus: u32,
    pub dwEventCookie: u32,
    pub dwPropNotifyCookie: u32,
    pub dwPointerActivationPolicy: u32,
}
pub type READYSTATE = i32;
pub const READYSTATE_COMPLETE: READYSTATE = 4;
pub const READYSTATE_INTERACTIVE: READYSTATE = 3;
pub const READYSTATE_LOADED: READYSTATE = 2;
pub const READYSTATE_LOADING: READYSTATE = 1;
pub const READYSTATE_UNINITIALIZED: READYSTATE = 0;
#[cfg(feature = "wingdi")]
pub type TEXTMETRICOLE = super::wingdi::TEXTMETRICW;
pub const TIFLAGS_EXTENDDISPATCHONLY: u32 = 1;
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
