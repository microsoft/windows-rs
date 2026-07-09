pub const CA_ACCESS_ADMIN: u32 = 1;
pub const CA_ACCESS_AUDITOR: u32 = 4;
pub const CA_ACCESS_ENROLL: u32 = 512;
pub const CA_ACCESS_MASKROLES: u32 = 255;
pub const CA_ACCESS_OFFICER: u32 = 2;
pub const CA_ACCESS_OPERATOR: u32 = 8;
pub const CA_ACCESS_READ: u32 = 256;
pub const CA_CRL_BASE: u32 = 1;
pub const CA_CRL_DELTA: u32 = 2;
pub const CA_CRL_REPUBLISH: u32 = 16;
pub const CA_DISP_ERROR: u32 = 1;
pub const CA_DISP_INCOMPLETE: u32 = 0;
pub const CA_DISP_INVALID: u32 = 4;
pub const CA_DISP_REVOKED: u32 = 2;
pub const CA_DISP_UNDER_SUBMISSION: u32 = 5;
pub const CA_DISP_VALID: u32 = 3;
pub const CCertAdmin: windows_core::GUID = windows_core::GUID::from_u128(0x37eabaf0_7fb6_11d0_8817_00a0c903b83c);
pub const CCertView: windows_core::GUID = windows_core::GUID::from_u128(0xa12d0f7a_1e84_11d1_9bd6_00c04fb683fa);
pub const CDR_EXPIRED: u32 = 1;
pub const CDR_REQUEST_LAST_CHANGED: u32 = 2;
pub const ICF_ALLOWFOREIGN: u32 = 65536;
pub const ICF_EXISTINGROW: u32 = 131072;
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertAdmin, ICertAdmin_Vtbl, 0x34df6950_7fb6_11d0_8817_00a0c903b83c);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertAdmin {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertAdmin, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ICertAdmin {
    pub unsafe fn IsValidCertificate(&self, strconfig: &windows_core::BSTR, strserialnumber: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsValidCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strserialnumber), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRevocationReason(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRevocationReason)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RevokeCertificate(&self, strconfig: &windows_core::BSTR, strserialnumber: &windows_core::BSTR, reason: i32, date: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RevokeCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strserialnumber), reason, date) }
    }
    pub unsafe fn SetRequestAttributes(&self, strconfig: &windows_core::BSTR, requestid: i32, strattributes: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRequestAttributes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), requestid, core::mem::transmute_copy(strattributes)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetCertificateExtension(&self, strconfig: &windows_core::BSTR, requestid: i32, strextensionname: &windows_core::BSTR, r#type: i32, flags: i32, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCertificateExtension)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), requestid, core::mem::transmute_copy(strextensionname), r#type, flags, core::mem::transmute(pvarvalue)) }
    }
    pub unsafe fn DenyRequest(&self, strconfig: &windows_core::BSTR, requestid: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DenyRequest)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), requestid) }
    }
    pub unsafe fn ResubmitRequest(&self, strconfig: &windows_core::BSTR, requestid: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResubmitRequest)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), requestid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PublishCRL(&self, strconfig: &windows_core::BSTR, date: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PublishCRL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), date) }
    }
    pub unsafe fn GetCRL(&self, strconfig: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCRL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ImportCertificate(&self, strconfig: &windows_core::BSTR, strcertificate: &windows_core::BSTR, flags: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strcertificate), flags, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertAdmin_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub IsValidCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetRevocationReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RevokeCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, f64) -> windows_core::HRESULT,
    pub SetRequestAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetCertificateExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32, i32, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetCertificateExtension: usize,
    pub DenyRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ResubmitRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub PublishCRL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetCRL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertAdmin_Impl: super::oaidl::IDispatch_Impl {
    fn IsValidCertificate(&self, strconfig: &windows_core::BSTR, strserialnumber: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn GetRevocationReason(&self) -> windows_core::Result<i32>;
    fn RevokeCertificate(&self, strconfig: &windows_core::BSTR, strserialnumber: &windows_core::BSTR, reason: i32, date: f64) -> windows_core::Result<()>;
    fn SetRequestAttributes(&self, strconfig: &windows_core::BSTR, requestid: i32, strattributes: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetCertificateExtension(&self, strconfig: &windows_core::BSTR, requestid: i32, strextensionname: &windows_core::BSTR, r#type: i32, flags: i32, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn DenyRequest(&self, strconfig: &windows_core::BSTR, requestid: i32) -> windows_core::Result<()>;
    fn ResubmitRequest(&self, strconfig: &windows_core::BSTR, requestid: i32) -> windows_core::Result<i32>;
    fn PublishCRL(&self, strconfig: &windows_core::BSTR, date: f64) -> windows_core::Result<()>;
    fn GetCRL(&self, strconfig: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn ImportCertificate(&self, strconfig: &windows_core::BSTR, strcertificate: &windows_core::BSTR, flags: i32) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertAdmin_Vtbl {
    pub const fn new<Identity: ICertAdmin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsValidCertificate<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strserialnumber: *mut core::ffi::c_void, pdisposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertAdmin_Impl::IsValidCertificate(this, core::mem::transmute(&strconfig), core::mem::transmute(&strserialnumber)) {
                    Ok(ok__) => {
                        pdisposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRevocationReason<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preason: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertAdmin_Impl::GetRevocationReason(this) {
                    Ok(ok__) => {
                        preason.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RevokeCertificate<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strserialnumber: *mut core::ffi::c_void, reason: i32, date: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin_Impl::RevokeCertificate(this, core::mem::transmute(&strconfig), core::mem::transmute(&strserialnumber), core::mem::transmute_copy(&reason), core::mem::transmute_copy(&date)).into()
            }
        }
        unsafe extern "system" fn SetRequestAttributes<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, requestid: i32, strattributes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin_Impl::SetRequestAttributes(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid), core::mem::transmute(&strattributes)).into()
            }
        }
        unsafe extern "system" fn SetCertificateExtension<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, requestid: i32, strextensionname: *mut core::ffi::c_void, r#type: i32, flags: i32, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin_Impl::SetCertificateExtension(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid), core::mem::transmute(&strextensionname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn DenyRequest<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, requestid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin_Impl::DenyRequest(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid)).into()
            }
        }
        unsafe extern "system" fn ResubmitRequest<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, requestid: i32, pdisposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertAdmin_Impl::ResubmitRequest(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid)) {
                    Ok(ok__) => {
                        pdisposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PublishCRL<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, date: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin_Impl::PublishCRL(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&date)).into()
            }
        }
        unsafe extern "system" fn GetCRL<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, flags: i32, pstrcrl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertAdmin_Impl::GetCRL(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pstrcrl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportCertificate<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strcertificate: *mut core::ffi::c_void, flags: i32, prequestid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertAdmin_Impl::ImportCertificate(this, core::mem::transmute(&strconfig), core::mem::transmute(&strcertificate), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        prequestid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsValidCertificate: IsValidCertificate::<Identity, OFFSET>,
            GetRevocationReason: GetRevocationReason::<Identity, OFFSET>,
            RevokeCertificate: RevokeCertificate::<Identity, OFFSET>,
            SetRequestAttributes: SetRequestAttributes::<Identity, OFFSET>,
            SetCertificateExtension: SetCertificateExtension::<Identity, OFFSET>,
            DenyRequest: DenyRequest::<Identity, OFFSET>,
            ResubmitRequest: ResubmitRequest::<Identity, OFFSET>,
            PublishCRL: PublishCRL::<Identity, OFFSET>,
            GetCRL: GetCRL::<Identity, OFFSET>,
            ImportCertificate: ImportCertificate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertAdmin as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertAdmin {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertAdmin2, ICertAdmin2_Vtbl, 0xf7c3ac41_b8ce_4fb4_aa58_3d1dc0e36b39);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertAdmin2 {
    type Target = ICertAdmin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertAdmin2, windows_core::IUnknown, super::oaidl::IDispatch, ICertAdmin);
#[cfg(feature = "Win32_oaidl")]
impl ICertAdmin2 {
    pub unsafe fn PublishCRLs(&self, strconfig: &windows_core::BSTR, date: f64, crlflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PublishCRLs)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), date, crlflags) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetCAProperty(&self, strconfig: &windows_core::BSTR, propid: i32, propindex: i32, proptype: i32, flags: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCAProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), propid, propindex, proptype, flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetCAProperty(&self, strconfig: &windows_core::BSTR, propid: i32, propindex: i32, proptype: i32, pvarpropertyvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCAProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), propid, propindex, proptype, core::mem::transmute(pvarpropertyvalue)) }
    }
    pub unsafe fn GetCAPropertyFlags(&self, strconfig: &windows_core::BSTR, propid: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCAPropertyFlags)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), propid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCAPropertyDisplayName(&self, strconfig: &windows_core::BSTR, propid: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCAPropertyDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), propid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetArchivedKey(&self, strconfig: &windows_core::BSTR, requestid: i32, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetArchivedKey)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), requestid, flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetConfigEntry(&self, strconfig: &windows_core::BSTR, strnodepath: &windows_core::BSTR, strentryname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConfigEntry)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strnodepath), core::mem::transmute_copy(strentryname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetConfigEntry(&self, strconfig: &windows_core::BSTR, strnodepath: &windows_core::BSTR, strentryname: &windows_core::BSTR, pvarentry: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConfigEntry)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strnodepath), core::mem::transmute_copy(strentryname), core::mem::transmute(pvarentry)) }
    }
    pub unsafe fn ImportKey(&self, strconfig: &windows_core::BSTR, requestid: i32, strcerthash: &windows_core::BSTR, flags: i32, strkey: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ImportKey)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), requestid, core::mem::transmute_copy(strcerthash), flags, core::mem::transmute_copy(strkey)) }
    }
    pub unsafe fn GetMyRoles(&self, strconfig: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMyRoles)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeleteRow(&self, strconfig: &windows_core::BSTR, flags: i32, date: f64, table: i32, rowid: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeleteRow)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), flags, date, table, rowid, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertAdmin2_Vtbl {
    pub base__: ICertAdmin_Vtbl,
    pub PublishCRLs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f64, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetCAProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, i32, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetCAProperty: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetCAProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, i32, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetCAProperty: usize,
    pub GetCAPropertyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetCAPropertyDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetArchivedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetConfigEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetConfigEntry: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetConfigEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetConfigEntry: usize,
    pub ImportKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMyRoles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeleteRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, f64, i32, i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertAdmin2_Impl: ICertAdmin_Impl {
    fn PublishCRLs(&self, strconfig: &windows_core::BSTR, date: f64, crlflags: i32) -> windows_core::Result<()>;
    fn GetCAProperty(&self, strconfig: &windows_core::BSTR, propid: i32, propindex: i32, proptype: i32, flags: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetCAProperty(&self, strconfig: &windows_core::BSTR, propid: i32, propindex: i32, proptype: i32, pvarpropertyvalue: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn GetCAPropertyFlags(&self, strconfig: &windows_core::BSTR, propid: i32) -> windows_core::Result<i32>;
    fn GetCAPropertyDisplayName(&self, strconfig: &windows_core::BSTR, propid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetArchivedKey(&self, strconfig: &windows_core::BSTR, requestid: i32, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetConfigEntry(&self, strconfig: &windows_core::BSTR, strnodepath: &windows_core::BSTR, strentryname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetConfigEntry(&self, strconfig: &windows_core::BSTR, strnodepath: &windows_core::BSTR, strentryname: &windows_core::BSTR, pvarentry: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn ImportKey(&self, strconfig: &windows_core::BSTR, requestid: i32, strcerthash: &windows_core::BSTR, flags: i32, strkey: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetMyRoles(&self, strconfig: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn DeleteRow(&self, strconfig: &windows_core::BSTR, flags: i32, date: f64, table: i32, rowid: i32) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertAdmin2_Vtbl {
    pub const fn new<Identity: ICertAdmin2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PublishCRLs<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, date: f64, crlflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin2_Impl::PublishCRLs(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&date), core::mem::transmute_copy(&crlflags)).into()
            }
        }
        unsafe extern "system" fn GetCAProperty<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, propid: i32, propindex: i32, proptype: i32, flags: i32, pvarpropertyvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertAdmin2_Impl::GetCAProperty(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid), core::mem::transmute_copy(&propindex), core::mem::transmute_copy(&proptype), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pvarpropertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCAProperty<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, propid: i32, propindex: i32, proptype: i32, pvarpropertyvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin2_Impl::SetCAProperty(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid), core::mem::transmute_copy(&propindex), core::mem::transmute_copy(&proptype), core::mem::transmute_copy(&pvarpropertyvalue)).into()
            }
        }
        unsafe extern "system" fn GetCAPropertyFlags<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, propid: i32, ppropflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertAdmin2_Impl::GetCAPropertyFlags(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid)) {
                    Ok(ok__) => {
                        ppropflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCAPropertyDisplayName<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, propid: i32, pstrdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertAdmin2_Impl::GetCAPropertyDisplayName(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid)) {
                    Ok(ok__) => {
                        pstrdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetArchivedKey<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, requestid: i32, flags: i32, pstrarchivedkey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertAdmin2_Impl::GetArchivedKey(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pstrarchivedkey.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConfigEntry<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strnodepath: *mut core::ffi::c_void, strentryname: *mut core::ffi::c_void, pvarentry: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertAdmin2_Impl::GetConfigEntry(this, core::mem::transmute(&strconfig), core::mem::transmute(&strnodepath), core::mem::transmute(&strentryname)) {
                    Ok(ok__) => {
                        pvarentry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetConfigEntry<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strnodepath: *mut core::ffi::c_void, strentryname: *mut core::ffi::c_void, pvarentry: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin2_Impl::SetConfigEntry(this, core::mem::transmute(&strconfig), core::mem::transmute(&strnodepath), core::mem::transmute(&strentryname), core::mem::transmute_copy(&pvarentry)).into()
            }
        }
        unsafe extern "system" fn ImportKey<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, requestid: i32, strcerthash: *mut core::ffi::c_void, flags: i32, strkey: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin2_Impl::ImportKey(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid), core::mem::transmute(&strcerthash), core::mem::transmute_copy(&flags), core::mem::transmute(&strkey)).into()
            }
        }
        unsafe extern "system" fn GetMyRoles<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, proles: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertAdmin2_Impl::GetMyRoles(this, core::mem::transmute(&strconfig)) {
                    Ok(ok__) => {
                        proles.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteRow<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, flags: i32, date: f64, table: i32, rowid: i32, pcdeleted: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertAdmin2_Impl::DeleteRow(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&date), core::mem::transmute_copy(&table), core::mem::transmute_copy(&rowid)) {
                    Ok(ok__) => {
                        pcdeleted.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICertAdmin_Vtbl::new::<Identity, OFFSET>(),
            PublishCRLs: PublishCRLs::<Identity, OFFSET>,
            GetCAProperty: GetCAProperty::<Identity, OFFSET>,
            SetCAProperty: SetCAProperty::<Identity, OFFSET>,
            GetCAPropertyFlags: GetCAPropertyFlags::<Identity, OFFSET>,
            GetCAPropertyDisplayName: GetCAPropertyDisplayName::<Identity, OFFSET>,
            GetArchivedKey: GetArchivedKey::<Identity, OFFSET>,
            GetConfigEntry: GetConfigEntry::<Identity, OFFSET>,
            SetConfigEntry: SetConfigEntry::<Identity, OFFSET>,
            ImportKey: ImportKey::<Identity, OFFSET>,
            GetMyRoles: GetMyRoles::<Identity, OFFSET>,
            DeleteRow: DeleteRow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertAdmin2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ICertAdmin as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertAdmin2 {}
pub const IKF_OVERWRITE: u32 = 65536;
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IOCSPAdmin, IOCSPAdmin_Vtbl, 0x322e830d_67db_4fe9_9577_4596d9f09294);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IOCSPAdmin {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IOCSPAdmin, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IOCSPAdmin {
    pub unsafe fn OCSPServiceProperties(&self) -> windows_core::Result<IOCSPPropertyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OCSPServiceProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OCSPCAConfigurationCollection(&self) -> windows_core::Result<IOCSPCAConfigurationCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OCSPCAConfigurationCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn GetConfiguration(&self, bstrservername: &windows_core::BSTR, bforce: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetConfiguration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername), bforce) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetConfiguration(&self, bstrservername: &windows_core::BSTR, bforce: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConfiguration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername), bforce) }
    }
    pub unsafe fn GetMyRoles(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMyRoles)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Ping(&self, bstrservername: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Ping)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername)) }
    }
    pub unsafe fn SetSecurity(&self, bstrservername: &windows_core::BSTR, bstrval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername), core::mem::transmute_copy(bstrval)) }
    }
    pub unsafe fn GetSecurity(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurity)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetSigningCertificates(&self, bstrservername: &windows_core::BSTR, pcacertvar: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSigningCertificates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername), core::mem::transmute(pcacertvar), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetHashAlgorithms(&self, bstrservername: &windows_core::BSTR, bstrcaid: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHashAlgorithms)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername), core::mem::transmute_copy(bstrcaid), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOCSPAdmin_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub OCSPServiceProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OCSPCAConfigurationCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub GetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    GetConfiguration: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetConfiguration: usize,
    pub GetMyRoles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Ping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetSigningCertificates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetSigningCertificates: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetHashAlgorithms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetHashAlgorithms: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IOCSPAdmin_Impl: super::oaidl::IDispatch_Impl {
    fn OCSPServiceProperties(&self) -> windows_core::Result<IOCSPPropertyCollection>;
    fn OCSPCAConfigurationCollection(&self) -> windows_core::Result<IOCSPCAConfigurationCollection>;
    fn GetConfiguration(&self, bstrservername: &windows_core::BSTR, bforce: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetConfiguration(&self, bstrservername: &windows_core::BSTR, bforce: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetMyRoles(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn Ping(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSecurity(&self, bstrservername: &windows_core::BSTR, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetSecurity(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetSigningCertificates(&self, bstrservername: &windows_core::BSTR, pcacertvar: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetHashAlgorithms(&self, bstrservername: &windows_core::BSTR, bstrcaid: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IOCSPAdmin_Vtbl {
    pub const fn new<Identity: IOCSPAdmin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OCSPServiceProperties<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPAdmin_Impl::OCSPServiceProperties(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OCSPCAConfigurationCollection<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPAdmin_Impl::OCSPCAConfigurationCollection(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConfiguration<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, bforce: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPAdmin_Impl::GetConfiguration(this, core::mem::transmute(&bstrservername), core::mem::transmute_copy(&bforce)).into()
            }
        }
        unsafe extern "system" fn SetConfiguration<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, bforce: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPAdmin_Impl::SetConfiguration(this, core::mem::transmute(&bstrservername), core::mem::transmute_copy(&bforce)).into()
            }
        }
        unsafe extern "system" fn GetMyRoles<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, proles: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPAdmin_Impl::GetMyRoles(this, core::mem::transmute(&bstrservername)) {
                    Ok(ok__) => {
                        proles.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Ping<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPAdmin_Impl::Ping(this, core::mem::transmute(&bstrservername)).into()
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPAdmin_Impl::SetSecurity(this, core::mem::transmute(&bstrservername), core::mem::transmute(&bstrval)).into()
            }
        }
        unsafe extern "system" fn GetSecurity<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPAdmin_Impl::GetSecurity(this, core::mem::transmute(&bstrservername)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSigningCertificates<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, pcacertvar: *const super::oaidl::VARIANT, pval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPAdmin_Impl::GetSigningCertificates(this, core::mem::transmute(&bstrservername), core::mem::transmute_copy(&pcacertvar)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHashAlgorithms<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, bstrcaid: *mut core::ffi::c_void, pval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPAdmin_Impl::GetHashAlgorithms(this, core::mem::transmute(&bstrservername), core::mem::transmute(&bstrcaid)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OCSPServiceProperties: OCSPServiceProperties::<Identity, OFFSET>,
            OCSPCAConfigurationCollection: OCSPCAConfigurationCollection::<Identity, OFFSET>,
            GetConfiguration: GetConfiguration::<Identity, OFFSET>,
            SetConfiguration: SetConfiguration::<Identity, OFFSET>,
            GetMyRoles: GetMyRoles::<Identity, OFFSET>,
            Ping: Ping::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
            GetSecurity: GetSecurity::<Identity, OFFSET>,
            GetSigningCertificates: GetSigningCertificates::<Identity, OFFSET>,
            GetHashAlgorithms: GetHashAlgorithms::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPAdmin as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IOCSPAdmin {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IOCSPCAConfiguration, IOCSPCAConfiguration_Vtbl, 0xaec92b40_3d46_433f_87d1_b84d5c1e790d);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IOCSPCAConfiguration {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IOCSPCAConfiguration, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IOCSPCAConfiguration {
    pub unsafe fn Identifier(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Identifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn CACertificate(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CACertificate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetHashAlgorithm(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    pub unsafe fn SigningFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SigningFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSigningFlags(&self, newval: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSigningFlags)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SigningCertificate(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SigningCertificate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetSigningCertificate(&self, newval: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSigningCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    pub unsafe fn ReminderDuration(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReminderDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReminderDuration(&self, newval: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReminderDuration)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn ErrorCode(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ErrorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CSPName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CSPName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn KeySpec(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeySpec)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProviderCLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProviderCLSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetProviderCLSID(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProviderCLSID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn ProviderProperties(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProviderProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetProviderProperties(&self, newval: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProviderProperties)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn Modified(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Modified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn LocalRevocationInformation(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalRevocationInformation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetLocalRevocationInformation(&self, newval: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLocalRevocationInformation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    pub unsafe fn SigningCertificateTemplate(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SigningCertificateTemplate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSigningCertificateTemplate(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSigningCertificateTemplate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    pub unsafe fn CAConfig(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CAConfig)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCAConfig(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCAConfig)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOCSPCAConfiguration_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Identifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub CACertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    CACertificate: usize,
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SigningFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSigningFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SigningCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SigningCertificate: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetSigningCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetSigningCertificate: usize,
    pub ReminderDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReminderDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CSPName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KeySpec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ProviderCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProviderCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub ProviderProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    ProviderProperties: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetProviderProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetProviderProperties: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    Modified: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub LocalRevocationInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    LocalRevocationInformation: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetLocalRevocationInformation: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetLocalRevocationInformation: usize,
    pub SigningCertificateTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSigningCertificateTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CAConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCAConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IOCSPCAConfiguration_Impl: super::oaidl::IDispatch_Impl {
    fn Identifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CACertificate(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn HashAlgorithm(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHashAlgorithm(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SigningFlags(&self) -> windows_core::Result<u32>;
    fn SetSigningFlags(&self, newval: u32) -> windows_core::Result<()>;
    fn SigningCertificate(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetSigningCertificate(&self, newval: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn ReminderDuration(&self) -> windows_core::Result<u32>;
    fn SetReminderDuration(&self, newval: u32) -> windows_core::Result<()>;
    fn ErrorCode(&self) -> windows_core::Result<u32>;
    fn CSPName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn KeySpec(&self) -> windows_core::Result<u32>;
    fn ProviderCLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProviderCLSID(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ProviderProperties(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetProviderProperties(&self, newval: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Modified(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn LocalRevocationInformation(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetLocalRevocationInformation(&self, newval: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn SigningCertificateTemplate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSigningCertificateTemplate(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CAConfig(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCAConfig(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IOCSPCAConfiguration_Vtbl {
    pub const fn new<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Identifier<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::Identifier(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CACertificate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::CACertificate(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HashAlgorithm<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::HashAlgorithm(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::SetHashAlgorithm(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn SigningFlags<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::SigningFlags(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSigningFlags<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::SetSigningFlags(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn SigningCertificate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::SigningCertificate(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSigningCertificate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::SetSigningCertificate(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn ReminderDuration<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::ReminderDuration(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReminderDuration<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::SetReminderDuration(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn ErrorCode<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::ErrorCode(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CSPName<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::CSPName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeySpec<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::KeySpec(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProviderCLSID<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::ProviderCLSID(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProviderCLSID<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::SetProviderCLSID(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn ProviderProperties<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::ProviderProperties(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProviderProperties<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::SetProviderProperties(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn Modified<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::Modified(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LocalRevocationInformation<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::LocalRevocationInformation(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalRevocationInformation<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::SetLocalRevocationInformation(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn SigningCertificateTemplate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::SigningCertificateTemplate(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSigningCertificateTemplate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::SetSigningCertificateTemplate(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn CAConfig<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::CAConfig(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCAConfig<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::SetCAConfig(this, core::mem::transmute(&newval)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Identifier: Identifier::<Identity, OFFSET>,
            CACertificate: CACertificate::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            SigningFlags: SigningFlags::<Identity, OFFSET>,
            SetSigningFlags: SetSigningFlags::<Identity, OFFSET>,
            SigningCertificate: SigningCertificate::<Identity, OFFSET>,
            SetSigningCertificate: SetSigningCertificate::<Identity, OFFSET>,
            ReminderDuration: ReminderDuration::<Identity, OFFSET>,
            SetReminderDuration: SetReminderDuration::<Identity, OFFSET>,
            ErrorCode: ErrorCode::<Identity, OFFSET>,
            CSPName: CSPName::<Identity, OFFSET>,
            KeySpec: KeySpec::<Identity, OFFSET>,
            ProviderCLSID: ProviderCLSID::<Identity, OFFSET>,
            SetProviderCLSID: SetProviderCLSID::<Identity, OFFSET>,
            ProviderProperties: ProviderProperties::<Identity, OFFSET>,
            SetProviderProperties: SetProviderProperties::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
            LocalRevocationInformation: LocalRevocationInformation::<Identity, OFFSET>,
            SetLocalRevocationInformation: SetLocalRevocationInformation::<Identity, OFFSET>,
            SigningCertificateTemplate: SigningCertificateTemplate::<Identity, OFFSET>,
            SetSigningCertificateTemplate: SetSigningCertificateTemplate::<Identity, OFFSET>,
            CAConfig: CAConfig::<Identity, OFFSET>,
            SetCAConfig: SetCAConfig::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPCAConfiguration as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IOCSPCAConfiguration {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IOCSPCAConfigurationCollection, IOCSPCAConfigurationCollection_Vtbl, 0x2bebea0b_5ece_4f28_a91c_86b4bb20f0d3);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IOCSPCAConfigurationCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IOCSPCAConfigurationCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IOCSPCAConfigurationCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn ItemByName(&self, bstridentifier: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ItemByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstridentifier), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn CreateCAConfiguration(&self, bstridentifier: &windows_core::BSTR, varcacert: &super::oaidl::VARIANT) -> windows_core::Result<IOCSPCAConfiguration> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCAConfiguration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstridentifier), core::mem::transmute_copy(varcacert), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteCAConfiguration(&self, bstridentifier: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteCAConfiguration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstridentifier)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOCSPCAConfigurationCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub ItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    ItemByName: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub CreateCAConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    CreateCAConfiguration: usize,
    pub DeleteCAConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IOCSPCAConfigurationCollection_Impl: super::oaidl::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn ItemByName(&self, bstridentifier: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn CreateCAConfiguration(&self, bstridentifier: &windows_core::BSTR, varcacert: &super::oaidl::VARIANT) -> windows_core::Result<IOCSPCAConfiguration>;
    fn DeleteCAConfiguration(&self, bstridentifier: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IOCSPCAConfigurationCollection_Vtbl {
    pub const fn new<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfigurationCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfigurationCollection_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfigurationCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ItemByName<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstridentifier: *mut core::ffi::c_void, pval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfigurationCollection_Impl::ItemByName(this, core::mem::transmute(&bstridentifier)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCAConfiguration<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstridentifier: *mut core::ffi::c_void, varcacert: super::oaidl::VARIANT, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfigurationCollection_Impl::CreateCAConfiguration(this, core::mem::transmute(&bstridentifier), core::mem::transmute(&varcacert)) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteCAConfiguration<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstridentifier: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfigurationCollection_Impl::DeleteCAConfiguration(this, core::mem::transmute(&bstridentifier)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            ItemByName: ItemByName::<Identity, OFFSET>,
            CreateCAConfiguration: CreateCAConfiguration::<Identity, OFFSET>,
            DeleteCAConfiguration: DeleteCAConfiguration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPCAConfigurationCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IOCSPCAConfigurationCollection {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IOCSPProperty, IOCSPProperty_Vtbl, 0x66fb7839_5f04_4c25_ad18_9ff1a8376ee0);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IOCSPProperty {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IOCSPProperty, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IOCSPProperty {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetValue(&self, newval: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn Modified(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Modified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOCSPProperty_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Value: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetValue: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    Modified: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IOCSPProperty_Impl: super::oaidl::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Value(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetValue(&self, newval: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Modified(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IOCSPProperty_Vtbl {
    pub const fn new<Identity: IOCSPProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IOCSPProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPProperty_Impl::Name(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Value<Identity: IOCSPProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPProperty_Impl::Value(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IOCSPProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPProperty_Impl::SetValue(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn Modified<Identity: IOCSPProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPProperty_Impl::Modified(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPProperty as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IOCSPProperty {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IOCSPPropertyCollection, IOCSPPropertyCollection_Vtbl, 0x2597c18d_54e6_4b74_9fa9_a6bfda99cbbe);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IOCSPPropertyCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IOCSPPropertyCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IOCSPPropertyCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn ItemByName(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ItemByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn CreateProperty(&self, bstrpropname: &windows_core::BSTR, pvarpropvalue: *const super::oaidl::VARIANT) -> windows_core::Result<IOCSPProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), core::mem::transmute(pvarpropvalue), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteProperty(&self, bstrpropname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn InitializeFromProperties(&self, pvarproperties: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarproperties)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetAllProperties(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOCSPPropertyCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub ItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    ItemByName: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub CreateProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    CreateProperty: usize,
    pub DeleteProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub InitializeFromProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    InitializeFromProperties: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetAllProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetAllProperties: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IOCSPPropertyCollection_Impl: super::oaidl::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn ItemByName(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn CreateProperty(&self, bstrpropname: &windows_core::BSTR, pvarpropvalue: *const super::oaidl::VARIANT) -> windows_core::Result<IOCSPProperty>;
    fn DeleteProperty(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromProperties(&self, pvarproperties: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn GetAllProperties(&self) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IOCSPPropertyCollection_Vtbl {
    pub const fn new<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPPropertyCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPPropertyCollection_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPPropertyCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ItemByName<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, pval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPPropertyCollection_Impl::ItemByName(this, core::mem::transmute(&bstrpropname)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateProperty<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, pvarpropvalue: *const super::oaidl::VARIANT, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPPropertyCollection_Impl::CreateProperty(this, core::mem::transmute(&bstrpropname), core::mem::transmute_copy(&pvarpropvalue)) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteProperty<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPPropertyCollection_Impl::DeleteProperty(this, core::mem::transmute(&bstrpropname)).into()
            }
        }
        unsafe extern "system" fn InitializeFromProperties<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarproperties: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPPropertyCollection_Impl::InitializeFromProperties(this, core::mem::transmute_copy(&pvarproperties)).into()
            }
        }
        unsafe extern "system" fn GetAllProperties<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarproperties: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPPropertyCollection_Impl::GetAllProperties(this) {
                    Ok(ok__) => {
                        pvarproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            ItemByName: ItemByName::<Identity, OFFSET>,
            CreateProperty: CreateProperty::<Identity, OFFSET>,
            DeleteProperty: DeleteProperty::<Identity, OFFSET>,
            InitializeFromProperties: InitializeFromProperties::<Identity, OFFSET>,
            GetAllProperties: GetAllProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPPropertyCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IOCSPPropertyCollection {}
pub const KRA_DISP_EXPIRED: u32 = 0;
pub const KRA_DISP_INVALID: u32 = 4;
pub const KRA_DISP_NOTFOUND: u32 = 1;
pub const KRA_DISP_NOTLOADED: u32 = 6;
pub const KRA_DISP_REVOKED: u32 = 2;
pub const KRA_DISP_UNTRUSTED: u32 = 5;
pub const KRA_DISP_VALID: u32 = 3;
pub const OCSPAdmin: windows_core::GUID = windows_core::GUID::from_u128(0xd3f73511_92c9_47cb_8ff2_8d891a7c4de4);
pub const OCSPPropertyCollection: windows_core::GUID = windows_core::GUID::from_u128(0xf935a528_ba8a_4dd9_ba79_f283275cb2de);
pub type OCSPRequestFlag = i32;
pub type OCSPSigningFlag = i32;
pub const OCSP_RF_REJECT_SIGNED_REQUESTS: OCSPRequestFlag = 1;
pub const OCSP_SF_ALLOW_NONCE_EXTENSION: OCSPSigningFlag = 256;
pub const OCSP_SF_ALLOW_SIGNINGCERT_AUTOENROLLMENT: OCSPSigningFlag = 512;
pub const OCSP_SF_ALLOW_SIGNINGCERT_AUTORENEWAL: OCSPSigningFlag = 4;
pub const OCSP_SF_AUTODISCOVER_SIGNINGCERT: OCSPSigningFlag = 16;
pub const OCSP_SF_FORCE_SIGNINGCERT_ISSUER_ISCA: OCSPSigningFlag = 8;
pub const OCSP_SF_MANUAL_ASSIGN_SIGNINGCERT: OCSPSigningFlag = 32;
pub const OCSP_SF_RESPONDER_ID_KEYHASH: OCSPSigningFlag = 64;
pub const OCSP_SF_RESPONDER_ID_NAME: OCSPSigningFlag = 128;
pub const OCSP_SF_SILENT: OCSPSigningFlag = 1;
pub const OCSP_SF_USE_CACERT: OCSPSigningFlag = 2;
pub const wszOCSPCAPROP_CACERTIFICATE: windows_core::PCWSTR = windows_core::w!("CACertificate");
pub const wszOCSPCAPROP_CACONFIG: windows_core::PCWSTR = windows_core::w!("CAConfig");
pub const wszOCSPCAPROP_CSPNAME: windows_core::PCWSTR = windows_core::w!("CSPName");
pub const wszOCSPCAPROP_ERRORCODE: windows_core::PCWSTR = windows_core::w!("ErrorCode");
pub const wszOCSPCAPROP_HASHALGORITHMID: windows_core::PCWSTR = windows_core::w!("HashAlgorithmId");
pub const wszOCSPCAPROP_KEYSPEC: windows_core::PCWSTR = windows_core::w!("KeySpec");
pub const wszOCSPCAPROP_LOCALREVOCATIONINFORMATION: windows_core::PCWSTR = windows_core::w!("LocalRevocationInformation");
pub const wszOCSPCAPROP_PROVIDERCLSID: windows_core::PCWSTR = windows_core::w!("ProviderCLSID");
pub const wszOCSPCAPROP_PROVIDERPROPERTIES: windows_core::PCWSTR = windows_core::w!("Provider");
pub const wszOCSPCAPROP_REMINDERDURATION: windows_core::PCWSTR = windows_core::w!("ReminderDuration");
pub const wszOCSPCAPROP_SIGNINGCERTIFICATE: windows_core::PCWSTR = windows_core::w!("SigningCertificate");
pub const wszOCSPCAPROP_SIGNINGCERTIFICATETEMPLATE: windows_core::PCWSTR = windows_core::w!("SigningCertificateTemplate");
pub const wszOCSPCAPROP_SIGNINGFLAGS: windows_core::PCWSTR = windows_core::w!("SigningFlags");
pub const wszOCSPCOMMONPROP_MAXINCOMINGMESSAGESIZE: windows_core::PCWSTR = windows_core::w!("MaxIncomingMessageSize");
pub const wszOCSPCOMMONPROP_MAXNUMOFREQUESTENTRIES: windows_core::PCWSTR = windows_core::w!("MaxNumOfRequestEntries");
pub const wszOCSPCOMMONPROP_REQFLAGS: windows_core::PCWSTR = windows_core::w!("RequestFlags");
pub const wszOCSPISAPIPROP_DEBUG: windows_core::PCWSTR = windows_core::w!("ISAPIDebug");
pub const wszOCSPISAPIPROP_MAXAGE: windows_core::PCWSTR = windows_core::w!("MaxAge");
pub const wszOCSPISAPIPROP_MAXNUMOFCACHEENTRIES: windows_core::PCWSTR = windows_core::w!("MaxNumOfCacheEntries");
pub const wszOCSPISAPIPROP_NUMOFBACKENDCONNECTIONS: windows_core::PCWSTR = windows_core::w!("NumOfBackendConnections");
pub const wszOCSPISAPIPROP_NUMOFTHREADS: windows_core::PCWSTR = windows_core::w!("NumOfThreads");
pub const wszOCSPISAPIPROP_REFRESHRATE: windows_core::PCWSTR = windows_core::w!("RefreshRate");
pub const wszOCSPISAPIPROP_VIRTUALROOTNAME: windows_core::PCWSTR = windows_core::w!("VirtualRootName");
pub const wszOCSPPROP_ARRAYCONTROLLER: windows_core::PCWSTR = windows_core::w!("ArrayController");
pub const wszOCSPPROP_ARRAYMEMBERS: windows_core::PCWSTR = windows_core::w!("ArrayMembers");
pub const wszOCSPPROP_AUDITFILTER: windows_core::PCWSTR = windows_core::w!("AuditFilter");
pub const wszOCSPPROP_DEBUG: windows_core::PCWSTR = windows_core::w!("Debug");
pub const wszOCSPPROP_ENROLLPOLLINTERVAL: windows_core::PCWSTR = windows_core::w!("EnrollPollInterval");
pub const wszOCSPPROP_LOGLEVEL: windows_core::PCWSTR = windows_core::w!("LogLevel");
pub const wszOCSPREVPROP_ALLOWCAONLYCRLS: windows_core::PCWSTR = windows_core::w!("AllowCAOnlyCrls");
pub const wszOCSPREVPROP_ALLOWUSERONLYCRLS: windows_core::PCWSTR = windows_core::w!("AllowUserOnlyCrls");
pub const wszOCSPREVPROP_BASECRL: windows_core::PCWSTR = windows_core::w!("BaseCrl");
pub const wszOCSPREVPROP_BASECRLURLS: windows_core::PCWSTR = windows_core::w!("BaseCrlUrls");
pub const wszOCSPREVPROP_CRLURLTIMEOUT: windows_core::PCWSTR = windows_core::w!("CrlUrlTimeOut");
pub const wszOCSPREVPROP_DELTACRL: windows_core::PCWSTR = windows_core::w!("DeltaCrl");
pub const wszOCSPREVPROP_DELTACRLURLS: windows_core::PCWSTR = windows_core::w!("DeltaCrlUrls");
pub const wszOCSPREVPROP_ERRORCODE: windows_core::PCWSTR = windows_core::w!("RevocationErrorCode");
pub const wszOCSPREVPROP_REFRESHTIMEOUT: windows_core::PCWSTR = windows_core::w!("RefreshTimeOut");
pub const wszOCSPREVPROP_SERIALNUMBERSDIRS: windows_core::PCWSTR = windows_core::w!("IssuedSerialNumbersDirectories");
