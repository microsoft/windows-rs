pub const CVRC_COLUMN_MASK: u32 = 4095;
pub const CVRC_COLUMN_RESULT: u32 = 1;
pub const CVRC_COLUMN_SCHEMA: u32 = 0;
pub const CVRC_COLUMN_VALUE: u32 = 2;
pub const CVRC_TABLE_ATTRIBUTES: u32 = 16384;
pub const CVRC_TABLE_CRL: u32 = 20480;
pub const CVRC_TABLE_EXTENSIONS: u32 = 12288;
pub const CVRC_TABLE_MASK: u32 = 61440;
pub const CVRC_TABLE_REQCERT: u32 = 0;
pub const CVRC_TABLE_SHIFT: u32 = 12;
pub const CVR_SEEK_EQ: u32 = 1;
pub const CVR_SEEK_GE: u32 = 8;
pub const CVR_SEEK_GT: u32 = 16;
pub const CVR_SEEK_LE: u32 = 4;
pub const CVR_SEEK_LT: u32 = 2;
pub const CVR_SEEK_MASK: u32 = 255;
pub const CVR_SEEK_NODELTA: u32 = 4096;
pub const CVR_SEEK_NONE: u32 = 0;
pub const CVR_SORT_ASCEND: u32 = 1;
pub const CVR_SORT_DESCEND: u32 = 2;
pub const CVR_SORT_NONE: u32 = 0;
pub const CV_COLUMN_ATTRIBUTE_DEFAULT: i32 = -5;
pub const CV_COLUMN_CRL_DEFAULT: i32 = -6;
pub const CV_COLUMN_EXTENSION_DEFAULT: i32 = -4;
pub const CV_COLUMN_LOG_DEFAULT: i32 = -2;
pub const CV_COLUMN_LOG_FAILED_DEFAULT: i32 = -3;
pub const CV_COLUMN_LOG_REVOKED_DEFAULT: i32 = -7;
pub const CV_COLUMN_QUEUE_DEFAULT: i32 = -1;
pub const CV_OUT_BASE64: u32 = 1;
pub const CV_OUT_BASE64HEADER: u32 = 0;
pub const CV_OUT_BASE64REQUESTHEADER: u32 = 3;
pub const CV_OUT_BASE64X509CRLHEADER: u32 = 9;
pub const CV_OUT_BINARY: u32 = 2;
pub const CV_OUT_ENCODEMASK: u32 = 255;
pub const CV_OUT_HEX: u32 = 4;
pub const CV_OUT_HEXADDR: u32 = 10;
pub const CV_OUT_HEXASCII: u32 = 5;
pub const CV_OUT_HEXASCIIADDR: u32 = 11;
pub const CV_OUT_HEXRAW: u32 = 12;
pub const CV_OUT_NOCR: u32 = 2147483648;
pub const CV_OUT_NOCRLF: u32 = 1073741824;
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertView, ICertView_Vtbl, 0xc3fac344_1e84_11d1_9bd6_00c04fb683fa);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertView {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertView, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ICertView {
    pub unsafe fn OpenConnection(&self, strconfig: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OpenConnection)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig)) }
    }
    pub unsafe fn EnumCertViewColumn(&self, fresultcolumn: i32) -> windows_core::Result<IEnumCERTVIEWCOLUMN> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCertViewColumn)(windows_core::Interface::as_raw(self), fresultcolumn, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetColumnCount(&self, fresultcolumn: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnCount)(windows_core::Interface::as_raw(self), fresultcolumn, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetColumnIndex(&self, fresultcolumn: i32, strcolumnname: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnIndex)(windows_core::Interface::as_raw(self), fresultcolumn, core::mem::transmute_copy(strcolumnname), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetResultColumnCount(&self, cresultcolumn: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetResultColumnCount)(windows_core::Interface::as_raw(self), cresultcolumn) }
    }
    pub unsafe fn SetResultColumn(&self, columnindex: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetResultColumn)(windows_core::Interface::as_raw(self), columnindex) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetRestriction(&self, columnindex: i32, seekoperator: i32, sortorder: i32, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRestriction)(windows_core::Interface::as_raw(self), columnindex, seekoperator, sortorder, core::mem::transmute(pvarvalue)) }
    }
    pub unsafe fn OpenView(&self) -> windows_core::Result<IEnumCERTVIEWROW> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenView)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertView_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub OpenConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumCertViewColumn: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetColumnIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetResultColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetResultColumn: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetRestriction: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetRestriction: usize,
    pub OpenView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertView_Impl: super::oaidl::IDispatch_Impl {
    fn OpenConnection(&self, strconfig: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EnumCertViewColumn(&self, fresultcolumn: i32) -> windows_core::Result<IEnumCERTVIEWCOLUMN>;
    fn GetColumnCount(&self, fresultcolumn: i32) -> windows_core::Result<i32>;
    fn GetColumnIndex(&self, fresultcolumn: i32, strcolumnname: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn SetResultColumnCount(&self, cresultcolumn: i32) -> windows_core::Result<()>;
    fn SetResultColumn(&self, columnindex: i32) -> windows_core::Result<()>;
    fn SetRestriction(&self, columnindex: i32, seekoperator: i32, sortorder: i32, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn OpenView(&self) -> windows_core::Result<IEnumCERTVIEWROW>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertView_Vtbl {
    pub const fn new<Identity: ICertView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenConnection<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertView_Impl::OpenConnection(this, core::mem::transmute(&strconfig)).into()
            }
        }
        unsafe extern "system" fn EnumCertViewColumn<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fresultcolumn: i32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertView_Impl::EnumCertViewColumn(this, core::mem::transmute_copy(&fresultcolumn)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColumnCount<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fresultcolumn: i32, pccolumn: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertView_Impl::GetColumnCount(this, core::mem::transmute_copy(&fresultcolumn)) {
                    Ok(ok__) => {
                        pccolumn.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColumnIndex<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fresultcolumn: i32, strcolumnname: *mut core::ffi::c_void, pcolumnindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertView_Impl::GetColumnIndex(this, core::mem::transmute_copy(&fresultcolumn), core::mem::transmute(&strcolumnname)) {
                    Ok(ok__) => {
                        pcolumnindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetResultColumnCount<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cresultcolumn: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertView_Impl::SetResultColumnCount(this, core::mem::transmute_copy(&cresultcolumn)).into()
            }
        }
        unsafe extern "system" fn SetResultColumn<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, columnindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertView_Impl::SetResultColumn(this, core::mem::transmute_copy(&columnindex)).into()
            }
        }
        unsafe extern "system" fn SetRestriction<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, columnindex: i32, seekoperator: i32, sortorder: i32, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertView_Impl::SetRestriction(this, core::mem::transmute_copy(&columnindex), core::mem::transmute_copy(&seekoperator), core::mem::transmute_copy(&sortorder), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn OpenView<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertView_Impl::OpenView(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OpenConnection: OpenConnection::<Identity, OFFSET>,
            EnumCertViewColumn: EnumCertViewColumn::<Identity, OFFSET>,
            GetColumnCount: GetColumnCount::<Identity, OFFSET>,
            GetColumnIndex: GetColumnIndex::<Identity, OFFSET>,
            SetResultColumnCount: SetResultColumnCount::<Identity, OFFSET>,
            SetResultColumn: SetResultColumn::<Identity, OFFSET>,
            SetRestriction: SetRestriction::<Identity, OFFSET>,
            OpenView: OpenView::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertView as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertView {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertView2, ICertView2_Vtbl, 0xd594b282_8851_4b61_9c66_3edadf848863);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertView2 {
    type Target = ICertView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertView2, windows_core::IUnknown, super::oaidl::IDispatch, ICertView);
#[cfg(feature = "Win32_oaidl")]
impl ICertView2 {
    pub unsafe fn SetTable(&self, table: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTable)(windows_core::Interface::as_raw(self), table) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertView2_Vtbl {
    pub base__: ICertView_Vtbl,
    pub SetTable: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertView2_Impl: ICertView_Impl {
    fn SetTable(&self, table: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertView2_Vtbl {
    pub const fn new<Identity: ICertView2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTable<Identity: ICertView2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, table: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertView2_Impl::SetTable(this, core::mem::transmute_copy(&table)).into()
            }
        }
        Self { base__: ICertView_Vtbl::new::<Identity, OFFSET>(), SetTable: SetTable::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertView2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ICertView as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertView2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IEnumCERTVIEWATTRIBUTE, IEnumCERTVIEWATTRIBUTE_Vtbl, 0xe77db656_7653_11d1_9bde_00c04fb683fa);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IEnumCERTVIEWATTRIBUTE {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IEnumCERTVIEWATTRIBUTE, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IEnumCERTVIEWATTRIBUTE {
    pub unsafe fn Next(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Skip(&self, celt: i32) -> windows_core::HRESULT {
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
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCERTVIEWATTRIBUTE_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IEnumCERTVIEWATTRIBUTE_Impl: super::oaidl::IDispatch_Impl {
    fn Next(&self) -> windows_core::Result<i32>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Skip(&self, celt: i32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCERTVIEWATTRIBUTE>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IEnumCERTVIEWATTRIBUTE_Vtbl {
    pub const fn new<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWATTRIBUTE_Impl::Next(this) {
                    Ok(ok__) => {
                        pindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetName<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWATTRIBUTE_Impl::GetName(this) {
                    Ok(ok__) => {
                        pstrout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWATTRIBUTE_Impl::GetValue(this) {
                    Ok(ok__) => {
                        pstrout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCERTVIEWATTRIBUTE_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCERTVIEWATTRIBUTE_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWATTRIBUTE_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCERTVIEWATTRIBUTE as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IEnumCERTVIEWATTRIBUTE {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IEnumCERTVIEWCOLUMN, IEnumCERTVIEWCOLUMN_Vtbl, 0x9c735be2_57a5_11d1_9bdb_00c04fb683fa);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IEnumCERTVIEWCOLUMN {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IEnumCERTVIEWCOLUMN, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IEnumCERTVIEWCOLUMN {
    pub unsafe fn Next(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsIndexed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsIndexed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetValue(&self, flags: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Skip(&self, celt: i32) -> windows_core::HRESULT {
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
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCERTVIEWCOLUMN_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsIndexed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetValue: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IEnumCERTVIEWCOLUMN_Impl: super::oaidl::IDispatch_Impl {
    fn Next(&self) -> windows_core::Result<i32>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetType(&self) -> windows_core::Result<i32>;
    fn IsIndexed(&self) -> windows_core::Result<i32>;
    fn GetMaxLength(&self) -> windows_core::Result<i32>;
    fn GetValue(&self, flags: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn Skip(&self, celt: i32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCERTVIEWCOLUMN>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IEnumCERTVIEWCOLUMN_Vtbl {
    pub const fn new<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWCOLUMN_Impl::Next(this) {
                    Ok(ok__) => {
                        pindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetName<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWCOLUMN_Impl::GetName(this) {
                    Ok(ok__) => {
                        pstrout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWCOLUMN_Impl::GetDisplayName(this) {
                    Ok(ok__) => {
                        pstrout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWCOLUMN_Impl::GetType(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsIndexed<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexed: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWCOLUMN_Impl::IsIndexed(this) {
                    Ok(ok__) => {
                        pindexed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxLength<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWCOLUMN_Impl::GetMaxLength(this) {
                    Ok(ok__) => {
                        pmaxlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pvarvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWCOLUMN_Impl::GetValue(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCERTVIEWCOLUMN_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCERTVIEWCOLUMN_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWCOLUMN_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            IsIndexed: IsIndexed::<Identity, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCERTVIEWCOLUMN as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IEnumCERTVIEWCOLUMN {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IEnumCERTVIEWEXTENSION, IEnumCERTVIEWEXTENSION_Vtbl, 0xe7dd1466_7653_11d1_9bde_00c04fb683fa);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IEnumCERTVIEWEXTENSION {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IEnumCERTVIEWEXTENSION, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IEnumCERTVIEWEXTENSION {
    pub unsafe fn Next(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetValue(&self, r#type: i32, flags: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), r#type, flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Skip(&self, celt: i32) -> windows_core::HRESULT {
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
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCERTVIEWEXTENSION_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetValue: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IEnumCERTVIEWEXTENSION_Impl: super::oaidl::IDispatch_Impl {
    fn Next(&self) -> windows_core::Result<i32>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFlags(&self) -> windows_core::Result<i32>;
    fn GetValue(&self, r#type: i32, flags: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn Skip(&self, celt: i32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCERTVIEWEXTENSION>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IEnumCERTVIEWEXTENSION_Vtbl {
    pub const fn new<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWEXTENSION_Impl::Next(this) {
                    Ok(ok__) => {
                        pindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetName<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWEXTENSION_Impl::GetName(this) {
                    Ok(ok__) => {
                        pstrout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWEXTENSION_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, flags: i32, pvarvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWEXTENSION_Impl::GetValue(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCERTVIEWEXTENSION_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCERTVIEWEXTENSION_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWEXTENSION_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCERTVIEWEXTENSION as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IEnumCERTVIEWEXTENSION {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IEnumCERTVIEWROW, IEnumCERTVIEWROW_Vtbl, 0xd1157f4c_5af2_11d1_9bdc_00c04fb683fa);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IEnumCERTVIEWROW {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IEnumCERTVIEWROW, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IEnumCERTVIEWROW {
    pub unsafe fn Next(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumCertViewColumn(&self) -> windows_core::Result<IEnumCERTVIEWCOLUMN> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCertViewColumn)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumCertViewAttribute(&self, flags: i32) -> windows_core::Result<IEnumCERTVIEWATTRIBUTE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCertViewAttribute)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumCertViewExtension(&self, flags: i32) -> windows_core::Result<IEnumCERTVIEWEXTENSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCertViewExtension)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Skip(&self, celt: i32) -> windows_core::HRESULT {
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
    pub unsafe fn GetMaxIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCERTVIEWROW_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumCertViewColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumCertViewAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumCertViewExtension: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMaxIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IEnumCERTVIEWROW_Impl: super::oaidl::IDispatch_Impl {
    fn Next(&self) -> windows_core::Result<i32>;
    fn EnumCertViewColumn(&self) -> windows_core::Result<IEnumCERTVIEWCOLUMN>;
    fn EnumCertViewAttribute(&self, flags: i32) -> windows_core::Result<IEnumCERTVIEWATTRIBUTE>;
    fn EnumCertViewExtension(&self, flags: i32) -> windows_core::Result<IEnumCERTVIEWEXTENSION>;
    fn Skip(&self, celt: i32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCERTVIEWROW>;
    fn GetMaxIndex(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IEnumCERTVIEWROW_Vtbl {
    pub const fn new<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWROW_Impl::Next(this) {
                    Ok(ok__) => {
                        pindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumCertViewColumn<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWROW_Impl::EnumCertViewColumn(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumCertViewAttribute<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWROW_Impl::EnumCertViewAttribute(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumCertViewExtension<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWROW_Impl::EnumCertViewExtension(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCERTVIEWROW_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCERTVIEWROW_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWROW_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxIndex<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCERTVIEWROW_Impl::GetMaxIndex(this) {
                    Ok(ok__) => {
                        pindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            EnumCertViewColumn: EnumCertViewColumn::<Identity, OFFSET>,
            EnumCertViewAttribute: EnumCertViewAttribute::<Identity, OFFSET>,
            EnumCertViewExtension: EnumCertViewExtension::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetMaxIndex: GetMaxIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCERTVIEWROW as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IEnumCERTVIEWROW {}
