#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub trait IOpcCertificateEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl IOpcCertificateEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcCertificateEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcCertificateEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcCertificateEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcCertificateEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcCertificateEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: *const *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IOpcCertificateEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcCertificateSetImpl: Sized {
    fn Add();
    fn Remove();
    fn GetEnumerator();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl IOpcCertificateSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcCertificateSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcCertificateSetVtbl {
        unsafe extern "system" fn Add<Impl: IOpcCertificateSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IOpcCertificateSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcCertificateSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcDigitalSignatureImpl: Sized {
    fn GetNamespaces();
    fn GetSignatureId();
    fn GetSignaturePartName();
    fn GetSignatureMethod();
    fn GetCanonicalizationMethod();
    fn GetSignatureValue();
    fn GetSignaturePartReferenceEnumerator();
    fn GetSignatureRelationshipReferenceEnumerator();
    fn GetSigningTime();
    fn GetTimeFormat();
    fn GetPackageObjectReference();
    fn GetCertificateEnumerator();
    fn GetCustomReferenceEnumerator();
    fn GetCustomObjectEnumerator();
    fn GetSignatureXml();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcDigitalSignatureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcDigitalSignatureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcDigitalSignatureVtbl {
        unsafe extern "system" fn GetNamespaces<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefixes: *mut *mut super::super::super::Foundation::PWSTR, namespaces: *mut *mut super::super::super::Foundation::PWSTR, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureId<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureMethod<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCanonicalizationMethod<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureValue<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignaturePartReferenceEnumerator<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceEnumerator<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSigningTime<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signingtime: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimeFormat<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPackageObjectReference<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageobjectreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateEnumerator<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureXml<Impl: IOpcDigitalSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcDigitalSignatureEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcDigitalSignatureEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcDigitalSignatureEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcDigitalSignatureEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcDigitalSignatureEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcDigitalSignatureEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcDigitalSignatureEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digitalsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IOpcDigitalSignatureEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcDigitalSignatureManagerImpl: Sized {
    fn GetSignatureOriginPartName();
    fn SetSignatureOriginPartName();
    fn GetSignatureEnumerator();
    fn RemoveSignature();
    fn CreateSigningOptions();
    fn Validate();
    fn Sign();
    fn ReplaceSignatureXml();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_System_Com"))]
impl IOpcDigitalSignatureManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcDigitalSignatureManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcDigitalSignatureManagerVtbl {
        unsafe extern "system" fn GetSignatureOriginPartName<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureEnumerator<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveSignature<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSigningOptions<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signingoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Validate<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: ::windows::core::RawPtr, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Sign<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: ::windows::core::RawPtr, digitalsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReplaceSignatureXml<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr, newsignaturexml: *const u8, count: u32, digitalsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcFactoryImpl: Sized {
    fn CreatePackageRootUri();
    fn CreatePartUri();
    fn CreateStreamOnFile();
    fn CreatePackage();
    fn ReadPackageFromStream();
    fn WritePackageToStream();
    fn CreateDigitalSignatureManager();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
impl IOpcFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcFactoryVtbl {
        unsafe extern "system" fn CreatePackageRootUri<Impl: IOpcFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rooturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePartUri<Impl: IOpcFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzuri: super::super::super::Foundation::PWSTR, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStreamOnFile<Impl: IOpcFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePackage<Impl: IOpcFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadPackageFromStream<Impl: IOpcFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, flags: OPC_READ_FLAGS, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WritePackageToStream<Impl: IOpcFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, flags: OPC_WRITE_FLAGS, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDigitalSignatureManager<Impl: IOpcFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, signaturemanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcPackageImpl: Sized {
    fn GetPartSet();
    fn GetRelationshipSet();
}
impl IOpcPackageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcPackageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcPackageVtbl {
        unsafe extern "system" fn GetPartSet<Impl: IOpcPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRelationshipSet<Impl: IOpcPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcPartImpl: Sized {
    fn GetRelationshipSet();
    fn GetContentStream();
    fn GetName();
    fn GetContentType();
    fn GetCompressionOptions();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcPartVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcPartImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcPartVtbl {
        unsafe extern "system" fn GetRelationshipSet<Impl: IOpcPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentStream<Impl: IOpcPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: IOpcPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentType<Impl: IOpcPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompressionOptions<Impl: IOpcPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compressionoptions: *mut OPC_COMPRESSION_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcPartEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcPartEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcPartEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcPartEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcPartEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcPartEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcPartEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IOpcPartEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcPartSetImpl: Sized {
    fn GetPart();
    fn CreatePart();
    fn DeletePart();
    fn PartExists();
    fn GetEnumerator();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcPartSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcPartSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcPartSetVtbl {
        unsafe extern "system" fn GetPart<Impl: IOpcPartSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePart<Impl: IOpcPartSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::RawPtr, contenttype: super::super::super::Foundation::PWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeletePart<Impl: IOpcPartSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PartExists<Impl: IOpcPartSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::RawPtr, partexists: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcPartSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcPartUriImpl: Sized + IUriImpl + IOpcUriImpl {
    fn ComparePartUri();
    fn GetSourceUri();
    fn IsRelationshipsPartUri();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcPartUriVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcPartUriImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcPartUriVtbl {
        unsafe extern "system" fn ComparePartUri<Impl: IOpcPartUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, comparisonresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceUri<Impl: IOpcPartUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRelationshipsPartUri<Impl: IOpcPartUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isrelationshipuri: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IOpcUriVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IOpcRelationshipImpl: Sized {
    fn GetId();
    fn GetRelationshipType();
    fn GetSourceUri();
    fn GetTargetUri();
    fn GetTargetMode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcRelationshipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcRelationshipImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcRelationshipVtbl {
        unsafe extern "system" fn GetId<Impl: IOpcRelationshipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRelationshipType<Impl: IOpcRelationshipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshiptype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceUri<Impl: IOpcRelationshipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTargetUri<Impl: IOpcRelationshipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTargetMode<Impl: IOpcRelationshipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetmode: *mut OPC_URI_TARGET_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcRelationshipEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcRelationshipEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcRelationshipEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcRelationshipEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcRelationshipEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcRelationshipEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcRelationshipEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IOpcRelationshipEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcRelationshipSelectorImpl: Sized {
    fn GetSelectorType();
    fn GetSelectionCriterion();
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcRelationshipSelectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcRelationshipSelectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcRelationshipSelectorVtbl {
        unsafe extern "system" fn GetSelectorType<Impl: IOpcRelationshipSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selector: *mut OPC_RELATIONSHIP_SELECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSelectionCriterion<Impl: IOpcRelationshipSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectioncriterion: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcRelationshipSelectorEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcRelationshipSelectorEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcRelationshipSelectorEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcRelationshipSelectorEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcRelationshipSelectorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcRelationshipSelectorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcRelationshipSelectorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipselector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IOpcRelationshipSelectorEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcRelationshipSelectorSetImpl: Sized {
    fn Create();
    fn Delete();
    fn GetEnumerator();
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcRelationshipSelectorSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcRelationshipSelectorSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcRelationshipSelectorSetVtbl {
        unsafe extern "system" fn Create<Impl: IOpcRelationshipSelectorSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: super::super::super::Foundation::PWSTR, relationshipselector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IOpcRelationshipSelectorSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcRelationshipSelectorSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipselectorenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcRelationshipSetImpl: Sized {
    fn GetRelationship();
    fn CreateRelationship();
    fn DeleteRelationship();
    fn RelationshipExists();
    fn GetEnumerator();
    fn GetEnumeratorForType();
    fn GetRelationshipsContentStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcRelationshipSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcRelationshipSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcRelationshipSetVtbl {
        unsafe extern "system" fn GetRelationship<Impl: IOpcRelationshipSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: super::super::super::Foundation::PWSTR, relationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRelationship<Impl: IOpcRelationshipSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: super::super::super::Foundation::PWSTR, relationshiptype: super::super::super::Foundation::PWSTR, targeturi: ::windows::core::RawPtr, targetmode: OPC_URI_TARGET_MODE, relationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteRelationship<Impl: IOpcRelationshipSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RelationshipExists<Impl: IOpcRelationshipSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipidentifier: super::super::super::Foundation::PWSTR, relationshipexists: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcRelationshipSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumeratorForType<Impl: IOpcRelationshipSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshiptype: super::super::super::Foundation::PWSTR, relationshipenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRelationshipsContentStream<Impl: IOpcRelationshipSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcSignatureCustomObjectImpl: Sized {
    fn GetXml();
}
impl IOpcSignatureCustomObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureCustomObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureCustomObjectVtbl {
        unsafe extern "system" fn GetXml<Impl: IOpcSignatureCustomObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetXml: GetXml::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcSignatureCustomObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOpcSignatureCustomObjectEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcSignatureCustomObjectEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureCustomObjectEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureCustomObjectEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcSignatureCustomObjectEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcSignatureCustomObjectEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcSignatureCustomObjectEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IOpcSignatureCustomObjectEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcSignatureCustomObjectSetImpl: Sized {
    fn Create();
    fn Delete();
    fn GetEnumerator();
}
impl IOpcSignatureCustomObjectSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureCustomObjectSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureCustomObjectSetVtbl {
        unsafe extern "system" fn Create<Impl: IOpcSignatureCustomObjectSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlmarkup: *const u8, count: u32, customobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IOpcSignatureCustomObjectSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcSignatureCustomObjectSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcSignaturePartReferenceImpl: Sized {
    fn GetPartName();
    fn GetContentType();
    fn GetDigestMethod();
    fn GetDigestValue();
    fn GetTransformMethod();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSignaturePartReferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignaturePartReferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignaturePartReferenceVtbl {
        unsafe extern "system" fn GetPartName<Impl: IOpcSignaturePartReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentType<Impl: IOpcSignaturePartReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDigestMethod<Impl: IOpcSignaturePartReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDigestValue<Impl: IOpcSignaturePartReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformMethod<Impl: IOpcSignaturePartReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcSignaturePartReferenceEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcSignaturePartReferenceEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignaturePartReferenceEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignaturePartReferenceEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcSignaturePartReferenceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcSignaturePartReferenceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcSignaturePartReferenceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IOpcSignaturePartReferenceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcSignaturePartReferenceSetImpl: Sized {
    fn Create();
    fn Delete();
    fn GetEnumerator();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSignaturePartReferenceSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignaturePartReferenceSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignaturePartReferenceSetVtbl {
        unsafe extern "system" fn Create<Impl: IOpcSignaturePartReferenceSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, digestmethod: super::super::super::Foundation::PWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, partreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IOpcSignaturePartReferenceSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcSignaturePartReferenceSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcSignatureReferenceImpl: Sized {
    fn GetId();
    fn GetUri();
    fn GetType();
    fn GetTransformMethod();
    fn GetDigestMethod();
    fn GetDigestValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSignatureReferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureReferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureReferenceVtbl {
        unsafe extern "system" fn GetId<Impl: IOpcSignatureReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referenceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUri<Impl: IOpcSignatureReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referenceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IOpcSignatureReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformMethod<Impl: IOpcSignatureReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDigestMethod<Impl: IOpcSignatureReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDigestValue<Impl: IOpcSignatureReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcSignatureReferenceEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcSignatureReferenceEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureReferenceEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureReferenceEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcSignatureReferenceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcSignatureReferenceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcSignatureReferenceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IOpcSignatureReferenceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcSignatureReferenceSetImpl: Sized {
    fn Create();
    fn Delete();
    fn GetEnumerator();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSignatureReferenceSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureReferenceSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureReferenceSetVtbl {
        unsafe extern "system" fn Create<Impl: IOpcSignatureReferenceSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referenceuri: ::windows::core::RawPtr, referenceid: super::super::super::Foundation::PWSTR, r#type: super::super::super::Foundation::PWSTR, digestmethod: super::super::super::Foundation::PWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, reference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IOpcSignatureReferenceSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcSignatureReferenceSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcSignatureRelationshipReferenceImpl: Sized {
    fn GetSourceUri();
    fn GetDigestMethod();
    fn GetDigestValue();
    fn GetTransformMethod();
    fn GetRelationshipSigningOption();
    fn GetRelationshipSelectorEnumerator();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSignatureRelationshipReferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureRelationshipReferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureRelationshipReferenceVtbl {
        unsafe extern "system" fn GetSourceUri<Impl: IOpcSignatureRelationshipReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDigestMethod<Impl: IOpcSignatureRelationshipReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDigestValue<Impl: IOpcSignatureRelationshipReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformMethod<Impl: IOpcSignatureRelationshipReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRelationshipSigningOption<Impl: IOpcSignatureRelationshipReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipsigningoption: *mut OPC_RELATIONSHIPS_SIGNING_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRelationshipSelectorEnumerator<Impl: IOpcSignatureRelationshipReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectorenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcSignatureRelationshipReferenceEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IOpcSignatureRelationshipReferenceEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureRelationshipReferenceEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureRelationshipReferenceEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcSignatureRelationshipReferenceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcSignatureRelationshipReferenceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcSignatureRelationshipReferenceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IOpcSignatureRelationshipReferenceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcSignatureRelationshipReferenceSetImpl: Sized {
    fn Create();
    fn CreateRelationshipSelectorSet();
    fn Delete();
    fn GetEnumerator();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSignatureRelationshipReferenceSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSignatureRelationshipReferenceSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSignatureRelationshipReferenceSetVtbl {
        unsafe extern "system" fn Create<Impl: IOpcSignatureRelationshipReferenceSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceuri: ::windows::core::RawPtr, digestmethod: super::super::super::Foundation::PWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: ::windows::core::RawPtr, transformmethod: OPC_CANONICALIZATION_METHOD, relationshipreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRelationshipSelectorSet<Impl: IOpcSignatureRelationshipReferenceSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectorset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IOpcSignatureRelationshipReferenceSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcSignatureRelationshipReferenceSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcSigningOptionsImpl: Sized {
    fn GetSignatureId();
    fn SetSignatureId();
    fn GetSignatureMethod();
    fn SetSignatureMethod();
    fn GetDefaultDigestMethod();
    fn SetDefaultDigestMethod();
    fn GetCertificateEmbeddingOption();
    fn SetCertificateEmbeddingOption();
    fn GetTimeFormat();
    fn SetTimeFormat();
    fn GetSignaturePartReferenceSet();
    fn GetSignatureRelationshipReferenceSet();
    fn GetCustomObjectSet();
    fn GetCustomReferenceSet();
    fn GetCertificateSet();
    fn GetSignaturePartName();
    fn SetSignaturePartName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcSigningOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcSigningOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcSigningOptionsVtbl {
        unsafe extern "system" fn GetSignatureId<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignatureId<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureMethod<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignatureMethod<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultDigestMethod<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultDigestMethod<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateEmbeddingOption<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, embeddingoption: *mut OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCertificateEmbeddingOption<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimeFormat<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTimeFormat<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignaturePartReferenceSet<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceSet<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCustomObjectSet<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCustomReferenceSet<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateSet<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignaturePartName<Impl: IOpcSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IOpcUriImpl: Sized + IUriImpl {
    fn GetRelationshipsPartUri();
    fn GetRelativeUri();
    fn CombinePartUri();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOpcUriVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOpcUriImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOpcUriVtbl {
        unsafe extern "system" fn GetRelationshipsPartUri<Impl: IOpcUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relationshipparturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRelativeUri<Impl: IOpcUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetparturi: ::windows::core::RawPtr, relativeuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CombinePartUri<Impl: IOpcUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeuri: ::windows::core::RawPtr, combineduri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUriVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetRelationshipsPartUri: GetRelationshipsPartUri::<Impl, IMPL_OFFSET>,
            GetRelativeUri: GetRelativeUri::<Impl, IMPL_OFFSET>,
            CombinePartUri: CombinePartUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOpcUri as ::windows::core::Interface>::IID
    }
}
