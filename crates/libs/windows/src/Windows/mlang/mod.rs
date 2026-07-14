pub const CMLangConvertCharset: windows_core::GUID = windows_core::GUID::from_u128(0xd66d6f99_cdaa_11d0_b822_00c04fc9b31f);
pub const CMLangString: windows_core::GUID = windows_core::GUID::from_u128(0xc04d65cf_b70d_11d0_b188_00aa0038c969);
pub const CMultiLanguage: windows_core::GUID = windows_core::GUID::from_u128(0x275c23e2_3747_11d0_9fea_00aa003f8646);
pub const CPIOD_FORCE_PROMPT: u32 = 2147483648;
pub const CPIOD_PEEK: u32 = 1073741824;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DetectEncodingInfo {
    pub nLangID: u32,
    pub nCodePage: u32,
    pub nDocPercent: i32,
    pub nConfidence: i32,
}
windows_core::imp::define_interface!(IEnumCodePage, IEnumCodePage_Vtbl, 0x275c23e3_3747_11d0_9fea_00aa003f8646);
windows_core::imp::interface_hierarchy!(IEnumCodePage, windows_core::IUnknown);
impl IEnumCodePage {
    pub unsafe fn Clone(&self, ppenum: Option<*const Option<Self>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), ppenum.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCodePage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MIMECPINFO, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumCodePage_Impl: windows_core::IUnknownImpl {
    fn Clone(&self, ppenum: *const Option<IEnumCodePage>) -> windows_core::Result<()>;
    fn Next(&self, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
}
impl IEnumCodePage_Vtbl {
    pub const fn new<Identity: IEnumCodePage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumCodePage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCodePage_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumCodePage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCodePage_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumCodePage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCodePage_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumCodePage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCodePage_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCodePage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumCodePage {}
windows_core::imp::define_interface!(IEnumRfc1766, IEnumRfc1766_Vtbl, 0x3dc39d1d_c030_11d0_b81b_00c04fc9b31f);
windows_core::imp::interface_hierarchy!(IEnumRfc1766, windows_core::IUnknown);
impl IEnumRfc1766 {
    pub unsafe fn Clone(&self, ppenum: Option<*const Option<Self>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), ppenum.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRfc1766_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut RFC1766INFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait IEnumRfc1766_Impl: windows_core::IUnknownImpl {
    fn Clone(&self, ppenum: *const Option<IEnumRfc1766>) -> windows_core::Result<()>;
    fn Next(&self, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IEnumRfc1766_Vtbl {
    pub const fn new<Identity: IEnumRfc1766_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumRfc1766_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRfc1766_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumRfc1766_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRfc1766_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumRfc1766_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRfc1766_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumRfc1766_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRfc1766_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumRfc1766 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IEnumRfc1766 {}
windows_core::imp::define_interface!(IEnumScript, IEnumScript_Vtbl, 0xae5f1430_388b_11d2_8380_00c04f8f5da1);
windows_core::imp::interface_hierarchy!(IEnumScript, windows_core::IUnknown);
impl IEnumScript {
    pub unsafe fn Clone(&self, ppenum: Option<*const Option<Self>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), ppenum.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumScript_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SCRIPTINFO, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumScript_Impl: windows_core::IUnknownImpl {
    fn Clone(&self, ppenum: *const Option<IEnumScript>) -> windows_core::Result<()>;
    fn Next(&self, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
}
impl IEnumScript_Vtbl {
    pub const fn new<Identity: IEnumScript_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumScript_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumScript_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumScript_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumScript_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumScript as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumScript {}
windows_core::imp::define_interface!(IMLangCodePages, IMLangCodePages_Vtbl, 0x359f3443_bd4a_11d0_b188_00aa0038c969);
windows_core::imp::interface_hierarchy!(IMLangCodePages, windows_core::IUnknown);
impl IMLangCodePages {
    pub unsafe fn GetCharCodePages(&self, chsrc: u16) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCharCodePages)(windows_core::Interface::as_raw(self), chsrc, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStrCodePages(&self, pszsrc: &[u16], dwprioritycodepages: u32, pdwcodepages: Option<*mut u32>, pcchcodepages: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStrCodePages)(windows_core::Interface::as_raw(self), pszsrc.as_ptr(), pszsrc.len().try_into().unwrap(), dwprioritycodepages, pdwcodepages.unwrap_or(core::mem::zeroed()) as _, pcchcodepages.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CodePageToCodePages(&self, ucodepage: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CodePageToCodePages)(windows_core::Interface::as_raw(self), ucodepage, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CodePagesToCodePage(&self, dwcodepages: u32, udefaultcodepage: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CodePagesToCodePage)(windows_core::Interface::as_raw(self), dwcodepages, udefaultcodepage, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangCodePages_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCharCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u32) -> windows_core::HRESULT,
    pub GetStrCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, u32, *mut u32, *mut i32) -> windows_core::HRESULT,
    pub CodePageToCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub CodePagesToCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IMLangCodePages_Impl: windows_core::IUnknownImpl {
    fn GetCharCodePages(&self, chsrc: u16) -> windows_core::Result<u32>;
    fn GetStrCodePages(&self, pszsrc: *const u16, cchsrc: i32, dwprioritycodepages: u32, pdwcodepages: *mut u32, pcchcodepages: *mut i32) -> windows_core::Result<()>;
    fn CodePageToCodePages(&self, ucodepage: u32) -> windows_core::Result<u32>;
    fn CodePagesToCodePage(&self, dwcodepages: u32, udefaultcodepage: u32) -> windows_core::Result<u32>;
}
impl IMLangCodePages_Vtbl {
    pub const fn new<Identity: IMLangCodePages_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCharCodePages<Identity: IMLangCodePages_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, chsrc: u16, pdwcodepages: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangCodePages_Impl::GetCharCodePages(this, core::mem::transmute_copy(&chsrc)) {
                    Ok(ok__) => {
                        pdwcodepages.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStrCodePages<Identity: IMLangCodePages_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsrc: *const u16, cchsrc: i32, dwprioritycodepages: u32, pdwcodepages: *mut u32, pcchcodepages: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangCodePages_Impl::GetStrCodePages(this, core::mem::transmute_copy(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&dwprioritycodepages), core::mem::transmute_copy(&pdwcodepages), core::mem::transmute_copy(&pcchcodepages)).into()
            }
        }
        unsafe extern "system" fn CodePageToCodePages<Identity: IMLangCodePages_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ucodepage: u32, pdwcodepages: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangCodePages_Impl::CodePageToCodePages(this, core::mem::transmute_copy(&ucodepage)) {
                    Ok(ok__) => {
                        pdwcodepages.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CodePagesToCodePage<Identity: IMLangCodePages_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcodepages: u32, udefaultcodepage: u32, pucodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangCodePages_Impl::CodePagesToCodePage(this, core::mem::transmute_copy(&dwcodepages), core::mem::transmute_copy(&udefaultcodepage)) {
                    Ok(ok__) => {
                        pucodepage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCharCodePages: GetCharCodePages::<Identity, OFFSET>,
            GetStrCodePages: GetStrCodePages::<Identity, OFFSET>,
            CodePageToCodePages: CodePageToCodePages::<Identity, OFFSET>,
            CodePagesToCodePage: CodePagesToCodePage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangCodePages as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLangCodePages {}
windows_core::imp::define_interface!(IMLangConvertCharset, IMLangConvertCharset_Vtbl, 0xd66d6f98_cdaa_11d0_b822_00c04fc9b31f);
windows_core::imp::interface_hierarchy!(IMLangConvertCharset, windows_core::IUnknown);
impl IMLangConvertCharset {
    pub unsafe fn Initialize(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), uisrccodepage, uidstcodepage, dwproperty) }
    }
    pub unsafe fn GetSourceCodePage(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceCodePage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDestinationCodePage(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDestinationCodePage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProperty(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DoConversion(&self, psrcstr: *const u8, pcsrcsize: Option<*mut u32>, pdststr: *mut u8, pcdstsize: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DoConversion)(windows_core::Interface::as_raw(self), psrcstr, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn DoConversionToUnicode(&self, psrcstr: *const i8, pcsrcsize: Option<*mut u32>, pdststr: *mut u16, pcdstsize: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DoConversionToUnicode)(windows_core::Interface::as_raw(self), psrcstr, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn DoConversionFromUnicode(&self, psrcstr: *const u16, pcsrcsize: Option<*mut u32>, pdststr: *mut i8, pcdstsize: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DoConversionFromUnicode)(windows_core::Interface::as_raw(self), psrcstr, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangConvertCharset_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub GetSourceCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDestinationCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DoConversion: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub DoConversionToUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *const i8, *mut u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub DoConversionFromUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut u32, *mut i8, *mut u32) -> windows_core::HRESULT,
}
pub trait IMLangConvertCharset_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<()>;
    fn GetSourceCodePage(&self) -> windows_core::Result<u32>;
    fn GetDestinationCodePage(&self) -> windows_core::Result<u32>;
    fn GetProperty(&self) -> windows_core::Result<u32>;
    fn DoConversion(&self, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn DoConversionToUnicode(&self, psrcstr: *const i8, pcsrcsize: *mut u32, pdststr: *mut u16, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn DoConversionFromUnicode(&self, psrcstr: *const u16, pcsrcsize: *mut u32, pdststr: *mut i8, pcdstsize: *mut u32) -> windows_core::Result<()>;
}
impl IMLangConvertCharset_Vtbl {
    pub const fn new<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangConvertCharset_Impl::Initialize(this, core::mem::transmute_copy(&uisrccodepage), core::mem::transmute_copy(&uidstcodepage), core::mem::transmute_copy(&dwproperty)).into()
            }
        }
        unsafe extern "system" fn GetSourceCodePage<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puisrccodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangConvertCharset_Impl::GetSourceCodePage(this) {
                    Ok(ok__) => {
                        puisrccodepage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDestinationCodePage<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puidstcodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangConvertCharset_Impl::GetDestinationCodePage(this) {
                    Ok(ok__) => {
                        puidstcodepage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwproperty: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangConvertCharset_Impl::GetProperty(this) {
                    Ok(ok__) => {
                        pdwproperty.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoConversion<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangConvertCharset_Impl::DoConversion(this, core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn DoConversionToUnicode<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcstr: *const i8, pcsrcsize: *mut u32, pdststr: *mut u16, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangConvertCharset_Impl::DoConversionToUnicode(this, core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn DoConversionFromUnicode<Identity: IMLangConvertCharset_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcstr: *const u16, pcsrcsize: *mut u32, pdststr: *mut i8, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangConvertCharset_Impl::DoConversionFromUnicode(this, core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetSourceCodePage: GetSourceCodePage::<Identity, OFFSET>,
            GetDestinationCodePage: GetDestinationCodePage::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            DoConversion: DoConversion::<Identity, OFFSET>,
            DoConversionToUnicode: DoConversionToUnicode::<Identity, OFFSET>,
            DoConversionFromUnicode: DoConversionFromUnicode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangConvertCharset as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLangConvertCharset {}
windows_core::imp::define_interface!(IMLangFontLink, IMLangFontLink_Vtbl, 0x359f3441_bd4a_11d0_b188_00aa0038c969);
impl core::ops::Deref for IMLangFontLink {
    type Target = IMLangCodePages;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangFontLink, windows_core::IUnknown, IMLangCodePages);
impl IMLangFontLink {
    #[cfg(feature = "windef")]
    pub unsafe fn GetFontCodePages(&self, hdc: super::windef::HDC, hfont: super::windef::HFONT, pdwcodepages: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontCodePages)(windows_core::Interface::as_raw(self), hdc, hfont, pdwcodepages.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn MapFont(&self, hdc: super::windef::HDC, dwcodepages: u32, hsrcfont: super::windef::HFONT, phdestfont: Option<*mut super::windef::HFONT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MapFont)(windows_core::Interface::as_raw(self), hdc, dwcodepages, hsrcfont, phdestfont.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ReleaseFont(&self, hfont: super::windef::HFONT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseFont)(windows_core::Interface::as_raw(self), hfont) }
    }
    pub unsafe fn ResetFontMapping(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetFontMapping)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangFontLink_Vtbl {
    pub base__: IMLangCodePages_Vtbl,
    #[cfg(feature = "windef")]
    pub GetFontCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, super::windef::HFONT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetFontCodePages: usize,
    #[cfg(feature = "windef")]
    pub MapFont: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, u32, super::windef::HFONT, *mut super::windef::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    MapFont: usize,
    #[cfg(feature = "windef")]
    pub ReleaseFont: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ReleaseFont: usize,
    pub ResetFontMapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IMLangFontLink_Impl: IMLangCodePages_Impl {
    fn GetFontCodePages(&self, hdc: super::windef::HDC, hfont: super::windef::HFONT, pdwcodepages: *mut u32) -> windows_core::Result<()>;
    fn MapFont(&self, hdc: super::windef::HDC, dwcodepages: u32, hsrcfont: super::windef::HFONT, phdestfont: *mut super::windef::HFONT) -> windows_core::Result<()>;
    fn ReleaseFont(&self, hfont: super::windef::HFONT) -> windows_core::Result<()>;
    fn ResetFontMapping(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IMLangFontLink_Vtbl {
    pub const fn new<Identity: IMLangFontLink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontCodePages<Identity: IMLangFontLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, hfont: super::windef::HFONT, pdwcodepages: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink_Impl::GetFontCodePages(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&hfont), core::mem::transmute_copy(&pdwcodepages)).into()
            }
        }
        unsafe extern "system" fn MapFont<Identity: IMLangFontLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, dwcodepages: u32, hsrcfont: super::windef::HFONT, phdestfont: *mut super::windef::HFONT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink_Impl::MapFont(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&dwcodepages), core::mem::transmute_copy(&hsrcfont), core::mem::transmute_copy(&phdestfont)).into()
            }
        }
        unsafe extern "system" fn ReleaseFont<Identity: IMLangFontLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfont: super::windef::HFONT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink_Impl::ReleaseFont(this, core::mem::transmute_copy(&hfont)).into()
            }
        }
        unsafe extern "system" fn ResetFontMapping<Identity: IMLangFontLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink_Impl::ResetFontMapping(this).into()
            }
        }
        Self {
            base__: IMLangCodePages_Vtbl::new::<Identity, OFFSET>(),
            GetFontCodePages: GetFontCodePages::<Identity, OFFSET>,
            MapFont: MapFont::<Identity, OFFSET>,
            ReleaseFont: ReleaseFont::<Identity, OFFSET>,
            ResetFontMapping: ResetFontMapping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangFontLink as windows_core::Interface>::IID || iid == &<IMLangCodePages as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IMLangFontLink {}
windows_core::imp::define_interface!(IMLangFontLink2, IMLangFontLink2_Vtbl, 0xdccfc162_2b38_11d2_b7ec_00c04f8f5d9a);
impl core::ops::Deref for IMLangFontLink2 {
    type Target = IMLangCodePages;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangFontLink2, windows_core::IUnknown, IMLangCodePages);
impl IMLangFontLink2 {
    #[cfg(feature = "windef")]
    pub unsafe fn GetFontCodePages(&self, hdc: super::windef::HDC, hfont: super::windef::HFONT, pdwcodepages: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontCodePages)(windows_core::Interface::as_raw(self), hdc, hfont, pdwcodepages.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ReleaseFont(&self, hfont: super::windef::HFONT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseFont)(windows_core::Interface::as_raw(self), hfont) }
    }
    pub unsafe fn ResetFontMapping(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetFontMapping)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn MapFont(&self, hdc: super::windef::HDC, dwcodepages: u32, chsrc: u16, pfont: Option<*mut super::windef::HFONT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MapFont)(windows_core::Interface::as_raw(self), hdc, dwcodepages, chsrc, pfont.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetFontUnicodeRanges(&self, hdc: super::windef::HDC, puiranges: *const u32, puranges: Option<*mut UNICODERANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontUnicodeRanges)(windows_core::Interface::as_raw(self), hdc, puiranges, puranges.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetScriptFontInfo(&self, sid: SCRIPT_ID, dwflags: u32, puifonts: *mut u32, pscriptfont: Option<*mut SCRIPTFONTINFO>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScriptFontInfo)(windows_core::Interface::as_raw(self), sid, dwflags, puifonts as _, pscriptfont.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CodePageToScriptID(&self, uicodepage: u32) -> windows_core::Result<SCRIPT_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CodePageToScriptID)(windows_core::Interface::as_raw(self), uicodepage, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangFontLink2_Vtbl {
    pub base__: IMLangCodePages_Vtbl,
    #[cfg(feature = "windef")]
    pub GetFontCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, super::windef::HFONT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetFontCodePages: usize,
    #[cfg(feature = "windef")]
    pub ReleaseFont: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ReleaseFont: usize,
    pub ResetFontMapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub MapFont: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, u32, u16, *mut super::windef::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    MapFont: usize,
    #[cfg(feature = "windef")]
    pub GetFontUnicodeRanges: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, *const u32, *mut UNICODERANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetFontUnicodeRanges: usize,
    pub GetScriptFontInfo: unsafe extern "system" fn(*mut core::ffi::c_void, SCRIPT_ID, u32, *mut u32, *mut SCRIPTFONTINFO) -> windows_core::HRESULT,
    pub CodePageToScriptID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SCRIPT_ID) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IMLangFontLink2_Impl: IMLangCodePages_Impl {
    fn GetFontCodePages(&self, hdc: super::windef::HDC, hfont: super::windef::HFONT, pdwcodepages: *mut u32) -> windows_core::Result<()>;
    fn ReleaseFont(&self, hfont: super::windef::HFONT) -> windows_core::Result<()>;
    fn ResetFontMapping(&self) -> windows_core::Result<()>;
    fn MapFont(&self, hdc: super::windef::HDC, dwcodepages: u32, chsrc: u16, pfont: *mut super::windef::HFONT) -> windows_core::Result<()>;
    fn GetFontUnicodeRanges(&self, hdc: super::windef::HDC, puiranges: *const u32, puranges: *mut UNICODERANGE) -> windows_core::Result<()>;
    fn GetScriptFontInfo(&self, sid: SCRIPT_ID, dwflags: u32, puifonts: *mut u32, pscriptfont: *mut SCRIPTFONTINFO) -> windows_core::Result<()>;
    fn CodePageToScriptID(&self, uicodepage: u32) -> windows_core::Result<SCRIPT_ID>;
}
#[cfg(feature = "windef")]
impl IMLangFontLink2_Vtbl {
    pub const fn new<Identity: IMLangFontLink2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontCodePages<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, hfont: super::windef::HFONT, pdwcodepages: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink2_Impl::GetFontCodePages(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&hfont), core::mem::transmute_copy(&pdwcodepages)).into()
            }
        }
        unsafe extern "system" fn ReleaseFont<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfont: super::windef::HFONT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink2_Impl::ReleaseFont(this, core::mem::transmute_copy(&hfont)).into()
            }
        }
        unsafe extern "system" fn ResetFontMapping<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink2_Impl::ResetFontMapping(this).into()
            }
        }
        unsafe extern "system" fn MapFont<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, dwcodepages: u32, chsrc: u16, pfont: *mut super::windef::HFONT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink2_Impl::MapFont(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&dwcodepages), core::mem::transmute_copy(&chsrc), core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn GetFontUnicodeRanges<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, puiranges: *const u32, puranges: *mut UNICODERANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink2_Impl::GetFontUnicodeRanges(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&puiranges), core::mem::transmute_copy(&puranges)).into()
            }
        }
        unsafe extern "system" fn GetScriptFontInfo<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sid: SCRIPT_ID, dwflags: u32, puifonts: *mut u32, pscriptfont: *mut SCRIPTFONTINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangFontLink2_Impl::GetScriptFontInfo(this, core::mem::transmute_copy(&sid), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&puifonts), core::mem::transmute_copy(&pscriptfont)).into()
            }
        }
        unsafe extern "system" fn CodePageToScriptID<Identity: IMLangFontLink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, psid: *mut SCRIPT_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLangFontLink2_Impl::CodePageToScriptID(this, core::mem::transmute_copy(&uicodepage)) {
                    Ok(ok__) => {
                        psid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMLangCodePages_Vtbl::new::<Identity, OFFSET>(),
            GetFontCodePages: GetFontCodePages::<Identity, OFFSET>,
            ReleaseFont: ReleaseFont::<Identity, OFFSET>,
            ResetFontMapping: ResetFontMapping::<Identity, OFFSET>,
            MapFont: MapFont::<Identity, OFFSET>,
            GetFontUnicodeRanges: GetFontUnicodeRanges::<Identity, OFFSET>,
            GetScriptFontInfo: GetScriptFontInfo::<Identity, OFFSET>,
            CodePageToScriptID: CodePageToScriptID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangFontLink2 as windows_core::Interface>::IID || iid == &<IMLangCodePages as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IMLangFontLink2 {}
windows_core::imp::define_interface!(IMLangLineBreakConsole, IMLangLineBreakConsole_Vtbl, 0xf5be2ee1_bfd7_11d0_b188_00aa0038c969);
windows_core::imp::interface_hierarchy!(IMLangLineBreakConsole, windows_core::IUnknown);
impl IMLangLineBreakConsole {
    pub unsafe fn BreakLineML<P0>(&self, psrcmlstr: P0, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: Option<*mut i32>, plskiplen: Option<*mut i32>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMLangString>,
    {
        unsafe { (windows_core::Interface::vtable(self).BreakLineML)(windows_core::Interface::as_raw(self), psrcmlstr.param().abi(), lsrcpos, lsrclen, cmincolumns, cmaxcolumns, pllinelen.unwrap_or(core::mem::zeroed()) as _, plskiplen.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn BreakLineW(&self, locale: super::winnt::LCID, pszsrc: &[u16], cmaxcolumns: i32, pcchline: Option<*mut i32>, pcchskip: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BreakLineW)(windows_core::Interface::as_raw(self), locale, pszsrc.as_ptr(), pszsrc.len().try_into().unwrap(), cmaxcolumns, pcchline.unwrap_or(core::mem::zeroed()) as _, pcchskip.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn BreakLineA(&self, locale: super::winnt::LCID, ucodepage: u32, pszsrc: &[i8], cmaxcolumns: i32, pcchline: Option<*mut i32>, pcchskip: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BreakLineA)(windows_core::Interface::as_raw(self), locale, ucodepage, pszsrc.as_ptr(), pszsrc.len().try_into().unwrap(), cmaxcolumns, pcchline.unwrap_or(core::mem::zeroed()) as _, pcchskip.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangLineBreakConsole_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BreakLineML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, i32, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub BreakLineW: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LCID, *const u16, i32, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    BreakLineW: usize,
    #[cfg(feature = "winnt")]
    pub BreakLineA: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LCID, u32, *const i8, i32, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    BreakLineA: usize,
}
#[cfg(feature = "winnt")]
pub trait IMLangLineBreakConsole_Impl: windows_core::IUnknownImpl {
    fn BreakLineML(&self, psrcmlstr: windows_core::Ref<IMLangString>, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: *mut i32, plskiplen: *mut i32) -> windows_core::Result<()>;
    fn BreakLineW(&self, locale: super::winnt::LCID, pszsrc: *const u16, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> windows_core::Result<()>;
    fn BreakLineA(&self, locale: super::winnt::LCID, ucodepage: u32, pszsrc: *const i8, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IMLangLineBreakConsole_Vtbl {
    pub const fn new<Identity: IMLangLineBreakConsole_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BreakLineML<Identity: IMLangLineBreakConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcmlstr: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: *mut i32, plskiplen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangLineBreakConsole_Impl::BreakLineML(this, core::mem::transmute_copy(&psrcmlstr), core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&cmincolumns), core::mem::transmute_copy(&cmaxcolumns), core::mem::transmute_copy(&pllinelen), core::mem::transmute_copy(&plskiplen)).into()
            }
        }
        unsafe extern "system" fn BreakLineW<Identity: IMLangLineBreakConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: super::winnt::LCID, pszsrc: *const u16, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangLineBreakConsole_Impl::BreakLineW(this, core::mem::transmute_copy(&locale), core::mem::transmute_copy(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&cmaxcolumns), core::mem::transmute_copy(&pcchline), core::mem::transmute_copy(&pcchskip)).into()
            }
        }
        unsafe extern "system" fn BreakLineA<Identity: IMLangLineBreakConsole_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: super::winnt::LCID, ucodepage: u32, pszsrc: *const i8, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangLineBreakConsole_Impl::BreakLineA(this, core::mem::transmute_copy(&locale), core::mem::transmute_copy(&ucodepage), core::mem::transmute_copy(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&cmaxcolumns), core::mem::transmute_copy(&pcchline), core::mem::transmute_copy(&pcchskip)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BreakLineML: BreakLineML::<Identity, OFFSET>,
            BreakLineW: BreakLineW::<Identity, OFFSET>,
            BreakLineA: BreakLineA::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangLineBreakConsole as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IMLangLineBreakConsole {}
windows_core::imp::define_interface!(IMLangString, IMLangString_Vtbl, 0xc04d65ce_b70d_11d0_b188_00aa0038c969);
windows_core::imp::interface_hierarchy!(IMLangString, windows_core::IUnknown);
impl IMLangString {
    pub unsafe fn Sync(&self, fnoaccess: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Sync)(windows_core::Interface::as_raw(self), fnoaccess.into()) }
    }
    pub unsafe fn GetLength(&self, pllen: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self), pllen.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetMLStr<P2>(&self, ldestpos: i32, ldestlen: i32, psrcmlstr: P2, lsrcpos: i32, lsrclen: i32) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMLStr)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, psrcmlstr.param().abi(), lsrcpos, lsrclen) }
    }
    pub unsafe fn GetMLStr<P2>(&self, lsrcpos: i32, lsrclen: i32, punkouter: P2, dwclscontext: u32, piid: *const windows_core::GUID, ppdestmlstr: *mut Option<windows_core::IUnknown>, pldestpos: Option<*mut i32>, pldestlen: Option<*mut i32>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMLStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, punkouter.param().abi(), dwclscontext, piid, core::mem::transmute(ppdestmlstr), pldestpos.unwrap_or(core::mem::zeroed()) as _, pldestlen.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangString_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Sync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMLStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetMLStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
}
pub trait IMLangString_Impl: windows_core::IUnknownImpl {
    fn Sync(&self, fnoaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLength(&self, pllen: *mut i32) -> windows_core::Result<()>;
    fn SetMLStr(&self, ldestpos: i32, ldestlen: i32, psrcmlstr: windows_core::Ref<windows_core::IUnknown>, lsrcpos: i32, lsrclen: i32) -> windows_core::Result<()>;
    fn GetMLStr(&self, lsrcpos: i32, lsrclen: i32, punkouter: windows_core::Ref<windows_core::IUnknown>, dwclscontext: u32, piid: *const windows_core::GUID, ppdestmlstr: windows_core::OutRef<windows_core::IUnknown>, pldestpos: *mut i32, pldestlen: *mut i32) -> windows_core::Result<()>;
}
impl IMLangString_Vtbl {
    pub const fn new<Identity: IMLangString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Sync<Identity: IMLangString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnoaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangString_Impl::Sync(this, core::mem::transmute_copy(&fnoaccess)).into()
            }
        }
        unsafe extern "system" fn GetLength<Identity: IMLangString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangString_Impl::GetLength(this, core::mem::transmute_copy(&pllen)).into()
            }
        }
        unsafe extern "system" fn SetMLStr<Identity: IMLangString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcmlstr: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangString_Impl::SetMLStr(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&psrcmlstr), core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen)).into()
            }
        }
        unsafe extern "system" fn GetMLStr<Identity: IMLangString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, punkouter: *mut core::ffi::c_void, dwclscontext: u32, piid: *const windows_core::GUID, ppdestmlstr: *mut *mut core::ffi::c_void, pldestpos: *mut i32, pldestlen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangString_Impl::GetMLStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&piid), core::mem::transmute_copy(&ppdestmlstr), core::mem::transmute_copy(&pldestpos), core::mem::transmute_copy(&pldestlen)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Sync: Sync::<Identity, OFFSET>,
            GetLength: GetLength::<Identity, OFFSET>,
            SetMLStr: SetMLStr::<Identity, OFFSET>,
            GetMLStr: GetMLStr::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangString as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLangString {}
windows_core::imp::define_interface!(IMLangStringAStr, IMLangStringAStr_Vtbl, 0xc04d65d2_b70d_11d0_b188_00aa0038c969);
impl core::ops::Deref for IMLangStringAStr {
    type Target = IMLangString;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangStringAStr, windows_core::IUnknown, IMLangString);
impl IMLangStringAStr {
    pub unsafe fn SetAStr(&self, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: &[u8], pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAStr)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, ucodepage, core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetStrBufA<P3>(&self, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: P3, pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IMLangStringBufA>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStrBufA)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, ucodepage, psrcbuf.param().abi(), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetAStr(&self, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: Option<*const u32>, pszdest: Option<&mut [u8]>, pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, ucodepagein, pucodepageout.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszdest.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), pszdest.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetStrBufA(&self, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: Option<*mut u32>, ppdestbuf: *mut Option<IMLangStringBufA>, pldestlen: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStrBufA)(windows_core::Interface::as_raw(self), lsrcpos, lsrcmaxlen, pudestcodepage.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppdestbuf), pldestlen.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn LockAStr(&self, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: Option<*mut u32>, ppszdest: *mut windows_core::PSTR, pcchdest: Option<*mut i32>, pldestlen: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockAStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, lflags, ucodepagein, cchrequest, pucodepageout.unwrap_or(core::mem::zeroed()) as _, ppszdest as _, pcchdest.unwrap_or(core::mem::zeroed()) as _, pldestlen.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn UnlockAStr(&self, pszsrc: &[u8], pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockAStr)(windows_core::Interface::as_raw(self), core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SetLocale(&self, ldestpos: i32, ldestlen: i32, locale: super::winnt::LCID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLocale)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, locale) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetLocale(&self, lsrcpos: i32, lsrcmaxlen: i32, plocale: Option<*mut super::winnt::LCID>, pllocalepos: Option<*mut i32>, pllocalelen: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLocale)(windows_core::Interface::as_raw(self), lsrcpos, lsrcmaxlen, plocale.unwrap_or(core::mem::zeroed()) as _, pllocalepos.unwrap_or(core::mem::zeroed()) as _, pllocalelen.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangStringAStr_Vtbl {
    pub base__: IMLangString_Vtbl,
    pub SetAStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, windows_core::PCSTR, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetStrBufA: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetAStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *const u32, windows_core::PSTR, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetStrBufA: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut u32, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LockAStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, u32, i32, *mut u32, *mut windows_core::PSTR, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub UnlockAStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub SetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, super::winnt::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetLocale: usize,
    #[cfg(feature = "winnt")]
    pub GetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut super::winnt::LCID, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetLocale: usize,
}
#[cfg(feature = "winnt")]
pub trait IMLangStringAStr_Impl: IMLangString_Impl {
    fn SetAStr(&self, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: &windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn SetStrBufA(&self, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: windows_core::Ref<IMLangStringBufA>, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn GetAStr(&self, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: *const u32, pszdest: windows_core::PSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn GetStrBufA(&self, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: *mut u32, ppdestbuf: windows_core::OutRef<IMLangStringBufA>, pldestlen: *mut i32) -> windows_core::Result<()>;
    fn LockAStr(&self, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: *mut u32, ppszdest: *mut windows_core::PSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> windows_core::Result<()>;
    fn UnlockAStr(&self, pszsrc: &windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn SetLocale(&self, ldestpos: i32, ldestlen: i32, locale: super::winnt::LCID) -> windows_core::Result<()>;
    fn GetLocale(&self, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut super::winnt::LCID, pllocalepos: *mut i32, pllocalelen: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IMLangStringAStr_Vtbl {
    pub const fn new<Identity: IMLangStringAStr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAStr<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::SetAStr(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&ucodepage), core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn SetStrBufA<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: *mut core::ffi::c_void, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::SetStrBufA(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&ucodepage), core::mem::transmute_copy(&psrcbuf), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn GetAStr<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: *const u32, pszdest: windows_core::PSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::GetAStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&ucodepagein), core::mem::transmute_copy(&pucodepageout), core::mem::transmute_copy(&pszdest), core::mem::transmute_copy(&cchdest), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn GetStrBufA<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: *mut u32, ppdestbuf: *mut *mut core::ffi::c_void, pldestlen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::GetStrBufA(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrcmaxlen), core::mem::transmute_copy(&pudestcodepage), core::mem::transmute_copy(&ppdestbuf), core::mem::transmute_copy(&pldestlen)).into()
            }
        }
        unsafe extern "system" fn LockAStr<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: *mut u32, ppszdest: *mut windows_core::PSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::LockAStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&ucodepagein), core::mem::transmute_copy(&cchrequest), core::mem::transmute_copy(&pucodepageout), core::mem::transmute_copy(&ppszdest), core::mem::transmute_copy(&pcchdest), core::mem::transmute_copy(&pldestlen)).into()
            }
        }
        unsafe extern "system" fn UnlockAStr<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsrc: windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::UnlockAStr(this, core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn SetLocale<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: super::winnt::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::SetLocale(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&locale)).into()
            }
        }
        unsafe extern "system" fn GetLocale<Identity: IMLangStringAStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut super::winnt::LCID, pllocalepos: *mut i32, pllocalelen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringAStr_Impl::GetLocale(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrcmaxlen), core::mem::transmute_copy(&plocale), core::mem::transmute_copy(&pllocalepos), core::mem::transmute_copy(&pllocalelen)).into()
            }
        }
        Self {
            base__: IMLangString_Vtbl::new::<Identity, OFFSET>(),
            SetAStr: SetAStr::<Identity, OFFSET>,
            SetStrBufA: SetStrBufA::<Identity, OFFSET>,
            GetAStr: GetAStr::<Identity, OFFSET>,
            GetStrBufA: GetStrBufA::<Identity, OFFSET>,
            LockAStr: LockAStr::<Identity, OFFSET>,
            UnlockAStr: UnlockAStr::<Identity, OFFSET>,
            SetLocale: SetLocale::<Identity, OFFSET>,
            GetLocale: GetLocale::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangStringAStr as windows_core::Interface>::IID || iid == &<IMLangString as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IMLangStringAStr {}
windows_core::imp::define_interface!(IMLangStringBufA, IMLangStringBufA_Vtbl, 0xd24acd23_ba72_11d0_b188_00aa0038c969);
windows_core::imp::interface_hierarchy!(IMLangStringBufA, windows_core::IUnknown);
impl IMLangStringBufA {
    pub unsafe fn GetStatus(&self, plflags: Option<*mut i32>, pcchbuf: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), plflags.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn LockBuf(&self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut i8, pcchbuf: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockBuf)(windows_core::Interface::as_raw(self), cchoffset, cchmaxlock, ppszbuf as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn UnlockBuf(&self, pszbuf: *const i8, cchoffset: i32, cchwrite: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockBuf)(windows_core::Interface::as_raw(self), pszbuf, cchoffset, cchwrite) }
    }
    pub unsafe fn Insert(&self, cchoffset: i32, cchmaxinsert: i32, pcchactual: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), cchoffset, cchmaxinsert, pcchactual.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Delete(&self, cchoffset: i32, cchdelete: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), cchoffset, cchdelete) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangStringBufA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub LockBuf: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut i8, *mut i32) -> windows_core::HRESULT,
    pub UnlockBuf: unsafe extern "system" fn(*mut core::ffi::c_void, *const i8, i32, i32) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
pub trait IMLangStringBufA_Impl: windows_core::IUnknownImpl {
    fn GetStatus(&self, plflags: *mut i32, pcchbuf: *mut i32) -> windows_core::Result<()>;
    fn LockBuf(&self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut i8, pcchbuf: *mut i32) -> windows_core::Result<()>;
    fn UnlockBuf(&self, pszbuf: *const i8, cchoffset: i32, cchwrite: i32) -> windows_core::Result<()>;
    fn Insert(&self, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> windows_core::Result<()>;
    fn Delete(&self, cchoffset: i32, cchdelete: i32) -> windows_core::Result<()>;
}
impl IMLangStringBufA_Vtbl {
    pub const fn new<Identity: IMLangStringBufA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatus<Identity: IMLangStringBufA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufA_Impl::GetStatus(this, core::mem::transmute_copy(&plflags), core::mem::transmute_copy(&pcchbuf)).into()
            }
        }
        unsafe extern "system" fn LockBuf<Identity: IMLangStringBufA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut i8, pcchbuf: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufA_Impl::LockBuf(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchmaxlock), core::mem::transmute_copy(&ppszbuf), core::mem::transmute_copy(&pcchbuf)).into()
            }
        }
        unsafe extern "system" fn UnlockBuf<Identity: IMLangStringBufA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbuf: *const i8, cchoffset: i32, cchwrite: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufA_Impl::UnlockBuf(this, core::mem::transmute_copy(&pszbuf), core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchwrite)).into()
            }
        }
        unsafe extern "system" fn Insert<Identity: IMLangStringBufA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufA_Impl::Insert(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchmaxinsert), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IMLangStringBufA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufA_Impl::Delete(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchdelete)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            LockBuf: LockBuf::<Identity, OFFSET>,
            UnlockBuf: UnlockBuf::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangStringBufA as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLangStringBufA {}
windows_core::imp::define_interface!(IMLangStringBufW, IMLangStringBufW_Vtbl, 0xd24acd21_ba72_11d0_b188_00aa0038c969);
windows_core::imp::interface_hierarchy!(IMLangStringBufW, windows_core::IUnknown);
impl IMLangStringBufW {
    pub unsafe fn GetStatus(&self, plflags: Option<*mut i32>, pcchbuf: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), plflags.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn LockBuf(&self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockBuf)(windows_core::Interface::as_raw(self), cchoffset, cchmaxlock, ppszbuf as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn UnlockBuf(&self, pszbuf: *const u16, cchoffset: i32, cchwrite: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockBuf)(windows_core::Interface::as_raw(self), pszbuf, cchoffset, cchwrite) }
    }
    pub unsafe fn Insert(&self, cchoffset: i32, cchmaxinsert: i32, pcchactual: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), cchoffset, cchmaxinsert, pcchactual.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Delete(&self, cchoffset: i32, cchdelete: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), cchoffset, cchdelete) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangStringBufW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub LockBuf: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut u16, *mut i32) -> windows_core::HRESULT,
    pub UnlockBuf: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, i32) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
pub trait IMLangStringBufW_Impl: windows_core::IUnknownImpl {
    fn GetStatus(&self, plflags: *mut i32, pcchbuf: *mut i32) -> windows_core::Result<()>;
    fn LockBuf(&self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: *mut i32) -> windows_core::Result<()>;
    fn UnlockBuf(&self, pszbuf: *const u16, cchoffset: i32, cchwrite: i32) -> windows_core::Result<()>;
    fn Insert(&self, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> windows_core::Result<()>;
    fn Delete(&self, cchoffset: i32, cchdelete: i32) -> windows_core::Result<()>;
}
impl IMLangStringBufW_Vtbl {
    pub const fn new<Identity: IMLangStringBufW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatus<Identity: IMLangStringBufW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufW_Impl::GetStatus(this, core::mem::transmute_copy(&plflags), core::mem::transmute_copy(&pcchbuf)).into()
            }
        }
        unsafe extern "system" fn LockBuf<Identity: IMLangStringBufW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufW_Impl::LockBuf(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchmaxlock), core::mem::transmute_copy(&ppszbuf), core::mem::transmute_copy(&pcchbuf)).into()
            }
        }
        unsafe extern "system" fn UnlockBuf<Identity: IMLangStringBufW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbuf: *const u16, cchoffset: i32, cchwrite: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufW_Impl::UnlockBuf(this, core::mem::transmute_copy(&pszbuf), core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchwrite)).into()
            }
        }
        unsafe extern "system" fn Insert<Identity: IMLangStringBufW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufW_Impl::Insert(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchmaxinsert), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IMLangStringBufW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringBufW_Impl::Delete(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchdelete)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            LockBuf: LockBuf::<Identity, OFFSET>,
            UnlockBuf: UnlockBuf::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangStringBufW as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLangStringBufW {}
windows_core::imp::define_interface!(IMLangStringWStr, IMLangStringWStr_Vtbl, 0xc04d65d0_b70d_11d0_b188_00aa0038c969);
impl core::ops::Deref for IMLangStringWStr {
    type Target = IMLangString;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLangStringWStr, windows_core::IUnknown, IMLangString);
impl IMLangStringWStr {
    pub unsafe fn SetWStr(&self, ldestpos: i32, ldestlen: i32, pszsrc: &[u16], pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWStr)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetStrBufW<P2>(&self, ldestpos: i32, ldestlen: i32, psrcbuf: P2, pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IMLangStringBufW>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStrBufW)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, psrcbuf.param().abi(), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetWStr(&self, lsrcpos: i32, lsrclen: i32, pszdest: Option<&mut [u16]>, pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, core::mem::transmute(pszdest.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), pszdest.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetStrBufW(&self, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: *mut Option<IMLangStringBufW>, pldestlen: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStrBufW)(windows_core::Interface::as_raw(self), lsrcpos, lsrcmaxlen, core::mem::transmute(ppdestbuf), pldestlen.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn LockWStr(&self, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: *mut windows_core::PWSTR, pcchdest: Option<*mut i32>, pldestlen: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockWStr)(windows_core::Interface::as_raw(self), lsrcpos, lsrclen, lflags, cchrequest, ppszdest as _, pcchdest.unwrap_or(core::mem::zeroed()) as _, pldestlen.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn UnlockWStr(&self, pszsrc: &[u16], pcchactual: Option<*mut i32>, plactuallen: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockWStr)(windows_core::Interface::as_raw(self), core::mem::transmute(pszsrc.as_ptr()), pszsrc.len().try_into().unwrap(), pcchactual.unwrap_or(core::mem::zeroed()) as _, plactuallen.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SetLocale(&self, ldestpos: i32, ldestlen: i32, locale: super::winnt::LCID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLocale)(windows_core::Interface::as_raw(self), ldestpos, ldestlen, locale) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetLocale(&self, lsrcpos: i32, lsrcmaxlen: i32, plocale: Option<*mut super::winnt::LCID>, pllocalepos: Option<*mut i32>, pllocalelen: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLocale)(windows_core::Interface::as_raw(self), lsrcpos, lsrcmaxlen, plocale.unwrap_or(core::mem::zeroed()) as _, pllocalepos.unwrap_or(core::mem::zeroed()) as _, pllocalelen.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLangStringWStr_Vtbl {
    pub base__: IMLangString_Vtbl,
    pub SetWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PCWSTR, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetStrBufW: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PWSTR, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetStrBufW: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LockWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, *mut windows_core::PWSTR, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub UnlockWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub SetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, super::winnt::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetLocale: usize,
    #[cfg(feature = "winnt")]
    pub GetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut super::winnt::LCID, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetLocale: usize,
}
#[cfg(feature = "winnt")]
pub trait IMLangStringWStr_Impl: IMLangString_Impl {
    fn SetWStr(&self, ldestpos: i32, ldestlen: i32, pszsrc: &windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn SetStrBufW(&self, ldestpos: i32, ldestlen: i32, psrcbuf: windows_core::Ref<IMLangStringBufW>, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn GetWStr(&self, lsrcpos: i32, lsrclen: i32, pszdest: windows_core::PWSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn GetStrBufW(&self, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: windows_core::OutRef<IMLangStringBufW>, pldestlen: *mut i32) -> windows_core::Result<()>;
    fn LockWStr(&self, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: *mut windows_core::PWSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> windows_core::Result<()>;
    fn UnlockWStr(&self, pszsrc: &windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn SetLocale(&self, ldestpos: i32, ldestlen: i32, locale: super::winnt::LCID) -> windows_core::Result<()>;
    fn GetLocale(&self, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut super::winnt::LCID, pllocalepos: *mut i32, pllocalelen: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IMLangStringWStr_Vtbl {
    pub const fn new<Identity: IMLangStringWStr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetWStr<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, pszsrc: windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::SetWStr(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn SetStrBufW<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcbuf: *mut core::ffi::c_void, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::SetStrBufW(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&psrcbuf), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn GetWStr<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, pszdest: windows_core::PWSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::GetWStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&pszdest), core::mem::transmute_copy(&cchdest), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn GetStrBufW<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: *mut *mut core::ffi::c_void, pldestlen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::GetStrBufW(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrcmaxlen), core::mem::transmute_copy(&ppdestbuf), core::mem::transmute_copy(&pldestlen)).into()
            }
        }
        unsafe extern "system" fn LockWStr<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: *mut windows_core::PWSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::LockWStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&cchrequest), core::mem::transmute_copy(&ppszdest), core::mem::transmute_copy(&pcchdest), core::mem::transmute_copy(&pldestlen)).into()
            }
        }
        unsafe extern "system" fn UnlockWStr<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsrc: windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::UnlockWStr(this, core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
            }
        }
        unsafe extern "system" fn SetLocale<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: super::winnt::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::SetLocale(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&locale)).into()
            }
        }
        unsafe extern "system" fn GetLocale<Identity: IMLangStringWStr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut super::winnt::LCID, pllocalepos: *mut i32, pllocalelen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLangStringWStr_Impl::GetLocale(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrcmaxlen), core::mem::transmute_copy(&plocale), core::mem::transmute_copy(&pllocalepos), core::mem::transmute_copy(&pllocalelen)).into()
            }
        }
        Self {
            base__: IMLangString_Vtbl::new::<Identity, OFFSET>(),
            SetWStr: SetWStr::<Identity, OFFSET>,
            SetStrBufW: SetStrBufW::<Identity, OFFSET>,
            GetWStr: GetWStr::<Identity, OFFSET>,
            GetStrBufW: GetStrBufW::<Identity, OFFSET>,
            LockWStr: LockWStr::<Identity, OFFSET>,
            UnlockWStr: UnlockWStr::<Identity, OFFSET>,
            SetLocale: SetLocale::<Identity, OFFSET>,
            GetLocale: GetLocale::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangStringWStr as windows_core::Interface>::IID || iid == &<IMLangString as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IMLangStringWStr {}
windows_core::imp::define_interface!(IMultiLanguage, IMultiLanguage_Vtbl, 0x275c23e1_3747_11d0_9fea_00aa003f8646);
windows_core::imp::interface_hierarchy!(IMultiLanguage, windows_core::IUnknown);
impl IMultiLanguage {
    pub unsafe fn GetNumberOfCodePageInfo(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfCodePageInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCodePageInfo(&self, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCodePageInfo)(windows_core::Interface::as_raw(self), uicodepage, pcodepageinfo as _) }
    }
    pub unsafe fn GetFamilyCodePage(&self, uicodepage: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFamilyCodePage)(windows_core::Interface::as_raw(self), uicodepage, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumCodePages(&self, grfflags: u32) -> windows_core::Result<IEnumCodePage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCodePages)(windows_core::Interface::as_raw(self), grfflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCharsetInfo(&self, charset: &windows_core::BSTR, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCharsetInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(charset), pcharsetinfo as _) }
    }
    pub unsafe fn IsConvertible(&self, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsConvertible)(windows_core::Interface::as_raw(self), dwsrcencoding, dwdstencoding) }
    }
    pub unsafe fn ConvertString(&self, pdwmode: Option<*mut u32>, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: Option<*const u8>, pcsrcsize: Option<*mut u32>, pdststr: Option<*mut u8>, pcdstsize: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConvertString)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwsrcencoding, dwdstencoding, psrcstr.unwrap_or(core::mem::zeroed()) as _, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr.unwrap_or(core::mem::zeroed()) as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn ConvertStringToUnicode(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: Option<*const i8>, pcsrcsize: Option<*mut u32>, pdststr: Option<*mut u16>, pcdstsize: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringToUnicode)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwencoding, psrcstr.unwrap_or(core::mem::zeroed()) as _, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr.unwrap_or(core::mem::zeroed()) as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn ConvertStringFromUnicode(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: Option<*const u16>, pcsrcsize: Option<*mut u32>, pdststr: Option<*mut i8>, pcdstsize: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringFromUnicode)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwencoding, psrcstr.unwrap_or(core::mem::zeroed()) as _, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr.unwrap_or(core::mem::zeroed()) as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn ConvertStringReset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringReset)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetRfc1766FromLcid(&self, locale: super::winnt::LCID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRfc1766FromLcid)(windows_core::Interface::as_raw(self), locale, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetLcidFromRfc1766(&self, plocale: *mut super::winnt::LCID, bstrrfc1766: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLcidFromRfc1766)(windows_core::Interface::as_raw(self), plocale as _, core::mem::transmute_copy(bstrrfc1766)) }
    }
    pub unsafe fn EnumRfc1766(&self) -> windows_core::Result<IEnumRfc1766> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRfc1766)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetRfc1766Info(&self, locale: super::winnt::LCID, prfc1766info: *mut RFC1766INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRfc1766Info)(windows_core::Interface::as_raw(self), locale, prfc1766info as _) }
    }
    pub unsafe fn CreateConvertCharset(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<IMLangConvertCharset> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateConvertCharset)(windows_core::Interface::as_raw(self), uisrccodepage, uidstcodepage, dwproperty, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiLanguage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNumberOfCodePageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCodePageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MIMECPINFO) -> windows_core::HRESULT,
    pub GetFamilyCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCharsetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut MIMECSETINFO) -> windows_core::HRESULT,
    pub IsConvertible: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ConvertString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, u32, *const u8, *mut u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringToUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *const i8, *mut u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringFromUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *const u16, *mut u32, *mut i8, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringReset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub GetRfc1766FromLcid: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LCID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetRfc1766FromLcid: usize,
    #[cfg(feature = "winnt")]
    pub GetLcidFromRfc1766: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::LCID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetLcidFromRfc1766: usize,
    pub EnumRfc1766: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub GetRfc1766Info: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LCID, *mut RFC1766INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetRfc1766Info: usize,
    pub CreateConvertCharset: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait IMultiLanguage_Impl: windows_core::IUnknownImpl {
    fn GetNumberOfCodePageInfo(&self) -> windows_core::Result<u32>;
    fn GetCodePageInfo(&self, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> windows_core::Result<()>;
    fn GetFamilyCodePage(&self, uicodepage: u32) -> windows_core::Result<u32>;
    fn EnumCodePages(&self, grfflags: u32) -> windows_core::Result<IEnumCodePage>;
    fn GetCharsetInfo(&self, charset: &windows_core::BSTR, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::Result<()>;
    fn IsConvertible(&self, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::Result<()>;
    fn ConvertString(&self, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringToUnicode(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: *const i8, pcsrcsize: *mut u32, pdststr: *mut u16, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringFromUnicode(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: *const u16, pcsrcsize: *mut u32, pdststr: *mut i8, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringReset(&self) -> windows_core::Result<()>;
    fn GetRfc1766FromLcid(&self, locale: super::winnt::LCID) -> windows_core::Result<windows_core::BSTR>;
    fn GetLcidFromRfc1766(&self, plocale: *mut super::winnt::LCID, bstrrfc1766: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EnumRfc1766(&self) -> windows_core::Result<IEnumRfc1766>;
    fn GetRfc1766Info(&self, locale: super::winnt::LCID, prfc1766info: *mut RFC1766INFO) -> windows_core::Result<()>;
    fn CreateConvertCharset(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<IMLangConvertCharset>;
}
#[cfg(feature = "winnt")]
impl IMultiLanguage_Vtbl {
    pub const fn new<Identity: IMultiLanguage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage_Impl::GetNumberOfCodePageInfo(this) {
                    Ok(ok__) => {
                        pccodepage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCodePageInfo<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::GetCodePageInfo(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&pcodepageinfo)).into()
            }
        }
        unsafe extern "system" fn GetFamilyCodePage<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage_Impl::GetFamilyCodePage(this, core::mem::transmute_copy(&uicodepage)) {
                    Ok(ok__) => {
                        puifamilycodepage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumCodePages<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, ppenumcodepage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage_Impl::EnumCodePages(this, core::mem::transmute_copy(&grfflags)) {
                    Ok(ok__) => {
                        ppenumcodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCharsetInfo<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, charset: *mut core::ffi::c_void, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::GetCharsetInfo(this, core::mem::transmute(&charset), core::mem::transmute_copy(&pcharsetinfo)).into()
            }
        }
        unsafe extern "system" fn IsConvertible<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::IsConvertible(this, core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding)).into()
            }
        }
        unsafe extern "system" fn ConvertString<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::ConvertString(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding), core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn ConvertStringToUnicode<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: *const i8, pcsrcsize: *mut u32, pdststr: *mut u16, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::ConvertStringToUnicode(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: *const u16, pcsrcsize: *mut u32, pdststr: *mut i8, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::ConvertStringFromUnicode(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn ConvertStringReset<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::ConvertStringReset(this).into()
            }
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: super::winnt::LCID, pbstrrfc1766: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage_Impl::GetRfc1766FromLcid(this, core::mem::transmute_copy(&locale)) {
                    Ok(ok__) => {
                        pbstrrfc1766.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocale: *mut super::winnt::LCID, bstrrfc1766: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::GetLcidFromRfc1766(this, core::mem::transmute_copy(&plocale), core::mem::transmute(&bstrrfc1766)).into()
            }
        }
        unsafe extern "system" fn EnumRfc1766<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumrfc1766: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage_Impl::EnumRfc1766(this) {
                    Ok(ok__) => {
                        ppenumrfc1766.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRfc1766Info<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: super::winnt::LCID, prfc1766info: *mut RFC1766INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage_Impl::GetRfc1766Info(this, core::mem::transmute_copy(&locale), core::mem::transmute_copy(&prfc1766info)).into()
            }
        }
        unsafe extern "system" fn CreateConvertCharset<Identity: IMultiLanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage_Impl::CreateConvertCharset(this, core::mem::transmute_copy(&uisrccodepage), core::mem::transmute_copy(&uidstcodepage), core::mem::transmute_copy(&dwproperty)) {
                    Ok(ok__) => {
                        ppmlangconvertcharset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNumberOfCodePageInfo: GetNumberOfCodePageInfo::<Identity, OFFSET>,
            GetCodePageInfo: GetCodePageInfo::<Identity, OFFSET>,
            GetFamilyCodePage: GetFamilyCodePage::<Identity, OFFSET>,
            EnumCodePages: EnumCodePages::<Identity, OFFSET>,
            GetCharsetInfo: GetCharsetInfo::<Identity, OFFSET>,
            IsConvertible: IsConvertible::<Identity, OFFSET>,
            ConvertString: ConvertString::<Identity, OFFSET>,
            ConvertStringToUnicode: ConvertStringToUnicode::<Identity, OFFSET>,
            ConvertStringFromUnicode: ConvertStringFromUnicode::<Identity, OFFSET>,
            ConvertStringReset: ConvertStringReset::<Identity, OFFSET>,
            GetRfc1766FromLcid: GetRfc1766FromLcid::<Identity, OFFSET>,
            GetLcidFromRfc1766: GetLcidFromRfc1766::<Identity, OFFSET>,
            EnumRfc1766: EnumRfc1766::<Identity, OFFSET>,
            GetRfc1766Info: GetRfc1766Info::<Identity, OFFSET>,
            CreateConvertCharset: CreateConvertCharset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiLanguage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IMultiLanguage {}
windows_core::imp::define_interface!(IMultiLanguage2, IMultiLanguage2_Vtbl, 0xdccfc164_2b38_11d2_b7ec_00c04f8f5d9a);
windows_core::imp::interface_hierarchy!(IMultiLanguage2, windows_core::IUnknown);
impl IMultiLanguage2 {
    pub unsafe fn GetNumberOfCodePageInfo(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfCodePageInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetCodePageInfo(&self, uicodepage: u32, langid: super::winnt::LANGID, pcodepageinfo: *mut MIMECPINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCodePageInfo)(windows_core::Interface::as_raw(self), uicodepage, langid, pcodepageinfo as _) }
    }
    pub unsafe fn GetFamilyCodePage(&self, uicodepage: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFamilyCodePage)(windows_core::Interface::as_raw(self), uicodepage, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn EnumCodePages(&self, grfflags: u32, langid: super::winnt::LANGID) -> windows_core::Result<IEnumCodePage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCodePages)(windows_core::Interface::as_raw(self), grfflags, langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCharsetInfo(&self, charset: &windows_core::BSTR, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCharsetInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(charset), pcharsetinfo as _) }
    }
    pub unsafe fn IsConvertible(&self, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsConvertible)(windows_core::Interface::as_raw(self), dwsrcencoding, dwdstencoding) }
    }
    pub unsafe fn ConvertString(&self, pdwmode: Option<*mut u32>, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: Option<*const u8>, pcsrcsize: Option<*mut u32>, pdststr: Option<*mut u8>, pcdstsize: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConvertString)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwsrcencoding, dwdstencoding, psrcstr.unwrap_or(core::mem::zeroed()) as _, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr.unwrap_or(core::mem::zeroed()) as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn ConvertStringToUnicode(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: Option<*const i8>, pcsrcsize: Option<*mut u32>, pdststr: Option<*mut u16>, pcdstsize: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringToUnicode)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwencoding, psrcstr.unwrap_or(core::mem::zeroed()) as _, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr.unwrap_or(core::mem::zeroed()) as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn ConvertStringFromUnicode(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: Option<*const u16>, pcsrcsize: Option<*mut u32>, pdststr: Option<*mut i8>, pcdstsize: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringFromUnicode)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwencoding, psrcstr.unwrap_or(core::mem::zeroed()) as _, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr.unwrap_or(core::mem::zeroed()) as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn ConvertStringReset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringReset)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetRfc1766FromLcid(&self, locale: super::winnt::LCID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRfc1766FromLcid)(windows_core::Interface::as_raw(self), locale, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetLcidFromRfc1766(&self, plocale: *mut super::winnt::LCID, bstrrfc1766: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLcidFromRfc1766)(windows_core::Interface::as_raw(self), plocale as _, core::mem::transmute_copy(bstrrfc1766)) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn EnumRfc1766(&self, langid: super::winnt::LANGID) -> windows_core::Result<IEnumRfc1766> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRfc1766)(windows_core::Interface::as_raw(self), langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetRfc1766Info(&self, locale: super::winnt::LCID, langid: super::winnt::LANGID, prfc1766info: *mut RFC1766INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRfc1766Info)(windows_core::Interface::as_raw(self), locale, langid, prfc1766info as _) }
    }
    pub unsafe fn CreateConvertCharset(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<IMLangConvertCharset> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateConvertCharset)(windows_core::Interface::as_raw(self), uisrccodepage, uidstcodepage, dwproperty, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn ConvertStringInIStream<P5, P6>(&self, pdwmode: Option<*mut u32>, dwflag: u32, lpfallback: Option<*const u16>, dwsrcencoding: u32, dwdstencoding: u32, pstmin: P5, pstmout: P6) -> windows_core::HRESULT
    where
        P5: windows_core::Param<super::objidlbase::IStream>,
        P6: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringInIStream)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwflag, lpfallback.unwrap_or(core::mem::zeroed()) as _, dwsrcencoding, dwdstencoding, pstmin.param().abi(), pstmout.param().abi()) }
    }
    pub unsafe fn ConvertStringToUnicodeEx(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: *const i8, pcsrcsize: Option<*mut u32>, pdststr: *mut u16, pcdstsize: Option<*mut u32>, dwflag: u32, lpfallback: Option<*const u16>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringToUnicodeEx)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwencoding, psrcstr, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _, dwflag, lpfallback.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn ConvertStringFromUnicodeEx(&self, pdwmode: Option<*mut u32>, dwencoding: u32, psrcstr: *const u16, pcsrcsize: Option<*mut u32>, pdststr: *mut i8, pcdstsize: Option<*mut u32>, dwflag: u32, lpfallback: Option<*const u16>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConvertStringFromUnicodeEx)(windows_core::Interface::as_raw(self), pdwmode.unwrap_or(core::mem::zeroed()) as _, dwencoding, psrcstr, pcsrcsize.unwrap_or(core::mem::zeroed()) as _, pdststr as _, pcdstsize.unwrap_or(core::mem::zeroed()) as _, dwflag, lpfallback.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn DetectCodepageInIStream<P2>(&self, dwflag: u32, dwprefwincodepage: u32, pstmin: P2, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).DetectCodepageInIStream)(windows_core::Interface::as_raw(self), dwflag, dwprefwincodepage, pstmin.param().abi(), lpencoding as _, pnscores as _) }
    }
    pub unsafe fn DetectInputCodepage(&self, dwflag: u32, dwprefwincodepage: u32, psrcstr: *const i8, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DetectInputCodepage)(windows_core::Interface::as_raw(self), dwflag, dwprefwincodepage, psrcstr, pcsrcsize as _, lpencoding as _, pnscores as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ValidateCodePage(&self, uicodepage: u32, hwnd: super::windef::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ValidateCodePage)(windows_core::Interface::as_raw(self), uicodepage, hwnd) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetCodePageDescription(&self, uicodepage: u32, lcid: super::winnt::LCID, lpwidecharstr: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCodePageDescription)(windows_core::Interface::as_raw(self), uicodepage, lcid, core::mem::transmute(lpwidecharstr.as_mut_ptr()), lpwidecharstr.len().try_into().unwrap()) }
    }
    pub unsafe fn IsCodePageInstallable(&self, uicodepage: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsCodePageInstallable)(windows_core::Interface::as_raw(self), uicodepage) }
    }
    pub unsafe fn SetMimeDBSource(&self, dwsource: MIMECONTF) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMimeDBSource)(windows_core::Interface::as_raw(self), dwsource) }
    }
    pub unsafe fn GetNumberOfScripts(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfScripts)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn EnumScripts(&self, dwflags: u32, langid: super::winnt::LANGID) -> windows_core::Result<IEnumScript> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumScripts)(windows_core::Interface::as_raw(self), dwflags, langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ValidateCodePageEx(&self, uicodepage: u32, hwnd: super::windef::HWND, dwfiodcontrol: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ValidateCodePageEx)(windows_core::Interface::as_raw(self), uicodepage, hwnd, dwfiodcontrol) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiLanguage2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNumberOfCodePageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub GetCodePageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::winnt::LANGID, *mut MIMECPINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetCodePageInfo: usize,
    pub GetFamilyCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub EnumCodePages: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::winnt::LANGID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    EnumCodePages: usize,
    pub GetCharsetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut MIMECSETINFO) -> windows_core::HRESULT,
    pub IsConvertible: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ConvertString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, u32, *const u8, *mut u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringToUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *const i8, *mut u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringFromUnicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *const u16, *mut u32, *mut i8, *mut u32) -> windows_core::HRESULT,
    pub ConvertStringReset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub GetRfc1766FromLcid: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LCID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetRfc1766FromLcid: usize,
    #[cfg(feature = "winnt")]
    pub GetLcidFromRfc1766: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::LCID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetLcidFromRfc1766: usize,
    #[cfg(feature = "winnt")]
    pub EnumRfc1766: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LANGID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    EnumRfc1766: usize,
    #[cfg(feature = "winnt")]
    pub GetRfc1766Info: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LCID, super::winnt::LANGID, *mut RFC1766INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetRfc1766Info: usize,
    pub CreateConvertCharset: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub ConvertStringInIStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *const u16, u32, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    ConvertStringInIStream: usize,
    pub ConvertStringToUnicodeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *const i8, *mut u32, *mut u16, *mut u32, u32, *const u16) -> windows_core::HRESULT,
    pub ConvertStringFromUnicodeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *const u16, *mut u32, *mut i8, *mut u32, u32, *const u16) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub DetectCodepageInIStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, *mut DetectEncodingInfo, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    DetectCodepageInIStream: usize,
    pub DetectInputCodepage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const i8, *mut i32, *mut DetectEncodingInfo, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub ValidateCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ValidateCodePage: usize,
    #[cfg(feature = "winnt")]
    pub GetCodePageDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::winnt::LCID, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetCodePageDescription: usize,
    pub IsCodePageInstallable: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetMimeDBSource: unsafe extern "system" fn(*mut core::ffi::c_void, MIMECONTF) -> windows_core::HRESULT,
    pub GetNumberOfScripts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub EnumScripts: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::winnt::LANGID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    EnumScripts: usize,
    #[cfg(feature = "windef")]
    pub ValidateCodePageEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::windef::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ValidateCodePageEx: usize,
}
#[cfg(all(feature = "objidlbase", feature = "windef", feature = "winnt"))]
pub trait IMultiLanguage2_Impl: windows_core::IUnknownImpl {
    fn GetNumberOfCodePageInfo(&self) -> windows_core::Result<u32>;
    fn GetCodePageInfo(&self, uicodepage: u32, langid: super::winnt::LANGID, pcodepageinfo: *mut MIMECPINFO) -> windows_core::Result<()>;
    fn GetFamilyCodePage(&self, uicodepage: u32) -> windows_core::Result<u32>;
    fn EnumCodePages(&self, grfflags: u32, langid: super::winnt::LANGID) -> windows_core::Result<IEnumCodePage>;
    fn GetCharsetInfo(&self, charset: &windows_core::BSTR, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::Result<()>;
    fn IsConvertible(&self, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::Result<()>;
    fn ConvertString(&self, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringToUnicode(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: *const i8, pcsrcsize: *mut u32, pdststr: *mut u16, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringFromUnicode(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: *const u16, pcsrcsize: *mut u32, pdststr: *mut i8, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringReset(&self) -> windows_core::Result<()>;
    fn GetRfc1766FromLcid(&self, locale: super::winnt::LCID) -> windows_core::Result<windows_core::BSTR>;
    fn GetLcidFromRfc1766(&self, plocale: *mut super::winnt::LCID, bstrrfc1766: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EnumRfc1766(&self, langid: super::winnt::LANGID) -> windows_core::Result<IEnumRfc1766>;
    fn GetRfc1766Info(&self, locale: super::winnt::LCID, langid: super::winnt::LANGID, prfc1766info: *mut RFC1766INFO) -> windows_core::Result<()>;
    fn CreateConvertCharset(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<IMLangConvertCharset>;
    fn ConvertStringInIStream(&self, pdwmode: *mut u32, dwflag: u32, lpfallback: *const u16, dwsrcencoding: u32, dwdstencoding: u32, pstmin: windows_core::Ref<super::objidlbase::IStream>, pstmout: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn ConvertStringToUnicodeEx(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: *const i8, pcsrcsize: *mut u32, pdststr: *mut u16, pcdstsize: *mut u32, dwflag: u32, lpfallback: *const u16) -> windows_core::Result<()>;
    fn ConvertStringFromUnicodeEx(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: *const u16, pcsrcsize: *mut u32, pdststr: *mut i8, pcdstsize: *mut u32, dwflag: u32, lpfallback: *const u16) -> windows_core::Result<()>;
    fn DetectCodepageInIStream(&self, dwflag: u32, dwprefwincodepage: u32, pstmin: windows_core::Ref<super::objidlbase::IStream>, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::Result<()>;
    fn DetectInputCodepage(&self, dwflag: u32, dwprefwincodepage: u32, psrcstr: *const i8, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::Result<()>;
    fn ValidateCodePage(&self, uicodepage: u32, hwnd: super::windef::HWND) -> windows_core::Result<()>;
    fn GetCodePageDescription(&self, uicodepage: u32, lcid: super::winnt::LCID, lpwidecharstr: windows_core::PWSTR, cchwidechar: i32) -> windows_core::Result<()>;
    fn IsCodePageInstallable(&self, uicodepage: u32) -> windows_core::Result<()>;
    fn SetMimeDBSource(&self, dwsource: MIMECONTF) -> windows_core::Result<()>;
    fn GetNumberOfScripts(&self) -> windows_core::Result<u32>;
    fn EnumScripts(&self, dwflags: u32, langid: super::winnt::LANGID) -> windows_core::Result<IEnumScript>;
    fn ValidateCodePageEx(&self, uicodepage: u32, hwnd: super::windef::HWND, dwfiodcontrol: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "windef", feature = "winnt"))]
impl IMultiLanguage2_Vtbl {
    pub const fn new<Identity: IMultiLanguage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::GetNumberOfCodePageInfo(this) {
                    Ok(ok__) => {
                        pccodepage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCodePageInfo<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, langid: super::winnt::LANGID, pcodepageinfo: *mut MIMECPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::GetCodePageInfo(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&pcodepageinfo)).into()
            }
        }
        unsafe extern "system" fn GetFamilyCodePage<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::GetFamilyCodePage(this, core::mem::transmute_copy(&uicodepage)) {
                    Ok(ok__) => {
                        puifamilycodepage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumCodePages<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, langid: super::winnt::LANGID, ppenumcodepage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::EnumCodePages(this, core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&langid)) {
                    Ok(ok__) => {
                        ppenumcodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCharsetInfo<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, charset: *mut core::ffi::c_void, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::GetCharsetInfo(this, core::mem::transmute(&charset), core::mem::transmute_copy(&pcharsetinfo)).into()
            }
        }
        unsafe extern "system" fn IsConvertible<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::IsConvertible(this, core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding)).into()
            }
        }
        unsafe extern "system" fn ConvertString<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertString(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding), core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn ConvertStringToUnicode<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: *const i8, pcsrcsize: *mut u32, pdststr: *mut u16, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertStringToUnicode(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: *const u16, pcsrcsize: *mut u32, pdststr: *mut i8, pcdstsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertStringFromUnicode(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
            }
        }
        unsafe extern "system" fn ConvertStringReset<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertStringReset(this).into()
            }
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: super::winnt::LCID, pbstrrfc1766: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::GetRfc1766FromLcid(this, core::mem::transmute_copy(&locale)) {
                    Ok(ok__) => {
                        pbstrrfc1766.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocale: *mut super::winnt::LCID, bstrrfc1766: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::GetLcidFromRfc1766(this, core::mem::transmute_copy(&plocale), core::mem::transmute(&bstrrfc1766)).into()
            }
        }
        unsafe extern "system" fn EnumRfc1766<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: super::winnt::LANGID, ppenumrfc1766: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::EnumRfc1766(this, core::mem::transmute_copy(&langid)) {
                    Ok(ok__) => {
                        ppenumrfc1766.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRfc1766Info<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: super::winnt::LCID, langid: super::winnt::LANGID, prfc1766info: *mut RFC1766INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::GetRfc1766Info(this, core::mem::transmute_copy(&locale), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&prfc1766info)).into()
            }
        }
        unsafe extern "system" fn CreateConvertCharset<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::CreateConvertCharset(this, core::mem::transmute_copy(&uisrccodepage), core::mem::transmute_copy(&uidstcodepage), core::mem::transmute_copy(&dwproperty)) {
                    Ok(ok__) => {
                        ppmlangconvertcharset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConvertStringInIStream<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwflag: u32, lpfallback: *const u16, dwsrcencoding: u32, dwdstencoding: u32, pstmin: *mut core::ffi::c_void, pstmout: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertStringInIStream(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwflag), core::mem::transmute_copy(&lpfallback), core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding), core::mem::transmute_copy(&pstmin), core::mem::transmute_copy(&pstmout)).into()
            }
        }
        unsafe extern "system" fn ConvertStringToUnicodeEx<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: *const i8, pcsrcsize: *mut u32, pdststr: *mut u16, pcdstsize: *mut u32, dwflag: u32, lpfallback: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertStringToUnicodeEx(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize), core::mem::transmute_copy(&dwflag), core::mem::transmute_copy(&lpfallback)).into()
            }
        }
        unsafe extern "system" fn ConvertStringFromUnicodeEx<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: *const u16, pcsrcsize: *mut u32, pdststr: *mut i8, pcdstsize: *mut u32, dwflag: u32, lpfallback: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ConvertStringFromUnicodeEx(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize), core::mem::transmute_copy(&dwflag), core::mem::transmute_copy(&lpfallback)).into()
            }
        }
        unsafe extern "system" fn DetectCodepageInIStream<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, pstmin: *mut core::ffi::c_void, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::DetectCodepageInIStream(this, core::mem::transmute_copy(&dwflag), core::mem::transmute_copy(&dwprefwincodepage), core::mem::transmute_copy(&pstmin), core::mem::transmute_copy(&lpencoding), core::mem::transmute_copy(&pnscores)).into()
            }
        }
        unsafe extern "system" fn DetectInputCodepage<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, psrcstr: *const i8, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::DetectInputCodepage(this, core::mem::transmute_copy(&dwflag), core::mem::transmute_copy(&dwprefwincodepage), core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&lpencoding), core::mem::transmute_copy(&pnscores)).into()
            }
        }
        unsafe extern "system" fn ValidateCodePage<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, hwnd: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ValidateCodePage(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn GetCodePageDescription<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, lcid: super::winnt::LCID, lpwidecharstr: windows_core::PWSTR, cchwidechar: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::GetCodePageDescription(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&lpwidecharstr), core::mem::transmute_copy(&cchwidechar)).into()
            }
        }
        unsafe extern "system" fn IsCodePageInstallable<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::IsCodePageInstallable(this, core::mem::transmute_copy(&uicodepage)).into()
            }
        }
        unsafe extern "system" fn SetMimeDBSource<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsource: MIMECONTF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::SetMimeDBSource(this, core::mem::transmute_copy(&dwsource)).into()
            }
        }
        unsafe extern "system" fn GetNumberOfScripts<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnscripts: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::GetNumberOfScripts(this) {
                    Ok(ok__) => {
                        pnscripts.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumScripts<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, langid: super::winnt::LANGID, ppenumscript: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultiLanguage2_Impl::EnumScripts(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&langid)) {
                    Ok(ok__) => {
                        ppenumscript.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ValidateCodePageEx<Identity: IMultiLanguage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, hwnd: super::windef::HWND, dwfiodcontrol: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage2_Impl::ValidateCodePageEx(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwfiodcontrol)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNumberOfCodePageInfo: GetNumberOfCodePageInfo::<Identity, OFFSET>,
            GetCodePageInfo: GetCodePageInfo::<Identity, OFFSET>,
            GetFamilyCodePage: GetFamilyCodePage::<Identity, OFFSET>,
            EnumCodePages: EnumCodePages::<Identity, OFFSET>,
            GetCharsetInfo: GetCharsetInfo::<Identity, OFFSET>,
            IsConvertible: IsConvertible::<Identity, OFFSET>,
            ConvertString: ConvertString::<Identity, OFFSET>,
            ConvertStringToUnicode: ConvertStringToUnicode::<Identity, OFFSET>,
            ConvertStringFromUnicode: ConvertStringFromUnicode::<Identity, OFFSET>,
            ConvertStringReset: ConvertStringReset::<Identity, OFFSET>,
            GetRfc1766FromLcid: GetRfc1766FromLcid::<Identity, OFFSET>,
            GetLcidFromRfc1766: GetLcidFromRfc1766::<Identity, OFFSET>,
            EnumRfc1766: EnumRfc1766::<Identity, OFFSET>,
            GetRfc1766Info: GetRfc1766Info::<Identity, OFFSET>,
            CreateConvertCharset: CreateConvertCharset::<Identity, OFFSET>,
            ConvertStringInIStream: ConvertStringInIStream::<Identity, OFFSET>,
            ConvertStringToUnicodeEx: ConvertStringToUnicodeEx::<Identity, OFFSET>,
            ConvertStringFromUnicodeEx: ConvertStringFromUnicodeEx::<Identity, OFFSET>,
            DetectCodepageInIStream: DetectCodepageInIStream::<Identity, OFFSET>,
            DetectInputCodepage: DetectInputCodepage::<Identity, OFFSET>,
            ValidateCodePage: ValidateCodePage::<Identity, OFFSET>,
            GetCodePageDescription: GetCodePageDescription::<Identity, OFFSET>,
            IsCodePageInstallable: IsCodePageInstallable::<Identity, OFFSET>,
            SetMimeDBSource: SetMimeDBSource::<Identity, OFFSET>,
            GetNumberOfScripts: GetNumberOfScripts::<Identity, OFFSET>,
            EnumScripts: EnumScripts::<Identity, OFFSET>,
            ValidateCodePageEx: ValidateCodePageEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiLanguage2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IMultiLanguage2 {}
windows_core::imp::define_interface!(IMultiLanguage3, IMultiLanguage3_Vtbl, 0x4e5868ab_b157_4623_9acc_6a1d9caebe04);
impl core::ops::Deref for IMultiLanguage3 {
    type Target = IMultiLanguage2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMultiLanguage3, windows_core::IUnknown, IMultiLanguage2);
impl IMultiLanguage3 {
    pub unsafe fn DetectOutboundCodePage<P7>(&self, dwflags: u32, lpwidecharstr: &[u16], puipreferredcodepages: Option<&[u32]>, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: P7) -> windows_core::HRESULT
    where
        P7: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DetectOutboundCodePage)(windows_core::Interface::as_raw(self), dwflags, core::mem::transmute(lpwidecharstr.as_ptr()), lpwidecharstr.len().try_into().unwrap(), puipreferredcodepages.map_or(core::ptr::null(), |slice| slice.as_ptr()), puipreferredcodepages.map_or(0, |slice| slice.len().try_into().unwrap()), puidetectedcodepages as _, pndetectedcodepages as _, lpspecialchar.param().abi()) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn DetectOutboundCodePageInIStream<P1, P6>(&self, dwflags: u32, pstrin: P1, puipreferredcodepages: Option<&[u32]>, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: P6) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidlbase::IStream>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DetectOutboundCodePageInIStream)(windows_core::Interface::as_raw(self), dwflags, pstrin.param().abi(), puipreferredcodepages.map_or(core::ptr::null(), |slice| slice.as_ptr()), puipreferredcodepages.map_or(0, |slice| slice.len().try_into().unwrap()), puidetectedcodepages as _, pndetectedcodepages as _, lpspecialchar.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiLanguage3_Vtbl {
    pub base__: IMultiLanguage2_Vtbl,
    pub DetectOutboundCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, *const u32, u32, *mut u32, *mut u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub DetectOutboundCodePageInIStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *const u32, u32, *mut u32, *mut u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    DetectOutboundCodePageInIStream: usize,
}
#[cfg(all(feature = "objidlbase", feature = "windef", feature = "winnt"))]
pub trait IMultiLanguage3_Impl: IMultiLanguage2_Impl {
    fn DetectOutboundCodePage(&self, dwflags: u32, lpwidecharstr: &windows_core::PCWSTR, cchwidechar: u32, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DetectOutboundCodePageInIStream(&self, dwflags: u32, pstrin: windows_core::Ref<super::objidlbase::IStream>, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "windef", feature = "winnt"))]
impl IMultiLanguage3_Vtbl {
    pub const fn new<Identity: IMultiLanguage3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DetectOutboundCodePage<Identity: IMultiLanguage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, lpwidecharstr: windows_core::PCWSTR, cchwidechar: u32, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage3_Impl::DetectOutboundCodePage(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&lpwidecharstr), core::mem::transmute_copy(&cchwidechar), core::mem::transmute_copy(&puipreferredcodepages), core::mem::transmute_copy(&npreferredcodepages), core::mem::transmute_copy(&puidetectedcodepages), core::mem::transmute_copy(&pndetectedcodepages), core::mem::transmute(&lpspecialchar)).into()
            }
        }
        unsafe extern "system" fn DetectOutboundCodePageInIStream<Identity: IMultiLanguage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pstrin: *mut core::ffi::c_void, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiLanguage3_Impl::DetectOutboundCodePageInIStream(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pstrin), core::mem::transmute_copy(&puipreferredcodepages), core::mem::transmute_copy(&npreferredcodepages), core::mem::transmute_copy(&puidetectedcodepages), core::mem::transmute_copy(&pndetectedcodepages), core::mem::transmute(&lpspecialchar)).into()
            }
        }
        Self {
            base__: IMultiLanguage2_Vtbl::new::<Identity, OFFSET>(),
            DetectOutboundCodePage: DetectOutboundCodePage::<Identity, OFFSET>,
            DetectOutboundCodePageInIStream: DetectOutboundCodePageInIStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiLanguage3 as windows_core::Interface>::IID || iid == &<IMultiLanguage2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IMultiLanguage3 {}
pub const MAX_LOCALE_NAME: u32 = 32;
pub const MAX_MIMECP_NAME: u32 = 64;
pub const MAX_MIMECSET_NAME: u32 = 50;
pub const MAX_MIMEFACE_NAME: u32 = 32;
pub const MAX_RFC1766_NAME: u32 = 6;
pub const MAX_SCRIPT_NAME: u32 = 48;
pub type MIMECONTF = i32;
pub const MIMECONTF_BROWSER: MIMECONTF = 2;
pub const MIMECONTF_EXPORT: MIMECONTF = 1024;
pub const MIMECONTF_IMPORT: MIMECONTF = 8;
pub const MIMECONTF_MAILNEWS: MIMECONTF = 1;
pub const MIMECONTF_MIME_IE4: MIMECONTF = 268435456;
pub const MIMECONTF_MIME_LATEST: MIMECONTF = 536870912;
pub const MIMECONTF_MIME_REGISTRY: MIMECONTF = 1073741824;
pub const MIMECONTF_MINIMAL: MIMECONTF = 4;
pub const MIMECONTF_PRIVCONVERTER: MIMECONTF = 65536;
pub const MIMECONTF_SAVABLE_BROWSER: MIMECONTF = 512;
pub const MIMECONTF_SAVABLE_MAILNEWS: MIMECONTF = 256;
pub const MIMECONTF_VALID: MIMECONTF = 131072;
pub const MIMECONTF_VALID_NLS: MIMECONTF = 262144;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIMECPINFO {
    pub dwFlags: u32,
    pub uiCodePage: u32,
    pub uiFamilyCodePage: u32,
    pub wszDescription: [u16; 64],
    pub wszWebCharset: [u16; 50],
    pub wszHeaderCharset: [u16; 50],
    pub wszBodyCharset: [u16; 50],
    pub wszFixedWidthFont: [u16; 32],
    pub wszProportionalFont: [u16; 32],
    pub bGDICharset: u8,
}
impl Default for MIMECPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIMECSETINFO {
    pub uiCodePage: u32,
    pub uiInternetEncoding: u32,
    pub wszCharset: [u16; 50],
}
impl Default for MIMECSETINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MLCONVCHAR = i32;
pub const MLCONVCHARF_AUTODETECT: MLCONVCHAR = 1;
pub const MLCONVCHARF_DETECTJPN: MLCONVCHAR = 32;
pub const MLCONVCHARF_ENTITIZE: MLCONVCHAR = 2;
pub const MLCONVCHARF_NAME_ENTITIZE: MLCONVCHAR = 4;
pub const MLCONVCHARF_NCR_ENTITIZE: MLCONVCHAR = 2;
pub const MLCONVCHARF_NOBESTFITCHARS: MLCONVCHAR = 16;
pub const MLCONVCHARF_USEDEFCHAR: MLCONVCHAR = 8;
pub type MLCP = i32;
pub type MLDETECTCP = i32;
pub const MLDETECTCP_7BIT: MLDETECTCP = 1;
pub const MLDETECTCP_8BIT: MLDETECTCP = 2;
pub const MLDETECTCP_DBCS: MLDETECTCP = 4;
pub const MLDETECTCP_HTML: MLDETECTCP = 8;
pub const MLDETECTCP_NONE: MLDETECTCP = 0;
pub const MLDETECTCP_NUMBER: MLDETECTCP = 16;
pub const MLDETECTF_BROWSER: MLCP = 2;
pub const MLDETECTF_EURO_UTF8: MLCP = 128;
pub const MLDETECTF_FILTER_SPECIALCHAR: MLCP = 64;
pub const MLDETECTF_MAILNEWS: MLCP = 1;
pub const MLDETECTF_PREFERRED_ONLY: MLCP = 32;
pub const MLDETECTF_PRESERVE_ORDER: MLCP = 16;
pub const MLDETECTF_VALID: MLCP = 4;
pub const MLDETECTF_VALID_NLS: MLCP = 8;
pub type MLSTR_FLAGS = i32;
pub const MLSTR_READ: MLSTR_FLAGS = 1;
pub const MLSTR_WRITE: MLSTR_FLAGS = 2;
pub type PMIMECPINFO = *mut MIMECPINFO;
pub type PMIMECSETINFO = *mut MIMECSETINFO;
#[cfg(feature = "winnt")]
pub type PRFC1766INFO = *mut RFC1766INFO;
pub type PSCRIPTFONTINFO = *mut SCRIPTFONTINFO;
pub type PSCRIPTINFO = *mut SCRIPTINFO;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RFC1766INFO {
    pub lcid: super::winnt::LCID,
    pub wszRfc1766: [u16; 6],
    pub wszLocaleName: [u16; 32],
}
#[cfg(feature = "winnt")]
impl Default for RFC1766INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SCRIPTCONTF = i32;
pub const SCRIPTCONTF_FIXED_FONT: SCRIPTFONTCONTF = 1;
pub const SCRIPTCONTF_PROPORTIONAL_FONT: SCRIPTFONTCONTF = 2;
pub const SCRIPTCONTF_SCRIPT_HIDE: SCRIPTFONTCONTF = 131072;
pub const SCRIPTCONTF_SCRIPT_SYSTEM: SCRIPTFONTCONTF = 262144;
pub const SCRIPTCONTF_SCRIPT_USER: SCRIPTFONTCONTF = 65536;
pub type SCRIPTFONTCONTF = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCRIPTFONTINFO {
    pub scripts: SCRIPT_IDS,
    pub wszFont: [u16; 32],
}
impl Default for SCRIPTFONTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCRIPTINFO {
    pub ScriptId: SCRIPT_ID,
    pub uiCodePage: u32,
    pub wszDescription: [u16; 48],
    pub wszFixedWidthFont: [u16; 32],
    pub wszProportionalFont: [u16; 32],
}
impl Default for SCRIPTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SCRIPT_ID(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SCRIPT_IDS(pub i64);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UNICODERANGE {
    pub wcFrom: u16,
    pub wcTo: u16,
}
pub type pDetectEncodingInfo = *mut DetectEncodingInfo;
pub const sidArabic: SCRIPTCONTF = 9;
pub const sidArmenian: SCRIPTCONTF = 7;
pub const sidAsciiLatin: SCRIPTCONTF = 3;
pub const sidAsciiSym: SCRIPTCONTF = 2;
pub const sidBengali: SCRIPTCONTF = 11;
pub const sidBopomofo: SCRIPTCONTF = 25;
pub const sidBraille: SCRIPTCONTF = 31;
pub const sidBurmese: SCRIPTCONTF = 36;
pub const sidCanSyllabic: SCRIPTCONTF = 28;
pub const sidCherokee: SCRIPTCONTF = 29;
pub const sidCyrillic: SCRIPTCONTF = 6;
pub const sidDefault: SCRIPTCONTF = 0;
pub const sidDevanagari: SCRIPTCONTF = 10;
pub const sidEthiopic: SCRIPTCONTF = 27;
pub const sidFEFirst: SCRIPTCONTF = 23;
pub const sidFELast: SCRIPTCONTF = 26;
pub const sidGeorgian: SCRIPTCONTF = 22;
pub const sidGreek: SCRIPTCONTF = 5;
pub const sidGujarati: SCRIPTCONTF = 13;
pub const sidGurmukhi: SCRIPTCONTF = 12;
pub const sidHan: SCRIPTCONTF = 26;
pub const sidHangul: SCRIPTCONTF = 23;
pub const sidHebrew: SCRIPTCONTF = 8;
pub const sidKana: SCRIPTCONTF = 24;
pub const sidKannada: SCRIPTCONTF = 17;
pub const sidKhmer: SCRIPTCONTF = 37;
pub const sidLao: SCRIPTCONTF = 20;
pub const sidLatin: SCRIPTCONTF = 4;
pub const sidLim: SCRIPTCONTF = 41;
pub const sidMalayalam: SCRIPTCONTF = 18;
pub const sidMerge: SCRIPTCONTF = 1;
pub const sidMongolian: SCRIPTCONTF = 39;
pub const sidOgham: SCRIPTCONTF = 33;
pub const sidOriya: SCRIPTCONTF = 14;
pub const sidRunic: SCRIPTCONTF = 32;
pub const sidSinhala: SCRIPTCONTF = 34;
pub const sidSyriac: SCRIPTCONTF = 35;
pub const sidTamil: SCRIPTCONTF = 15;
pub const sidTelugu: SCRIPTCONTF = 16;
pub const sidThaana: SCRIPTCONTF = 38;
pub const sidThai: SCRIPTCONTF = 19;
pub const sidTibetan: SCRIPTCONTF = 21;
pub const sidUserDefined: SCRIPTCONTF = 40;
pub const sidYi: SCRIPTCONTF = 30;
