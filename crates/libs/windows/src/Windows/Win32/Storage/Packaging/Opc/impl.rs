#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub trait IOpcCertificateEnumerator_Impl: Sized {
    fn MoveNext(&self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn MovePrevious(&self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCurrent(&self, certificate: *const *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IOpcCertificateEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::windows::core::RuntimeName for IOpcCertificateEnumerator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl IOpcCertificateEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcCertificateEnumerator_Impl, const OFFSET: isize>() -> IOpcCertificateEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveNext(::core::mem::transmute_copy(&hasnext)).into()
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MovePrevious(::core::mem::transmute_copy(&hasprevious)).into()
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: *const *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrent(::core::mem::transmute_copy(&certificate)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcCertificateEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub trait IOpcCertificateSet_Impl: Sized {
    fn Add(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<()>;
    fn Remove(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<()>;
    fn GetEnumerator(&self) -> ::windows::core::Result<IOpcCertificateEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::windows::core::RuntimeName for IOpcCertificateSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl IOpcCertificateSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcCertificateSet_Impl, const OFFSET: isize>() -> IOpcCertificateSet_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcCertificateSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute_copy(&certificate)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcCertificateSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&certificate)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcCertificateSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(certificateenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcCertificateSet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcDigitalSignature_Impl: Sized {
    fn GetNamespaces(&self, prefixes: *mut *mut ::windows::core::PWSTR, namespaces: *mut *mut ::windows::core::PWSTR, count: *mut u32) -> ::windows::core::Result<()>;
    fn GetSignatureId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetSignaturePartName(&self) -> ::windows::core::Result<IOpcPartUri>;
    fn GetSignatureMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetCanonicalizationMethod(&self, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<()>;
    fn GetSignatureValue(&self, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
    fn GetSignaturePartReferenceEnumerator(&self) -> ::windows::core::Result<IOpcSignaturePartReferenceEnumerator>;
    fn GetSignatureRelationshipReferenceEnumerator(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
    fn GetSigningTime(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetTimeFormat(&self, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::Result<()>;
    fn GetPackageObjectReference(&self) -> ::windows::core::Result<IOpcSignatureReference>;
    fn GetCertificateEnumerator(&self) -> ::windows::core::Result<IOpcCertificateEnumerator>;
    fn GetCustomReferenceEnumerator(&self) -> ::windows::core::Result<IOpcSignatureReferenceEnumerator>;
    fn GetCustomObjectEnumerator(&self) -> ::windows::core::Result<IOpcSignatureCustomObjectEnumerator>;
    fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IOpcDigitalSignature {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcDigitalSignature_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>() -> IOpcDigitalSignature_Vtbl {
        unsafe extern "system" fn GetNamespaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefixes: *mut *mut ::windows::core::PWSTR, namespaces: *mut *mut ::windows::core::PWSTR, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNamespaces(::core::mem::transmute_copy(&prefixes), ::core::mem::transmute_copy(&namespaces), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetSignatureId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignaturePartName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturepartname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturemethod, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCanonicalizationMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCanonicalizationMethod(::core::mem::transmute_copy(&canonicalizationmethod)).into()
        }
        unsafe extern "system" fn GetSignatureValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSignatureValue(::core::mem::transmute_copy(&signaturevalue), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetSignaturePartReferenceEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignaturePartReferenceEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partreferenceenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureRelationshipReferenceEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipreferenceenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signingtime: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSigningTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signingtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTimeFormat(::core::mem::transmute_copy(&timeformat)).into()
        }
        unsafe extern "system" fn GetPackageObjectReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPackageObjectReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageobjectreference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCertificateEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(certificateenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustomReferenceEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customreferenceenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustomObjectEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobjectenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureXml<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSignatureXml(::core::mem::transmute_copy(&signaturexml), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetNamespaces: GetNamespaces::<Identity, Impl, OFFSET>,
            GetSignatureId: GetSignatureId::<Identity, Impl, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, Impl, OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Identity, Impl, OFFSET>,
            GetCanonicalizationMethod: GetCanonicalizationMethod::<Identity, Impl, OFFSET>,
            GetSignatureValue: GetSignatureValue::<Identity, Impl, OFFSET>,
            GetSignaturePartReferenceEnumerator: GetSignaturePartReferenceEnumerator::<Identity, Impl, OFFSET>,
            GetSignatureRelationshipReferenceEnumerator: GetSignatureRelationshipReferenceEnumerator::<Identity, Impl, OFFSET>,
            GetSigningTime: GetSigningTime::<Identity, Impl, OFFSET>,
            GetTimeFormat: GetTimeFormat::<Identity, Impl, OFFSET>,
            GetPackageObjectReference: GetPackageObjectReference::<Identity, Impl, OFFSET>,
            GetCertificateEnumerator: GetCertificateEnumerator::<Identity, Impl, OFFSET>,
            GetCustomReferenceEnumerator: GetCustomReferenceEnumerator::<Identity, Impl, OFFSET>,
            GetCustomObjectEnumerator: GetCustomObjectEnumerator::<Identity, Impl, OFFSET>,
            GetSignatureXml: GetSignatureXml::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcDigitalSignature as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcDigitalSignatureEnumerator_Impl: Sized {
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> ::windows::core::Result<IOpcDigitalSignature>;
    fn Clone(&self) -> ::windows::core::Result<IOpcDigitalSignatureEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IOpcDigitalSignatureEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IOpcDigitalSignatureEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>() -> IOpcDigitalSignatureEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digitalsignature, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcDigitalSignatureEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
pub trait IOpcDigitalSignatureManager_Impl: Sized {
    fn GetSignatureOriginPartName(&self) -> ::windows::core::Result<IOpcPartUri>;
    fn SetSignatureOriginPartName(&self, signatureoriginpartname: &::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<()>;
    fn GetSignatureEnumerator(&self) -> ::windows::core::Result<IOpcDigitalSignatureEnumerator>;
    fn RemoveSignature(&self, signaturepartname: &::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<()>;
    fn CreateSigningOptions(&self) -> ::windows::core::Result<IOpcSigningOptions>;
    fn Validate(&self, signature: &::core::option::Option<IOpcDigitalSignature>, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows::core::Result<()>;
    fn Sign(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: &::core::option::Option<IOpcSigningOptions>) -> ::windows::core::Result<IOpcDigitalSignature>;
    fn ReplaceSignatureXml(&self, signaturepartname: &::core::option::Option<IOpcPartUri>, newsignaturexml: *const u8, count: u32) -> ::windows::core::Result<IOpcDigitalSignature>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IOpcDigitalSignatureManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
impl IOpcDigitalSignatureManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>() -> IOpcDigitalSignatureManager_Vtbl {
        unsafe extern "system" fn GetSignatureOriginPartName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureOriginPartName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureoriginpartname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSignatureOriginPartName(::core::mem::transmute(&signatureoriginpartname)).into()
        }
        unsafe extern "system" fn GetSignatureEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSignature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveSignature(::core::mem::transmute(&signaturepartname)).into()
        }
        unsafe extern "system" fn CreateSigningOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signingoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSigningOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signingoptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Validate(::core::mem::transmute(&signature), ::core::mem::transmute_copy(&certificate), ::core::mem::transmute_copy(&validationresult)).into()
        }
        unsafe extern "system" fn Sign<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: *mut ::core::ffi::c_void, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Sign(::core::mem::transmute_copy(&certificate), ::core::mem::transmute(&signingoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digitalsignature, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceSignatureXml<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void, newsignaturexml: *const u8, count: u32, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReplaceSignatureXml(::core::mem::transmute(&signaturepartname), ::core::mem::transmute_copy(&newsignaturexml), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digitalsignature, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSignatureOriginPartName: GetSignatureOriginPartName::<Identity, Impl, OFFSET>,
            SetSignatureOriginPartName: SetSignatureOriginPartName::<Identity, Impl, OFFSET>,
            GetSignatureEnumerator: GetSignatureEnumerator::<Identity, Impl, OFFSET>,
            RemoveSignature: RemoveSignature::<Identity, Impl, OFFSET>,
            CreateSigningOptions: CreateSigningOptions::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
            Sign: Sign::<Identity, Impl, OFFSET>,
            ReplaceSignatureXml: ReplaceSignatureXml::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcDigitalSignatureManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
pub trait IOpcFactory_Impl: Sized {
    fn CreatePackageRootUri(&self) -> ::windows::core::Result<IOpcUri>;
    fn CreatePartUri(&self, pwzuri: &::windows::core::PCWSTR) -> ::windows::core::Result<IOpcPartUri>;
    fn CreateStreamOnFile(&self, filename: &::windows::core::PCWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
    fn CreatePackage(&self) -> ::windows::core::Result<IOpcPackage>;
    fn ReadPackageFromStream(&self, stream: &::core::option::Option<super::super::super::System::Com::IStream>, flags: OPC_READ_FLAGS) -> ::windows::core::Result<IOpcPackage>;
    fn WritePackageToStream(&self, package: &::core::option::Option<IOpcPackage>, flags: OPC_WRITE_FLAGS, stream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn CreateDigitalSignatureManager(&self, package: &::core::option::Option<IOpcPackage>) -> ::windows::core::Result<IOpcDigitalSignatureManager>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IOpcFactory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
impl IOpcFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: isize>() -> IOpcFactory_Vtbl {
        unsafe extern "system" fn CreatePackageRootUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rooturi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackageRootUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rooturi, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzuri: ::windows::core::PCWSTR, parturi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePartUri(::core::mem::transmute(&pwzuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parturi, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStreamOnFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStreamOnFile(::core::mem::transmute(&filename), ::core::mem::transmute_copy(&iomode), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&dwflagsandattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadPackageFromStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, flags: OPC_READ_FLAGS, package: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadPackageFromStream(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WritePackageToStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, flags: OPC_WRITE_FLAGS, stream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WritePackageToStream(::core::mem::transmute(&package), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&stream)).into()
        }
        unsafe extern "system" fn CreateDigitalSignatureManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, signaturemanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDigitalSignatureManager(::core::mem::transmute(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturemanager, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreatePackageRootUri: CreatePackageRootUri::<Identity, Impl, OFFSET>,
            CreatePartUri: CreatePartUri::<Identity, Impl, OFFSET>,
            CreateStreamOnFile: CreateStreamOnFile::<Identity, Impl, OFFSET>,
            CreatePackage: CreatePackage::<Identity, Impl, OFFSET>,
            ReadPackageFromStream: ReadPackageFromStream::<Identity, Impl, OFFSET>,
            WritePackageToStream: WritePackageToStream::<Identity, Impl, OFFSET>,
            CreateDigitalSignatureManager: CreateDigitalSignatureManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcFactory as ::windows::core::Interface>::IID
    }
}
pub trait IOpcPackage_Impl: Sized {
    fn GetPartSet(&self) -> ::windows::core::Result<IOpcPartSet>;
    fn GetRelationshipSet(&self) -> ::windows::core::Result<IOpcRelationshipSet>;
}
impl ::windows::core::RuntimeName for IOpcPackage {}
impl IOpcPackage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPackage_Impl, const OFFSET: isize>() -> IOpcPackage_Vtbl {
        unsafe extern "system" fn GetPartSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRelationshipSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPartSet: GetPartSet::<Identity, Impl, OFFSET>,
            GetRelationshipSet: GetRelationshipSet::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcPackage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcPart_Impl: Sized {
    fn GetRelationshipSet(&self) -> ::windows::core::Result<IOpcRelationshipSet>;
    fn GetContentStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
    fn GetName(&self) -> ::windows::core::Result<IOpcPartUri>;
    fn GetContentType(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetCompressionOptions(&self) -> ::windows::core::Result<OPC_COMPRESSION_OPTIONS>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IOpcPart {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcPart_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPart_Impl, const OFFSET: isize>() -> IOpcPart_Vtbl {
        unsafe extern "system" fn GetRelationshipSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRelationshipSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContentStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contenttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompressionOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compressionoptions: *mut OPC_COMPRESSION_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCompressionOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(compressionoptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRelationshipSet: GetRelationshipSet::<Identity, Impl, OFFSET>,
            GetContentStream: GetContentStream::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetContentType: GetContentType::<Identity, Impl, OFFSET>,
            GetCompressionOptions: GetCompressionOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcPart as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcPartEnumerator_Impl: Sized {
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> ::windows::core::Result<IOpcPart>;
    fn Clone(&self) -> ::windows::core::Result<IOpcPartEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IOpcPartEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IOpcPartEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartEnumerator_Impl, const OFFSET: isize>() -> IOpcPartEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(part, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcPartEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcPartSet_Impl: Sized {
    fn GetPart(&self, name: &::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<IOpcPart>;
    fn CreatePart(&self, name: &::core::option::Option<IOpcPartUri>, contenttype: &::windows::core::PCWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS) -> ::windows::core::Result<IOpcPart>;
    fn DeletePart(&self, name: &::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<()>;
    fn PartExists(&self, name: &::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetEnumerator(&self) -> ::windows::core::Result<IOpcPartEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IOpcPartSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcPartSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartSet_Impl, const OFFSET: isize>() -> IOpcPartSet_Vtbl {
        unsafe extern "system" fn GetPart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPart(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(part, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, contenttype: ::windows::core::PCWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS, part: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePart(::core::mem::transmute(&name), ::core::mem::transmute(&contenttype), ::core::mem::transmute_copy(&compressionoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(part, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePart(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn PartExists<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, partexists: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PartExists(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partexists, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPart: GetPart::<Identity, Impl, OFFSET>,
            CreatePart: CreatePart::<Identity, Impl, OFFSET>,
            DeletePart: DeletePart::<Identity, Impl, OFFSET>,
            PartExists: PartExists::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcPartSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcPartUri_Impl: Sized + super::super::super::System::Com::IUri_Impl + IOpcUri_Impl {
    fn ComparePartUri(&self, parturi: &::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<i32>;
    fn GetSourceUri(&self) -> ::windows::core::Result<IOpcUri>;
    fn IsRelationshipsPartUri(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IOpcPartUri {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcPartUri_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartUri_Impl, const OFFSET: isize>() -> IOpcPartUri_Vtbl {
        unsafe extern "system" fn ComparePartUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, comparisonresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComparePartUri(::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(comparisonresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sourceuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRelationshipsPartUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcPartUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isrelationshipuri: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRelationshipsPartUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isrelationshipuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IOpcUri_Vtbl::new::<Identity, Impl, OFFSET>(),
            ComparePartUri: ComparePartUri::<Identity, Impl, OFFSET>,
            GetSourceUri: GetSourceUri::<Identity, Impl, OFFSET>,
            IsRelationshipsPartUri: IsRelationshipsPartUri::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcPartUri as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IUri as ::windows::core::Interface>::IID || iid == &<IOpcUri as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcRelationship_Impl: Sized {
    fn GetId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetRelationshipType(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetSourceUri(&self) -> ::windows::core::Result<IOpcUri>;
    fn GetTargetUri(&self) -> ::windows::core::Result<super::super::super::System::Com::IUri>;
    fn GetTargetMode(&self) -> ::windows::core::Result<OPC_URI_TARGET_MODE>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IOpcRelationship {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcRelationship_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationship_Impl, const OFFSET: isize>() -> IOpcRelationship_Vtbl {
        unsafe extern "system" fn GetId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipidentifier, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshiptype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRelationshipType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshiptype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sourceuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targeturi, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetmode: *mut OPC_URI_TARGET_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetRelationshipType: GetRelationshipType::<Identity, Impl, OFFSET>,
            GetSourceUri: GetSourceUri::<Identity, Impl, OFFSET>,
            GetTargetUri: GetTargetUri::<Identity, Impl, OFFSET>,
            GetTargetMode: GetTargetMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcRelationship as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcRelationshipEnumerator_Impl: Sized {
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> ::windows::core::Result<IOpcRelationship>;
    fn Clone(&self) -> ::windows::core::Result<IOpcRelationshipEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IOpcRelationshipEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IOpcRelationshipEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>() -> IOpcRelationshipEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationship: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationship, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcRelationshipEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IOpcRelationshipSelector_Impl: Sized {
    fn GetSelectorType(&self) -> ::windows::core::Result<OPC_RELATIONSHIP_SELECTOR>;
    fn GetSelectionCriterion(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for IOpcRelationshipSelector {}
impl IOpcRelationshipSelector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSelector_Impl, const OFFSET: isize>() -> IOpcRelationshipSelector_Vtbl {
        unsafe extern "system" fn GetSelectorType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selector: *mut OPC_RELATIONSHIP_SELECTOR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelectorType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selector, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectionCriterion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectioncriterion: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelectionCriterion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selectioncriterion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSelectorType: GetSelectorType::<Identity, Impl, OFFSET>,
            GetSelectionCriterion: GetSelectionCriterion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcRelationshipSelector as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcRelationshipSelectorEnumerator_Impl: Sized {
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> ::windows::core::Result<IOpcRelationshipSelector>;
    fn Clone(&self) -> ::windows::core::Result<IOpcRelationshipSelectorEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IOpcRelationshipSelectorEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IOpcRelationshipSelectorEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>() -> IOpcRelationshipSelectorEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipselector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipselector, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcRelationshipSelectorEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IOpcRelationshipSelectorSet_Impl: Sized {
    fn Create(&self, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: &::windows::core::PCWSTR) -> ::windows::core::Result<IOpcRelationshipSelector>;
    fn Delete(&self, relationshipselector: &::core::option::Option<IOpcRelationshipSelector>) -> ::windows::core::Result<()>;
    fn GetEnumerator(&self) -> ::windows::core::Result<IOpcRelationshipSelectorEnumerator>;
}
impl ::windows::core::RuntimeName for IOpcRelationshipSelectorSet {}
impl IOpcRelationshipSelectorSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>() -> IOpcRelationshipSelectorSet_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: ::windows::core::PCWSTR, relationshipselector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute_copy(&selector), ::core::mem::transmute(&selectioncriterion)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipselector, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipselector: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&relationshipselector)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipselectorenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipselectorenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcRelationshipSelectorSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcRelationshipSet_Impl: Sized {
    fn GetRelationship(&self, relationshipidentifier: &::windows::core::PCWSTR) -> ::windows::core::Result<IOpcRelationship>;
    fn CreateRelationship(&self, relationshipidentifier: &::windows::core::PCWSTR, relationshiptype: &::windows::core::PCWSTR, targeturi: &::core::option::Option<super::super::super::System::Com::IUri>, targetmode: OPC_URI_TARGET_MODE) -> ::windows::core::Result<IOpcRelationship>;
    fn DeleteRelationship(&self, relationshipidentifier: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn RelationshipExists(&self, relationshipidentifier: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetEnumerator(&self) -> ::windows::core::Result<IOpcRelationshipEnumerator>;
    fn GetEnumeratorForType(&self, relationshiptype: &::windows::core::PCWSTR) -> ::windows::core::Result<IOpcRelationshipEnumerator>;
    fn GetRelationshipsContentStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IOpcRelationshipSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcRelationshipSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>() -> IOpcRelationshipSet_Vtbl {
        unsafe extern "system" fn GetRelationship<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows::core::PCWSTR, relationship: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRelationship(::core::mem::transmute(&relationshipidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationship, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRelationship<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows::core::PCWSTR, relationshiptype: ::windows::core::PCWSTR, targeturi: *mut ::core::ffi::c_void, targetmode: OPC_URI_TARGET_MODE, relationship: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRelationship(::core::mem::transmute(&relationshipidentifier), ::core::mem::transmute(&relationshiptype), ::core::mem::transmute(&targeturi), ::core::mem::transmute_copy(&targetmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationship, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRelationship<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRelationship(::core::mem::transmute(&relationshipidentifier)).into()
        }
        unsafe extern "system" fn RelationshipExists<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows::core::PCWSTR, relationshipexists: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RelationshipExists(::core::mem::transmute(&relationshipidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipexists, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumeratorForType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshiptype: ::windows::core::PCWSTR, relationshipenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumeratorForType(::core::mem::transmute(&relationshiptype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipsContentStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contents: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRelationshipsContentStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contents, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRelationship: GetRelationship::<Identity, Impl, OFFSET>,
            CreateRelationship: CreateRelationship::<Identity, Impl, OFFSET>,
            DeleteRelationship: DeleteRelationship::<Identity, Impl, OFFSET>,
            RelationshipExists: RelationshipExists::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
            GetEnumeratorForType: GetEnumeratorForType::<Identity, Impl, OFFSET>,
            GetRelationshipsContentStream: GetRelationshipsContentStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcRelationshipSet as ::windows::core::Interface>::IID
    }
}
pub trait IOpcSignatureCustomObject_Impl: Sized {
    fn GetXml(&self, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IOpcSignatureCustomObject {}
impl IOpcSignatureCustomObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureCustomObject_Impl, const OFFSET: isize>() -> IOpcSignatureCustomObject_Vtbl {
        unsafe extern "system" fn GetXml<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureCustomObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetXml(::core::mem::transmute_copy(&xmlmarkup), ::core::mem::transmute_copy(&count)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetXml: GetXml::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureCustomObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcSignatureCustomObjectEnumerator_Impl: Sized {
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> ::windows::core::Result<IOpcSignatureCustomObject>;
    fn Clone(&self) -> ::windows::core::Result<IOpcSignatureCustomObjectEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IOpcSignatureCustomObjectEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IOpcSignatureCustomObjectEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>() -> IOpcSignatureCustomObjectEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureCustomObjectEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IOpcSignatureCustomObjectSet_Impl: Sized {
    fn Create(&self, xmlmarkup: *const u8, count: u32) -> ::windows::core::Result<IOpcSignatureCustomObject>;
    fn Delete(&self, customobject: &::core::option::Option<IOpcSignatureCustomObject>) -> ::windows::core::Result<()>;
    fn GetEnumerator(&self) -> ::windows::core::Result<IOpcSignatureCustomObjectEnumerator>;
}
impl ::windows::core::RuntimeName for IOpcSignatureCustomObjectSet {}
impl IOpcSignatureCustomObjectSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>() -> IOpcSignatureCustomObjectSet_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlmarkup: *const u8, count: u32, customobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute_copy(&xmlmarkup), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&customobject)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobjectenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureCustomObjectSet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignaturePartReference_Impl: Sized {
    fn GetPartName(&self) -> ::windows::core::Result<IOpcPartUri>;
    fn GetContentType(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetDigestMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
    fn GetTransformMethod(&self) -> ::windows::core::Result<OPC_CANONICALIZATION_METHOD>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IOpcSignaturePartReference {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignaturePartReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReference_Impl, const OFFSET: isize>() -> IOpcSignaturePartReference_Vtbl {
        unsafe extern "system" fn GetPartName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contenttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDigestMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digestmethod, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDigestValue(::core::mem::transmute_copy(&digestvalue), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetTransformMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransformMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transformmethod, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPartName: GetPartName::<Identity, Impl, OFFSET>,
            GetContentType: GetContentType::<Identity, Impl, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, Impl, OFFSET>,
            GetDigestValue: GetDigestValue::<Identity, Impl, OFFSET>,
            GetTransformMethod: GetTransformMethod::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignaturePartReference as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcSignaturePartReferenceEnumerator_Impl: Sized {
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> ::windows::core::Result<IOpcSignaturePartReference>;
    fn Clone(&self) -> ::windows::core::Result<IOpcSignaturePartReferenceEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IOpcSignaturePartReferenceEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IOpcSignaturePartReferenceEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>() -> IOpcSignaturePartReferenceEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partreference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignaturePartReferenceEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignaturePartReferenceSet_Impl: Sized {
    fn Create(&self, parturi: &::core::option::Option<IOpcPartUri>, digestmethod: &::windows::core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<IOpcSignaturePartReference>;
    fn Delete(&self, partreference: &::core::option::Option<IOpcSignaturePartReference>) -> ::windows::core::Result<()>;
    fn GetEnumerator(&self) -> ::windows::core::Result<IOpcSignaturePartReferenceEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IOpcSignaturePartReferenceSet {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignaturePartReferenceSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>() -> IOpcSignaturePartReferenceSet_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, digestmethod: ::windows::core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, partreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute(&parturi), ::core::mem::transmute(&digestmethod), ::core::mem::transmute_copy(&transformmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partreference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreference: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&partreference)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partreferenceenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignaturePartReferenceSet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureReference_Impl: Sized {
    fn GetId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetUri(&self) -> ::windows::core::Result<super::super::super::System::Com::IUri>;
    fn GetType(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetTransformMethod(&self) -> ::windows::core::Result<OPC_CANONICALIZATION_METHOD>;
    fn GetDigestMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IOpcSignatureReference {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignatureReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: isize>() -> IOpcSignatureReference_Vtbl {
        unsafe extern "system" fn GetId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referenceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(referenceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referenceuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(referenceuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransformMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transformmethod, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDigestMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digestmethod, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDigestValue(::core::mem::transmute_copy(&digestvalue), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetUri: GetUri::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetTransformMethod: GetTransformMethod::<Identity, Impl, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, Impl, OFFSET>,
            GetDigestValue: GetDigestValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureReference as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcSignatureReferenceEnumerator_Impl: Sized {
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> ::windows::core::Result<IOpcSignatureReference>;
    fn Clone(&self) -> ::windows::core::Result<IOpcSignatureReferenceEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IOpcSignatureReferenceEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IOpcSignatureReferenceEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>() -> IOpcSignatureReferenceEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureReferenceEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureReferenceSet_Impl: Sized {
    fn Create(&self, referenceuri: &::core::option::Option<super::super::super::System::Com::IUri>, referenceid: &::windows::core::PCWSTR, r#type: &::windows::core::PCWSTR, digestmethod: &::windows::core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<IOpcSignatureReference>;
    fn Delete(&self, reference: &::core::option::Option<IOpcSignatureReference>) -> ::windows::core::Result<()>;
    fn GetEnumerator(&self) -> ::windows::core::Result<IOpcSignatureReferenceEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IOpcSignatureReferenceSet {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignatureReferenceSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>() -> IOpcSignatureReferenceSet_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referenceuri: *mut ::core::ffi::c_void, referenceid: ::windows::core::PCWSTR, r#type: ::windows::core::PCWSTR, digestmethod: ::windows::core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, reference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute(&referenceuri), ::core::mem::transmute(&referenceid), ::core::mem::transmute(&r#type), ::core::mem::transmute(&digestmethod), ::core::mem::transmute_copy(&transformmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&reference)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(referenceenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureReferenceSet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureRelationshipReference_Impl: Sized {
    fn GetSourceUri(&self) -> ::windows::core::Result<IOpcUri>;
    fn GetDigestMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
    fn GetTransformMethod(&self) -> ::windows::core::Result<OPC_CANONICALIZATION_METHOD>;
    fn GetRelationshipSigningOption(&self) -> ::windows::core::Result<OPC_RELATIONSHIPS_SIGNING_OPTION>;
    fn GetRelationshipSelectorEnumerator(&self) -> ::windows::core::Result<IOpcRelationshipSelectorEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IOpcSignatureRelationshipReference {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignatureRelationshipReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>() -> IOpcSignatureRelationshipReference_Vtbl {
        unsafe extern "system" fn GetSourceUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sourceuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDigestMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digestmethod, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDigestValue(::core::mem::transmute_copy(&digestvalue), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetTransformMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransformMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transformmethod, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipSigningOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipsigningoption: *mut OPC_RELATIONSHIPS_SIGNING_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRelationshipSigningOption() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipsigningoption, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipSelectorEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectorenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRelationshipSelectorEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selectorenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSourceUri: GetSourceUri::<Identity, Impl, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, Impl, OFFSET>,
            GetDigestValue: GetDigestValue::<Identity, Impl, OFFSET>,
            GetTransformMethod: GetTransformMethod::<Identity, Impl, OFFSET>,
            GetRelationshipSigningOption: GetRelationshipSigningOption::<Identity, Impl, OFFSET>,
            GetRelationshipSelectorEnumerator: GetRelationshipSelectorEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureRelationshipReference as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcSignatureRelationshipReferenceEnumerator_Impl: Sized {
    fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReference>;
    fn Clone(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IOpcSignatureRelationshipReferenceEnumerator {}
#[cfg(feature = "Win32_Foundation")]
impl IOpcSignatureRelationshipReferenceEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>() -> IOpcSignatureRelationshipReferenceEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasprevious, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipreference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            MovePrevious: MovePrevious::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureRelationshipReferenceEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSignatureRelationshipReferenceSet_Impl: Sized {
    fn Create(&self, sourceuri: &::core::option::Option<IOpcUri>, digestmethod: &::windows::core::PCWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: &::core::option::Option<IOpcRelationshipSelectorSet>, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<IOpcSignatureRelationshipReference>;
    fn CreateRelationshipSelectorSet(&self) -> ::windows::core::Result<IOpcRelationshipSelectorSet>;
    fn Delete(&self, relationshipreference: &::core::option::Option<IOpcSignatureRelationshipReference>) -> ::windows::core::Result<()>;
    fn GetEnumerator(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IOpcSignatureRelationshipReferenceSet {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSignatureRelationshipReferenceSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>() -> IOpcSignatureRelationshipReferenceSet_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceuri: *mut ::core::ffi::c_void, digestmethod: ::windows::core::PCWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: *mut ::core::ffi::c_void, transformmethod: OPC_CANONICALIZATION_METHOD, relationshipreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute(&sourceuri), ::core::mem::transmute(&digestmethod), ::core::mem::transmute_copy(&relationshipsigningoption), ::core::mem::transmute(&selectorset), ::core::mem::transmute_copy(&transformmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipreference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRelationshipSelectorSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectorset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRelationshipSelectorSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selectorset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreference: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&relationshipreference)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipreferenceenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, Impl, OFFSET>,
            CreateRelationshipSelectorSet: CreateRelationshipSelectorSet::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureRelationshipReferenceSet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcSigningOptions_Impl: Sized {
    fn GetSignatureId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetSignatureId(&self, signatureid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetSignatureMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetSignatureMethod(&self, signaturemethod: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetDefaultDigestMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetDefaultDigestMethod(&self, digestmethod: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetCertificateEmbeddingOption(&self) -> ::windows::core::Result<OPC_CERTIFICATE_EMBEDDING_OPTION>;
    fn SetCertificateEmbeddingOption(&self, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::Result<()>;
    fn GetTimeFormat(&self) -> ::windows::core::Result<OPC_SIGNATURE_TIME_FORMAT>;
    fn SetTimeFormat(&self, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::Result<()>;
    fn GetSignaturePartReferenceSet(&self) -> ::windows::core::Result<IOpcSignaturePartReferenceSet>;
    fn GetSignatureRelationshipReferenceSet(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceSet>;
    fn GetCustomObjectSet(&self) -> ::windows::core::Result<IOpcSignatureCustomObjectSet>;
    fn GetCustomReferenceSet(&self) -> ::windows::core::Result<IOpcSignatureReferenceSet>;
    fn GetCertificateSet(&self) -> ::windows::core::Result<IOpcCertificateSet>;
    fn GetSignaturePartName(&self) -> ::windows::core::Result<IOpcPartUri>;
    fn SetSignaturePartName(&self, signaturepartname: &::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IOpcSigningOptions {}
#[cfg(feature = "Win32_System_Com")]
impl IOpcSigningOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>() -> IOpcSigningOptions_Vtbl {
        unsafe extern "system" fn GetSignatureId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSignatureId(::core::mem::transmute(&signatureid)).into()
        }
        unsafe extern "system" fn GetSignatureMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturemethod, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSignatureMethod(::core::mem::transmute(&signaturemethod)).into()
        }
        unsafe extern "system" fn GetDefaultDigestMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultDigestMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digestmethod, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultDigestMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultDigestMethod(::core::mem::transmute(&digestmethod)).into()
        }
        unsafe extern "system" fn GetCertificateEmbeddingOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, embeddingoption: *mut OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCertificateEmbeddingOption() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(embeddingoption, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateEmbeddingOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCertificateEmbeddingOption(::core::mem::transmute_copy(&embeddingoption)).into()
        }
        unsafe extern "system" fn GetTimeFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTimeFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timeformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTimeFormat(::core::mem::transmute_copy(&timeformat)).into()
        }
        unsafe extern "system" fn GetSignaturePartReferenceSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignaturePartReferenceSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partreferenceset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureRelationshipReferenceSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipreferenceset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustomObjectSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobjectset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustomReferenceSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customreferenceset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCertificateSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(certificateset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignaturePartName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturepartname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignaturePartName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSignaturePartName(::core::mem::transmute(&signaturepartname)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSignatureId: GetSignatureId::<Identity, Impl, OFFSET>,
            SetSignatureId: SetSignatureId::<Identity, Impl, OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Identity, Impl, OFFSET>,
            SetSignatureMethod: SetSignatureMethod::<Identity, Impl, OFFSET>,
            GetDefaultDigestMethod: GetDefaultDigestMethod::<Identity, Impl, OFFSET>,
            SetDefaultDigestMethod: SetDefaultDigestMethod::<Identity, Impl, OFFSET>,
            GetCertificateEmbeddingOption: GetCertificateEmbeddingOption::<Identity, Impl, OFFSET>,
            SetCertificateEmbeddingOption: SetCertificateEmbeddingOption::<Identity, Impl, OFFSET>,
            GetTimeFormat: GetTimeFormat::<Identity, Impl, OFFSET>,
            SetTimeFormat: SetTimeFormat::<Identity, Impl, OFFSET>,
            GetSignaturePartReferenceSet: GetSignaturePartReferenceSet::<Identity, Impl, OFFSET>,
            GetSignatureRelationshipReferenceSet: GetSignatureRelationshipReferenceSet::<Identity, Impl, OFFSET>,
            GetCustomObjectSet: GetCustomObjectSet::<Identity, Impl, OFFSET>,
            GetCustomReferenceSet: GetCustomReferenceSet::<Identity, Impl, OFFSET>,
            GetCertificateSet: GetCertificateSet::<Identity, Impl, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, Impl, OFFSET>,
            SetSignaturePartName: SetSignaturePartName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSigningOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcUri_Impl: Sized + super::super::super::System::Com::IUri_Impl {
    fn GetRelationshipsPartUri(&self) -> ::windows::core::Result<IOpcPartUri>;
    fn GetRelativeUri(&self, targetparturi: &::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<super::super::super::System::Com::IUri>;
    fn CombinePartUri(&self, relativeuri: &::core::option::Option<super::super::super::System::Com::IUri>) -> ::windows::core::Result<IOpcPartUri>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IOpcUri {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcUri_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcUri_Impl, const OFFSET: isize>() -> IOpcUri_Vtbl {
        unsafe extern "system" fn GetRelationshipsPartUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipparturi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRelationshipsPartUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relationshipparturi, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetparturi: *mut ::core::ffi::c_void, relativeuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRelativeUri(::core::mem::transmute(&targetparturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relativeuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CombinePartUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOpcUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeuri: *mut ::core::ffi::c_void, combineduri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CombinePartUri(::core::mem::transmute(&relativeuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(combineduri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IUri_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetRelationshipsPartUri: GetRelationshipsPartUri::<Identity, Impl, OFFSET>,
            GetRelativeUri: GetRelativeUri::<Identity, Impl, OFFSET>,
            CombinePartUri: CombinePartUri::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcUri as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IUri as ::windows::core::Interface>::IID
    }
}
