#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut super::wtypes::VARIANT_BOOL,
}
#[cfg(feature = "wtypes")]
impl Default for CABOOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::BSTR,
}
impl Default for CABSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut super::wtypes::BSTRBLOB,
}
#[cfg(feature = "wtypes")]
impl Default for CABSTRBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAC {
    pub cElems: u32,
    pub pElems: *mut i8,
}
impl Default for CAC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut super::wtypes::CLIPDATA,
}
#[cfg(feature = "wtypes")]
impl Default for CACLIPDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut windows_core::GUID,
}
impl Default for CACLSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut super::wtypes::CY,
}
#[cfg(feature = "wtypes")]
impl Default for CACY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl Default for CADATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl Default for CADBL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut super::minwindef::FILETIME,
}
#[cfg(feature = "minwindef")]
impl Default for CAFILETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
impl Default for CAFLT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
impl Default for CAH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl Default for CAI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl Default for CAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::PSTR,
}
impl Default for CALPSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::PWSTR,
}
impl Default for CALPWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Default for CAPROPVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut super::wtypesbase::SCODE,
}
#[cfg(feature = "wtypesbase")]
impl Default for CASCODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
impl Default for CAUB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
impl Default for CAUH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
impl Default for CAUI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl Default for CAUL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(IEnumSTATPROPSETSTG, IEnumSTATPROPSETSTG_Vtbl, 0x0000013b_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumSTATPROPSETSTG, windows_core::IUnknown);
impl IEnumSTATPROPSETSTG {
    #[cfg(feature = "minwindef")]
    pub unsafe fn Next(&self, rgelt: &mut [STATPROPSETSTG], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), rgelt.as_mut_ptr(), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
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
pub struct IEnumSTATPROPSETSTG_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut STATPROPSETSTG, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "minwindef")]
pub trait IEnumSTATPROPSETSTG_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATPROPSETSTG>;
}
#[cfg(feature = "minwindef")]
impl IEnumSTATPROPSETSTG_Vtbl {
    pub const fn new<Identity: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSETSTG_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumSTATPROPSETSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
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
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for IEnumSTATPROPSETSTG {}
windows_core::imp::define_interface!(IEnumSTATPROPSTG, IEnumSTATPROPSTG_Vtbl, 0x00000139_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumSTATPROPSTG, windows_core::IUnknown);
impl IEnumSTATPROPSTG {
    #[cfg(feature = "wtypes")]
    pub unsafe fn Next(&self, rgelt: &mut [STATPROPSTG], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), rgelt.as_mut_ptr(), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
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
pub struct IEnumSTATPROPSTG_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut STATPROPSTG, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "wtypes")]
pub trait IEnumSTATPROPSTG_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATPROPSTG>;
}
#[cfg(feature = "wtypes")]
impl IEnumSTATPROPSTG_Vtbl {
    pub const fn new<Identity: IEnumSTATPROPSTG_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATPROPSTG_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumSTATPROPSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
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
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IEnumSTATPROPSTG {}
windows_core::imp::define_interface!(IPropertySetStorage, IPropertySetStorage_Vtbl, 0x0000013a_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IPropertySetStorage, windows_core::IUnknown);
impl IPropertySetStorage {
    pub unsafe fn Create(&self, rfmtid: *const windows_core::GUID, pclsid: *const windows_core::GUID, grfflags: u32, grfmode: u32) -> windows_core::Result<IPropertyStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), rfmtid, pclsid, grfflags, grfmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Open(&self, rfmtid: *const windows_core::GUID, grfmode: u32) -> windows_core::Result<IPropertyStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), rfmtid, grfmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self, rfmtid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), rfmtid) }
    }
    pub unsafe fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSETSTG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySetStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Enum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPropertySetStorage_Impl: windows_core::IUnknownImpl {
    fn Create(&self, rfmtid: *const windows_core::GUID, pclsid: *const windows_core::GUID, grfflags: u32, grfmode: u32) -> windows_core::Result<IPropertyStorage>;
    fn Open(&self, rfmtid: *const windows_core::GUID, grfmode: u32) -> windows_core::Result<IPropertyStorage>;
    fn Delete(&self, rfmtid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSETSTG>;
}
impl IPropertySetStorage_Vtbl {
    pub const fn new<Identity: IPropertySetStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rfmtid: *const windows_core::GUID, pclsid: *const windows_core::GUID, grfflags: u32, grfmode: u32, ppprstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Open<Identity: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rfmtid: *const windows_core::GUID, grfmode: u32, ppprstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Delete<Identity: IPropertySetStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rfmtid: *const windows_core::GUID) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IPropertySetStorage {}
windows_core::imp::define_interface!(IPropertyStorage, IPropertyStorage_Vtbl, 0x00000138_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IPropertyStorage, windows_core::IUnknown);
impl IPropertyStorage {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ReadMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC) -> windows_core::Result<PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn WriteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: super::wtypes::PROPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec, core::mem::transmute(rgpropvar), propidnamefirst) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn DeleteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const super::wtypes::PROPID) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadPropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const super::wtypes::PROPID, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WritePropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn DeletePropertyNames(&self, cpropid: u32, rgpropid: *const super::wtypes::PROPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeletePropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid) }
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags) }
    }
    pub unsafe fn Revert(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSTG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn SetTimes(&self, pctime: *const super::minwindef::FILETIME, patime: *const super::minwindef::FILETIME, pmtime: *const super::minwindef::FILETIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTimes)(windows_core::Interface::as_raw(self), pctime, patime, pmtime) }
    }
    pub unsafe fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClass)(windows_core::Interface::as_raw(self), clsid) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn Stat(&self, pstatpsstg: *mut STATPROPSETSTG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatpsstg as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub ReadMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PROPSPEC, *mut PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase")))]
    ReadMultiple: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub WriteMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PROPSPEC, *const PROPVARIANT, super::wtypes::PROPID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase")))]
    WriteMultiple: usize,
    #[cfg(feature = "wtypes")]
    pub DeleteMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PROPSPEC) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    DeleteMultiple: usize,
    #[cfg(feature = "wtypes")]
    pub ReadPropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::wtypes::PROPID, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ReadPropertyNames: usize,
    #[cfg(feature = "wtypes")]
    pub WritePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::wtypes::PROPID, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    WritePropertyNames: usize,
    #[cfg(feature = "wtypes")]
    pub DeletePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::wtypes::PROPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    DeletePropertyNames: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "minwindef")]
    pub SetTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::minwindef::FILETIME, *const super::minwindef::FILETIME, *const super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    SetTimes: usize,
    pub SetClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "minwindef")]
    pub Stat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STATPROPSETSTG) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    Stat: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyStorage_Impl: windows_core::IUnknownImpl {
    fn ReadMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC) -> windows_core::Result<PROPVARIANT>;
    fn WriteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: super::wtypes::PROPID) -> windows_core::Result<()>;
    fn DeleteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC) -> windows_core::Result<()>;
    fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const super::wtypes::PROPID) -> windows_core::Result<windows_core::PWSTR>;
    fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const super::wtypes::PROPID, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeletePropertyNames(&self, cpropid: u32, rgpropid: *const super::wtypes::PROPID) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn Enum(&self) -> windows_core::Result<IEnumSTATPROPSTG>;
    fn SetTimes(&self, pctime: *const super::minwindef::FILETIME, patime: *const super::minwindef::FILETIME, pmtime: *const super::minwindef::FILETIME) -> windows_core::Result<()>;
    fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Stat(&self, pstatpsstg: *mut STATPROPSETSTG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyStorage_Vtbl {
    pub const fn new<Identity: IPropertyStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadMultiple<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStorage_Impl::ReadMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec)) {
                    Ok(ok__) => {
                        rgpropvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteMultiple<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: super::wtypes::PROPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::WriteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar), core::mem::transmute_copy(&propidnamefirst)).into()
            }
        }
        unsafe extern "system" fn DeleteMultiple<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::DeleteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec)).into()
            }
        }
        unsafe extern "system" fn ReadPropertyNames<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const super::wtypes::PROPID, rglpwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStorage_Impl::ReadPropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid)) {
                    Ok(ok__) => {
                        rglpwstrname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WritePropertyNames<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const super::wtypes::PROPID, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::WritePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
            }
        }
        unsafe extern "system" fn DeletePropertyNames<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const super::wtypes::PROPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::DeletePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetTimes<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctime: *const super::minwindef::FILETIME, patime: *const super::minwindef::FILETIME, pmtime: *const super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStorage_Impl::SetTimes(this, core::mem::transmute_copy(&pctime), core::mem::transmute_copy(&patime), core::mem::transmute_copy(&pmtime)).into()
            }
        }
        unsafe extern "system" fn SetClass<Identity: IPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyStorage {}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub type LPPROPVARIANT = *mut PROPVARIANT;
#[cfg(feature = "objidlbase")]
pub type LPVERSIONEDSTREAM = *mut VERSIONEDSTREAM;
pub const PID_BEHAVIOR: u32 = 2147483651;
pub const PID_CODEPAGE: u32 = 1;
pub const PID_DICTIONARY: u32 = 0;
pub const PID_FIRST_NAME_DEFAULT: u32 = 4095;
pub const PID_FIRST_USABLE: u32 = 2;
pub const PID_ILLEGAL: u32 = 4294967295;
pub const PID_LOCALE: u32 = 2147483648;
pub const PID_MAX_READONLY: u32 = 3221225471;
pub const PID_MIN_READONLY: u32 = 2147483648;
pub const PID_MODIFY_TIME: u32 = 2147483649;
pub const PID_SECURITY: u32 = 2147483650;
pub const PROPSETFLAG_ANSI: u32 = 2;
pub const PROPSETFLAG_CASE_SENSITIVE: u32 = 8;
pub const PROPSETFLAG_DEFAULT: u32 = 0;
pub const PROPSETFLAG_NONSIMPLE: u32 = 1;
pub const PROPSETFLAG_UNBUFFERED: u32 = 4;
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295;
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: u32 = 1;
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy)]
pub struct PROPSPEC {
    pub ulKind: u32,
    pub Anonymous: PROPSPEC_0,
}
#[cfg(feature = "wtypes")]
impl Default for PROPSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy)]
pub union PROPSPEC_0 {
    pub propid: super::wtypes::PROPID,
    pub lpwstr: windows_core::PWSTR,
}
#[cfg(feature = "wtypes")]
impl Default for PROPSPEC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for PROPVARIANT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Default for PROPVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub union PROPVARIANT_0 {
    pub Anonymous: core::mem::ManuallyDrop<PROPVARIANT_0_0>,
    pub decVal: super::wtypes::DECIMAL,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for PROPVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Default for PROPVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub struct PROPVARIANT_0_0 {
    pub vt: super::wtypes::VARTYPE,
    pub wReserved1: PROPVAR_PAD1,
    pub wReserved2: PROPVAR_PAD2,
    pub wReserved3: PROPVAR_PAD3,
    pub Anonymous: PROPVARIANT_0_0_0,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for PROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Default for PROPVARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub union PROPVARIANT_0_0_0 {
    pub cVal: i8,
    pub bVal: u8,
    pub iVal: i16,
    pub uiVal: u16,
    pub lVal: i32,
    pub ulVal: u32,
    pub intVal: i32,
    pub uintVal: u32,
    pub hVal: i64,
    pub uhVal: u64,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::wtypes::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::wtypes::VARIANT_BOOL,
    pub scode: super::wtypesbase::SCODE,
    pub cyVal: super::wtypes::CY,
    pub date: f64,
    pub filetime: super::minwindef::FILETIME,
    pub puuid: *mut windows_core::GUID,
    pub pclipdata: *mut super::wtypes::CLIPDATA,
    pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrblobVal: super::wtypes::BSTRBLOB,
    pub blob: super::wtypesbase::BLOB,
    pub pszVal: windows_core::PSTR,
    pub pwszVal: windows_core::PWSTR,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<super::oaidl::IDispatch>>,
    pub pStream: core::mem::ManuallyDrop<Option<super::objidlbase::IStream>>,
    pub pStorage: core::mem::ManuallyDrop<Option<super::objidl::IStorage>>,
    pub pVersionedStream: LPVERSIONEDSTREAM,
    pub parray: super::oaidl::LPSAFEARRAY,
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
    pub pcVal: *mut i8,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub puiVal: *mut u16,
    pub plVal: *mut i32,
    pub pulVal: *mut u32,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::wtypes::VARIANT_BOOL,
    pub pdecVal: *mut super::wtypes::DECIMAL,
    pub pscode: *mut super::wtypesbase::SCODE,
    pub pcyVal: *mut super::wtypes::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut windows_core::BSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<super::oaidl::IDispatch>,
    pub pparray: *mut super::oaidl::LPSAFEARRAY,
    pub pvarVal: *mut PROPVARIANT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for PROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Default for PROPVARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PROPVAR_PAD1(pub u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PROPVAR_PAD2(pub u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PROPVAR_PAD3(pub u16);
pub const PRSPEC_INVALID: u32 = 4294967295;
pub const PRSPEC_LPWSTR: u32 = 0;
pub const PRSPEC_PROPID: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STATPROPSETSTG {
    pub fmtid: windows_core::GUID,
    pub clsid: windows_core::GUID,
    pub grfFlags: u32,
    pub mtime: super::minwindef::FILETIME,
    pub ctime: super::minwindef::FILETIME,
    pub atime: super::minwindef::FILETIME,
    pub dwOSVersion: u32,
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STATPROPSTG {
    pub lpwstrName: windows_core::PWSTR,
    pub propid: super::wtypes::PROPID,
    pub vt: super::wtypes::VARTYPE,
}
#[repr(C)]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VERSIONEDSTREAM {
    pub guidVersion: windows_core::GUID,
    pub pStream: core::mem::ManuallyDrop<Option<super::objidlbase::IStream>>,
}
