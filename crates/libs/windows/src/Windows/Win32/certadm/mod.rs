pub const CA_ACCESS_ADMIN: i32 = 1;
pub const CA_ACCESS_AUDITOR: i32 = 4;
pub const CA_ACCESS_ENROLL: i32 = 512;
pub const CA_ACCESS_MASKROLES: i32 = 255;
pub const CA_ACCESS_OFFICER: i32 = 2;
pub const CA_ACCESS_OPERATOR: i32 = 8;
pub const CA_ACCESS_READ: i32 = 256;
pub const CA_CRL_BASE: i32 = 1;
pub const CA_CRL_DELTA: i32 = 2;
pub const CA_CRL_REPUBLISH: i32 = 16;
pub const CA_DISP_ERROR: i32 = 1;
pub const CA_DISP_INCOMPLETE: i32 = 0;
pub const CA_DISP_INVALID: i32 = 4;
pub const CA_DISP_REVOKED: i32 = 2;
pub const CA_DISP_UNDER_SUBMISSION: i32 = 5;
pub const CA_DISP_VALID: i32 = 3;
pub const CCertAdmin: windows_core::GUID = windows_core::GUID::from_u128(0x37eabaf0_7fb6_11d0_8817_00a0c903b83c);
pub const CCertView: windows_core::GUID = windows_core::GUID::from_u128(0xa12d0f7a_1e84_11d1_9bd6_00c04fb683fa);
pub const CDR_EXPIRED: i32 = 1;
pub const CDR_REQUEST_LAST_CHANGED: i32 = 2;
pub const ICF_ALLOWFOREIGN: i32 = 65536;
pub const ICF_EXISTINGROW: i32 = 131072;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertAdmin, ICertAdmin_Vtbl, 0x34df6950_7fb6_11d0_8817_00a0c903b83c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertAdmin {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertAdmin, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ICertAdmin {
    #[cfg(feature = "shared")]
    pub unsafe fn IsValidCertificate(&self, strconfig: &windows_core::BSTR, strserialnumber: &windows_core::BSTR) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsValidCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strserialnumber), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRevocationReason(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRevocationReason)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn RevokeCertificate(&self, strconfig: &windows_core::BSTR, strserialnumber: &windows_core::BSTR, reason: super::LONG, date: super::DATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RevokeCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strserialnumber), reason, date) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetRequestAttributes(&self, strconfig: &windows_core::BSTR, requestid: super::LONG, strattributes: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRequestAttributes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), requestid, core::mem::transmute_copy(strattributes)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetCertificateExtension(&self, strconfig: &windows_core::BSTR, requestid: super::LONG, strextensionname: &windows_core::BSTR, r#type: super::LONG, flags: super::LONG, pvarvalue: *const super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCertificateExtension)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), requestid, core::mem::transmute_copy(strextensionname), r#type, flags, pvarvalue) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DenyRequest(&self, strconfig: &windows_core::BSTR, requestid: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DenyRequest)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), requestid) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ResubmitRequest(&self, strconfig: &windows_core::BSTR, requestid: super::LONG) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResubmitRequest)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), requestid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn PublishCRL(&self, strconfig: &windows_core::BSTR, date: super::DATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PublishCRL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), date) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCRL(&self, strconfig: &windows_core::BSTR, flags: super::LONG) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCRL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ImportCertificate(&self, strconfig: &windows_core::BSTR, strcertificate: &windows_core::BSTR, flags: super::LONG) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strcertificate), flags, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertAdmin_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub IsValidCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    IsValidCertificate: usize,
    #[cfg(feature = "shared")]
    pub GetRevocationReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRevocationReason: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub RevokeCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, super::DATE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    RevokeCertificate: usize,
    #[cfg(feature = "shared")]
    pub SetRequestAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRequestAttributes: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetCertificateExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, super::LONG, super::LONG, *const super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetCertificateExtension: usize,
    #[cfg(feature = "shared")]
    pub DenyRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DenyRequest: usize,
    #[cfg(feature = "shared")]
    pub ResubmitRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ResubmitRequest: usize,
    #[cfg(feature = "wtypes")]
    pub PublishCRL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    PublishCRL: usize,
    #[cfg(feature = "shared")]
    pub GetCRL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCRL: usize,
    #[cfg(feature = "shared")]
    pub ImportCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ImportCertificate: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertAdmin_Impl: super::IDispatch_Impl {
    fn IsValidCertificate(&self, strconfig: &windows_core::BSTR, strserialnumber: &windows_core::BSTR) -> windows_core::Result<super::LONG>;
    fn GetRevocationReason(&self) -> windows_core::Result<super::LONG>;
    fn RevokeCertificate(&self, strconfig: &windows_core::BSTR, strserialnumber: &windows_core::BSTR, reason: super::LONG, date: super::DATE) -> windows_core::Result<()>;
    fn SetRequestAttributes(&self, strconfig: &windows_core::BSTR, requestid: super::LONG, strattributes: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetCertificateExtension(&self, strconfig: &windows_core::BSTR, requestid: super::LONG, strextensionname: &windows_core::BSTR, r#type: super::LONG, flags: super::LONG, pvarvalue: *const super::VARIANT) -> windows_core::Result<()>;
    fn DenyRequest(&self, strconfig: &windows_core::BSTR, requestid: super::LONG) -> windows_core::Result<()>;
    fn ResubmitRequest(&self, strconfig: &windows_core::BSTR, requestid: super::LONG) -> windows_core::Result<super::LONG>;
    fn PublishCRL(&self, strconfig: &windows_core::BSTR, date: super::DATE) -> windows_core::Result<()>;
    fn GetCRL(&self, strconfig: &windows_core::BSTR, flags: super::LONG) -> windows_core::Result<windows_core::BSTR>;
    fn ImportCertificate(&self, strconfig: &windows_core::BSTR, strcertificate: &windows_core::BSTR, flags: super::LONG) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertAdmin_Vtbl {
    pub const fn new<Identity: ICertAdmin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsValidCertificate<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strserialnumber: *mut core::ffi::c_void, pdisposition: *mut super::LONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetRevocationReason<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preason: *mut super::LONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn RevokeCertificate<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strserialnumber: *mut core::ffi::c_void, reason: super::LONG, date: super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin_Impl::RevokeCertificate(this, core::mem::transmute(&strconfig), core::mem::transmute(&strserialnumber), core::mem::transmute_copy(&reason), core::mem::transmute_copy(&date)).into()
            }
        }
        unsafe extern "system" fn SetRequestAttributes<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, requestid: super::LONG, strattributes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin_Impl::SetRequestAttributes(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid), core::mem::transmute(&strattributes)).into()
            }
        }
        unsafe extern "system" fn SetCertificateExtension<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, requestid: super::LONG, strextensionname: *mut core::ffi::c_void, r#type: super::LONG, flags: super::LONG, pvarvalue: *const super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin_Impl::SetCertificateExtension(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid), core::mem::transmute(&strextensionname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn DenyRequest<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, requestid: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin_Impl::DenyRequest(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid)).into()
            }
        }
        unsafe extern "system" fn ResubmitRequest<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, requestid: super::LONG, pdisposition: *mut super::LONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn PublishCRL<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, date: super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin_Impl::PublishCRL(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&date)).into()
            }
        }
        unsafe extern "system" fn GetCRL<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, flags: super::LONG, pstrcrl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ImportCertificate<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strcertificate: *mut core::ffi::c_void, flags: super::LONG, prequestid: *mut super::LONG) -> windows_core::HRESULT {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<ICertAdmin as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertAdmin {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertAdmin2, ICertAdmin2_Vtbl, 0xf7c3ac41_b8ce_4fb4_aa58_3d1dc0e36b39);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertAdmin2 {
    type Target = ICertAdmin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertAdmin2, windows_core::IUnknown, super::IDispatch, ICertAdmin);
#[cfg(feature = "oaidl")]
impl ICertAdmin2 {
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn PublishCRLs(&self, strconfig: &windows_core::BSTR, date: super::DATE, crlflags: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PublishCRLs)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), date, crlflags) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCAProperty(&self, strconfig: &windows_core::BSTR, propid: super::LONG, propindex: super::LONG, proptype: super::LONG, flags: super::LONG) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCAProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), propid, propindex, proptype, flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetCAProperty(&self, strconfig: &windows_core::BSTR, propid: super::LONG, propindex: super::LONG, proptype: super::LONG, pvarpropertyvalue: *const super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCAProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), propid, propindex, proptype, pvarpropertyvalue) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCAPropertyFlags(&self, strconfig: &windows_core::BSTR, propid: super::LONG) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCAPropertyFlags)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), propid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCAPropertyDisplayName(&self, strconfig: &windows_core::BSTR, propid: super::LONG) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCAPropertyDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), propid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetArchivedKey(&self, strconfig: &windows_core::BSTR, requestid: super::LONG, flags: super::LONG) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetArchivedKey)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), requestid, flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetConfigEntry(&self, strconfig: &windows_core::BSTR, strnodepath: &windows_core::BSTR, strentryname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConfigEntry)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strnodepath), core::mem::transmute_copy(strentryname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetConfigEntry(&self, strconfig: &windows_core::BSTR, strnodepath: &windows_core::BSTR, strentryname: &windows_core::BSTR, pvarentry: *const super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConfigEntry)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strnodepath), core::mem::transmute_copy(strentryname), pvarentry) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ImportKey(&self, strconfig: &windows_core::BSTR, requestid: super::LONG, strcerthash: &windows_core::BSTR, flags: super::LONG, strkey: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ImportKey)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), requestid, core::mem::transmute_copy(strcerthash), flags, core::mem::transmute_copy(strkey)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMyRoles(&self, strconfig: &windows_core::BSTR) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMyRoles)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn DeleteRow(&self, strconfig: &windows_core::BSTR, flags: super::LONG, date: super::DATE, table: super::LONG, rowid: super::LONG) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeleteRow)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), flags, date, table, rowid, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertAdmin2_Vtbl {
    pub base__: ICertAdmin_Vtbl,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub PublishCRLs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DATE, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    PublishCRLs: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCAProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, super::LONG, super::LONG, super::LONG, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetCAProperty: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetCAProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, super::LONG, super::LONG, *const super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetCAProperty: usize,
    #[cfg(feature = "shared")]
    pub GetCAPropertyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCAPropertyFlags: usize,
    #[cfg(feature = "shared")]
    pub GetCAPropertyDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCAPropertyDisplayName: usize,
    #[cfg(feature = "shared")]
    pub GetArchivedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetArchivedKey: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetConfigEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetConfigEntry: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetConfigEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetConfigEntry: usize,
    #[cfg(feature = "shared")]
    pub ImportKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ImportKey: usize,
    #[cfg(feature = "shared")]
    pub GetMyRoles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMyRoles: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub DeleteRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, super::DATE, super::LONG, super::LONG, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    DeleteRow: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertAdmin2_Impl: ICertAdmin_Impl {
    fn PublishCRLs(&self, strconfig: &windows_core::BSTR, date: super::DATE, crlflags: super::LONG) -> windows_core::Result<()>;
    fn GetCAProperty(&self, strconfig: &windows_core::BSTR, propid: super::LONG, propindex: super::LONG, proptype: super::LONG, flags: super::LONG) -> windows_core::Result<super::VARIANT>;
    fn SetCAProperty(&self, strconfig: &windows_core::BSTR, propid: super::LONG, propindex: super::LONG, proptype: super::LONG, pvarpropertyvalue: *const super::VARIANT) -> windows_core::Result<()>;
    fn GetCAPropertyFlags(&self, strconfig: &windows_core::BSTR, propid: super::LONG) -> windows_core::Result<super::LONG>;
    fn GetCAPropertyDisplayName(&self, strconfig: &windows_core::BSTR, propid: super::LONG) -> windows_core::Result<windows_core::BSTR>;
    fn GetArchivedKey(&self, strconfig: &windows_core::BSTR, requestid: super::LONG, flags: super::LONG) -> windows_core::Result<windows_core::BSTR>;
    fn GetConfigEntry(&self, strconfig: &windows_core::BSTR, strnodepath: &windows_core::BSTR, strentryname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT>;
    fn SetConfigEntry(&self, strconfig: &windows_core::BSTR, strnodepath: &windows_core::BSTR, strentryname: &windows_core::BSTR, pvarentry: *const super::VARIANT) -> windows_core::Result<()>;
    fn ImportKey(&self, strconfig: &windows_core::BSTR, requestid: super::LONG, strcerthash: &windows_core::BSTR, flags: super::LONG, strkey: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetMyRoles(&self, strconfig: &windows_core::BSTR) -> windows_core::Result<super::LONG>;
    fn DeleteRow(&self, strconfig: &windows_core::BSTR, flags: super::LONG, date: super::DATE, table: super::LONG, rowid: super::LONG) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertAdmin2_Vtbl {
    pub const fn new<Identity: ICertAdmin2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PublishCRLs<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, date: super::DATE, crlflags: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin2_Impl::PublishCRLs(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&date), core::mem::transmute_copy(&crlflags)).into()
            }
        }
        unsafe extern "system" fn GetCAProperty<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, propid: super::LONG, propindex: super::LONG, proptype: super::LONG, flags: super::LONG, pvarpropertyvalue: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetCAProperty<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, propid: super::LONG, propindex: super::LONG, proptype: super::LONG, pvarpropertyvalue: *const super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin2_Impl::SetCAProperty(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid), core::mem::transmute_copy(&propindex), core::mem::transmute_copy(&proptype), core::mem::transmute_copy(&pvarpropertyvalue)).into()
            }
        }
        unsafe extern "system" fn GetCAPropertyFlags<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, propid: super::LONG, ppropflags: *mut super::LONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetCAPropertyDisplayName<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, propid: super::LONG, pstrdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetArchivedKey<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, requestid: super::LONG, flags: super::LONG, pstrarchivedkey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetConfigEntry<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strnodepath: *mut core::ffi::c_void, strentryname: *mut core::ffi::c_void, pvarentry: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetConfigEntry<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strnodepath: *mut core::ffi::c_void, strentryname: *mut core::ffi::c_void, pvarentry: *const super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin2_Impl::SetConfigEntry(this, core::mem::transmute(&strconfig), core::mem::transmute(&strnodepath), core::mem::transmute(&strentryname), core::mem::transmute_copy(&pvarentry)).into()
            }
        }
        unsafe extern "system" fn ImportKey<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, requestid: super::LONG, strcerthash: *mut core::ffi::c_void, flags: super::LONG, strkey: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertAdmin2_Impl::ImportKey(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid), core::mem::transmute(&strcerthash), core::mem::transmute_copy(&flags), core::mem::transmute(&strkey)).into()
            }
        }
        unsafe extern "system" fn GetMyRoles<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, proles: *mut super::LONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn DeleteRow<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, flags: super::LONG, date: super::DATE, table: super::LONG, rowid: super::LONG, pcdeleted: *mut super::LONG) -> windows_core::HRESULT {
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
        iid == &<ICertAdmin2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ICertAdmin as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertAdmin2 {}
pub const IKF_OVERWRITE: i32 = 65536;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IOCSPAdmin, IOCSPAdmin_Vtbl, 0x322e830d_67db_4fe9_9577_4596d9f09294);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IOCSPAdmin {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IOCSPAdmin, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IOCSPAdmin {
    pub unsafe fn get_OCSPServiceProperties(&self) -> windows_core::Result<IOCSPPropertyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OCSPServiceProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_OCSPCAConfigurationCollection(&self) -> windows_core::Result<IOCSPCAConfigurationCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OCSPCAConfigurationCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetConfiguration(&self, bstrservername: &windows_core::BSTR, bforce: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetConfiguration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername), bforce) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetConfiguration(&self, bstrservername: &windows_core::BSTR, bforce: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConfiguration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername), bforce) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMyRoles(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<super::LONG> {
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
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetSigningCertificates(&self, bstrservername: &windows_core::BSTR, pcacertvar: *const super::VARIANT) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSigningCertificates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername), pcacertvar, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetHashAlgorithms(&self, bstrservername: &windows_core::BSTR, bstrcaid: &windows_core::BSTR) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHashAlgorithms)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername), core::mem::transmute_copy(bstrcaid), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOCSPAdmin_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_OCSPServiceProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_OCSPCAConfigurationCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub GetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetConfiguration: usize,
    #[cfg(feature = "wtypes")]
    pub SetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetConfiguration: usize,
    #[cfg(feature = "shared")]
    pub GetMyRoles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMyRoles: usize,
    pub Ping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetSigningCertificates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::VARIANT, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetSigningCertificates: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetHashAlgorithms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetHashAlgorithms: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IOCSPAdmin_Impl: super::IDispatch_Impl {
    fn get_OCSPServiceProperties(&self) -> windows_core::Result<IOCSPPropertyCollection>;
    fn get_OCSPCAConfigurationCollection(&self) -> windows_core::Result<IOCSPCAConfigurationCollection>;
    fn GetConfiguration(&self, bstrservername: &windows_core::BSTR, bforce: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetConfiguration(&self, bstrservername: &windows_core::BSTR, bforce: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetMyRoles(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<super::LONG>;
    fn Ping(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSecurity(&self, bstrservername: &windows_core::BSTR, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetSecurity(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetSigningCertificates(&self, bstrservername: &windows_core::BSTR, pcacertvar: *const super::VARIANT) -> windows_core::Result<super::VARIANT>;
    fn GetHashAlgorithms(&self, bstrservername: &windows_core::BSTR, bstrcaid: &windows_core::BSTR) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IOCSPAdmin_Vtbl {
    pub const fn new<Identity: IOCSPAdmin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_OCSPServiceProperties<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPAdmin_Impl::get_OCSPServiceProperties(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_OCSPCAConfigurationCollection<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPAdmin_Impl::get_OCSPCAConfigurationCollection(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConfiguration<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, bforce: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPAdmin_Impl::GetConfiguration(this, core::mem::transmute(&bstrservername), core::mem::transmute_copy(&bforce)).into()
            }
        }
        unsafe extern "system" fn SetConfiguration<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, bforce: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPAdmin_Impl::SetConfiguration(this, core::mem::transmute(&bstrservername), core::mem::transmute_copy(&bforce)).into()
            }
        }
        unsafe extern "system" fn GetMyRoles<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, proles: *mut super::LONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetSigningCertificates<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, pcacertvar: *const super::VARIANT, pval: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetHashAlgorithms<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, bstrcaid: *mut core::ffi::c_void, pval: *mut super::VARIANT) -> windows_core::HRESULT {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_OCSPServiceProperties: get_OCSPServiceProperties::<Identity, OFFSET>,
            get_OCSPCAConfigurationCollection: get_OCSPCAConfigurationCollection::<Identity, OFFSET>,
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
        iid == &<IOCSPAdmin as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IOCSPAdmin {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IOCSPCAConfiguration, IOCSPCAConfiguration_Vtbl, 0xaec92b40_3d46_433f_87d1_b84d5c1e790d);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IOCSPCAConfiguration {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IOCSPCAConfiguration, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IOCSPCAConfiguration {
    pub unsafe fn get_Identifier(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Identifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_CACertificate(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CACertificate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_HashAlgorithm(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_HashAlgorithm(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_HashAlgorithm)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_SigningFlags(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SigningFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_SigningFlags(&self, newval: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SigningFlags)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_SigningCertificate(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SigningCertificate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_SigningCertificate(&self, newval: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SigningCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_ReminderDuration(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReminderDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_ReminderDuration(&self, newval: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ReminderDuration)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_ErrorCode(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ErrorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CSPName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CSPName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_KeySpec(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KeySpec)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ProviderCLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProviderCLSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ProviderCLSID(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ProviderCLSID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_ProviderProperties(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProviderProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_ProviderProperties(&self, newval: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ProviderProperties)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Modified(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Modified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_LocalRevocationInformation(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LocalRevocationInformation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_LocalRevocationInformation(&self, newval: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LocalRevocationInformation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    pub unsafe fn get_SigningCertificateTemplate(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SigningCertificateTemplate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_SigningCertificateTemplate(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SigningCertificateTemplate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    pub unsafe fn get_CAConfig(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CAConfig)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_CAConfig(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CAConfig)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOCSPCAConfiguration_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Identifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_CACertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_CACertificate: usize,
    pub get_HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_SigningFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_SigningFlags: usize,
    #[cfg(feature = "shared")]
    pub put_SigningFlags: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_SigningFlags: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_SigningCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_SigningCertificate: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_SigningCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_SigningCertificate: usize,
    #[cfg(feature = "shared")]
    pub get_ReminderDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_ReminderDuration: usize,
    #[cfg(feature = "shared")]
    pub put_ReminderDuration: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_ReminderDuration: usize,
    #[cfg(feature = "shared")]
    pub get_ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_ErrorCode: usize,
    pub get_CSPName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_KeySpec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_KeySpec: usize,
    pub get_ProviderCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ProviderCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_ProviderProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_ProviderProperties: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_ProviderProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_ProviderProperties: usize,
    #[cfg(feature = "wtypes")]
    pub get_Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Modified: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_LocalRevocationInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_LocalRevocationInformation: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_LocalRevocationInformation: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_LocalRevocationInformation: usize,
    pub get_SigningCertificateTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_SigningCertificateTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CAConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_CAConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IOCSPCAConfiguration_Impl: super::IDispatch_Impl {
    fn get_Identifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CACertificate(&self) -> windows_core::Result<super::VARIANT>;
    fn get_HashAlgorithm(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_HashAlgorithm(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SigningFlags(&self) -> windows_core::Result<super::ULONG>;
    fn put_SigningFlags(&self, newval: super::ULONG) -> windows_core::Result<()>;
    fn get_SigningCertificate(&self) -> windows_core::Result<super::VARIANT>;
    fn put_SigningCertificate(&self, newval: &super::VARIANT) -> windows_core::Result<()>;
    fn get_ReminderDuration(&self) -> windows_core::Result<super::ULONG>;
    fn put_ReminderDuration(&self, newval: super::ULONG) -> windows_core::Result<()>;
    fn get_ErrorCode(&self) -> windows_core::Result<super::ULONG>;
    fn get_CSPName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_KeySpec(&self) -> windows_core::Result<super::ULONG>;
    fn get_ProviderCLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ProviderCLSID(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ProviderProperties(&self) -> windows_core::Result<super::VARIANT>;
    fn put_ProviderProperties(&self, newval: &super::VARIANT) -> windows_core::Result<()>;
    fn get_Modified(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_LocalRevocationInformation(&self) -> windows_core::Result<super::VARIANT>;
    fn put_LocalRevocationInformation(&self, newval: &super::VARIANT) -> windows_core::Result<()>;
    fn get_SigningCertificateTemplate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_SigningCertificateTemplate(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_CAConfig(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_CAConfig(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IOCSPCAConfiguration_Vtbl {
    pub const fn new<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Identifier<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_Identifier(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CACertificate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_CACertificate(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_HashAlgorithm<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_HashAlgorithm(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_HashAlgorithm<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::put_HashAlgorithm(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn get_SigningFlags<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_SigningFlags(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SigningFlags<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::put_SigningFlags(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_SigningCertificate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_SigningCertificate(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SigningCertificate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::put_SigningCertificate(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn get_ReminderDuration<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_ReminderDuration(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ReminderDuration<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::put_ReminderDuration(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_ErrorCode<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_ErrorCode(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CSPName<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_CSPName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_KeySpec<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_KeySpec(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ProviderCLSID<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_ProviderCLSID(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ProviderCLSID<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::put_ProviderCLSID(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn get_ProviderProperties<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_ProviderProperties(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ProviderProperties<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::put_ProviderProperties(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn get_Modified<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_Modified(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LocalRevocationInformation<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_LocalRevocationInformation(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LocalRevocationInformation<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::put_LocalRevocationInformation(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn get_SigningCertificateTemplate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_SigningCertificateTemplate(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SigningCertificateTemplate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::put_SigningCertificateTemplate(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn get_CAConfig<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfiguration_Impl::get_CAConfig(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_CAConfig<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPCAConfiguration_Impl::put_CAConfig(this, core::mem::transmute(&newval)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Identifier: get_Identifier::<Identity, OFFSET>,
            get_CACertificate: get_CACertificate::<Identity, OFFSET>,
            get_HashAlgorithm: get_HashAlgorithm::<Identity, OFFSET>,
            put_HashAlgorithm: put_HashAlgorithm::<Identity, OFFSET>,
            get_SigningFlags: get_SigningFlags::<Identity, OFFSET>,
            put_SigningFlags: put_SigningFlags::<Identity, OFFSET>,
            get_SigningCertificate: get_SigningCertificate::<Identity, OFFSET>,
            put_SigningCertificate: put_SigningCertificate::<Identity, OFFSET>,
            get_ReminderDuration: get_ReminderDuration::<Identity, OFFSET>,
            put_ReminderDuration: put_ReminderDuration::<Identity, OFFSET>,
            get_ErrorCode: get_ErrorCode::<Identity, OFFSET>,
            get_CSPName: get_CSPName::<Identity, OFFSET>,
            get_KeySpec: get_KeySpec::<Identity, OFFSET>,
            get_ProviderCLSID: get_ProviderCLSID::<Identity, OFFSET>,
            put_ProviderCLSID: put_ProviderCLSID::<Identity, OFFSET>,
            get_ProviderProperties: get_ProviderProperties::<Identity, OFFSET>,
            put_ProviderProperties: put_ProviderProperties::<Identity, OFFSET>,
            get_Modified: get_Modified::<Identity, OFFSET>,
            get_LocalRevocationInformation: get_LocalRevocationInformation::<Identity, OFFSET>,
            put_LocalRevocationInformation: put_LocalRevocationInformation::<Identity, OFFSET>,
            get_SigningCertificateTemplate: get_SigningCertificateTemplate::<Identity, OFFSET>,
            put_SigningCertificateTemplate: put_SigningCertificateTemplate::<Identity, OFFSET>,
            get_CAConfig: get_CAConfig::<Identity, OFFSET>,
            put_CAConfig: put_CAConfig::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPCAConfiguration as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IOCSPCAConfiguration {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IOCSPCAConfigurationCollection, IOCSPCAConfigurationCollection_Vtbl, 0x2bebea0b_5ece_4f28_a91c_86b4bb20f0d3);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IOCSPCAConfigurationCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IOCSPCAConfigurationCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IOCSPCAConfigurationCollection {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Item(&self, index: super::LONG) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_ItemByName(&self, bstridentifier: &windows_core::BSTR) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ItemByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstridentifier), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreateCAConfiguration(&self, bstridentifier: &windows_core::BSTR, varcacert: &super::VARIANT) -> windows_core::Result<IOCSPCAConfiguration> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCAConfiguration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstridentifier), core::mem::transmute_copy(varcacert), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteCAConfiguration(&self, bstridentifier: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteCAConfiguration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstridentifier)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOCSPCAConfigurationCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Item: usize,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_ItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_ItemByName: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub CreateCAConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    CreateCAConfiguration: usize,
    pub DeleteCAConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IOCSPCAConfigurationCollection_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, index: super::LONG) -> windows_core::Result<super::VARIANT>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
    fn get_ItemByName(&self, bstridentifier: &windows_core::BSTR) -> windows_core::Result<super::VARIANT>;
    fn CreateCAConfiguration(&self, bstridentifier: &windows_core::BSTR, varcacert: &super::VARIANT) -> windows_core::Result<IOCSPCAConfiguration>;
    fn DeleteCAConfiguration(&self, bstridentifier: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IOCSPCAConfigurationCollection_Vtbl {
    pub const fn new<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfigurationCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, pval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfigurationCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfigurationCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ItemByName<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstridentifier: *mut core::ffi::c_void, pval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPCAConfigurationCollection_Impl::get_ItemByName(this, core::mem::transmute(&bstridentifier)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCAConfiguration<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstridentifier: *mut core::ffi::c_void, varcacert: super::VARIANT, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            get_ItemByName: get_ItemByName::<Identity, OFFSET>,
            CreateCAConfiguration: CreateCAConfiguration::<Identity, OFFSET>,
            DeleteCAConfiguration: DeleteCAConfiguration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPCAConfigurationCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IOCSPCAConfigurationCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IOCSPProperty, IOCSPProperty_Vtbl, 0x66fb7839_5f04_4c25_ad18_9ff1a8376ee0);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IOCSPProperty {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IOCSPProperty, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IOCSPProperty {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Value(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Value(&self, newval: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Value)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Modified(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Modified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOCSPProperty_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Value: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Value: usize,
    #[cfg(feature = "wtypes")]
    pub get_Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Modified: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IOCSPProperty_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Value(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Value(&self, newval: &super::VARIANT) -> windows_core::Result<()>;
    fn get_Modified(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IOCSPProperty_Vtbl {
    pub const fn new<Identity: IOCSPProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: IOCSPProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPProperty_Impl::get_Name(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Value<Identity: IOCSPProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPProperty_Impl::get_Value(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Value<Identity: IOCSPProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPProperty_Impl::put_Value(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn get_Modified<Identity: IOCSPProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPProperty_Impl::get_Modified(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            get_Value: get_Value::<Identity, OFFSET>,
            put_Value: put_Value::<Identity, OFFSET>,
            get_Modified: get_Modified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPProperty as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IOCSPProperty {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IOCSPPropertyCollection, IOCSPPropertyCollection_Vtbl, 0x2597c18d_54e6_4b74_9fa9_a6bfda99cbbe);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IOCSPPropertyCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IOCSPPropertyCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IOCSPPropertyCollection {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Item(&self, index: super::LONG) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_ItemByName(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ItemByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreateProperty(&self, bstrpropname: &windows_core::BSTR, pvarpropvalue: *const super::VARIANT) -> windows_core::Result<IOCSPProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), pvarpropvalue, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteProperty(&self, bstrpropname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn InitializeFromProperties(&self, pvarproperties: *const super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromProperties)(windows_core::Interface::as_raw(self), pvarproperties) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllProperties(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IOCSPPropertyCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Item: usize,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_ItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_ItemByName: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub CreateProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    CreateProperty: usize,
    pub DeleteProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub InitializeFromProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    InitializeFromProperties: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetAllProperties: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IOCSPPropertyCollection_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, index: super::LONG) -> windows_core::Result<super::VARIANT>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
    fn get_ItemByName(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT>;
    fn CreateProperty(&self, bstrpropname: &windows_core::BSTR, pvarpropvalue: *const super::VARIANT) -> windows_core::Result<IOCSPProperty>;
    fn DeleteProperty(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromProperties(&self, pvarproperties: *const super::VARIANT) -> windows_core::Result<()>;
    fn GetAllProperties(&self) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IOCSPPropertyCollection_Vtbl {
    pub const fn new<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPPropertyCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, pval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPPropertyCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPPropertyCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ItemByName<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, pval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOCSPPropertyCollection_Impl::get_ItemByName(this, core::mem::transmute(&bstrpropname)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateProperty<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, pvarpropvalue: *const super::VARIANT, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn InitializeFromProperties<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarproperties: *const super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOCSPPropertyCollection_Impl::InitializeFromProperties(this, core::mem::transmute_copy(&pvarproperties)).into()
            }
        }
        unsafe extern "system" fn GetAllProperties<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarproperties: *mut super::VARIANT) -> windows_core::HRESULT {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            get_ItemByName: get_ItemByName::<Identity, OFFSET>,
            CreateProperty: CreateProperty::<Identity, OFFSET>,
            DeleteProperty: DeleteProperty::<Identity, OFFSET>,
            InitializeFromProperties: InitializeFromProperties::<Identity, OFFSET>,
            GetAllProperties: GetAllProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPPropertyCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IOCSPPropertyCollection {}
pub const KRA_DISP_EXPIRED: i32 = 0;
pub const KRA_DISP_INVALID: i32 = 4;
pub const KRA_DISP_NOTFOUND: i32 = 1;
pub const KRA_DISP_NOTLOADED: i32 = 6;
pub const KRA_DISP_REVOKED: i32 = 2;
pub const KRA_DISP_UNTRUSTED: i32 = 5;
pub const KRA_DISP_VALID: i32 = 3;
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
