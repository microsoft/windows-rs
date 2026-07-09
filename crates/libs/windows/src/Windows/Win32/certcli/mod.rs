pub const CAIF_DSENTRY: u32 = 1;
pub const CAIF_LOCAL: u32 = 8;
pub const CAIF_REGISTRY: u32 = 4;
pub const CAIF_REGISTRYPARENT: u32 = 16;
pub const CAIF_SHAREDFOLDERENTRY: u32 = 2;
pub const CC_DEFAULTCONFIG: u32 = 0;
pub const CC_FIRSTCONFIG: u32 = 2;
pub const CC_LOCALACTIVECONFIG: u32 = 4;
pub const CC_LOCALCONFIG: u32 = 3;
pub const CC_UIPICKCONFIG: u32 = 1;
pub const CC_UIPICKCONFIGSKIPLOCALCA: u32 = 5;
pub const CCertConfig: windows_core::GUID = windows_core::GUID::from_u128(0x372fce38_4324_11d0_8810_00a0c903b83c);
pub const CCertGetConfig: windows_core::GUID = windows_core::GUID::from_u128(0xc6cc49b0_ce17_11d0_8833_00a0c903b83c);
pub const CCertRequest: windows_core::GUID = windows_core::GUID::from_u128(0x98aff3f0_5524_11d0_8812_00a0c903b83c);
pub const CCertServerExit: windows_core::GUID = windows_core::GUID::from_u128(0x4c4a5e40_732c_11d0_8816_00a0c903b83c);
pub const CCertServerPolicy: windows_core::GUID = windows_core::GUID::from_u128(0xaa000926_ffbe_11cf_8800_00a0c903b83c);
pub const CR_DISP_DENIED: u32 = 2;
pub const CR_DISP_ERROR: u32 = 1;
pub const CR_DISP_INCOMPLETE: u32 = 0;
pub const CR_DISP_ISSUED: u32 = 3;
pub const CR_DISP_ISSUED_OUT_OF_BAND: u32 = 4;
pub const CR_DISP_REVOKED: u32 = 6;
pub const CR_DISP_UNDER_SUBMISSION: u32 = 5;
pub const CR_GEMT_DEFAULT: u32 = 0;
pub const CR_GEMT_HRESULT_STRING: u32 = 1;
pub const CR_GEMT_HTTP_ERROR: u32 = 2;
pub const CR_IN_BASE64: u32 = 1;
pub const CR_IN_BASE64HEADER: u32 = 0;
pub const CR_IN_BINARY: u32 = 2;
pub const CR_IN_CERTIFICATETRANSPARENCY: u32 = 67108864;
pub const CR_IN_CHALLENGERESPONSE: u32 = 1280;
pub const CR_IN_CLIENTFLAGSMASK: u32 = 30540031;
pub const CR_IN_CLIENTIDNONE: u32 = 4194304;
pub const CR_IN_CMC: u32 = 1024;
pub const CR_IN_CONNECTONLY: u32 = 8388608;
pub const CR_IN_CRLS: u32 = 524288;
pub const CR_IN_ENCODEANY: u32 = 255;
pub const CR_IN_ENCODEMASK: u32 = 255;
pub const CR_IN_FORMATANY: u32 = 0;
pub const CR_IN_FORMATMASK: u32 = 65280;
pub const CR_IN_FULLRESPONSE: u32 = 262144;
pub const CR_IN_HTTP: u32 = 196608;
pub const CR_IN_KEYGEN: u32 = 512;
pub const CR_IN_MACHINE: u32 = 1048576;
pub const CR_IN_PKCS10: u32 = 256;
pub const CR_IN_PKCS7: u32 = 768;
pub const CR_IN_PRESIGN: u32 = 134217728;
pub const CR_IN_RETURNCHALLENGE: u32 = 16777216;
pub const CR_IN_ROBO: u32 = 2097152;
pub const CR_IN_RPC: u32 = 131072;
pub const CR_IN_SCEP: u32 = 65536;
pub const CR_IN_SCEPPOST: u32 = 33554432;
pub const CR_IN_SIGNEDCERTIFICATETIMESTAMPLIST: u32 = 1536;
pub const CR_OUT_BASE64: u32 = 1;
pub const CR_OUT_BASE64HEADER: u32 = 0;
pub const CR_OUT_BASE64REQUESTHEADER: u32 = 3;
pub const CR_OUT_BASE64X509CRLHEADER: u32 = 9;
pub const CR_OUT_BINARY: u32 = 2;
pub const CR_OUT_CHAIN: u32 = 256;
pub const CR_OUT_CRLS: u32 = 512;
pub const CR_OUT_ENCODEMASK: u32 = 255;
pub const CR_OUT_HEX: u32 = 4;
pub const CR_OUT_HEXADDR: u32 = 10;
pub const CR_OUT_HEXASCII: u32 = 5;
pub const CR_OUT_HEXASCIIADDR: u32 = 11;
pub const CR_OUT_HEXRAW: u32 = 12;
pub const CR_OUT_NOCR: u32 = 2147483648;
pub const CR_OUT_NOCRLF: u32 = 1073741824;
pub const CR_PROP_ADVANCEDSERVER: u32 = 28;
pub const CR_PROP_BASECRL: u32 = 17;
pub const CR_PROP_BASECRLPUBLISHSTATUS: u32 = 30;
pub const CR_PROP_CABACKWARDCROSSCERT: u32 = 36;
pub const CR_PROP_CABACKWARDCROSSCERTSTATE: u32 = 38;
pub const CR_PROP_CACERTSTATE: u32 = 19;
pub const CR_PROP_CACERTSTATUSCODE: u32 = 34;
pub const CR_PROP_CACERTVERSION: u32 = 39;
pub const CR_PROP_CAFORWARDCROSSCERT: u32 = 35;
pub const CR_PROP_CAFORWARDCROSSCERTSTATE: u32 = 37;
pub const CR_PROP_CANAME: u32 = 6;
pub const CR_PROP_CAPROPIDMAX: u32 = 21;
pub const CR_PROP_CASIGCERT: u32 = 12;
pub const CR_PROP_CASIGCERTCHAIN: u32 = 13;
pub const CR_PROP_CASIGCERTCOUNT: u32 = 11;
pub const CR_PROP_CASIGCERTCRLCHAIN: u32 = 32;
pub const CR_PROP_CATYPE: u32 = 10;
pub const CR_PROP_CAXCHGCERT: u32 = 15;
pub const CR_PROP_CAXCHGCERTCHAIN: u32 = 16;
pub const CR_PROP_CAXCHGCERTCOUNT: u32 = 14;
pub const CR_PROP_CAXCHGCERTCRLCHAIN: u32 = 33;
pub const CR_PROP_CERTAIAOCSPURLS: u32 = 43;
pub const CR_PROP_CERTAIAURLS: u32 = 42;
pub const CR_PROP_CERTCDPURLS: u32 = 41;
pub const CR_PROP_CRLPARTITIONCOUNT: u32 = 46;
pub const CR_PROP_CRLSTATE: u32 = 20;
pub const CR_PROP_DELTACRL: u32 = 18;
pub const CR_PROP_DELTACRLPUBLISHSTATUS: u32 = 31;
pub const CR_PROP_DNSNAME: u32 = 22;
pub const CR_PROP_EXITCOUNT: u32 = 3;
pub const CR_PROP_EXITDESCRIPTION: u32 = 4;
pub const CR_PROP_FILEVERSION: u32 = 1;
pub const CR_PROP_KRACERT: u32 = 26;
pub const CR_PROP_KRACERTCOUNT: u32 = 25;
pub const CR_PROP_KRACERTSTATE: u32 = 27;
pub const CR_PROP_KRACERTUSEDCOUNT: u32 = 24;
pub const CR_PROP_LOCALENAME: u32 = 44;
pub const CR_PROP_NONE: u32 = 0;
pub const CR_PROP_PARENTCA: u32 = 9;
pub const CR_PROP_PARTITIONED_BASECRL: u32 = 47;
pub const CR_PROP_PARTITIONED_BASECRLPUBLISHSTATUS: u32 = 49;
pub const CR_PROP_PARTITIONED_DELTACRL: u32 = 48;
pub const CR_PROP_PARTITIONED_DELTACRLPUBLISHSTATUS: u32 = 50;
pub const CR_PROP_POLICYDESCRIPTION: u32 = 5;
pub const CR_PROP_PRODUCTVERSION: u32 = 2;
pub const CR_PROP_ROLESEPARATIONENABLED: u32 = 23;
pub const CR_PROP_SANITIZEDCANAME: u32 = 7;
pub const CR_PROP_SANITIZEDCASHORTNAME: u32 = 40;
pub const CR_PROP_SCEPMAX: u32 = 1002;
pub const CR_PROP_SCEPMIN: u32 = 1000;
pub const CR_PROP_SCEPSERVERCAPABILITIES: u32 = 1001;
pub const CR_PROP_SCEPSERVERCERTS: u32 = 1000;
pub const CR_PROP_SCEPSERVERCERTSCHAIN: u32 = 1002;
pub const CR_PROP_SHAREDFOLDER: u32 = 8;
pub const CR_PROP_SUBJECTTEMPLATE_OIDS: u32 = 45;
pub const CR_PROP_TEMPLATES: u32 = 29;
pub const FR_PROP_ATTESTATIONCHALLENGE: u32 = 20;
pub const FR_PROP_ATTESTATIONPROVIDERNAME: u32 = 21;
pub const FR_PROP_BODYPARTSTRING: u32 = 3;
pub const FR_PROP_CAEXCHANGECERTIFICATE: u32 = 17;
pub const FR_PROP_CAEXCHANGECERTIFICATECHAIN: u32 = 18;
pub const FR_PROP_CAEXCHANGECERTIFICATECRLCHAIN: u32 = 19;
pub const FR_PROP_CAEXCHANGECERTIFICATEHASH: u32 = 16;
pub const FR_PROP_CLAIMCHALLENGE: u32 = 22;
pub const FR_PROP_ENCRYPTEDKEYHASH: u32 = 14;
pub const FR_PROP_FAILINFO: u32 = 7;
pub const FR_PROP_FULLRESPONSE: u32 = 1;
pub const FR_PROP_FULLRESPONSENOPKCS7: u32 = 15;
pub const FR_PROP_ISSUEDCERTIFICATE: u32 = 11;
pub const FR_PROP_ISSUEDCERTIFICATECHAIN: u32 = 12;
pub const FR_PROP_ISSUEDCERTIFICATECRLCHAIN: u32 = 13;
pub const FR_PROP_ISSUEDCERTIFICATEHASH: u32 = 10;
pub const FR_PROP_NONE: u32 = 0;
pub const FR_PROP_OTHERINFOCHOICE: u32 = 6;
pub const FR_PROP_PENDINFOTIME: u32 = 9;
pub const FR_PROP_PENDINFOTOKEN: u32 = 8;
pub const FR_PROP_STATUS: u32 = 4;
pub const FR_PROP_STATUSINFOCOUNT: u32 = 2;
pub const FR_PROP_STATUSSTRING: u32 = 5;
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertConfig, ICertConfig_Vtbl, 0x372fce34_4324_11d0_8810_00a0c903b83c);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertConfig {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertConfig, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ICertConfig {
    pub unsafe fn Reset(&self, index: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Next(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetField(&self, strfieldname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetField)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strfieldname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetConfig(&self, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConfig)(windows_core::Interface::as_raw(self), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertConfig_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetField: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConfig: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertConfig_Impl: super::oaidl::IDispatch_Impl {
    fn Reset(&self, index: i32) -> windows_core::Result<i32>;
    fn Next(&self) -> windows_core::Result<i32>;
    fn GetField(&self, strfieldname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetConfig(&self, flags: i32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertConfig_Vtbl {
    pub const fn new<Identity: ICertConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: ICertConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertConfig_Impl::Reset(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: ICertConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertConfig_Impl::Next(this) {
                    Ok(ok__) => {
                        pindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetField<Identity: ICertConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfieldname: *mut core::ffi::c_void, pstrout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertConfig_Impl::GetField(this, core::mem::transmute(&strfieldname)) {
                    Ok(ok__) => {
                        pstrout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConfig<Identity: ICertConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pstrout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertConfig_Impl::GetConfig(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pstrout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            GetField: GetField::<Identity, OFFSET>,
            GetConfig: GetConfig::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertConfig as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertConfig {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertConfig2, ICertConfig2_Vtbl, 0x7a18edde_7e78_4163_8ded_78e2c9cee924);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertConfig2 {
    type Target = ICertConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertConfig2, windows_core::IUnknown, super::oaidl::IDispatch, ICertConfig);
#[cfg(feature = "Win32_oaidl")]
impl ICertConfig2 {
    pub unsafe fn SetSharedFolder(&self, strsharedfolder: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSharedFolder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strsharedfolder)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertConfig2_Vtbl {
    pub base__: ICertConfig_Vtbl,
    pub SetSharedFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertConfig2_Impl: ICertConfig_Impl {
    fn SetSharedFolder(&self, strsharedfolder: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertConfig2_Vtbl {
    pub const fn new<Identity: ICertConfig2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSharedFolder<Identity: ICertConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsharedfolder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertConfig2_Impl::SetSharedFolder(this, core::mem::transmute(&strsharedfolder)).into()
            }
        }
        Self { base__: ICertConfig_Vtbl::new::<Identity, OFFSET>(), SetSharedFolder: SetSharedFolder::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertConfig2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ICertConfig as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertConfig2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertGetConfig, ICertGetConfig_Vtbl, 0xc7ea09c0_ce17_11d0_8833_00a0c903b83c);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertGetConfig {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertGetConfig, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ICertGetConfig {
    pub unsafe fn GetConfig(&self, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConfig)(windows_core::Interface::as_raw(self), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertGetConfig_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub GetConfig: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertGetConfig_Impl: super::oaidl::IDispatch_Impl {
    fn GetConfig(&self, flags: i32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertGetConfig_Vtbl {
    pub const fn new<Identity: ICertGetConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConfig<Identity: ICertGetConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pstrout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertGetConfig_Impl::GetConfig(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pstrout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetConfig: GetConfig::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertGetConfig as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertGetConfig {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertRequest, ICertRequest_Vtbl, 0x014e4840_5523_11d0_8812_00a0c903b83c);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertRequest {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertRequest, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ICertRequest {
    pub unsafe fn Submit(&self, flags: i32, strrequest: &windows_core::BSTR, strattributes: &windows_core::BSTR, strconfig: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), flags, core::mem::transmute_copy(strrequest), core::mem::transmute_copy(strattributes), core::mem::transmute_copy(strconfig), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RetrievePending(&self, requestid: i32, strconfig: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RetrievePending)(windows_core::Interface::as_raw(self), requestid, core::mem::transmute_copy(strconfig), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLastStatus(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRequestId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequestId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDispositionMessage(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDispositionMessage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCACertificate(&self, fexchangecertificate: i32, strconfig: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCACertificate)(windows_core::Interface::as_raw(self), fexchangecertificate, core::mem::transmute_copy(strconfig), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCertificate(&self, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificate)(windows_core::Interface::as_raw(self), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertRequest_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RetrievePending: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetLastStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetRequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetDispositionMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCACertificate: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertRequest_Impl: super::oaidl::IDispatch_Impl {
    fn Submit(&self, flags: i32, strrequest: &windows_core::BSTR, strattributes: &windows_core::BSTR, strconfig: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn RetrievePending(&self, requestid: i32, strconfig: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn GetLastStatus(&self) -> windows_core::Result<i32>;
    fn GetRequestId(&self) -> windows_core::Result<i32>;
    fn GetDispositionMessage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCACertificate(&self, fexchangecertificate: i32, strconfig: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetCertificate(&self, flags: i32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertRequest_Vtbl {
    pub const fn new<Identity: ICertRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Submit<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, strrequest: *mut core::ffi::c_void, strattributes: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, pdisposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest_Impl::Submit(this, core::mem::transmute_copy(&flags), core::mem::transmute(&strrequest), core::mem::transmute(&strattributes), core::mem::transmute(&strconfig)) {
                    Ok(ok__) => {
                        pdisposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RetrievePending<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: i32, strconfig: *mut core::ffi::c_void, pdisposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest_Impl::RetrievePending(this, core::mem::transmute_copy(&requestid), core::mem::transmute(&strconfig)) {
                    Ok(ok__) => {
                        pdisposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastStatus<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest_Impl::GetLastStatus(this) {
                    Ok(ok__) => {
                        pstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRequestId<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequestid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest_Impl::GetRequestId(this) {
                    Ok(ok__) => {
                        prequestid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDispositionMessage<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrdispositionmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest_Impl::GetDispositionMessage(this) {
                    Ok(ok__) => {
                        pstrdispositionmessage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCACertificate<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fexchangecertificate: i32, strconfig: *mut core::ffi::c_void, flags: i32, pstrcertificate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest_Impl::GetCACertificate(this, core::mem::transmute_copy(&fexchangecertificate), core::mem::transmute(&strconfig), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pstrcertificate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCertificate<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pstrcertificate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest_Impl::GetCertificate(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pstrcertificate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Submit: Submit::<Identity, OFFSET>,
            RetrievePending: RetrievePending::<Identity, OFFSET>,
            GetLastStatus: GetLastStatus::<Identity, OFFSET>,
            GetRequestId: GetRequestId::<Identity, OFFSET>,
            GetDispositionMessage: GetDispositionMessage::<Identity, OFFSET>,
            GetCACertificate: GetCACertificate::<Identity, OFFSET>,
            GetCertificate: GetCertificate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertRequest as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertRequest {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertRequest2, ICertRequest2_Vtbl, 0xa4772988_4a85_4fa9_824e_b5cf5c16405a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertRequest2 {
    type Target = ICertRequest;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertRequest2, windows_core::IUnknown, super::oaidl::IDispatch, ICertRequest);
#[cfg(feature = "Win32_oaidl")]
impl ICertRequest2 {
    pub unsafe fn GetIssuedCertificate(&self, strconfig: &windows_core::BSTR, requestid: i32, strserialnumber: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIssuedCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), requestid, core::mem::transmute_copy(strserialnumber), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetErrorMessageText(&self, hrmessage: i32, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorMessageText)(windows_core::Interface::as_raw(self), hrmessage, flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetCAProperty(&self, strconfig: &windows_core::BSTR, propid: i32, propindex: i32, proptype: i32, flags: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCAProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), propid, propindex, proptype, flags, &mut result__).map(|| core::mem::transmute(result__))
        }
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
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetFullResponseProperty(&self, propid: i32, propindex: i32, proptype: i32, flags: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFullResponseProperty)(windows_core::Interface::as_raw(self), propid, propindex, proptype, flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertRequest2_Vtbl {
    pub base__: ICertRequest_Vtbl,
    pub GetIssuedCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetErrorMessageText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetCAProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, i32, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetCAProperty: usize,
    pub GetCAPropertyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetCAPropertyDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetFullResponseProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetFullResponseProperty: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertRequest2_Impl: ICertRequest_Impl {
    fn GetIssuedCertificate(&self, strconfig: &windows_core::BSTR, requestid: i32, strserialnumber: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn GetErrorMessageText(&self, hrmessage: i32, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetCAProperty(&self, strconfig: &windows_core::BSTR, propid: i32, propindex: i32, proptype: i32, flags: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetCAPropertyFlags(&self, strconfig: &windows_core::BSTR, propid: i32) -> windows_core::Result<i32>;
    fn GetCAPropertyDisplayName(&self, strconfig: &windows_core::BSTR, propid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetFullResponseProperty(&self, propid: i32, propindex: i32, proptype: i32, flags: i32) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertRequest2_Vtbl {
    pub const fn new<Identity: ICertRequest2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIssuedCertificate<Identity: ICertRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, requestid: i32, strserialnumber: *mut core::ffi::c_void, pdisposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest2_Impl::GetIssuedCertificate(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid), core::mem::transmute(&strserialnumber)) {
                    Ok(ok__) => {
                        pdisposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorMessageText<Identity: ICertRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrmessage: i32, flags: i32, pstrerrormessagetext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest2_Impl::GetErrorMessageText(this, core::mem::transmute_copy(&hrmessage), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pstrerrormessagetext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCAProperty<Identity: ICertRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, propid: i32, propindex: i32, proptype: i32, flags: i32, pvarpropertyvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest2_Impl::GetCAProperty(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid), core::mem::transmute_copy(&propindex), core::mem::transmute_copy(&proptype), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pvarpropertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCAPropertyFlags<Identity: ICertRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, propid: i32, ppropflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest2_Impl::GetCAPropertyFlags(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid)) {
                    Ok(ok__) => {
                        ppropflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCAPropertyDisplayName<Identity: ICertRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, propid: i32, pstrdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest2_Impl::GetCAPropertyDisplayName(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid)) {
                    Ok(ok__) => {
                        pstrdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFullResponseProperty<Identity: ICertRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: i32, propindex: i32, proptype: i32, flags: i32, pvarpropertyvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest2_Impl::GetFullResponseProperty(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&propindex), core::mem::transmute_copy(&proptype), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pvarpropertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICertRequest_Vtbl::new::<Identity, OFFSET>(),
            GetIssuedCertificate: GetIssuedCertificate::<Identity, OFFSET>,
            GetErrorMessageText: GetErrorMessageText::<Identity, OFFSET>,
            GetCAProperty: GetCAProperty::<Identity, OFFSET>,
            GetCAPropertyFlags: GetCAPropertyFlags::<Identity, OFFSET>,
            GetCAPropertyDisplayName: GetCAPropertyDisplayName::<Identity, OFFSET>,
            GetFullResponseProperty: GetFullResponseProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertRequest2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ICertRequest as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertRequest2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertRequest3, ICertRequest3_Vtbl, 0xafc8f92b_33a2_4861_bf36_2933b7cd67b3);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertRequest3 {
    type Target = ICertRequest2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertRequest3, windows_core::IUnknown, super::oaidl::IDispatch, ICertRequest, ICertRequest2);
#[cfg(feature = "Win32_oaidl")]
impl ICertRequest3 {
    pub unsafe fn SetCredential(&self, hwnd: i32, authtype: X509EnrollmentAuthFlags, strcredential: &windows_core::BSTR, strpassword: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCredential)(windows_core::Interface::as_raw(self), hwnd, authtype, core::mem::transmute_copy(strcredential), core::mem::transmute_copy(strpassword)) }
    }
    pub unsafe fn GetRequestIdString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequestIdString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetIssuedCertificate2(&self, strconfig: &windows_core::BSTR, strrequestid: &windows_core::BSTR, strserialnumber: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIssuedCertificate2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strrequestid), core::mem::transmute_copy(strserialnumber), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn GetRefreshPolicy(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRefreshPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertRequest3_Vtbl {
    pub base__: ICertRequest2_Vtbl,
    pub SetCredential: unsafe extern "system" fn(*mut core::ffi::c_void, i32, X509EnrollmentAuthFlags, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRequestIdString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIssuedCertificate2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub GetRefreshPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    GetRefreshPolicy: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertRequest3_Impl: ICertRequest2_Impl {
    fn SetCredential(&self, hwnd: i32, authtype: X509EnrollmentAuthFlags, strcredential: &windows_core::BSTR, strpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetRequestIdString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetIssuedCertificate2(&self, strconfig: &windows_core::BSTR, strrequestid: &windows_core::BSTR, strserialnumber: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn GetRefreshPolicy(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertRequest3_Vtbl {
    pub const fn new<Identity: ICertRequest3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCredential<Identity: ICertRequest3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: i32, authtype: X509EnrollmentAuthFlags, strcredential: *mut core::ffi::c_void, strpassword: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertRequest3_Impl::SetCredential(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&authtype), core::mem::transmute(&strcredential), core::mem::transmute(&strpassword)).into()
            }
        }
        unsafe extern "system" fn GetRequestIdString<Identity: ICertRequest3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrrequestid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest3_Impl::GetRequestIdString(this) {
                    Ok(ok__) => {
                        pstrrequestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIssuedCertificate2<Identity: ICertRequest3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strrequestid: *mut core::ffi::c_void, strserialnumber: *mut core::ffi::c_void, pdisposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest3_Impl::GetIssuedCertificate2(this, core::mem::transmute(&strconfig), core::mem::transmute(&strrequestid), core::mem::transmute(&strserialnumber)) {
                    Ok(ok__) => {
                        pdisposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRefreshPolicy<Identity: ICertRequest3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertRequest3_Impl::GetRefreshPolicy(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICertRequest2_Vtbl::new::<Identity, OFFSET>(),
            SetCredential: SetCredential::<Identity, OFFSET>,
            GetRequestIdString: GetRequestIdString::<Identity, OFFSET>,
            GetIssuedCertificate2: GetIssuedCertificate2::<Identity, OFFSET>,
            GetRefreshPolicy: GetRefreshPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertRequest3 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ICertRequest as windows_core::Interface>::IID || iid == &<ICertRequest2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertRequest3 {}
pub const X509AuthAnonymous: X509EnrollmentAuthFlags = 1;
pub const X509AuthCertificate: X509EnrollmentAuthFlags = 8;
pub const X509AuthKerberos: X509EnrollmentAuthFlags = 2;
pub const X509AuthNone: X509EnrollmentAuthFlags = 0;
pub const X509AuthUsername: X509EnrollmentAuthFlags = 4;
pub type X509EnrollmentAuthFlags = i32;
pub const wszCONFIG_AUTHORITY: windows_core::PCWSTR = windows_core::w!("Authority");
pub const wszCONFIG_COMMENT: windows_core::PCWSTR = windows_core::w!("Comment");
pub const wszCONFIG_COMMONNAME: windows_core::PCWSTR = windows_core::w!("CommonName");
pub const wszCONFIG_CONFIG: windows_core::PCWSTR = windows_core::w!("Config");
pub const wszCONFIG_COUNTRY: windows_core::PCWSTR = windows_core::w!("Country");
pub const wszCONFIG_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("Description");
pub const wszCONFIG_EXCHANGECERTIFICATE: windows_core::PCWSTR = windows_core::w!("ExchangeCertificate");
pub const wszCONFIG_FLAGS: windows_core::PCWSTR = windows_core::w!("Flags");
pub const wszCONFIG_LOCALITY: windows_core::PCWSTR = windows_core::w!("Locality");
pub const wszCONFIG_ORGANIZATION: windows_core::PCWSTR = windows_core::w!("Organization");
pub const wszCONFIG_ORGUNIT: windows_core::PCWSTR = windows_core::w!("OrgUnit");
pub const wszCONFIG_SANITIZEDNAME: windows_core::PCWSTR = windows_core::w!("SanitizedName");
pub const wszCONFIG_SANITIZEDSHORTNAME: windows_core::PCWSTR = windows_core::w!("SanitizedShortName");
pub const wszCONFIG_SERVER: windows_core::PCWSTR = windows_core::w!("Server");
pub const wszCONFIG_SHORTNAME: windows_core::PCWSTR = windows_core::w!("ShortName");
pub const wszCONFIG_SIGNATURECERTIFICATE: windows_core::PCWSTR = windows_core::w!("SignatureCertificate");
pub const wszCONFIG_STATE: windows_core::PCWSTR = windows_core::w!("State");
pub const wszCONFIG_WEBENROLLMENTSERVERS: windows_core::PCWSTR = windows_core::w!("WebEnrollmentServers");
