#[cfg(feature = "Win32_Security_Cryptography")]
pub trait IOpcCertificateEnumerator_Impl: Sized + windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<*mut super::super::super::Security::Cryptography::CERT_CONTEXT>;
    fn Clone(&self) -> windows_core::Result<IOpcCertificateEnumerator>;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::RuntimeName for IOpcCertificateEnumerator {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl IOpcCertificateEnumerator_Vtbl {
    pub const fn new<Identity: IOpcCertificateEnumerator_Impl, const OFFSET: isize>() -> IOpcCertificateEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcCertificateEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcCertificateEnumerator_Impl::MovePrevious(this) {
                Ok(ok__) => {
                    hasprevious.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificate: *mut *mut super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcCertificateEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    certificate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcCertificateEnumerator_Impl::Clone(this) {
                Ok(ok__) => {
                    copy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcCertificateEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
pub trait IOpcCertificateSet_Impl: Sized + windows_core::IUnknownImpl {
    fn Add(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<()>;
    fn Remove(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcCertificateEnumerator>;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::RuntimeName for IOpcCertificateSet {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl IOpcCertificateSet_Vtbl {
    pub const fn new<Identity: IOpcCertificateSet_Impl, const OFFSET: isize>() -> IOpcCertificateSet_Vtbl {
        unsafe extern "system" fn Add<Identity: IOpcCertificateSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcCertificateSet_Impl::Add(this, core::mem::transmute_copy(&certificate)).into()
        }
        unsafe extern "system" fn Remove<Identity: IOpcCertificateSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcCertificateSet_Impl::Remove(this, core::mem::transmute_copy(&certificate)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcCertificateSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificateenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcCertificateSet_Impl::GetEnumerator(this) {
                Ok(ok__) => {
                    certificateenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcCertificateSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcDigitalSignature_Impl: Sized + windows_core::IUnknownImpl {
    fn GetNamespaces(&self, prefixes: *mut *mut windows_core::PWSTR, namespaces: *mut *mut windows_core::PWSTR, count: *mut u32) -> windows_core::Result<()>;
    fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSignaturePartName(&self) -> windows_core::Result<IOpcPartUri>;
    fn GetSignatureMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetCanonicalizationMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD>;
    fn GetSignatureValue(&self, signaturevalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
    fn GetSignaturePartReferenceEnumerator(&self) -> windows_core::Result<IOpcSignaturePartReferenceEnumerator>;
    fn GetSignatureRelationshipReferenceEnumerator(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
    fn GetSigningTime(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetTimeFormat(&self) -> windows_core::Result<OPC_SIGNATURE_TIME_FORMAT>;
    fn GetPackageObjectReference(&self) -> windows_core::Result<IOpcSignatureReference>;
    fn GetCertificateEnumerator(&self) -> windows_core::Result<IOpcCertificateEnumerator>;
    fn GetCustomReferenceEnumerator(&self) -> windows_core::Result<IOpcSignatureReferenceEnumerator>;
    fn GetCustomObjectEnumerator(&self) -> windows_core::Result<IOpcSignatureCustomObjectEnumerator>;
    fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcDigitalSignature {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcDigitalSignature_Vtbl {
    pub const fn new<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>() -> IOpcDigitalSignature_Vtbl {
        unsafe extern "system" fn GetNamespaces<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefixes: *mut *mut windows_core::PWSTR, namespaces: *mut *mut windows_core::PWSTR, count: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcDigitalSignature_Impl::GetNamespaces(this, core::mem::transmute_copy(&prefixes), core::mem::transmute_copy(&namespaces), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetSignatureId<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignature_Impl::GetSignatureId(this) {
                Ok(ok__) => {
                    signatureid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignature_Impl::GetSignaturePartName(this) {
                Ok(ok__) => {
                    signaturepartname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureMethod<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturemethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignature_Impl::GetSignatureMethod(this) {
                Ok(ok__) => {
                    signaturemethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCanonicalizationMethod<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignature_Impl::GetCanonicalizationMethod(this) {
                Ok(ok__) => {
                    canonicalizationmethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureValue<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturevalue: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcDigitalSignature_Impl::GetSignatureValue(this, core::mem::transmute_copy(&signaturevalue), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetSignaturePartReferenceEnumerator<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partreferenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignature_Impl::GetSignaturePartReferenceEnumerator(this) {
                Ok(ok__) => {
                    partreferenceenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceEnumerator<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipreferenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignature_Impl::GetSignatureRelationshipReferenceEnumerator(this) {
                Ok(ok__) => {
                    relationshipreferenceenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTime<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signingtime: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignature_Impl::GetSigningTime(this) {
                Ok(ok__) => {
                    signingtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeFormat<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignature_Impl::GetTimeFormat(this) {
                Ok(ok__) => {
                    timeformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageObjectReference<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageobjectreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignature_Impl::GetPackageObjectReference(this) {
                Ok(ok__) => {
                    packageobjectreference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateEnumerator<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificateenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignature_Impl::GetCertificateEnumerator(this) {
                Ok(ok__) => {
                    certificateenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customreferenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignature_Impl::GetCustomReferenceEnumerator(this) {
                Ok(ok__) => {
                    customreferenceenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobjectenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignature_Impl::GetCustomObjectEnumerator(this) {
                Ok(ok__) => {
                    customobjectenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureXml<Identity: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcDigitalSignature_Impl::GetSignatureXml(this, core::mem::transmute_copy(&signaturexml), core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNamespaces: GetNamespaces::<Identity, OFFSET>,
            GetSignatureId: GetSignatureId::<Identity, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Identity, OFFSET>,
            GetCanonicalizationMethod: GetCanonicalizationMethod::<Identity, OFFSET>,
            GetSignatureValue: GetSignatureValue::<Identity, OFFSET>,
            GetSignaturePartReferenceEnumerator: GetSignaturePartReferenceEnumerator::<Identity, OFFSET>,
            GetSignatureRelationshipReferenceEnumerator: GetSignatureRelationshipReferenceEnumerator::<Identity, OFFSET>,
            GetSigningTime: GetSigningTime::<Identity, OFFSET>,
            GetTimeFormat: GetTimeFormat::<Identity, OFFSET>,
            GetPackageObjectReference: GetPackageObjectReference::<Identity, OFFSET>,
            GetCertificateEnumerator: GetCertificateEnumerator::<Identity, OFFSET>,
            GetCustomReferenceEnumerator: GetCustomReferenceEnumerator::<Identity, OFFSET>,
            GetCustomObjectEnumerator: GetCustomObjectEnumerator::<Identity, OFFSET>,
            GetSignatureXml: GetSignatureXml::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcDigitalSignature as windows_core::Interface>::IID
    }
}
pub trait IOpcDigitalSignatureEnumerator_Impl: Sized + windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcDigitalSignature>;
    fn Clone(&self) -> windows_core::Result<IOpcDigitalSignatureEnumerator>;
}
impl windows_core::RuntimeName for IOpcDigitalSignatureEnumerator {}
impl IOpcDigitalSignatureEnumerator_Vtbl {
    pub const fn new<Identity: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>() -> IOpcDigitalSignatureEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignatureEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignatureEnumerator_Impl::MovePrevious(this) {
                Ok(ok__) => {
                    hasprevious.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digitalsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignatureEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    digitalsignature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignatureEnumerator_Impl::Clone(this) {
                Ok(ok__) => {
                    copy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcDigitalSignatureEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
pub trait IOpcDigitalSignatureManager_Impl: Sized + windows_core::IUnknownImpl {
    fn GetSignatureOriginPartName(&self) -> windows_core::Result<IOpcPartUri>;
    fn SetSignatureOriginPartName(&self, signatureoriginpartname: Option<&IOpcPartUri>) -> windows_core::Result<()>;
    fn GetSignatureEnumerator(&self) -> windows_core::Result<IOpcDigitalSignatureEnumerator>;
    fn RemoveSignature(&self, signaturepartname: Option<&IOpcPartUri>) -> windows_core::Result<()>;
    fn CreateSigningOptions(&self) -> windows_core::Result<IOpcSigningOptions>;
    fn Validate(&self, signature: Option<&IOpcDigitalSignature>, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<OPC_SIGNATURE_VALIDATION_RESULT>;
    fn Sign(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: Option<&IOpcSigningOptions>) -> windows_core::Result<IOpcDigitalSignature>;
    fn ReplaceSignatureXml(&self, signaturepartname: Option<&IOpcPartUri>, newsignaturexml: *const u8, count: u32) -> windows_core::Result<IOpcDigitalSignature>;
}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IOpcDigitalSignatureManager {}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
impl IOpcDigitalSignatureManager_Vtbl {
    pub const fn new<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>() -> IOpcDigitalSignatureManager_Vtbl {
        unsafe extern "system" fn GetSignatureOriginPartName<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureoriginpartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignatureManager_Impl::GetSignatureOriginPartName(this) {
                Ok(ok__) => {
                    signatureoriginpartname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureoriginpartname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcDigitalSignatureManager_Impl::SetSignatureOriginPartName(this, windows_core::from_raw_borrowed(&signatureoriginpartname)).into()
        }
        unsafe extern "system" fn GetSignatureEnumerator<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignatureManager_Impl::GetSignatureEnumerator(this) {
                Ok(ok__) => {
                    signatureenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSignature<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcDigitalSignatureManager_Impl::RemoveSignature(this, windows_core::from_raw_borrowed(&signaturepartname)).into()
        }
        unsafe extern "system" fn CreateSigningOptions<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signingoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignatureManager_Impl::CreateSigningOptions(this) {
                Ok(ok__) => {
                    signingoptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signature: *mut core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignatureManager_Impl::Validate(this, windows_core::from_raw_borrowed(&signature), core::mem::transmute_copy(&certificate)) {
                Ok(ok__) => {
                    validationresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sign<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: *mut core::ffi::c_void, digitalsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignatureManager_Impl::Sign(this, core::mem::transmute_copy(&certificate), windows_core::from_raw_borrowed(&signingoptions)) {
                Ok(ok__) => {
                    digitalsignature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceSignatureXml<Identity: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut core::ffi::c_void, newsignaturexml: *const u8, count: u32, digitalsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcDigitalSignatureManager_Impl::ReplaceSignatureXml(this, windows_core::from_raw_borrowed(&signaturepartname), core::mem::transmute_copy(&newsignaturexml), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    digitalsignature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignatureOriginPartName: GetSignatureOriginPartName::<Identity, OFFSET>,
            SetSignatureOriginPartName: SetSignatureOriginPartName::<Identity, OFFSET>,
            GetSignatureEnumerator: GetSignatureEnumerator::<Identity, OFFSET>,
            RemoveSignature: RemoveSignature::<Identity, OFFSET>,
            CreateSigningOptions: CreateSigningOptions::<Identity, OFFSET>,
            Validate: Validate::<Identity, OFFSET>,
            Sign: Sign::<Identity, OFFSET>,
            ReplaceSignatureXml: ReplaceSignatureXml::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcDigitalSignatureManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Com"))]
pub trait IOpcFactory_Impl: Sized + windows_core::IUnknownImpl {
    fn CreatePackageRootUri(&self) -> windows_core::Result<IOpcUri>;
    fn CreatePartUri(&self, pwzuri: &windows_core::PCWSTR) -> windows_core::Result<IOpcPartUri>;
    fn CreateStreamOnFile(&self, filename: &windows_core::PCWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32) -> windows_core::Result<super::super::super::System::Com::IStream>;
    fn CreatePackage(&self) -> windows_core::Result<IOpcPackage>;
    fn ReadPackageFromStream(&self, stream: Option<&super::super::super::System::Com::IStream>, flags: OPC_READ_FLAGS) -> windows_core::Result<IOpcPackage>;
    fn WritePackageToStream(&self, package: Option<&IOpcPackage>, flags: OPC_WRITE_FLAGS, stream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn CreateDigitalSignatureManager(&self, package: Option<&IOpcPackage>) -> windows_core::Result<IOpcDigitalSignatureManager>;
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IOpcFactory {}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Com"))]
impl IOpcFactory_Vtbl {
    pub const fn new<Identity: IOpcFactory_Impl, const OFFSET: isize>() -> IOpcFactory_Vtbl {
        unsafe extern "system" fn CreatePackageRootUri<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rooturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcFactory_Impl::CreatePackageRootUri(this) {
                Ok(ok__) => {
                    rooturi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUri<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzuri: windows_core::PCWSTR, parturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcFactory_Impl::CreatePartUri(this, core::mem::transmute(&pwzuri)) {
                Ok(ok__) => {
                    parturi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStreamOnFile<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcFactory_Impl::CreateStreamOnFile(this, core::mem::transmute(&filename), core::mem::transmute_copy(&iomode), core::mem::transmute_copy(&securityattributes), core::mem::transmute_copy(&dwflagsandattributes)) {
                Ok(ok__) => {
                    stream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackage<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcFactory_Impl::CreatePackage(this) {
                Ok(ok__) => {
                    package.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadPackageFromStream<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, flags: OPC_READ_FLAGS, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcFactory_Impl::ReadPackageFromStream(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    package.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WritePackageToStream<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut core::ffi::c_void, flags: OPC_WRITE_FLAGS, stream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcFactory_Impl::WritePackageToStream(this, windows_core::from_raw_borrowed(&package), core::mem::transmute_copy(&flags), windows_core::from_raw_borrowed(&stream)).into()
        }
        unsafe extern "system" fn CreateDigitalSignatureManager<Identity: IOpcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut core::ffi::c_void, signaturemanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcFactory_Impl::CreateDigitalSignatureManager(this, windows_core::from_raw_borrowed(&package)) {
                Ok(ok__) => {
                    signaturemanager.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePackageRootUri: CreatePackageRootUri::<Identity, OFFSET>,
            CreatePartUri: CreatePartUri::<Identity, OFFSET>,
            CreateStreamOnFile: CreateStreamOnFile::<Identity, OFFSET>,
            CreatePackage: CreatePackage::<Identity, OFFSET>,
            ReadPackageFromStream: ReadPackageFromStream::<Identity, OFFSET>,
            WritePackageToStream: WritePackageToStream::<Identity, OFFSET>,
            CreateDigitalSignatureManager: CreateDigitalSignatureManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcFactory as windows_core::Interface>::IID
    }
}
pub trait IOpcPackage_Impl: Sized + windows_core::IUnknownImpl {
    fn GetPartSet(&self) -> windows_core::Result<IOpcPartSet>;
    fn GetRelationshipSet(&self) -> windows_core::Result<IOpcRelationshipSet>;
}
impl windows_core::RuntimeName for IOpcPackage {}
impl IOpcPackage_Vtbl {
    pub const fn new<Identity: IOpcPackage_Impl, const OFFSET: isize>() -> IOpcPackage_Vtbl {
        unsafe extern "system" fn GetPartSet<Identity: IOpcPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPackage_Impl::GetPartSet(this) {
                Ok(ok__) => {
                    partset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipSet<Identity: IOpcPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPackage_Impl::GetRelationshipSet(this) {
                Ok(ok__) => {
                    relationshipset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPartSet: GetPartSet::<Identity, OFFSET>,
            GetRelationshipSet: GetRelationshipSet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcPackage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcPart_Impl: Sized + windows_core::IUnknownImpl {
    fn GetRelationshipSet(&self) -> windows_core::Result<IOpcRelationshipSet>;
    fn GetContentStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream>;
    fn GetName(&self) -> windows_core::Result<IOpcPartUri>;
    fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetCompressionOptions(&self) -> windows_core::Result<OPC_COMPRESSION_OPTIONS>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcPart {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcPart_Vtbl {
    pub const fn new<Identity: IOpcPart_Impl, const OFFSET: isize>() -> IOpcPart_Vtbl {
        unsafe extern "system" fn GetRelationshipSet<Identity: IOpcPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPart_Impl::GetRelationshipSet(this) {
                Ok(ok__) => {
                    relationshipset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentStream<Identity: IOpcPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPart_Impl::GetContentStream(this) {
                Ok(ok__) => {
                    stream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: IOpcPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPart_Impl::GetName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentType<Identity: IOpcPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPart_Impl::GetContentType(this) {
                Ok(ok__) => {
                    contenttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompressionOptions<Identity: IOpcPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, compressionoptions: *mut OPC_COMPRESSION_OPTIONS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPart_Impl::GetCompressionOptions(this) {
                Ok(ok__) => {
                    compressionoptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRelationshipSet: GetRelationshipSet::<Identity, OFFSET>,
            GetContentStream: GetContentStream::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetContentType: GetContentType::<Identity, OFFSET>,
            GetCompressionOptions: GetCompressionOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcPart as windows_core::Interface>::IID
    }
}
pub trait IOpcPartEnumerator_Impl: Sized + windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcPart>;
    fn Clone(&self) -> windows_core::Result<IOpcPartEnumerator>;
}
impl windows_core::RuntimeName for IOpcPartEnumerator {}
impl IOpcPartEnumerator_Vtbl {
    pub const fn new<Identity: IOpcPartEnumerator_Impl, const OFFSET: isize>() -> IOpcPartEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPartEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPartEnumerator_Impl::MovePrevious(this) {
                Ok(ok__) => {
                    hasprevious.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPartEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    part.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPartEnumerator_Impl::Clone(this) {
                Ok(ok__) => {
                    copy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcPartEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcPartSet_Impl: Sized + windows_core::IUnknownImpl {
    fn GetPart(&self, name: Option<&IOpcPartUri>) -> windows_core::Result<IOpcPart>;
    fn CreatePart(&self, name: Option<&IOpcPartUri>, contenttype: &windows_core::PCWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS) -> windows_core::Result<IOpcPart>;
    fn DeletePart(&self, name: Option<&IOpcPartUri>) -> windows_core::Result<()>;
    fn PartExists(&self, name: Option<&IOpcPartUri>) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcPartEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcPartSet {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcPartSet_Vtbl {
    pub const fn new<Identity: IOpcPartSet_Impl, const OFFSET: isize>() -> IOpcPartSet_Vtbl {
        unsafe extern "system" fn GetPart<Identity: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPartSet_Impl::GetPart(this, windows_core::from_raw_borrowed(&name)) {
                Ok(ok__) => {
                    part.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePart<Identity: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, contenttype: windows_core::PCWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPartSet_Impl::CreatePart(this, windows_core::from_raw_borrowed(&name), core::mem::transmute(&contenttype), core::mem::transmute_copy(&compressionoptions)) {
                Ok(ok__) => {
                    part.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePart<Identity: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcPartSet_Impl::DeletePart(this, windows_core::from_raw_borrowed(&name)).into()
        }
        unsafe extern "system" fn PartExists<Identity: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, partexists: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPartSet_Impl::PartExists(this, windows_core::from_raw_borrowed(&name)) {
                Ok(ok__) => {
                    partexists.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPartSet_Impl::GetEnumerator(this) {
                Ok(ok__) => {
                    partenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPart: GetPart::<Identity, OFFSET>,
            CreatePart: CreatePart::<Identity, OFFSET>,
            DeletePart: DeletePart::<Identity, OFFSET>,
            PartExists: PartExists::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcPartSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcPartUri_Impl: Sized + IOpcUri_Impl {
    fn ComparePartUri(&self, parturi: Option<&IOpcPartUri>) -> windows_core::Result<i32>;
    fn GetSourceUri(&self) -> windows_core::Result<IOpcUri>;
    fn IsRelationshipsPartUri(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcPartUri {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcPartUri_Vtbl {
    pub const fn new<Identity: IOpcPartUri_Impl, const OFFSET: isize>() -> IOpcPartUri_Vtbl {
        unsafe extern "system" fn ComparePartUri<Identity: IOpcPartUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, comparisonresult: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPartUri_Impl::ComparePartUri(this, windows_core::from_raw_borrowed(&parturi)) {
                Ok(ok__) => {
                    comparisonresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceUri<Identity: IOpcPartUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPartUri_Impl::GetSourceUri(this) {
                Ok(ok__) => {
                    sourceuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRelationshipsPartUri<Identity: IOpcPartUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isrelationshipuri: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcPartUri_Impl::IsRelationshipsPartUri(this) {
                Ok(ok__) => {
                    isrelationshipuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IOpcUri_Vtbl::new::<Identity, OFFSET>(),
            ComparePartUri: ComparePartUri::<Identity, OFFSET>,
            GetSourceUri: GetSourceUri::<Identity, OFFSET>,
            IsRelationshipsPartUri: IsRelationshipsPartUri::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcPartUri as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IUri as windows_core::Interface>::IID || iid == &<IOpcUri as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcRelationship_Impl: Sized + windows_core::IUnknownImpl {
    fn GetId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetRelationshipType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSourceUri(&self) -> windows_core::Result<IOpcUri>;
    fn GetTargetUri(&self) -> windows_core::Result<super::super::super::System::Com::IUri>;
    fn GetTargetMode(&self) -> windows_core::Result<OPC_URI_TARGET_MODE>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcRelationship {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcRelationship_Vtbl {
    pub const fn new<Identity: IOpcRelationship_Impl, const OFFSET: isize>() -> IOpcRelationship_Vtbl {
        unsafe extern "system" fn GetId<Identity: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipidentifier: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationship_Impl::GetId(this) {
                Ok(ok__) => {
                    relationshipidentifier.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipType<Identity: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshiptype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationship_Impl::GetRelationshipType(this) {
                Ok(ok__) => {
                    relationshiptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceUri<Identity: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationship_Impl::GetSourceUri(this) {
                Ok(ok__) => {
                    sourceuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetUri<Identity: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targeturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationship_Impl::GetTargetUri(this) {
                Ok(ok__) => {
                    targeturi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetMode<Identity: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetmode: *mut OPC_URI_TARGET_MODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationship_Impl::GetTargetMode(this) {
                Ok(ok__) => {
                    targetmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, OFFSET>,
            GetRelationshipType: GetRelationshipType::<Identity, OFFSET>,
            GetSourceUri: GetSourceUri::<Identity, OFFSET>,
            GetTargetUri: GetTargetUri::<Identity, OFFSET>,
            GetTargetMode: GetTargetMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcRelationship as windows_core::Interface>::IID
    }
}
pub trait IOpcRelationshipEnumerator_Impl: Sized + windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcRelationship>;
    fn Clone(&self) -> windows_core::Result<IOpcRelationshipEnumerator>;
}
impl windows_core::RuntimeName for IOpcRelationshipEnumerator {}
impl IOpcRelationshipEnumerator_Vtbl {
    pub const fn new<Identity: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>() -> IOpcRelationshipEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipEnumerator_Impl::MovePrevious(this) {
                Ok(ok__) => {
                    hasprevious.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationship: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    relationship.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipEnumerator_Impl::Clone(this) {
                Ok(ok__) => {
                    copy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcRelationshipEnumerator as windows_core::Interface>::IID
    }
}
pub trait IOpcRelationshipSelector_Impl: Sized + windows_core::IUnknownImpl {
    fn GetSelectorType(&self) -> windows_core::Result<OPC_RELATIONSHIP_SELECTOR>;
    fn GetSelectionCriterion(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IOpcRelationshipSelector {}
impl IOpcRelationshipSelector_Vtbl {
    pub const fn new<Identity: IOpcRelationshipSelector_Impl, const OFFSET: isize>() -> IOpcRelationshipSelector_Vtbl {
        unsafe extern "system" fn GetSelectorType<Identity: IOpcRelationshipSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selector: *mut OPC_RELATIONSHIP_SELECTOR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSelector_Impl::GetSelectorType(this) {
                Ok(ok__) => {
                    selector.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectionCriterion<Identity: IOpcRelationshipSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectioncriterion: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSelector_Impl::GetSelectionCriterion(this) {
                Ok(ok__) => {
                    selectioncriterion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSelectorType: GetSelectorType::<Identity, OFFSET>,
            GetSelectionCriterion: GetSelectionCriterion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcRelationshipSelector as windows_core::Interface>::IID
    }
}
pub trait IOpcRelationshipSelectorEnumerator_Impl: Sized + windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcRelationshipSelector>;
    fn Clone(&self) -> windows_core::Result<IOpcRelationshipSelectorEnumerator>;
}
impl windows_core::RuntimeName for IOpcRelationshipSelectorEnumerator {}
impl IOpcRelationshipSelectorEnumerator_Vtbl {
    pub const fn new<Identity: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>() -> IOpcRelationshipSelectorEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSelectorEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSelectorEnumerator_Impl::MovePrevious(this) {
                Ok(ok__) => {
                    hasprevious.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipselector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSelectorEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    relationshipselector.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSelectorEnumerator_Impl::Clone(this) {
                Ok(ok__) => {
                    copy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcRelationshipSelectorEnumerator as windows_core::Interface>::IID
    }
}
pub trait IOpcRelationshipSelectorSet_Impl: Sized + windows_core::IUnknownImpl {
    fn Create(&self, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: &windows_core::PCWSTR) -> windows_core::Result<IOpcRelationshipSelector>;
    fn Delete(&self, relationshipselector: Option<&IOpcRelationshipSelector>) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcRelationshipSelectorEnumerator>;
}
impl windows_core::RuntimeName for IOpcRelationshipSelectorSet {}
impl IOpcRelationshipSelectorSet_Vtbl {
    pub const fn new<Identity: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>() -> IOpcRelationshipSelectorSet_Vtbl {
        unsafe extern "system" fn Create<Identity: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: windows_core::PCWSTR, relationshipselector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSelectorSet_Impl::Create(this, core::mem::transmute_copy(&selector), core::mem::transmute(&selectioncriterion)) {
                Ok(ok__) => {
                    relationshipselector.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipselector: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcRelationshipSelectorSet_Impl::Delete(this, windows_core::from_raw_borrowed(&relationshipselector)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipselectorenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSelectorSet_Impl::GetEnumerator(this) {
                Ok(ok__) => {
                    relationshipselectorenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcRelationshipSelectorSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcRelationshipSet_Impl: Sized + windows_core::IUnknownImpl {
    fn GetRelationship(&self, relationshipidentifier: &windows_core::PCWSTR) -> windows_core::Result<IOpcRelationship>;
    fn CreateRelationship(&self, relationshipidentifier: &windows_core::PCWSTR, relationshiptype: &windows_core::PCWSTR, targeturi: Option<&super::super::super::System::Com::IUri>, targetmode: OPC_URI_TARGET_MODE) -> windows_core::Result<IOpcRelationship>;
    fn DeleteRelationship(&self, relationshipidentifier: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RelationshipExists(&self, relationshipidentifier: &windows_core::PCWSTR) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcRelationshipEnumerator>;
    fn GetEnumeratorForType(&self, relationshiptype: &windows_core::PCWSTR) -> windows_core::Result<IOpcRelationshipEnumerator>;
    fn GetRelationshipsContentStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcRelationshipSet {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcRelationshipSet_Vtbl {
    pub const fn new<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>() -> IOpcRelationshipSet_Vtbl {
        unsafe extern "system" fn GetRelationship<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipidentifier: windows_core::PCWSTR, relationship: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSet_Impl::GetRelationship(this, core::mem::transmute(&relationshipidentifier)) {
                Ok(ok__) => {
                    relationship.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRelationship<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipidentifier: windows_core::PCWSTR, relationshiptype: windows_core::PCWSTR, targeturi: *mut core::ffi::c_void, targetmode: OPC_URI_TARGET_MODE, relationship: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSet_Impl::CreateRelationship(this, core::mem::transmute(&relationshipidentifier), core::mem::transmute(&relationshiptype), windows_core::from_raw_borrowed(&targeturi), core::mem::transmute_copy(&targetmode)) {
                Ok(ok__) => {
                    relationship.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRelationship<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipidentifier: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcRelationshipSet_Impl::DeleteRelationship(this, core::mem::transmute(&relationshipidentifier)).into()
        }
        unsafe extern "system" fn RelationshipExists<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipidentifier: windows_core::PCWSTR, relationshipexists: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSet_Impl::RelationshipExists(this, core::mem::transmute(&relationshipidentifier)) {
                Ok(ok__) => {
                    relationshipexists.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSet_Impl::GetEnumerator(this) {
                Ok(ok__) => {
                    relationshipenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumeratorForType<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshiptype: windows_core::PCWSTR, relationshipenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSet_Impl::GetEnumeratorForType(this, core::mem::transmute(&relationshiptype)) {
                Ok(ok__) => {
                    relationshipenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipsContentStream<Identity: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcRelationshipSet_Impl::GetRelationshipsContentStream(this) {
                Ok(ok__) => {
                    contents.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRelationship: GetRelationship::<Identity, OFFSET>,
            CreateRelationship: CreateRelationship::<Identity, OFFSET>,
            DeleteRelationship: DeleteRelationship::<Identity, OFFSET>,
            RelationshipExists: RelationshipExists::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
            GetEnumeratorForType: GetEnumeratorForType::<Identity, OFFSET>,
            GetRelationshipsContentStream: GetRelationshipsContentStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcRelationshipSet as windows_core::Interface>::IID
    }
}
pub trait IOpcSignatureCustomObject_Impl: Sized + windows_core::IUnknownImpl {
    fn GetXml(&self, xmlmarkup: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOpcSignatureCustomObject {}
impl IOpcSignatureCustomObject_Vtbl {
    pub const fn new<Identity: IOpcSignatureCustomObject_Impl, const OFFSET: isize>() -> IOpcSignatureCustomObject_Vtbl {
        unsafe extern "system" fn GetXml<Identity: IOpcSignatureCustomObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xmlmarkup: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSignatureCustomObject_Impl::GetXml(this, core::mem::transmute_copy(&xmlmarkup), core::mem::transmute_copy(&count)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetXml: GetXml::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureCustomObject as windows_core::Interface>::IID
    }
}
pub trait IOpcSignatureCustomObjectEnumerator_Impl: Sized + windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcSignatureCustomObject>;
    fn Clone(&self) -> windows_core::Result<IOpcSignatureCustomObjectEnumerator>;
}
impl windows_core::RuntimeName for IOpcSignatureCustomObjectEnumerator {}
impl IOpcSignatureCustomObjectEnumerator_Vtbl {
    pub const fn new<Identity: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>() -> IOpcSignatureCustomObjectEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureCustomObjectEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureCustomObjectEnumerator_Impl::MovePrevious(this) {
                Ok(ok__) => {
                    hasprevious.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureCustomObjectEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    customobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureCustomObjectEnumerator_Impl::Clone(this) {
                Ok(ok__) => {
                    copy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureCustomObjectEnumerator as windows_core::Interface>::IID
    }
}
pub trait IOpcSignatureCustomObjectSet_Impl: Sized + windows_core::IUnknownImpl {
    fn Create(&self, xmlmarkup: *const u8, count: u32) -> windows_core::Result<IOpcSignatureCustomObject>;
    fn Delete(&self, customobject: Option<&IOpcSignatureCustomObject>) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcSignatureCustomObjectEnumerator>;
}
impl windows_core::RuntimeName for IOpcSignatureCustomObjectSet {}
impl IOpcSignatureCustomObjectSet_Vtbl {
    pub const fn new<Identity: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>() -> IOpcSignatureCustomObjectSet_Vtbl {
        unsafe extern "system" fn Create<Identity: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xmlmarkup: *const u8, count: u32, customobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureCustomObjectSet_Impl::Create(this, core::mem::transmute_copy(&xmlmarkup), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    customobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSignatureCustomObjectSet_Impl::Delete(this, windows_core::from_raw_borrowed(&customobject)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobjectenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureCustomObjectSet_Impl::GetEnumerator(this) {
                Ok(ok__) => {
                    customobjectenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureCustomObjectSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignaturePartReference_Impl: Sized + windows_core::IUnknownImpl {
    fn GetPartName(&self) -> windows_core::Result<IOpcPartUri>;
    fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
    fn GetTransformMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSignaturePartReference {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignaturePartReference_Vtbl {
    pub const fn new<Identity: IOpcSignaturePartReference_Impl, const OFFSET: isize>() -> IOpcSignaturePartReference_Vtbl {
        unsafe extern "system" fn GetPartName<Identity: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignaturePartReference_Impl::GetPartName(this) {
                Ok(ok__) => {
                    partname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentType<Identity: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignaturePartReference_Impl::GetContentType(this) {
                Ok(ok__) => {
                    contenttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestMethod<Identity: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignaturePartReference_Impl::GetDigestMethod(this) {
                Ok(ok__) => {
                    digestmethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestValue<Identity: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSignaturePartReference_Impl::GetDigestValue(this, core::mem::transmute_copy(&digestvalue), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetTransformMethod<Identity: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignaturePartReference_Impl::GetTransformMethod(this) {
                Ok(ok__) => {
                    transformmethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPartName: GetPartName::<Identity, OFFSET>,
            GetContentType: GetContentType::<Identity, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, OFFSET>,
            GetDigestValue: GetDigestValue::<Identity, OFFSET>,
            GetTransformMethod: GetTransformMethod::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignaturePartReference as windows_core::Interface>::IID
    }
}
pub trait IOpcSignaturePartReferenceEnumerator_Impl: Sized + windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcSignaturePartReference>;
    fn Clone(&self) -> windows_core::Result<IOpcSignaturePartReferenceEnumerator>;
}
impl windows_core::RuntimeName for IOpcSignaturePartReferenceEnumerator {}
impl IOpcSignaturePartReferenceEnumerator_Vtbl {
    pub const fn new<Identity: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>() -> IOpcSignaturePartReferenceEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignaturePartReferenceEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignaturePartReferenceEnumerator_Impl::MovePrevious(this) {
                Ok(ok__) => {
                    hasprevious.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignaturePartReferenceEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    partreference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignaturePartReferenceEnumerator_Impl::Clone(this) {
                Ok(ok__) => {
                    copy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignaturePartReferenceEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignaturePartReferenceSet_Impl: Sized + windows_core::IUnknownImpl {
    fn Create(&self, parturi: Option<&IOpcPartUri>, digestmethod: &windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD) -> windows_core::Result<IOpcSignaturePartReference>;
    fn Delete(&self, partreference: Option<&IOpcSignaturePartReference>) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcSignaturePartReferenceEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSignaturePartReferenceSet {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignaturePartReferenceSet_Vtbl {
    pub const fn new<Identity: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>() -> IOpcSignaturePartReferenceSet_Vtbl {
        unsafe extern "system" fn Create<Identity: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, digestmethod: windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, partreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignaturePartReferenceSet_Impl::Create(this, windows_core::from_raw_borrowed(&parturi), core::mem::transmute(&digestmethod), core::mem::transmute_copy(&transformmethod)) {
                Ok(ok__) => {
                    partreference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partreference: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSignaturePartReferenceSet_Impl::Delete(this, windows_core::from_raw_borrowed(&partreference)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partreferenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignaturePartReferenceSet_Impl::GetEnumerator(this) {
                Ok(ok__) => {
                    partreferenceenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignaturePartReferenceSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureReference_Impl: Sized + windows_core::IUnknownImpl {
    fn GetId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetUri(&self) -> windows_core::Result<super::super::super::System::Com::IUri>;
    fn GetType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetTransformMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD>;
    fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSignatureReference {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignatureReference_Vtbl {
    pub const fn new<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>() -> IOpcSignatureReference_Vtbl {
        unsafe extern "system" fn GetId<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referenceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureReference_Impl::GetId(this) {
                Ok(ok__) => {
                    referenceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUri<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referenceuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureReference_Impl::GetUri(this) {
                Ok(ok__) => {
                    referenceuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureReference_Impl::GetType(this) {
                Ok(ok__) => {
                    r#type.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformMethod<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureReference_Impl::GetTransformMethod(this) {
                Ok(ok__) => {
                    transformmethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestMethod<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureReference_Impl::GetDigestMethod(this) {
                Ok(ok__) => {
                    digestmethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestValue<Identity: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSignatureReference_Impl::GetDigestValue(this, core::mem::transmute_copy(&digestvalue), core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, OFFSET>,
            GetUri: GetUri::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetTransformMethod: GetTransformMethod::<Identity, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, OFFSET>,
            GetDigestValue: GetDigestValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureReference as windows_core::Interface>::IID
    }
}
pub trait IOpcSignatureReferenceEnumerator_Impl: Sized + windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcSignatureReference>;
    fn Clone(&self) -> windows_core::Result<IOpcSignatureReferenceEnumerator>;
}
impl windows_core::RuntimeName for IOpcSignatureReferenceEnumerator {}
impl IOpcSignatureReferenceEnumerator_Vtbl {
    pub const fn new<Identity: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>() -> IOpcSignatureReferenceEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureReferenceEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureReferenceEnumerator_Impl::MovePrevious(this) {
                Ok(ok__) => {
                    hasprevious.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureReferenceEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    reference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureReferenceEnumerator_Impl::Clone(this) {
                Ok(ok__) => {
                    copy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureReferenceEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureReferenceSet_Impl: Sized + windows_core::IUnknownImpl {
    fn Create(&self, referenceuri: Option<&super::super::super::System::Com::IUri>, referenceid: &windows_core::PCWSTR, r#type: &windows_core::PCWSTR, digestmethod: &windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD) -> windows_core::Result<IOpcSignatureReference>;
    fn Delete(&self, reference: Option<&IOpcSignatureReference>) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcSignatureReferenceEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSignatureReferenceSet {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignatureReferenceSet_Vtbl {
    pub const fn new<Identity: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>() -> IOpcSignatureReferenceSet_Vtbl {
        unsafe extern "system" fn Create<Identity: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referenceuri: *mut core::ffi::c_void, referenceid: windows_core::PCWSTR, r#type: windows_core::PCWSTR, digestmethod: windows_core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, reference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureReferenceSet_Impl::Create(this, windows_core::from_raw_borrowed(&referenceuri), core::mem::transmute(&referenceid), core::mem::transmute(&r#type), core::mem::transmute(&digestmethod), core::mem::transmute_copy(&transformmethod)) {
                Ok(ok__) => {
                    reference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reference: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSignatureReferenceSet_Impl::Delete(this, windows_core::from_raw_borrowed(&reference)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureReferenceSet_Impl::GetEnumerator(this) {
                Ok(ok__) => {
                    referenceenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureReferenceSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureRelationshipReference_Impl: Sized + windows_core::IUnknownImpl {
    fn GetSourceUri(&self) -> windows_core::Result<IOpcUri>;
    fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
    fn GetTransformMethod(&self) -> windows_core::Result<OPC_CANONICALIZATION_METHOD>;
    fn GetRelationshipSigningOption(&self) -> windows_core::Result<OPC_RELATIONSHIPS_SIGNING_OPTION>;
    fn GetRelationshipSelectorEnumerator(&self) -> windows_core::Result<IOpcRelationshipSelectorEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSignatureRelationshipReference {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignatureRelationshipReference_Vtbl {
    pub const fn new<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>() -> IOpcSignatureRelationshipReference_Vtbl {
        unsafe extern "system" fn GetSourceUri<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureRelationshipReference_Impl::GetSourceUri(this) {
                Ok(ok__) => {
                    sourceuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestMethod<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureRelationshipReference_Impl::GetDigestMethod(this) {
                Ok(ok__) => {
                    digestmethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestValue<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSignatureRelationshipReference_Impl::GetDigestValue(this, core::mem::transmute_copy(&digestvalue), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetTransformMethod<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureRelationshipReference_Impl::GetTransformMethod(this) {
                Ok(ok__) => {
                    transformmethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipSigningOption<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipsigningoption: *mut OPC_RELATIONSHIPS_SIGNING_OPTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureRelationshipReference_Impl::GetRelationshipSigningOption(this) {
                Ok(ok__) => {
                    relationshipsigningoption.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipSelectorEnumerator<Identity: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectorenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureRelationshipReference_Impl::GetRelationshipSelectorEnumerator(this) {
                Ok(ok__) => {
                    selectorenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSourceUri: GetSourceUri::<Identity, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, OFFSET>,
            GetDigestValue: GetDigestValue::<Identity, OFFSET>,
            GetTransformMethod: GetTransformMethod::<Identity, OFFSET>,
            GetRelationshipSigningOption: GetRelationshipSigningOption::<Identity, OFFSET>,
            GetRelationshipSelectorEnumerator: GetRelationshipSelectorEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureRelationshipReference as windows_core::Interface>::IID
    }
}
pub trait IOpcSignatureRelationshipReferenceEnumerator_Impl: Sized + windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> windows_core::Result<IOpcSignatureRelationshipReference>;
    fn Clone(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
}
impl windows_core::RuntimeName for IOpcSignatureRelationshipReferenceEnumerator {}
impl IOpcSignatureRelationshipReferenceEnumerator_Vtbl {
    pub const fn new<Identity: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>() -> IOpcSignatureRelationshipReferenceEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureRelationshipReferenceEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureRelationshipReferenceEnumerator_Impl::MovePrevious(this) {
                Ok(ok__) => {
                    hasprevious.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureRelationshipReferenceEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    relationshipreference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureRelationshipReferenceEnumerator_Impl::Clone(this) {
                Ok(ok__) => {
                    copy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            MovePrevious: MovePrevious::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureRelationshipReferenceEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureRelationshipReferenceSet_Impl: Sized + windows_core::IUnknownImpl {
    fn Create(&self, sourceuri: Option<&IOpcUri>, digestmethod: &windows_core::PCWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: Option<&IOpcRelationshipSelectorSet>, transformmethod: OPC_CANONICALIZATION_METHOD) -> windows_core::Result<IOpcSignatureRelationshipReference>;
    fn CreateRelationshipSelectorSet(&self) -> windows_core::Result<IOpcRelationshipSelectorSet>;
    fn Delete(&self, relationshipreference: Option<&IOpcSignatureRelationshipReference>) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSignatureRelationshipReferenceSet {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignatureRelationshipReferenceSet_Vtbl {
    pub const fn new<Identity: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>() -> IOpcSignatureRelationshipReferenceSet_Vtbl {
        unsafe extern "system" fn Create<Identity: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceuri: *mut core::ffi::c_void, digestmethod: windows_core::PCWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: *mut core::ffi::c_void, transformmethod: OPC_CANONICALIZATION_METHOD, relationshipreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureRelationshipReferenceSet_Impl::Create(this, windows_core::from_raw_borrowed(&sourceuri), core::mem::transmute(&digestmethod), core::mem::transmute_copy(&relationshipsigningoption), windows_core::from_raw_borrowed(&selectorset), core::mem::transmute_copy(&transformmethod)) {
                Ok(ok__) => {
                    relationshipreference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRelationshipSelectorSet<Identity: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectorset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureRelationshipReferenceSet_Impl::CreateRelationshipSelectorSet(this) {
                Ok(ok__) => {
                    selectorset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipreference: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSignatureRelationshipReferenceSet_Impl::Delete(this, windows_core::from_raw_borrowed(&relationshipreference)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipreferenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSignatureRelationshipReferenceSet_Impl::GetEnumerator(this) {
                Ok(ok__) => {
                    relationshipreferenceenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            CreateRelationshipSelectorSet: CreateRelationshipSelectorSet::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSignatureRelationshipReferenceSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSigningOptions_Impl: Sized + windows_core::IUnknownImpl {
    fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSignatureId(&self, signatureid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSignatureMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSignatureMethod(&self, signaturemethod: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDefaultDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDefaultDigestMethod(&self, digestmethod: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCertificateEmbeddingOption(&self) -> windows_core::Result<OPC_CERTIFICATE_EMBEDDING_OPTION>;
    fn SetCertificateEmbeddingOption(&self, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> windows_core::Result<()>;
    fn GetTimeFormat(&self) -> windows_core::Result<OPC_SIGNATURE_TIME_FORMAT>;
    fn SetTimeFormat(&self, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> windows_core::Result<()>;
    fn GetSignaturePartReferenceSet(&self) -> windows_core::Result<IOpcSignaturePartReferenceSet>;
    fn GetSignatureRelationshipReferenceSet(&self) -> windows_core::Result<IOpcSignatureRelationshipReferenceSet>;
    fn GetCustomObjectSet(&self) -> windows_core::Result<IOpcSignatureCustomObjectSet>;
    fn GetCustomReferenceSet(&self) -> windows_core::Result<IOpcSignatureReferenceSet>;
    fn GetCertificateSet(&self) -> windows_core::Result<IOpcCertificateSet>;
    fn GetSignaturePartName(&self) -> windows_core::Result<IOpcPartUri>;
    fn SetSignaturePartName(&self, signaturepartname: Option<&IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcSigningOptions {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSigningOptions_Vtbl {
    pub const fn new<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>() -> IOpcSigningOptions_Vtbl {
        unsafe extern "system" fn GetSignatureId<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSigningOptions_Impl::GetSignatureId(this) {
                Ok(ok__) => {
                    signatureid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureId<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureid: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSigningOptions_Impl::SetSignatureId(this, core::mem::transmute(&signatureid)).into()
        }
        unsafe extern "system" fn GetSignatureMethod<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturemethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSigningOptions_Impl::GetSignatureMethod(this) {
                Ok(ok__) => {
                    signaturemethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureMethod<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturemethod: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSigningOptions_Impl::SetSignatureMethod(this, core::mem::transmute(&signaturemethod)).into()
        }
        unsafe extern "system" fn GetDefaultDigestMethod<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSigningOptions_Impl::GetDefaultDigestMethod(this) {
                Ok(ok__) => {
                    digestmethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultDigestMethod<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSigningOptions_Impl::SetDefaultDigestMethod(this, core::mem::transmute(&digestmethod)).into()
        }
        unsafe extern "system" fn GetCertificateEmbeddingOption<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, embeddingoption: *mut OPC_CERTIFICATE_EMBEDDING_OPTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSigningOptions_Impl::GetCertificateEmbeddingOption(this) {
                Ok(ok__) => {
                    embeddingoption.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateEmbeddingOption<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSigningOptions_Impl::SetCertificateEmbeddingOption(this, core::mem::transmute_copy(&embeddingoption)).into()
        }
        unsafe extern "system" fn GetTimeFormat<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSigningOptions_Impl::GetTimeFormat(this) {
                Ok(ok__) => {
                    timeformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeFormat<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSigningOptions_Impl::SetTimeFormat(this, core::mem::transmute_copy(&timeformat)).into()
        }
        unsafe extern "system" fn GetSignaturePartReferenceSet<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partreferenceset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSigningOptions_Impl::GetSignaturePartReferenceSet(this) {
                Ok(ok__) => {
                    partreferenceset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceSet<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipreferenceset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSigningOptions_Impl::GetSignatureRelationshipReferenceSet(this) {
                Ok(ok__) => {
                    relationshipreferenceset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectSet<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSigningOptions_Impl::GetCustomObjectSet(this) {
                Ok(ok__) => {
                    customobjectset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceSet<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customreferenceset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSigningOptions_Impl::GetCustomReferenceSet(this) {
                Ok(ok__) => {
                    customreferenceset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateSet<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificateset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSigningOptions_Impl::GetCertificateSet(this) {
                Ok(ok__) => {
                    certificateset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcSigningOptions_Impl::GetSignaturePartName(this) {
                Ok(ok__) => {
                    signaturepartname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignaturePartName<Identity: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpcSigningOptions_Impl::SetSignaturePartName(this, windows_core::from_raw_borrowed(&signaturepartname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignatureId: GetSignatureId::<Identity, OFFSET>,
            SetSignatureId: SetSignatureId::<Identity, OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Identity, OFFSET>,
            SetSignatureMethod: SetSignatureMethod::<Identity, OFFSET>,
            GetDefaultDigestMethod: GetDefaultDigestMethod::<Identity, OFFSET>,
            SetDefaultDigestMethod: SetDefaultDigestMethod::<Identity, OFFSET>,
            GetCertificateEmbeddingOption: GetCertificateEmbeddingOption::<Identity, OFFSET>,
            SetCertificateEmbeddingOption: SetCertificateEmbeddingOption::<Identity, OFFSET>,
            GetTimeFormat: GetTimeFormat::<Identity, OFFSET>,
            SetTimeFormat: SetTimeFormat::<Identity, OFFSET>,
            GetSignaturePartReferenceSet: GetSignaturePartReferenceSet::<Identity, OFFSET>,
            GetSignatureRelationshipReferenceSet: GetSignatureRelationshipReferenceSet::<Identity, OFFSET>,
            GetCustomObjectSet: GetCustomObjectSet::<Identity, OFFSET>,
            GetCustomReferenceSet: GetCustomReferenceSet::<Identity, OFFSET>,
            GetCertificateSet: GetCertificateSet::<Identity, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, OFFSET>,
            SetSignaturePartName: SetSignaturePartName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcSigningOptions as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcUri_Impl: Sized + super::super::super::System::Com::IUri_Impl {
    fn GetRelationshipsPartUri(&self) -> windows_core::Result<IOpcPartUri>;
    fn GetRelativeUri(&self, targetparturi: Option<&IOpcPartUri>) -> windows_core::Result<super::super::super::System::Com::IUri>;
    fn CombinePartUri(&self, relativeuri: Option<&super::super::super::System::Com::IUri>) -> windows_core::Result<IOpcPartUri>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpcUri {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcUri_Vtbl {
    pub const fn new<Identity: IOpcUri_Impl, const OFFSET: isize>() -> IOpcUri_Vtbl {
        unsafe extern "system" fn GetRelationshipsPartUri<Identity: IOpcUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relationshipparturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcUri_Impl::GetRelationshipsPartUri(this) {
                Ok(ok__) => {
                    relationshipparturi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeUri<Identity: IOpcUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetparturi: *mut core::ffi::c_void, relativeuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcUri_Impl::GetRelativeUri(this, windows_core::from_raw_borrowed(&targetparturi)) {
                Ok(ok__) => {
                    relativeuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CombinePartUri<Identity: IOpcUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relativeuri: *mut core::ffi::c_void, combineduri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpcUri_Impl::CombinePartUri(this, windows_core::from_raw_borrowed(&relativeuri)) {
                Ok(ok__) => {
                    combineduri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IUri_Vtbl::new::<Identity, OFFSET>(),
            GetRelationshipsPartUri: GetRelationshipsPartUri::<Identity, OFFSET>,
            GetRelativeUri: GetRelativeUri::<Identity, OFFSET>,
            CombinePartUri: CombinePartUri::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpcUri as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IUri as windows_core::Interface>::IID
    }
}
