#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub trait IOpcCertificateEnumerator_Impl: Sized {
    fn MoveNext(&mut self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn MovePrevious(&mut self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCurrent(&mut self, certificate: *const *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IOpcCertificateEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl IOpcCertificateEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcCertificateEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcCertificateEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveNext(::core::mem::transmute_copy(&hasnext)).into()
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MovePrevious(::core::mem::transmute_copy(&hasprevious)).into()
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: *const *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrent(::core::mem::transmute_copy(&certificate)).into()
        }
        unsafe extern "system" fn Clone<Impl: IOpcCertificateEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *copy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
            MovePrevious: MovePrevious::<Impl, IMPL_OFFSET>,
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcCertificateEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub trait IOpcCertificateSet_Impl: Sized {
    fn Add(&mut self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<()>;
    fn Remove(&mut self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<()>;
    fn GetEnumerator(&mut self) -> ::windows::core::Result<IOpcCertificateEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl IOpcCertificateSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcCertificateSet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcCertificateSet_Vtbl {
        unsafe extern "system" fn Add<Impl: IOpcCertificateSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&certificate)).into()
        }
        unsafe extern "system" fn Remove<Impl: IOpcCertificateSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&certificate)).into()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcCertificateSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *certificateenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            GetEnumerator: GetEnumerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcCertificateSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcDigitalSignature_Impl: Sized {
    fn GetNamespaces(&mut self, prefixes: *mut *mut super::super::super::Foundation::PWSTR, namespaces: *mut *mut super::super::super::Foundation::PWSTR, count: *mut u32) -> ::windows::core::Result<()>;
    fn GetSignatureId(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetSignaturePartName(&mut self) -> ::windows::core::Result<IOpcPartUri>;
    fn GetSignatureMethod(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetCanonicalizationMethod(&mut self, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<()>;
    fn GetSignatureValue(&mut self, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
    fn GetSignaturePartReferenceEnumerator(&mut self) -> ::windows::core::Result<IOpcSignaturePartReferenceEnumerator>;
    fn GetSignatureRelationshipReferenceEnumerator(&mut self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
    fn GetSigningTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetTimeFormat(&mut self, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::Result<()>;
    fn GetPackageObjectReference(&mut self) -> ::windows::core::Result<IOpcSignatureReference>;
    fn GetCertificateEnumerator(&mut self) -> ::windows::core::Result<IOpcCertificateEnumerator>;
    fn GetCustomReferenceEnumerator(&mut self) -> ::windows::core::Result<IOpcSignatureReferenceEnumerator>;
    fn GetCustomObjectEnumerator(&mut self) -> ::windows::core::Result<IOpcSignatureCustomObjectEnumerator>;
    fn GetSignatureXml(&mut self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcDigitalSignature_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcDigitalSignature_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcDigitalSignature_Vtbl {
        unsafe extern "system" fn GetNamespaces<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefixes: *mut *mut super::super::super::Foundation::PWSTR, namespaces: *mut *mut super::super::super::Foundation::PWSTR, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNamespaces(::core::mem::transmute_copy(&prefixes), ::core::mem::transmute_copy(&namespaces), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetSignatureId<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureId() {
                ::core::result::Result::Ok(ok__) => {
                    *signatureid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartName() {
                ::core::result::Result::Ok(ok__) => {
                    *signaturepartname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureMethod<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *signaturemethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCanonicalizationMethod<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCanonicalizationMethod(::core::mem::transmute_copy(&canonicalizationmethod)).into()
        }
        unsafe extern "system" fn GetSignatureValue<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSignatureValue(::core::mem::transmute_copy(&signaturevalue), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetSignaturePartReferenceEnumerator<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartReferenceEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *partreferenceenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceEnumerator<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureRelationshipReferenceEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipreferenceenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTime<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signingtime: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSigningTime() {
                ::core::result::Result::Ok(ok__) => {
                    *signingtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeFormat<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTimeFormat(::core::mem::transmute_copy(&timeformat)).into()
        }
        unsafe extern "system" fn GetPackageObjectReference<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageobjectreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageObjectReference() {
                ::core::result::Result::Ok(ok__) => {
                    *packageobjectreference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateEnumerator<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *certificateenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomReferenceEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *customreferenceenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomObjectEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *customobjectenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureXml<Impl: IOpcDigitalSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSignatureXml(::core::mem::transmute_copy(&signaturexml), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetNamespaces: GetNamespaces::<Impl, IMPL_OFFSET>,
            GetSignatureId: GetSignatureId::<Impl, IMPL_OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Impl, IMPL_OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Impl, IMPL_OFFSET>,
            GetCanonicalizationMethod: GetCanonicalizationMethod::<Impl, IMPL_OFFSET>,
            GetSignatureValue: GetSignatureValue::<Impl, IMPL_OFFSET>,
            GetSignaturePartReferenceEnumerator: GetSignaturePartReferenceEnumerator::<Impl, IMPL_OFFSET>,
            GetSignatureRelationshipReferenceEnumerator: GetSignatureRelationshipReferenceEnumerator::<Impl, IMPL_OFFSET>,
            GetSigningTime: GetSigningTime::<Impl, IMPL_OFFSET>,
            GetTimeFormat: GetTimeFormat::<Impl, IMPL_OFFSET>,
            GetPackageObjectReference: GetPackageObjectReference::<Impl, IMPL_OFFSET>,
            GetCertificateEnumerator: GetCertificateEnumerator::<Impl, IMPL_OFFSET>,
            GetCustomReferenceEnumerator: GetCustomReferenceEnumerator::<Impl, IMPL_OFFSET>,
            GetCustomObjectEnumerator: GetCustomObjectEnumerator::<Impl, IMPL_OFFSET>,
            GetSignatureXml: GetSignatureXml::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcDigitalSignature as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcDigitalSignatureEnumerator_Impl: Sized {
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&mut self) -> ::windows::core::Result<IOpcDigitalSignature>;
    fn Clone(&mut self) -> ::windows::core::Result<IOpcDigitalSignatureEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcDigitalSignatureEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcDigitalSignatureEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcDigitalSignatureEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    *hasprevious = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digitalsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *digitalsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcDigitalSignatureEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *copy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
            MovePrevious: MovePrevious::<Impl, IMPL_OFFSET>,
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcDigitalSignatureEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
pub trait IOpcDigitalSignatureManager_Impl: Sized {
    fn GetSignatureOriginPartName(&mut self) -> ::windows::core::Result<IOpcPartUri>;
    fn SetSignatureOriginPartName(&mut self, signatureoriginpartname: ::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<()>;
    fn GetSignatureEnumerator(&mut self) -> ::windows::core::Result<IOpcDigitalSignatureEnumerator>;
    fn RemoveSignature(&mut self, signaturepartname: ::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<()>;
    fn CreateSigningOptions(&mut self) -> ::windows::core::Result<IOpcSigningOptions>;
    fn Validate(&mut self, signature: ::core::option::Option<IOpcDigitalSignature>, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows::core::Result<()>;
    fn Sign(&mut self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: ::core::option::Option<IOpcSigningOptions>) -> ::windows::core::Result<IOpcDigitalSignature>;
    fn ReplaceSignatureXml(&mut self, signaturepartname: ::core::option::Option<IOpcPartUri>, newsignaturexml: *const u8, count: u32) -> ::windows::core::Result<IOpcDigitalSignature>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
impl IOpcDigitalSignatureManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcDigitalSignatureManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcDigitalSignatureManager_Vtbl {
        unsafe extern "system" fn GetSignatureOriginPartName<Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureOriginPartName() {
                ::core::result::Result::Ok(ok__) => {
                    *signatureoriginpartname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignatureOriginPartName(::core::mem::transmute(&signatureoriginpartname)).into()
        }
        unsafe extern "system" fn GetSignatureEnumerator<Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *signatureenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSignature<Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSignature(::core::mem::transmute(&signaturepartname)).into()
        }
        unsafe extern "system" fn CreateSigningOptions<Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signingoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSigningOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *signingoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: ::windows::core::RawPtr, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Validate(::core::mem::transmute(&signature), ::core::mem::transmute_copy(&certificate), ::core::mem::transmute_copy(&validationresult)).into()
        }
        unsafe extern "system" fn Sign<Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: ::windows::core::RawPtr, digitalsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sign(::core::mem::transmute_copy(&certificate), ::core::mem::transmute(&signingoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *digitalsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceSignatureXml<Impl: IOpcDigitalSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr, newsignaturexml: *const u8, count: u32, digitalsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplaceSignatureXml(::core::mem::transmute(&signaturepartname), ::core::mem::transmute_copy(&newsignaturexml), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *digitalsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSignatureOriginPartName: GetSignatureOriginPartName::<Impl, IMPL_OFFSET>,
            SetSignatureOriginPartName: SetSignatureOriginPartName::<Impl, IMPL_OFFSET>,
            GetSignatureEnumerator: GetSignatureEnumerator::<Impl, IMPL_OFFSET>,
            RemoveSignature: RemoveSignature::<Impl, IMPL_OFFSET>,
            CreateSigningOptions: CreateSigningOptions::<Impl, IMPL_OFFSET>,
            Validate: Validate::<Impl, IMPL_OFFSET>,
            Sign: Sign::<Impl, IMPL_OFFSET>,
            ReplaceSignatureXml: ReplaceSignatureXml::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcDigitalSignatureManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
pub trait IOpcFactory_Impl: Sized {
    fn CreatePackageRootUri(&mut self) -> ::windows::core::Result<IOpcUri>;
    fn CreatePartUri(&mut self, pwzuri: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IOpcPartUri>;
    fn CreateStreamOnFile(&mut self, filename: super::super::super::Foundation::PWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
    fn CreatePackage(&mut self) -> ::windows::core::Result<IOpcPackage>;
    fn ReadPackageFromStream(&mut self, stream: ::core::option::Option<super::super::super::System::Com::IStream>, flags: OPC_READ_FLAGS) -> ::windows::core::Result<IOpcPackage>;
    fn WritePackageToStream(&mut self, package: ::core::option::Option<IOpcPackage>, flags: OPC_WRITE_FLAGS, stream: ::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn CreateDigitalSignatureManager(&mut self, package: ::core::option::Option<IOpcPackage>) -> ::windows::core::Result<IOpcDigitalSignatureManager>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
impl IOpcFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcFactory_Vtbl {
        unsafe extern "system" fn CreatePackageRootUri<Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rooturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageRootUri() {
                ::core::result::Result::Ok(ok__) => {
                    *rooturi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUri<Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzuri: super::super::super::Foundation::PWSTR, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePartUri(::core::mem::transmute_copy(&pwzuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *parturi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStreamOnFile<Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStreamOnFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&iomode), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&dwflagsandattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackage<Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackage() {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadPackageFromStream<Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, flags: OPC_READ_FLAGS, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadPackageFromStream(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WritePackageToStream<Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, flags: OPC_WRITE_FLAGS, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WritePackageToStream(::core::mem::transmute(&package), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&stream)).into()
        }
        unsafe extern "system" fn CreateDigitalSignatureManager<Impl: IOpcFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, signaturemanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDigitalSignatureManager(::core::mem::transmute(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    *signaturemanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreatePackageRootUri: CreatePackageRootUri::<Impl, IMPL_OFFSET>,
            CreatePartUri: CreatePartUri::<Impl, IMPL_OFFSET>,
            CreateStreamOnFile: CreateStreamOnFile::<Impl, IMPL_OFFSET>,
            CreatePackage: CreatePackage::<Impl, IMPL_OFFSET>,
            ReadPackageFromStream: ReadPackageFromStream::<Impl, IMPL_OFFSET>,
            WritePackageToStream: WritePackageToStream::<Impl, IMPL_OFFSET>,
            CreateDigitalSignatureManager: CreateDigitalSignatureManager::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcFactory as ::windows::core::Interface>::IID
    }
}
pub trait IOpcPackage_Impl: Sized {
    fn GetPartSet(&mut self) -> ::windows::core::Result<IOpcPartSet>;
    fn GetRelationshipSet(&mut self) -> ::windows::core::Result<IOpcRelationshipSet>;
}
impl IOpcPackage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcPackage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcPackage_Vtbl {
        unsafe extern "system" fn GetPartSet<Impl: IOpcPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartSet() {
                ::core::result::Result::Ok(ok__) => {
                    *partset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipSet<Impl: IOpcPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRelationshipSet() {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPartSet: GetPartSet::<Impl, IMPL_OFFSET>,
            GetRelationshipSet: GetRelationshipSet::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcPackage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcPart_Impl: Sized {
    fn GetRelationshipSet(&mut self) -> ::windows::core::Result<IOpcRelationshipSet>;
    fn GetContentStream(&mut self) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
    fn GetName(&mut self) -> ::windows::core::Result<IOpcPartUri>;
    fn GetContentType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetCompressionOptions(&mut self) -> ::windows::core::Result<OPC_COMPRESSION_OPTIONS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcPart_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcPart_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcPart_Vtbl {
        unsafe extern "system" fn GetRelationshipSet<Impl: IOpcPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRelationshipSet() {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentStream<Impl: IOpcPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentStream() {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IOpcPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentType<Impl: IOpcPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *contenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompressionOptions<Impl: IOpcPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compressionoptions: *mut OPC_COMPRESSION_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompressionOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *compressionoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRelationshipSet: GetRelationshipSet::<Impl, IMPL_OFFSET>,
            GetContentStream: GetContentStream::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetContentType: GetContentType::<Impl, IMPL_OFFSET>,
            GetCompressionOptions: GetCompressionOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcPart as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcPartEnumerator_Impl: Sized {
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&mut self) -> ::windows::core::Result<IOpcPart>;
    fn Clone(&mut self) -> ::windows::core::Result<IOpcPartEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcPartEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcPartEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcPartEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    *hasprevious = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *part = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcPartEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *copy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
            MovePrevious: MovePrevious::<Impl, IMPL_OFFSET>,
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcPartEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcPartSet_Impl: Sized {
    fn GetPart(&mut self, name: ::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<IOpcPart>;
    fn CreatePart(&mut self, name: ::core::option::Option<IOpcPartUri>, contenttype: super::super::super::Foundation::PWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS) -> ::windows::core::Result<IOpcPart>;
    fn DeletePart(&mut self, name: ::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<()>;
    fn PartExists(&mut self, name: ::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetEnumerator(&mut self) -> ::windows::core::Result<IOpcPartEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcPartSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcPartSet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcPartSet_Vtbl {
        unsafe extern "system" fn GetPart<Impl: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPart(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *part = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePart<Impl: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::RawPtr, contenttype: super::super::super::Foundation::PWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePart(::core::mem::transmute(&name), ::core::mem::transmute_copy(&contenttype), ::core::mem::transmute_copy(&compressionoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *part = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePart<Impl: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePart(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn PartExists<Impl: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::RawPtr, partexists: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PartExists(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *partexists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcPartSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *partenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPart: GetPart::<Impl, IMPL_OFFSET>,
            CreatePart: CreatePart::<Impl, IMPL_OFFSET>,
            DeletePart: DeletePart::<Impl, IMPL_OFFSET>,
            PartExists: PartExists::<Impl, IMPL_OFFSET>,
            GetEnumerator: GetEnumerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcPartSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcPartUri_Impl: Sized + super::super::super::System::Com::IUri_Impl + IOpcUri_Impl {
    fn ComparePartUri(&mut self, parturi: ::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<i32>;
    fn GetSourceUri(&mut self) -> ::windows::core::Result<IOpcUri>;
    fn IsRelationshipsPartUri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcPartUri_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcPartUri_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcPartUri_Vtbl {
        unsafe extern "system" fn ComparePartUri<Impl: IOpcPartUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, comparisonresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComparePartUri(::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *comparisonresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceUri<Impl: IOpcPartUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *sourceuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRelationshipsPartUri<Impl: IOpcPartUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isrelationshipuri: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRelationshipsPartUri() {
                ::core::result::Result::Ok(ok__) => {
                    *isrelationshipuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOpcUri_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ComparePartUri: ComparePartUri::<Impl, IMPL_OFFSET>,
            GetSourceUri: GetSourceUri::<Impl, IMPL_OFFSET>,
            IsRelationshipsPartUri: IsRelationshipsPartUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcPartUri as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcRelationship_Impl: Sized {
    fn GetId(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetRelationshipType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetSourceUri(&mut self) -> ::windows::core::Result<IOpcUri>;
    fn GetTargetUri(&mut self) -> ::windows::core::Result<super::super::super::System::Com::IUri>;
    fn GetTargetMode(&mut self) -> ::windows::core::Result<OPC_URI_TARGET_MODE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcRelationship_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcRelationship_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcRelationship_Vtbl {
        unsafe extern "system" fn GetId<Impl: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipidentifier = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipType<Impl: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshiptype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRelationshipType() {
                ::core::result::Result::Ok(ok__) => {
                    *relationshiptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceUri<Impl: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *sourceuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetUri<Impl: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetUri() {
                ::core::result::Result::Ok(ok__) => {
                    *targeturi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetMode<Impl: IOpcRelationship_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetmode: *mut OPC_URI_TARGET_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetMode() {
                ::core::result::Result::Ok(ok__) => {
                    *targetmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetId: GetId::<Impl, IMPL_OFFSET>,
            GetRelationshipType: GetRelationshipType::<Impl, IMPL_OFFSET>,
            GetSourceUri: GetSourceUri::<Impl, IMPL_OFFSET>,
            GetTargetUri: GetTargetUri::<Impl, IMPL_OFFSET>,
            GetTargetMode: GetTargetMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcRelationship as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcRelationshipEnumerator_Impl: Sized {
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&mut self) -> ::windows::core::Result<IOpcRelationship>;
    fn Clone(&mut self) -> ::windows::core::Result<IOpcRelationshipEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcRelationshipEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcRelationshipEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcRelationshipEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    *hasprevious = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *relationship = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcRelationshipEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *copy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
            MovePrevious: MovePrevious::<Impl, IMPL_OFFSET>,
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcRelationshipEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcRelationshipSelector_Impl: Sized {
    fn GetSelectorType(&mut self) -> ::windows::core::Result<OPC_RELATIONSHIP_SELECTOR>;
    fn GetSelectionCriterion(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcRelationshipSelector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcRelationshipSelector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcRelationshipSelector_Vtbl {
        unsafe extern "system" fn GetSelectorType<Impl: IOpcRelationshipSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selector: *mut OPC_RELATIONSHIP_SELECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelectorType() {
                ::core::result::Result::Ok(ok__) => {
                    *selector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectionCriterion<Impl: IOpcRelationshipSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectioncriterion: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelectionCriterion() {
                ::core::result::Result::Ok(ok__) => {
                    *selectioncriterion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSelectorType: GetSelectorType::<Impl, IMPL_OFFSET>,
            GetSelectionCriterion: GetSelectionCriterion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcRelationshipSelector as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcRelationshipSelectorEnumerator_Impl: Sized {
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&mut self) -> ::windows::core::Result<IOpcRelationshipSelector>;
    fn Clone(&mut self) -> ::windows::core::Result<IOpcRelationshipSelectorEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcRelationshipSelectorEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcRelationshipSelectorEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcRelationshipSelectorEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    *hasprevious = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipselector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipselector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcRelationshipSelectorEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *copy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
            MovePrevious: MovePrevious::<Impl, IMPL_OFFSET>,
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcRelationshipSelectorEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcRelationshipSelectorSet_Impl: Sized {
    fn Create(&mut self, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IOpcRelationshipSelector>;
    fn Delete(&mut self, relationshipselector: ::core::option::Option<IOpcRelationshipSelector>) -> ::windows::core::Result<()>;
    fn GetEnumerator(&mut self) -> ::windows::core::Result<IOpcRelationshipSelectorEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcRelationshipSelectorSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcRelationshipSelectorSet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcRelationshipSelectorSet_Vtbl {
        unsafe extern "system" fn Create<Impl: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: super::super::super::Foundation::PWSTR, relationshipselector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&selector), ::core::mem::transmute_copy(&selectioncriterion)) {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipselector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute(&relationshipselector)).into()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcRelationshipSelectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipselectorenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipselectorenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            GetEnumerator: GetEnumerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcRelationshipSelectorSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcRelationshipSet_Impl: Sized {
    fn GetRelationship(&mut self, relationshipidentifier: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IOpcRelationship>;
    fn CreateRelationship(&mut self, relationshipidentifier: super::super::super::Foundation::PWSTR, relationshiptype: super::super::super::Foundation::PWSTR, targeturi: ::core::option::Option<super::super::super::System::Com::IUri>, targetmode: OPC_URI_TARGET_MODE) -> ::windows::core::Result<IOpcRelationship>;
    fn DeleteRelationship(&mut self, relationshipidentifier: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RelationshipExists(&mut self, relationshipidentifier: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetEnumerator(&mut self) -> ::windows::core::Result<IOpcRelationshipEnumerator>;
    fn GetEnumeratorForType(&mut self, relationshiptype: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IOpcRelationshipEnumerator>;
    fn GetRelationshipsContentStream(&mut self) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcRelationshipSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcRelationshipSet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcRelationshipSet_Vtbl {
        unsafe extern "system" fn GetRelationship<Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: super::super::super::Foundation::PWSTR, relationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRelationship(::core::mem::transmute_copy(&relationshipidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *relationship = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRelationship<Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: super::super::super::Foundation::PWSTR, relationshiptype: super::super::super::Foundation::PWSTR, targeturi: ::windows::core::RawPtr, targetmode: OPC_URI_TARGET_MODE, relationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRelationship(::core::mem::transmute_copy(&relationshipidentifier), ::core::mem::transmute_copy(&relationshiptype), ::core::mem::transmute(&targeturi), ::core::mem::transmute_copy(&targetmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *relationship = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRelationship<Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteRelationship(::core::mem::transmute_copy(&relationshipidentifier)).into()
        }
        unsafe extern "system" fn RelationshipExists<Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: super::super::super::Foundation::PWSTR, relationshipexists: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelationshipExists(::core::mem::transmute_copy(&relationshipidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipexists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumeratorForType<Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshiptype: super::super::super::Foundation::PWSTR, relationshipenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumeratorForType(::core::mem::transmute_copy(&relationshiptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipsContentStream<Impl: IOpcRelationshipSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRelationshipsContentStream() {
                ::core::result::Result::Ok(ok__) => {
                    *contents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRelationship: GetRelationship::<Impl, IMPL_OFFSET>,
            CreateRelationship: CreateRelationship::<Impl, IMPL_OFFSET>,
            DeleteRelationship: DeleteRelationship::<Impl, IMPL_OFFSET>,
            RelationshipExists: RelationshipExists::<Impl, IMPL_OFFSET>,
            GetEnumerator: GetEnumerator::<Impl, IMPL_OFFSET>,
            GetEnumeratorForType: GetEnumeratorForType::<Impl, IMPL_OFFSET>,
            GetRelationshipsContentStream: GetRelationshipsContentStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcRelationshipSet as ::windows::core::Interface>::IID
    }
}
pub trait IOpcSignatureCustomObject_Impl: Sized {
    fn GetXml(&mut self, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
}
impl IOpcSignatureCustomObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureCustomObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureCustomObject_Vtbl {
        unsafe extern "system" fn GetXml<Impl: IOpcSignatureCustomObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetXml(::core::mem::transmute_copy(&xmlmarkup), ::core::mem::transmute_copy(&count)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetXml: GetXml::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureCustomObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcSignatureCustomObjectEnumerator_Impl: Sized {
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&mut self) -> ::windows::core::Result<IOpcSignatureCustomObject>;
    fn Clone(&mut self) -> ::windows::core::Result<IOpcSignatureCustomObjectEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcSignatureCustomObjectEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureCustomObjectEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureCustomObjectEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    *hasprevious = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *customobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcSignatureCustomObjectEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *copy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
            MovePrevious: MovePrevious::<Impl, IMPL_OFFSET>,
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureCustomObjectEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IOpcSignatureCustomObjectSet_Impl: Sized {
    fn Create(&mut self, xmlmarkup: *const u8, count: u32) -> ::windows::core::Result<IOpcSignatureCustomObject>;
    fn Delete(&mut self, customobject: ::core::option::Option<IOpcSignatureCustomObject>) -> ::windows::core::Result<()>;
    fn GetEnumerator(&mut self) -> ::windows::core::Result<IOpcSignatureCustomObjectEnumerator>;
}
impl IOpcSignatureCustomObjectSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureCustomObjectSet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureCustomObjectSet_Vtbl {
        unsafe extern "system" fn Create<Impl: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlmarkup: *const u8, count: u32, customobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&xmlmarkup), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *customobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute(&customobject)).into()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcSignatureCustomObjectSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *customobjectenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            GetEnumerator: GetEnumerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureCustomObjectSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcSignaturePartReference_Impl: Sized {
    fn GetPartName(&mut self) -> ::windows::core::Result<IOpcPartUri>;
    fn GetContentType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetDigestMethod(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetDigestValue(&mut self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
    fn GetTransformMethod(&mut self) -> ::windows::core::Result<OPC_CANONICALIZATION_METHOD>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSignaturePartReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignaturePartReference_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignaturePartReference_Vtbl {
        unsafe extern "system" fn GetPartName<Impl: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartName() {
                ::core::result::Result::Ok(ok__) => {
                    *partname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentType<Impl: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *contenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestMethod<Impl: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDigestMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *digestmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestValue<Impl: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDigestValue(::core::mem::transmute_copy(&digestvalue), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetTransformMethod<Impl: IOpcSignaturePartReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *transformmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPartName: GetPartName::<Impl, IMPL_OFFSET>,
            GetContentType: GetContentType::<Impl, IMPL_OFFSET>,
            GetDigestMethod: GetDigestMethod::<Impl, IMPL_OFFSET>,
            GetDigestValue: GetDigestValue::<Impl, IMPL_OFFSET>,
            GetTransformMethod: GetTransformMethod::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignaturePartReference as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcSignaturePartReferenceEnumerator_Impl: Sized {
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&mut self) -> ::windows::core::Result<IOpcSignaturePartReference>;
    fn Clone(&mut self) -> ::windows::core::Result<IOpcSignaturePartReferenceEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcSignaturePartReferenceEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignaturePartReferenceEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignaturePartReferenceEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    *hasprevious = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *partreference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcSignaturePartReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *copy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
            MovePrevious: MovePrevious::<Impl, IMPL_OFFSET>,
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignaturePartReferenceEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcSignaturePartReferenceSet_Impl: Sized {
    fn Create(&mut self, parturi: ::core::option::Option<IOpcPartUri>, digestmethod: super::super::super::Foundation::PWSTR, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<IOpcSignaturePartReference>;
    fn Delete(&mut self, partreference: ::core::option::Option<IOpcSignaturePartReference>) -> ::windows::core::Result<()>;
    fn GetEnumerator(&mut self) -> ::windows::core::Result<IOpcSignaturePartReferenceEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSignaturePartReferenceSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignaturePartReferenceSet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignaturePartReferenceSet_Vtbl {
        unsafe extern "system" fn Create<Impl: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, digestmethod: super::super::super::Foundation::PWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, partreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute(&parturi), ::core::mem::transmute_copy(&digestmethod), ::core::mem::transmute_copy(&transformmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *partreference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute(&partreference)).into()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcSignaturePartReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *partreferenceenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            GetEnumerator: GetEnumerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignaturePartReferenceSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcSignatureReference_Impl: Sized {
    fn GetId(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetUri(&mut self) -> ::windows::core::Result<super::super::super::System::Com::IUri>;
    fn GetType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetTransformMethod(&mut self) -> ::windows::core::Result<OPC_CANONICALIZATION_METHOD>;
    fn GetDigestMethod(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetDigestValue(&mut self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSignatureReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureReference_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureReference_Vtbl {
        unsafe extern "system" fn GetId<Impl: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referenceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *referenceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUri<Impl: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referenceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUri() {
                ::core::result::Result::Ok(ok__) => {
                    *referenceuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformMethod<Impl: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *transformmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestMethod<Impl: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDigestMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *digestmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestValue<Impl: IOpcSignatureReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDigestValue(::core::mem::transmute_copy(&digestvalue), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetId: GetId::<Impl, IMPL_OFFSET>,
            GetUri: GetUri::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetTransformMethod: GetTransformMethod::<Impl, IMPL_OFFSET>,
            GetDigestMethod: GetDigestMethod::<Impl, IMPL_OFFSET>,
            GetDigestValue: GetDigestValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureReference as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcSignatureReferenceEnumerator_Impl: Sized {
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&mut self) -> ::windows::core::Result<IOpcSignatureReference>;
    fn Clone(&mut self) -> ::windows::core::Result<IOpcSignatureReferenceEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcSignatureReferenceEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureReferenceEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureReferenceEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    *hasprevious = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *reference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcSignatureReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *copy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
            MovePrevious: MovePrevious::<Impl, IMPL_OFFSET>,
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureReferenceEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcSignatureReferenceSet_Impl: Sized {
    fn Create(&mut self, referenceuri: ::core::option::Option<super::super::super::System::Com::IUri>, referenceid: super::super::super::Foundation::PWSTR, r#type: super::super::super::Foundation::PWSTR, digestmethod: super::super::super::Foundation::PWSTR, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<IOpcSignatureReference>;
    fn Delete(&mut self, reference: ::core::option::Option<IOpcSignatureReference>) -> ::windows::core::Result<()>;
    fn GetEnumerator(&mut self) -> ::windows::core::Result<IOpcSignatureReferenceEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSignatureReferenceSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureReferenceSet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureReferenceSet_Vtbl {
        unsafe extern "system" fn Create<Impl: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referenceuri: ::windows::core::RawPtr, referenceid: super::super::super::Foundation::PWSTR, r#type: super::super::super::Foundation::PWSTR, digestmethod: super::super::super::Foundation::PWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, reference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute(&referenceuri), ::core::mem::transmute_copy(&referenceid), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&digestmethod), ::core::mem::transmute_copy(&transformmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *reference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute(&reference)).into()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcSignatureReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *referenceenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            GetEnumerator: GetEnumerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureReferenceSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcSignatureRelationshipReference_Impl: Sized {
    fn GetSourceUri(&mut self) -> ::windows::core::Result<IOpcUri>;
    fn GetDigestMethod(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetDigestValue(&mut self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
    fn GetTransformMethod(&mut self) -> ::windows::core::Result<OPC_CANONICALIZATION_METHOD>;
    fn GetRelationshipSigningOption(&mut self) -> ::windows::core::Result<OPC_RELATIONSHIPS_SIGNING_OPTION>;
    fn GetRelationshipSelectorEnumerator(&mut self) -> ::windows::core::Result<IOpcRelationshipSelectorEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSignatureRelationshipReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureRelationshipReference_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureRelationshipReference_Vtbl {
        unsafe extern "system" fn GetSourceUri<Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *sourceuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestMethod<Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDigestMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *digestmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestValue<Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDigestValue(::core::mem::transmute_copy(&digestvalue), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetTransformMethod<Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *transformmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipSigningOption<Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipsigningoption: *mut OPC_RELATIONSHIPS_SIGNING_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRelationshipSigningOption() {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipsigningoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipSelectorEnumerator<Impl: IOpcSignatureRelationshipReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectorenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRelationshipSelectorEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *selectorenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSourceUri: GetSourceUri::<Impl, IMPL_OFFSET>,
            GetDigestMethod: GetDigestMethod::<Impl, IMPL_OFFSET>,
            GetDigestValue: GetDigestValue::<Impl, IMPL_OFFSET>,
            GetTransformMethod: GetTransformMethod::<Impl, IMPL_OFFSET>,
            GetRelationshipSigningOption: GetRelationshipSigningOption::<Impl, IMPL_OFFSET>,
            GetRelationshipSelectorEnumerator: GetRelationshipSelectorEnumerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureRelationshipReference as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcSignatureRelationshipReferenceEnumerator_Impl: Sized {
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn MovePrevious(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetCurrent(&mut self) -> ::windows::core::Result<IOpcSignatureRelationshipReference>;
    fn Clone(&mut self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcSignatureRelationshipReferenceEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureRelationshipReferenceEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *hasnext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    *hasprevious = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipreference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcSignatureRelationshipReferenceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *copy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
            MovePrevious: MovePrevious::<Impl, IMPL_OFFSET>,
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureRelationshipReferenceEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcSignatureRelationshipReferenceSet_Impl: Sized {
    fn Create(&mut self, sourceuri: ::core::option::Option<IOpcUri>, digestmethod: super::super::super::Foundation::PWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: ::core::option::Option<IOpcRelationshipSelectorSet>, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<IOpcSignatureRelationshipReference>;
    fn CreateRelationshipSelectorSet(&mut self) -> ::windows::core::Result<IOpcRelationshipSelectorSet>;
    fn Delete(&mut self, relationshipreference: ::core::option::Option<IOpcSignatureRelationshipReference>) -> ::windows::core::Result<()>;
    fn GetEnumerator(&mut self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceEnumerator>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSignatureRelationshipReferenceSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureRelationshipReferenceSet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureRelationshipReferenceSet_Vtbl {
        unsafe extern "system" fn Create<Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceuri: ::windows::core::RawPtr, digestmethod: super::super::super::Foundation::PWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: ::windows::core::RawPtr, transformmethod: OPC_CANONICALIZATION_METHOD, relationshipreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute(&sourceuri), ::core::mem::transmute_copy(&digestmethod), ::core::mem::transmute_copy(&relationshipsigningoption), ::core::mem::transmute(&selectorset), ::core::mem::transmute_copy(&transformmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipreference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRelationshipSelectorSet<Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectorset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRelationshipSelectorSet() {
                ::core::result::Result::Ok(ok__) => {
                    *selectorset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute(&relationshipreference)).into()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcSignatureRelationshipReferenceSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipreferenceenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateRelationshipSelectorSet: CreateRelationshipSelectorSet::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            GetEnumerator: GetEnumerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureRelationshipReferenceSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcSigningOptions_Impl: Sized {
    fn GetSignatureId(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn SetSignatureId(&mut self, signatureid: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSignatureMethod(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn SetSignatureMethod(&mut self, signaturemethod: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetDefaultDigestMethod(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn SetDefaultDigestMethod(&mut self, digestmethod: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetCertificateEmbeddingOption(&mut self) -> ::windows::core::Result<OPC_CERTIFICATE_EMBEDDING_OPTION>;
    fn SetCertificateEmbeddingOption(&mut self, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::Result<()>;
    fn GetTimeFormat(&mut self) -> ::windows::core::Result<OPC_SIGNATURE_TIME_FORMAT>;
    fn SetTimeFormat(&mut self, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::Result<()>;
    fn GetSignaturePartReferenceSet(&mut self) -> ::windows::core::Result<IOpcSignaturePartReferenceSet>;
    fn GetSignatureRelationshipReferenceSet(&mut self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceSet>;
    fn GetCustomObjectSet(&mut self) -> ::windows::core::Result<IOpcSignatureCustomObjectSet>;
    fn GetCustomReferenceSet(&mut self) -> ::windows::core::Result<IOpcSignatureReferenceSet>;
    fn GetCertificateSet(&mut self) -> ::windows::core::Result<IOpcCertificateSet>;
    fn GetSignaturePartName(&mut self) -> ::windows::core::Result<IOpcPartUri>;
    fn SetSignaturePartName(&mut self, signaturepartname: ::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSigningOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSigningOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSigningOptions_Vtbl {
        unsafe extern "system" fn GetSignatureId<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureId() {
                ::core::result::Result::Ok(ok__) => {
                    *signatureid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureId<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignatureId(::core::mem::transmute_copy(&signatureid)).into()
        }
        unsafe extern "system" fn GetSignatureMethod<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *signaturemethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureMethod<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignatureMethod(::core::mem::transmute_copy(&signaturemethod)).into()
        }
        unsafe extern "system" fn GetDefaultDigestMethod<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultDigestMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *digestmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultDigestMethod<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultDigestMethod(::core::mem::transmute_copy(&digestmethod)).into()
        }
        unsafe extern "system" fn GetCertificateEmbeddingOption<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, embeddingoption: *mut OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateEmbeddingOption() {
                ::core::result::Result::Ok(ok__) => {
                    *embeddingoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateEmbeddingOption<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCertificateEmbeddingOption(::core::mem::transmute_copy(&embeddingoption)).into()
        }
        unsafe extern "system" fn GetTimeFormat<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimeFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *timeformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeFormat<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimeFormat(::core::mem::transmute_copy(&timeformat)).into()
        }
        unsafe extern "system" fn GetSignaturePartReferenceSet<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartReferenceSet() {
                ::core::result::Result::Ok(ok__) => {
                    *partreferenceset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceSet<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureRelationshipReferenceSet() {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipreferenceset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectSet<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomObjectSet() {
                ::core::result::Result::Ok(ok__) => {
                    *customobjectset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceSet<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomReferenceSet() {
                ::core::result::Result::Ok(ok__) => {
                    *customreferenceset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateSet<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateSet() {
                ::core::result::Result::Ok(ok__) => {
                    *certificateset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartName() {
                ::core::result::Result::Ok(ok__) => {
                    *signaturepartname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignaturePartName<Impl: IOpcSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignaturePartName(::core::mem::transmute(&signaturepartname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSignatureId: GetSignatureId::<Impl, IMPL_OFFSET>,
            SetSignatureId: SetSignatureId::<Impl, IMPL_OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Impl, IMPL_OFFSET>,
            SetSignatureMethod: SetSignatureMethod::<Impl, IMPL_OFFSET>,
            GetDefaultDigestMethod: GetDefaultDigestMethod::<Impl, IMPL_OFFSET>,
            SetDefaultDigestMethod: SetDefaultDigestMethod::<Impl, IMPL_OFFSET>,
            GetCertificateEmbeddingOption: GetCertificateEmbeddingOption::<Impl, IMPL_OFFSET>,
            SetCertificateEmbeddingOption: SetCertificateEmbeddingOption::<Impl, IMPL_OFFSET>,
            GetTimeFormat: GetTimeFormat::<Impl, IMPL_OFFSET>,
            SetTimeFormat: SetTimeFormat::<Impl, IMPL_OFFSET>,
            GetSignaturePartReferenceSet: GetSignaturePartReferenceSet::<Impl, IMPL_OFFSET>,
            GetSignatureRelationshipReferenceSet: GetSignatureRelationshipReferenceSet::<Impl, IMPL_OFFSET>,
            GetCustomObjectSet: GetCustomObjectSet::<Impl, IMPL_OFFSET>,
            GetCustomReferenceSet: GetCustomReferenceSet::<Impl, IMPL_OFFSET>,
            GetCertificateSet: GetCertificateSet::<Impl, IMPL_OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Impl, IMPL_OFFSET>,
            SetSignaturePartName: SetSignaturePartName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSigningOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpcUri_Impl: Sized + super::super::super::System::Com::IUri_Impl {
    fn GetRelationshipsPartUri(&mut self) -> ::windows::core::Result<IOpcPartUri>;
    fn GetRelativeUri(&mut self, targetparturi: ::core::option::Option<IOpcPartUri>) -> ::windows::core::Result<super::super::super::System::Com::IUri>;
    fn CombinePartUri(&mut self, relativeuri: ::core::option::Option<super::super::super::System::Com::IUri>) -> ::windows::core::Result<IOpcPartUri>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcUri_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcUri_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcUri_Vtbl {
        unsafe extern "system" fn GetRelationshipsPartUri<Impl: IOpcUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipparturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRelationshipsPartUri() {
                ::core::result::Result::Ok(ok__) => {
                    *relationshipparturi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeUri<Impl: IOpcUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetparturi: ::windows::core::RawPtr, relativeuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRelativeUri(::core::mem::transmute(&targetparturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *relativeuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CombinePartUri<Impl: IOpcUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeuri: ::windows::core::RawPtr, combineduri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CombinePartUri(::core::mem::transmute(&relativeuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *combineduri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IUri_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetRelationshipsPartUri: GetRelationshipsPartUri::<Impl, IMPL_OFFSET>,
            GetRelativeUri: GetRelativeUri::<Impl, IMPL_OFFSET>,
            CombinePartUri: CombinePartUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcUri as ::windows::core::Interface>::IID
    }
}
