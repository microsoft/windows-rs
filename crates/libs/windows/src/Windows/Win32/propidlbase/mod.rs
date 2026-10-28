#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy, Default)]
pub struct CABOOL {
    pub cElems: super::ULONG,
    pub pElems: *mut super::VARIANT_BOOL,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CABSTR {
    pub cElems: super::ULONG,
    pub pElems: *mut windows_core::BSTR,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy, Default)]
pub struct CABSTRBLOB {
    pub cElems: super::ULONG,
    pub pElems: *mut super::BSTRBLOB,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CAC {
    pub cElems: super::ULONG,
    pub pElems: *mut super::CHAR,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy, Default)]
pub struct CACLIPDATA {
    pub cElems: super::ULONG,
    pub pElems: *mut super::CLIPDATA,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct CACLSID {
    pub cElems: super::ULONG,
    pub pElems: *mut super::CLSID,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy, Default)]
pub struct CACY {
    pub cElems: super::ULONG,
    pub pElems: *mut super::CY,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy, Default)]
pub struct CADATE {
    pub cElems: super::ULONG,
    pub pElems: *mut super::DATE,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct CADBL {
    pub cElems: super::ULONG,
    pub pElems: *mut super::DOUBLE,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct CAFILETIME {
    pub cElems: super::ULONG,
    pub pElems: *mut super::FILETIME,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct CAFLT {
    pub cElems: super::ULONG,
    pub pElems: *mut super::FLOAT,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CAH {
    pub cElems: super::ULONG,
    pub pElems: *mut super::LARGE_INTEGER,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CAI {
    pub cElems: super::ULONG,
    pub pElems: *mut super::SHORT,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CAL {
    pub cElems: super::ULONG,
    pub pElems: *mut super::LONG,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CALPSTR {
    pub cElems: super::ULONG,
    pub pElems: *mut windows_core::PSTR,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CALPWSTR {
    pub cElems: super::ULONG,
    pub pElems: *mut windows_core::PWSTR,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct CAPROPVARIANT {
    pub cElems: super::ULONG,
    pub pElems: *mut PROPVARIANT,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct CASCODE {
    pub cElems: super::ULONG,
    pub pElems: *mut super::SCODE,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CAUB {
    pub cElems: super::ULONG,
    pub pElems: *mut super::UCHAR,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CAUH {
    pub cElems: super::ULONG,
    pub pElems: *mut super::ULARGE_INTEGER,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CAUI {
    pub cElems: super::ULONG,
    pub pElems: *mut super::USHORT,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CAUL {
    pub cElems: super::ULONG,
    pub pElems: *mut super::ULONG,
}
windows_core::imp::define_interface!(IEnumSTATPROPSETSTG, IEnumSTATPROPSETSTG_Vtbl, 0x0000013b_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumSTATPROPSETSTG, windows_core::IUnknown);
impl IEnumSTATPROPSETSTG {
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut STATPROPSETSTG, pceltfetched: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
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
pub struct IEnumSTATPROPSETSTG_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut STATPROPSETSTG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "shared")))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
pub trait IEnumSTATPROPSETSTG_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATPROPSETSTG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
impl IEnumSTATPROPSETSTG_Vtbl {
    pub const fn new<Identity: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSETSTG_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSETSTG_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSETSTG_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumSTATPROPSETSTG_Impl::Clone(this) {
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
        iid == &<IEnumSTATPROPSETSTG as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IEnumSTATPROPSETSTG {}
windows_core::imp::define_interface!(IEnumSTATPROPSTG, IEnumSTATPROPSTG_Vtbl, 0x00000139_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumSTATPROPSTG, windows_core::IUnknown);
impl IEnumSTATPROPSTG {
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut STATPROPSTG, pceltfetched: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
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
pub struct IEnumSTATPROPSTG_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut STATPROPSTG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes", feature = "wtypesbase")))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
pub trait IEnumSTATPROPSTG_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: *mut STATPROPSTG, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATPROPSTG>;
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl IEnumSTATPROPSTG_Vtbl {
    pub const fn new<Identity: IEnumSTATPROPSTG_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut STATPROPSTG, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSTG_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSTG_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSTG_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumSTATPROPSTG_Impl::Clone(this) {
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
        iid == &<IEnumSTATPROPSTG as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IEnumSTATPROPSTG {}
windows_core::imp::define_interface!(IPropertySetStorage, IPropertySetStorage_Vtbl, 0x0000013a_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IPropertySetStorage, windows_core::IUnknown);
impl IPropertySetStorage {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn Create(&self, rfmtid: *const super::IID, pclsid: Option<*const super::CLSID>, grfflags: super::DWORD, grfmode: super::DWORD) -> windows_core::Result<IPropertyStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), rfmtid, pclsid.unwrap_or(core::mem::zeroed()) as _, grfflags, grfmode, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn Open(&self, rfmtid: *const super::IID, grfmode: super::DWORD) -> windows_core::Result<IPropertyStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), rfmtid, grfmode, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn Delete(&self, rfmtid: *const super::IID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), rfmtid) }
    }
    pub unsafe fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSETSTG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySetStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *const super::CLSID, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    Create: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    Open: usize,
    #[cfg(feature = "guiddef")]
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    Delete: usize,
    pub Enum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IPropertySetStorage_Impl: windows_core::IUnknownImpl {
    fn Create(&self, rfmtid: *const super::IID, pclsid: *const super::CLSID, grfflags: super::DWORD, grfmode: super::DWORD) -> windows_core::Result<IPropertyStorage>;
    fn Open(&self, rfmtid: *const super::IID, grfmode: super::DWORD) -> windows_core::Result<IPropertyStorage>;
    fn Delete(&self, rfmtid: *const super::IID) -> windows_core::Result<()>;
    fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSETSTG>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IPropertySetStorage_Vtbl {
    pub const fn new<Identity: IPropertySetStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rfmtid: *const super::IID, pclsid: *const super::CLSID, grfflags: super::DWORD, grfmode: super::DWORD, ppprstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertySetStorage_Impl::Create(this, core::mem::transmute_copy(&rfmtid), core::mem::transmute_copy(&pclsid), core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&grfmode)) {
                    Ok(ok__) => {
                        ppprstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Open<Identity: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rfmtid: *const super::IID, grfmode: super::DWORD, ppprstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertySetStorage_Impl::Open(this, core::mem::transmute_copy(&rfmtid), core::mem::transmute_copy(&grfmode)) {
                    Ok(ok__) => {
                        ppprstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rfmtid: *const super::IID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySetStorage_Impl::Delete(this, core::mem::transmute_copy(&rfmtid)).into()
            }
        }
        unsafe extern "system" fn Enum<Identity: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertySetStorage_Impl::Enum(this) {
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
            Create: Create::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Enum: Enum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertySetStorage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IPropertySetStorage {}
windows_core::imp::define_interface!(IPropertyStorage, IPropertyStorage_Vtbl, 0x00000138_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IPropertyStorage, windows_core::IUnknown);
impl IPropertyStorage {
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ReadMultiple(&self, cpspec: super::ULONG, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec, rgpropvar) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn WriteMultiple(&self, cpspec: super::ULONG, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: super::PROPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec, rgpropvar, propidnamefirst) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn DeleteMultiple(&self, rgpspec: &[PROPSPEC]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteMultiple)(windows_core::Interface::as_raw(self), rgpspec.len().try_into().unwrap(), rgpspec.as_ptr()) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ReadPropertyNames(&self, cpropid: super::ULONG, rgpropid: *const super::PROPID, rglpwstrname: *mut super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadPropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname as _) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn WritePropertyNames(&self, cpropid: super::ULONG, rgpropid: *const super::PROPID, rglpwstrname: *const super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WritePropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn DeletePropertyNames(&self, rgpropid: &[super::PROPID]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeletePropertyNames)(windows_core::Interface::as_raw(self), rgpropid.len().try_into().unwrap(), rgpropid.as_ptr()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Commit(&self, grfcommitflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags) }
    }
    pub unsafe fn Revert(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSTG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn SetTimes(&self, pctime: *const super::FILETIME, patime: *const super::FILETIME, pmtime: *const super::FILETIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTimes)(windows_core::Interface::as_raw(self), pctime, patime, pmtime) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn SetClass(&self, clsid: *const super::IID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClass)(windows_core::Interface::as_raw(self), clsid) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
    pub unsafe fn Stat(&self, pstatpsstg: *mut STATPROPSETSTG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatpsstg as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ReadMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const PROPSPEC, *mut PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ReadMultiple: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub WriteMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const PROPSPEC, *const PROPVARIANT, super::PROPID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    WriteMultiple: usize,
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub DeleteMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const PROPSPEC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes", feature = "wtypesbase")))]
    DeleteMultiple: usize,
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub ReadPropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::PROPID, *mut super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes", feature = "wtypesbase")))]
    ReadPropertyNames: usize,
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub WritePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::PROPID, *const super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes", feature = "wtypesbase")))]
    WritePropertyNames: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub DeletePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::PROPID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    DeletePropertyNames: usize,
    #[cfg(feature = "shared")]
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Commit: usize,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub SetTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::FILETIME, *const super::FILETIME, *const super::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    SetTimes: usize,
    #[cfg(feature = "guiddef")]
    pub SetClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    SetClass: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
    pub Stat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STATPROPSETSTG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "shared")))]
    Stat: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyStorage_Impl: windows_core::IUnknownImpl {
    fn ReadMultiple(&self, cpspec: super::ULONG, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> windows_core::Result<()>;
    fn WriteMultiple(&self, cpspec: super::ULONG, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: super::PROPID) -> windows_core::Result<()>;
    fn DeleteMultiple(&self, cpspec: super::ULONG, rgpspec: *const PROPSPEC) -> windows_core::Result<()>;
    fn ReadPropertyNames(&self, cpropid: super::ULONG, rgpropid: *const super::PROPID, rglpwstrname: *mut super::LPOLESTR) -> windows_core::Result<()>;
    fn WritePropertyNames(&self, cpropid: super::ULONG, rgpropid: *const super::PROPID, rglpwstrname: *const super::LPOLESTR) -> windows_core::Result<()>;
    fn DeletePropertyNames(&self, cpropid: super::ULONG, rgpropid: *const super::PROPID) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: super::DWORD) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSTG>;
    fn SetTimes(&self, pctime: *const super::FILETIME, patime: *const super::FILETIME, pmtime: *const super::FILETIME) -> windows_core::Result<()>;
    fn SetClass(&self, clsid: *const super::IID) -> windows_core::Result<()>;
    fn Stat(&self, pstatpsstg: *mut STATPROPSETSTG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyStorage_Vtbl {
    pub const fn new<Identity: IPropertyStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadMultiple<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: super::ULONG, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::ReadMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar)).into()
            }
        }
        unsafe extern "system" fn WriteMultiple<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: super::ULONG, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: super::PROPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::WriteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar), core::mem::transmute_copy(&propidnamefirst)).into()
            }
        }
        unsafe extern "system" fn DeleteMultiple<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: super::ULONG, rgpspec: *const PROPSPEC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::DeleteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec)).into()
            }
        }
        unsafe extern "system" fn ReadPropertyNames<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: super::ULONG, rgpropid: *const super::PROPID, rglpwstrname: *mut super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::ReadPropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
            }
        }
        unsafe extern "system" fn WritePropertyNames<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: super::ULONG, rgpropid: *const super::PROPID, rglpwstrname: *const super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::WritePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
            }
        }
        unsafe extern "system" fn DeletePropertyNames<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: super::ULONG, rgpropid: *const super::PROPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::DeletePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::Commit(this, core::mem::transmute_copy(&grfcommitflags)).into()
            }
        }
        unsafe extern "system" fn Revert<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::Revert(this).into()
            }
        }
        unsafe extern "system" fn Enum<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStorage_Impl::Enum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTimes<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctime: *const super::FILETIME, patime: *const super::FILETIME, pmtime: *const super::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::SetTimes(this, core::mem::transmute_copy(&pctime), core::mem::transmute_copy(&patime), core::mem::transmute_copy(&pmtime)).into()
            }
        }
        unsafe extern "system" fn SetClass<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const super::IID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::SetClass(this, core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn Stat<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatpsstg: *mut STATPROPSETSTG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::Stat(this, core::mem::transmute_copy(&pstatpsstg)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadMultiple: ReadMultiple::<Identity, OFFSET>,
            WriteMultiple: WriteMultiple::<Identity, OFFSET>,
            DeleteMultiple: DeleteMultiple::<Identity, OFFSET>,
            ReadPropertyNames: ReadPropertyNames::<Identity, OFFSET>,
            WritePropertyNames: WritePropertyNames::<Identity, OFFSET>,
            DeletePropertyNames: DeletePropertyNames::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Revert: Revert::<Identity, OFFSET>,
            Enum: Enum::<Identity, OFFSET>,
            SetTimes: SetTimes::<Identity, OFFSET>,
            SetClass: SetClass::<Identity, OFFSET>,
            Stat: Stat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyStorage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyStorage {}
pub type LPENUMSTATPROPSETSTG = IEnumSTATPROPSETSTG;
pub type LPENUMSTATPROPSTG = IEnumSTATPROPSTG;
pub type LPPROPERTYSETSTORAGE = IPropertySetStorage;
pub type LPPROPERTYSTORAGE = IPropertyStorage;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPPROPVARIANT = *mut tagPROPVARIANT;
#[cfg(feature = "objidlbase")]
pub type LPVERSIONEDSTREAM = *mut VERSIONEDSTREAM;
pub const PID_BEHAVIOR: u32 = 2147483651;
pub const PID_CODEPAGE: i32 = 1;
pub const PID_DICTIONARY: i32 = 0;
pub const PID_FIRST_NAME_DEFAULT: i32 = 4095;
pub const PID_FIRST_USABLE: i32 = 2;
pub const PID_ILLEGAL: u32 = 4294967295;
pub const PID_LOCALE: u32 = 2147483648;
pub const PID_MAX_READONLY: u32 = 3221225471;
pub const PID_MIN_READONLY: u32 = 2147483648;
pub const PID_MODIFY_TIME: u32 = 2147483649;
pub const PID_SECURITY: u32 = 2147483650;
pub const PROPSETFLAG_ANSI: i32 = 2;
pub const PROPSETFLAG_CASE_SENSITIVE: i32 = 8;
pub const PROPSETFLAG_DEFAULT: i32 = 0;
pub const PROPSETFLAG_NONSIMPLE: i32 = 1;
pub const PROPSETFLAG_UNBUFFERED: i32 = 4;
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295;
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: i32 = 1;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct PROPSPEC {
    pub ulKind: super::ULONG,
    pub Anonymous: PROPSPEC_0,
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for PROPSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union PROPSPEC_0 {
    pub propid: super::PROPID,
    pub lpwstr: super::LPOLESTR,
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for PROPSPEC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type PROPVARIANT = tagPROPVARIANT;
pub type PROPVAR_PAD1 = u16;
pub type PROPVAR_PAD2 = u16;
pub type PROPVAR_PAD3 = u16;
pub const PRSPEC_INVALID: u32 = 4294967295;
pub const PRSPEC_LPWSTR: i32 = 0;
pub const PRSPEC_PROPID: i32 = 1;
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct STATPROPSETSTG {
    pub fmtid: super::FMTID,
    pub clsid: super::CLSID,
    pub grfFlags: super::DWORD,
    pub mtime: super::FILETIME,
    pub ctime: super::FILETIME,
    pub atime: super::FILETIME,
    pub dwOSVersion: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct STATPROPSTG {
    pub lpwstrName: super::LPOLESTR,
    pub propid: super::PROPID,
    pub vt: super::VARTYPE,
}
#[repr(C, packed(8))]
#[cfg(feature = "objidlbase")]
#[derive(Default)]
pub struct VERSIONEDSTREAM {
    pub guidVersion: windows_core::GUID,
    pub pStream: core::mem::ManuallyDrop<Option<super::IStream>>,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub struct tagPROPVARIANT {
    pub Anonymous: tagPROPVARIANT_0,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for tagPROPVARIANT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for tagPROPVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub union tagPROPVARIANT_0 {
    pub Anonymous: core::mem::ManuallyDrop<tagPROPVARIANT_0_0>,
    pub decVal: super::DECIMAL,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for tagPROPVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for tagPROPVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub struct tagPROPVARIANT_0_0 {
    pub vt: super::VARTYPE,
    pub wReserved1: PROPVAR_PAD1,
    pub wReserved2: PROPVAR_PAD2,
    pub wReserved3: PROPVAR_PAD3,
    pub Anonymous: tagPROPVARIANT_0_0_0,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for tagPROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for tagPROPVARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub union tagPROPVARIANT_0_0_0 {
    pub cVal: super::CHAR,
    pub bVal: super::UCHAR,
    pub iVal: super::SHORT,
    pub uiVal: super::USHORT,
    pub lVal: super::LONG,
    pub ulVal: super::ULONG,
    pub intVal: super::INT,
    pub uintVal: super::UINT,
    pub hVal: super::LARGE_INTEGER,
    pub uhVal: super::ULARGE_INTEGER,
    pub fltVal: super::FLOAT,
    pub dblVal: super::DOUBLE,
    pub boolVal: super::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::VARIANT_BOOL,
    pub scode: super::SCODE,
    pub cyVal: super::CY,
    pub date: super::DATE,
    pub filetime: super::FILETIME,
    pub puuid: *mut super::CLSID,
    pub pclipdata: *mut super::CLIPDATA,
    pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrblobVal: super::BSTRBLOB,
    pub blob: super::BLOB,
    pub pszVal: windows_core::PSTR,
    pub pwszVal: windows_core::PWSTR,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<super::IDispatch>>,
    pub pStream: core::mem::ManuallyDrop<Option<super::IStream>>,
    pub pStorage: core::mem::ManuallyDrop<Option<super::IStorage>>,
    pub pVersionedStream: LPVERSIONEDSTREAM,
    pub parray: super::LPSAFEARRAY,
    pub cac: CAC,
    pub caub: CAUB,
    pub cai: CAI,
    pub caui: CAUI,
    pub cal: CAL,
    pub caul: CAUL,
    pub cah: CAH,
    pub cauh: CAUH,
    pub caflt: CAFLT,
    pub cadbl: CADBL,
    pub cabool: CABOOL,
    pub cascode: CASCODE,
    pub cacy: CACY,
    pub cadate: CADATE,
    pub cafiletime: CAFILETIME,
    pub cauuid: CACLSID,
    pub caclipdata: CACLIPDATA,
    pub cabstr: CABSTR,
    pub cabstrblob: CABSTRBLOB,
    pub calpstr: CALPSTR,
    pub calpwstr: CALPWSTR,
    pub capropvar: CAPROPVARIANT,
    pub pcVal: *mut super::CHAR,
    pub pbVal: *mut super::UCHAR,
    pub piVal: *mut super::SHORT,
    pub puiVal: *mut super::USHORT,
    pub plVal: *mut super::LONG,
    pub pulVal: *mut super::ULONG,
    pub pintVal: *mut super::INT,
    pub puintVal: *mut super::UINT,
    pub pfltVal: *mut super::FLOAT,
    pub pdblVal: *mut super::DOUBLE,
    pub pboolVal: *mut super::VARIANT_BOOL,
    pub pdecVal: *mut super::DECIMAL,
    pub pscode: *mut super::SCODE,
    pub pcyVal: *mut super::CY,
    pub pdate: *mut super::DATE,
    pub pbstrVal: *mut windows_core::BSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<super::IDispatch>,
    pub pparray: *mut super::LPSAFEARRAY,
    pub pvarVal: *mut PROPVARIANT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for tagPROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for tagPROPVARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
