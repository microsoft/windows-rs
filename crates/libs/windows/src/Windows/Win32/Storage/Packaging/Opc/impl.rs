pub trait IOpcCertificateEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
impl ::windows::core::RuntimeName for IOpcCertificateEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcCertificateEnumerator";
}
impl IOpcCertificateEnumeratorVtbl {
    pub const fn new<Impl: IOpcCertificateEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcCertificateEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcCertificateEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveNext(&*(&hasnext as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcCertificateEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MovePrevious(&*(&hasprevious as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcCertificateEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: *const *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrent(&*(&certificate as *const <super::super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcCertificateEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcCertificateEnumerator>, base.5, MoveNext::<Impl, OFFSET>, MovePrevious::<Impl, OFFSET>, GetCurrent::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IOpcCertificateSetImpl: Sized {
    fn Add();
    fn Remove();
    fn GetEnumerator();
}
impl ::windows::core::RuntimeName for IOpcCertificateSet {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcCertificateSet";
}
impl IOpcCertificateSetVtbl {
    pub const fn new<Impl: IOpcCertificateSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcCertificateSetVtbl {
        unsafe extern "system" fn Add<Impl: IOpcCertificateSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(&*(&certificate as *const <super::super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IOpcCertificateSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&certificate as *const <super::super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcCertificateSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnumerator(::core::mem::transmute_copy(&certificateenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcCertificateSet>, base.5, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, GetEnumerator::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IOpcDigitalSignature {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcDigitalSignature";
}
impl IOpcDigitalSignatureVtbl {
    pub const fn new<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcDigitalSignatureVtbl {
        unsafe extern "system" fn GetNamespaces<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefixes: *mut *mut super::super::super::Foundation::PWSTR, namespaces: *mut *mut super::super::super::Foundation::PWSTR, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNamespaces(::core::mem::transmute_copy(&prefixes), ::core::mem::transmute_copy(&namespaces), count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureId<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureId(::core::mem::transmute_copy(&signatureid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartName(::core::mem::transmute_copy(&signaturepartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureMethod<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureMethod(::core::mem::transmute_copy(&signaturemethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCanonicalizationMethod<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCanonicalizationMethod(canonicalizationmethod) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureValue<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureValue(::core::mem::transmute_copy(&signaturevalue), count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartReferenceEnumerator<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartReferenceEnumerator(::core::mem::transmute_copy(&partreferenceenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceEnumerator<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureRelationshipReferenceEnumerator(::core::mem::transmute_copy(&relationshipreferenceenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTime<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signingtime: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSigningTime(::core::mem::transmute_copy(&signingtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeFormat<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTimeFormat(timeformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageObjectReference<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packageobjectreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPackageObjectReference(::core::mem::transmute_copy(&packageobjectreference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateEnumerator<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCertificateEnumerator(::core::mem::transmute_copy(&certificateenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCustomReferenceEnumerator(::core::mem::transmute_copy(&customreferenceenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCustomObjectEnumerator(::core::mem::transmute_copy(&customobjectenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureXml<Impl: IOpcDigitalSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureXml(signaturexml, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IOpcDigitalSignature>,
            base.5,
            GetNamespaces::<Impl, OFFSET>,
            GetSignatureId::<Impl, OFFSET>,
            GetSignaturePartName::<Impl, OFFSET>,
            GetSignatureMethod::<Impl, OFFSET>,
            GetCanonicalizationMethod::<Impl, OFFSET>,
            GetSignatureValue::<Impl, OFFSET>,
            GetSignaturePartReferenceEnumerator::<Impl, OFFSET>,
            GetSignatureRelationshipReferenceEnumerator::<Impl, OFFSET>,
            GetSigningTime::<Impl, OFFSET>,
            GetTimeFormat::<Impl, OFFSET>,
            GetPackageObjectReference::<Impl, OFFSET>,
            GetCertificateEnumerator::<Impl, OFFSET>,
            GetCustomReferenceEnumerator::<Impl, OFFSET>,
            GetCustomObjectEnumerator::<Impl, OFFSET>,
            GetSignatureXml::<Impl, OFFSET>,
        )
    }
}
pub trait IOpcDigitalSignatureEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
impl ::windows::core::RuntimeName for IOpcDigitalSignatureEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcDigitalSignatureEnumerator";
}
impl IOpcDigitalSignatureEnumeratorVtbl {
    pub const fn new<Impl: IOpcDigitalSignatureEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcDigitalSignatureEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcDigitalSignatureEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcDigitalSignatureEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MovePrevious(::core::mem::transmute_copy(&hasprevious)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcDigitalSignatureEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digitalsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&digitalsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcDigitalSignatureEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcDigitalSignatureEnumerator>, base.5, MoveNext::<Impl, OFFSET>, MovePrevious::<Impl, OFFSET>, GetCurrent::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IOpcDigitalSignatureManager {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcDigitalSignatureManager";
}
impl IOpcDigitalSignatureManagerVtbl {
    pub const fn new<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcDigitalSignatureManagerVtbl {
        unsafe extern "system" fn GetSignatureOriginPartName<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureOriginPartName(::core::mem::transmute_copy(&signatureoriginpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSignatureOriginPartName(&*(&signatureoriginpartname as *const <IOpcPartUri as ::windows::core::Abi>::Abi as *const <IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureEnumerator<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureEnumerator(::core::mem::transmute_copy(&signatureenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSignature<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveSignature(&*(&signaturepartname as *const <IOpcPartUri as ::windows::core::Abi>::Abi as *const <IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSigningOptions<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signingoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSigningOptions(::core::mem::transmute_copy(&signingoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signature: ::windows::core::RawPtr, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Validate(&*(&signature as *const <IOpcDigitalSignature as ::windows::core::Abi>::Abi as *const <IOpcDigitalSignature as ::windows::core::DefaultType>::DefaultType), &*(&certificate as *const <super::super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::DefaultType>::DefaultType), validationresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sign<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: ::windows::core::RawPtr, digitalsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Sign(&*(&certificate as *const <super::super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::DefaultType>::DefaultType), &*(&signingoptions as *const <IOpcSigningOptions as ::windows::core::Abi>::Abi as *const <IOpcSigningOptions as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&digitalsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceSignatureXml<Impl: IOpcDigitalSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr, newsignaturexml: *const u8, count: u32, digitalsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReplaceSignatureXml(&*(&signaturepartname as *const <IOpcPartUri as ::windows::core::Abi>::Abi as *const <IOpcPartUri as ::windows::core::DefaultType>::DefaultType), newsignaturexml, count, ::core::mem::transmute_copy(&digitalsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcDigitalSignatureManager>, base.5, GetSignatureOriginPartName::<Impl, OFFSET>, SetSignatureOriginPartName::<Impl, OFFSET>, GetSignatureEnumerator::<Impl, OFFSET>, RemoveSignature::<Impl, OFFSET>, CreateSigningOptions::<Impl, OFFSET>, Validate::<Impl, OFFSET>, Sign::<Impl, OFFSET>, ReplaceSignatureXml::<Impl, OFFSET>)
    }
}
pub trait IOpcFactoryImpl: Sized {
    fn CreatePackageRootUri();
    fn CreatePartUri();
    fn CreateStreamOnFile();
    fn CreatePackage();
    fn ReadPackageFromStream();
    fn WritePackageToStream();
    fn CreateDigitalSignatureManager();
}
impl ::windows::core::RuntimeName for IOpcFactory {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcFactory";
}
impl IOpcFactoryVtbl {
    pub const fn new<Impl: IOpcFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcFactoryVtbl {
        unsafe extern "system" fn CreatePackageRootUri<Impl: IOpcFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rooturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePackageRootUri(::core::mem::transmute_copy(&rooturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUri<Impl: IOpcFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzuri: super::super::super::Foundation::PWSTR, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePartUri(&*(&pwzuri as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStreamOnFile<Impl: IOpcFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStreamOnFile(
                &*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                iomode,
                &*(&securityattributes as *const <super::super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::Abi>::Abi as *const <super::super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::DefaultType>::DefaultType),
                dwflagsandattributes,
                ::core::mem::transmute_copy(&stream),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackage<Impl: IOpcFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePackage(::core::mem::transmute_copy(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadPackageFromStream<Impl: IOpcFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, flags: OPC_READ_FLAGS, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadPackageFromStream(&*(&stream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WritePackageToStream<Impl: IOpcFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, flags: OPC_WRITE_FLAGS, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WritePackageToStream(&*(&package as *const <IOpcPackage as ::windows::core::Abi>::Abi as *const <IOpcPackage as ::windows::core::DefaultType>::DefaultType), flags, &*(&stream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDigitalSignatureManager<Impl: IOpcFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, signaturemanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDigitalSignatureManager(&*(&package as *const <IOpcPackage as ::windows::core::Abi>::Abi as *const <IOpcPackage as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&signaturemanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcFactory>, base.5, CreatePackageRootUri::<Impl, OFFSET>, CreatePartUri::<Impl, OFFSET>, CreateStreamOnFile::<Impl, OFFSET>, CreatePackage::<Impl, OFFSET>, ReadPackageFromStream::<Impl, OFFSET>, WritePackageToStream::<Impl, OFFSET>, CreateDigitalSignatureManager::<Impl, OFFSET>)
    }
}
pub trait IOpcPackageImpl: Sized {
    fn GetPartSet();
    fn GetRelationshipSet();
}
impl ::windows::core::RuntimeName for IOpcPackage {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcPackage";
}
impl IOpcPackageVtbl {
    pub const fn new<Impl: IOpcPackageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcPackageVtbl {
        unsafe extern "system" fn GetPartSet<Impl: IOpcPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPartSet(::core::mem::transmute_copy(&partset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipSet<Impl: IOpcPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelationshipSet(::core::mem::transmute_copy(&relationshipset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcPackage>, base.5, GetPartSet::<Impl, OFFSET>, GetRelationshipSet::<Impl, OFFSET>)
    }
}
pub trait IOpcPartImpl: Sized {
    fn GetRelationshipSet();
    fn GetContentStream();
    fn GetName();
    fn GetContentType();
    fn GetCompressionOptions();
}
impl ::windows::core::RuntimeName for IOpcPart {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcPart";
}
impl IOpcPartVtbl {
    pub const fn new<Impl: IOpcPartImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcPartVtbl {
        unsafe extern "system" fn GetRelationshipSet<Impl: IOpcPartImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelationshipSet(::core::mem::transmute_copy(&relationshipset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentStream<Impl: IOpcPartImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetContentStream(::core::mem::transmute_copy(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IOpcPartImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentType<Impl: IOpcPartImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetContentType(::core::mem::transmute_copy(&contenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompressionOptions<Impl: IOpcPartImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compressionoptions: *mut OPC_COMPRESSION_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCompressionOptions(::core::mem::transmute_copy(&compressionoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcPart>, base.5, GetRelationshipSet::<Impl, OFFSET>, GetContentStream::<Impl, OFFSET>, GetName::<Impl, OFFSET>, GetContentType::<Impl, OFFSET>, GetCompressionOptions::<Impl, OFFSET>)
    }
}
pub trait IOpcPartEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
impl ::windows::core::RuntimeName for IOpcPartEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcPartEnumerator";
}
impl IOpcPartEnumeratorVtbl {
    pub const fn new<Impl: IOpcPartEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcPartEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcPartEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcPartEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MovePrevious(::core::mem::transmute_copy(&hasprevious)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcPartEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&part)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcPartEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcPartEnumerator>, base.5, MoveNext::<Impl, OFFSET>, MovePrevious::<Impl, OFFSET>, GetCurrent::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IOpcPartSetImpl: Sized {
    fn GetPart();
    fn CreatePart();
    fn DeletePart();
    fn PartExists();
    fn GetEnumerator();
}
impl ::windows::core::RuntimeName for IOpcPartSet {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcPartSet";
}
impl IOpcPartSetVtbl {
    pub const fn new<Impl: IOpcPartSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcPartSetVtbl {
        unsafe extern "system" fn GetPart<Impl: IOpcPartSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPart(&*(&name as *const <IOpcPartUri as ::windows::core::Abi>::Abi as *const <IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&part)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePart<Impl: IOpcPartSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows::core::RawPtr, contenttype: super::super::super::Foundation::PWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePart(&*(&name as *const <IOpcPartUri as ::windows::core::Abi>::Abi as *const <IOpcPartUri as ::windows::core::DefaultType>::DefaultType), &*(&contenttype as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), compressionoptions, ::core::mem::transmute_copy(&part)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePart<Impl: IOpcPartSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeletePart(&*(&name as *const <IOpcPartUri as ::windows::core::Abi>::Abi as *const <IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartExists<Impl: IOpcPartSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows::core::RawPtr, partexists: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PartExists(&*(&name as *const <IOpcPartUri as ::windows::core::Abi>::Abi as *const <IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&partexists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcPartSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnumerator(::core::mem::transmute_copy(&partenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcPartSet>, base.5, GetPart::<Impl, OFFSET>, CreatePart::<Impl, OFFSET>, DeletePart::<Impl, OFFSET>, PartExists::<Impl, OFFSET>, GetEnumerator::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcPartUriImpl: Sized + IOpcUriImpl + IUriImpl {
    fn ComparePartUri();
    fn GetSourceUri();
    fn IsRelationshipsPartUri();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IOpcPartUri {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcPartUri";
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcPartUriVtbl {
    pub const fn new<Impl: IOpcPartUriImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcPartUriVtbl {
        unsafe extern "system" fn ComparePartUri<Impl: IOpcPartUriImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, comparisonresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ComparePartUri(&*(&parturi as *const <IOpcPartUri as ::windows::core::Abi>::Abi as *const <IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&comparisonresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceUri<Impl: IOpcPartUriImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceUri(::core::mem::transmute_copy(&sourceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRelationshipsPartUri<Impl: IOpcPartUriImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isrelationshipuri: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRelationshipsPartUri(::core::mem::transmute_copy(&isrelationshipuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcPartUri>, base.5, ComparePartUri::<Impl, OFFSET>, GetSourceUri::<Impl, OFFSET>, IsRelationshipsPartUri::<Impl, OFFSET>)
    }
}
pub trait IOpcRelationshipImpl: Sized {
    fn GetId();
    fn GetRelationshipType();
    fn GetSourceUri();
    fn GetTargetUri();
    fn GetTargetMode();
}
impl ::windows::core::RuntimeName for IOpcRelationship {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcRelationship";
}
impl IOpcRelationshipVtbl {
    pub const fn new<Impl: IOpcRelationshipImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcRelationshipVtbl {
        unsafe extern "system" fn GetId<Impl: IOpcRelationshipImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipidentifier: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetId(::core::mem::transmute_copy(&relationshipidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipType<Impl: IOpcRelationshipImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshiptype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelationshipType(::core::mem::transmute_copy(&relationshiptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceUri<Impl: IOpcRelationshipImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceUri(::core::mem::transmute_copy(&sourceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetUri<Impl: IOpcRelationshipImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targeturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTargetUri(::core::mem::transmute_copy(&targeturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetMode<Impl: IOpcRelationshipImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetmode: *mut OPC_URI_TARGET_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTargetMode(::core::mem::transmute_copy(&targetmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcRelationship>, base.5, GetId::<Impl, OFFSET>, GetRelationshipType::<Impl, OFFSET>, GetSourceUri::<Impl, OFFSET>, GetTargetUri::<Impl, OFFSET>, GetTargetMode::<Impl, OFFSET>)
    }
}
pub trait IOpcRelationshipEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
impl ::windows::core::RuntimeName for IOpcRelationshipEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcRelationshipEnumerator";
}
impl IOpcRelationshipEnumeratorVtbl {
    pub const fn new<Impl: IOpcRelationshipEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcRelationshipEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcRelationshipEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcRelationshipEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MovePrevious(::core::mem::transmute_copy(&hasprevious)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcRelationshipEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&relationship)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcRelationshipEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcRelationshipEnumerator>, base.5, MoveNext::<Impl, OFFSET>, MovePrevious::<Impl, OFFSET>, GetCurrent::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IOpcRelationshipSelectorImpl: Sized {
    fn GetSelectorType();
    fn GetSelectionCriterion();
}
impl ::windows::core::RuntimeName for IOpcRelationshipSelector {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcRelationshipSelector";
}
impl IOpcRelationshipSelectorVtbl {
    pub const fn new<Impl: IOpcRelationshipSelectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcRelationshipSelectorVtbl {
        unsafe extern "system" fn GetSelectorType<Impl: IOpcRelationshipSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selector: *mut OPC_RELATIONSHIP_SELECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSelectorType(::core::mem::transmute_copy(&selector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectionCriterion<Impl: IOpcRelationshipSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selectioncriterion: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSelectionCriterion(::core::mem::transmute_copy(&selectioncriterion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcRelationshipSelector>, base.5, GetSelectorType::<Impl, OFFSET>, GetSelectionCriterion::<Impl, OFFSET>)
    }
}
pub trait IOpcRelationshipSelectorEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
impl ::windows::core::RuntimeName for IOpcRelationshipSelectorEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcRelationshipSelectorEnumerator";
}
impl IOpcRelationshipSelectorEnumeratorVtbl {
    pub const fn new<Impl: IOpcRelationshipSelectorEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcRelationshipSelectorEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcRelationshipSelectorEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcRelationshipSelectorEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MovePrevious(::core::mem::transmute_copy(&hasprevious)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcRelationshipSelectorEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipselector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&relationshipselector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcRelationshipSelectorEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcRelationshipSelectorEnumerator>, base.5, MoveNext::<Impl, OFFSET>, MovePrevious::<Impl, OFFSET>, GetCurrent::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IOpcRelationshipSelectorSetImpl: Sized {
    fn Create();
    fn Delete();
    fn GetEnumerator();
}
impl ::windows::core::RuntimeName for IOpcRelationshipSelectorSet {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcRelationshipSelectorSet";
}
impl IOpcRelationshipSelectorSetVtbl {
    pub const fn new<Impl: IOpcRelationshipSelectorSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcRelationshipSelectorSetVtbl {
        unsafe extern "system" fn Create<Impl: IOpcRelationshipSelectorSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: super::super::super::Foundation::PWSTR, relationshipselector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(selector, &*(&selectioncriterion as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&relationshipselector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IOpcRelationshipSelectorSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&relationshipselector as *const <IOpcRelationshipSelector as ::windows::core::Abi>::Abi as *const <IOpcRelationshipSelector as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcRelationshipSelectorSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipselectorenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnumerator(::core::mem::transmute_copy(&relationshipselectorenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcRelationshipSelectorSet>, base.5, Create::<Impl, OFFSET>, Delete::<Impl, OFFSET>, GetEnumerator::<Impl, OFFSET>)
    }
}
pub trait IOpcRelationshipSetImpl: Sized {
    fn GetRelationship();
    fn CreateRelationship();
    fn DeleteRelationship();
    fn RelationshipExists();
    fn GetEnumerator();
    fn GetEnumeratorForType();
    fn GetRelationshipsContentStream();
}
impl ::windows::core::RuntimeName for IOpcRelationshipSet {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcRelationshipSet";
}
impl IOpcRelationshipSetVtbl {
    pub const fn new<Impl: IOpcRelationshipSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcRelationshipSetVtbl {
        unsafe extern "system" fn GetRelationship<Impl: IOpcRelationshipSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipidentifier: super::super::super::Foundation::PWSTR, relationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelationship(&*(&relationshipidentifier as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&relationship)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRelationship<Impl: IOpcRelationshipSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipidentifier: super::super::super::Foundation::PWSTR, relationshiptype: super::super::super::Foundation::PWSTR, targeturi: ::windows::core::RawPtr, targetmode: OPC_URI_TARGET_MODE, relationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRelationship(
                &*(&relationshipidentifier as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&relationshiptype as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&targeturi as *const <super::super::super::System::Com::IUri as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IUri as ::windows::core::DefaultType>::DefaultType),
                targetmode,
                ::core::mem::transmute_copy(&relationship),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRelationship<Impl: IOpcRelationshipSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipidentifier: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteRelationship(&*(&relationshipidentifier as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelationshipExists<Impl: IOpcRelationshipSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipidentifier: super::super::super::Foundation::PWSTR, relationshipexists: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelationshipExists(&*(&relationshipidentifier as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&relationshipexists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcRelationshipSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnumerator(::core::mem::transmute_copy(&relationshipenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumeratorForType<Impl: IOpcRelationshipSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshiptype: super::super::super::Foundation::PWSTR, relationshipenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnumeratorForType(&*(&relationshiptype as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&relationshipenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipsContentStream<Impl: IOpcRelationshipSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelationshipsContentStream(::core::mem::transmute_copy(&contents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcRelationshipSet>, base.5, GetRelationship::<Impl, OFFSET>, CreateRelationship::<Impl, OFFSET>, DeleteRelationship::<Impl, OFFSET>, RelationshipExists::<Impl, OFFSET>, GetEnumerator::<Impl, OFFSET>, GetEnumeratorForType::<Impl, OFFSET>, GetRelationshipsContentStream::<Impl, OFFSET>)
    }
}
pub trait IOpcSignatureCustomObjectImpl: Sized {
    fn GetXml();
}
impl ::windows::core::RuntimeName for IOpcSignatureCustomObject {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcSignatureCustomObject";
}
impl IOpcSignatureCustomObjectVtbl {
    pub const fn new<Impl: IOpcSignatureCustomObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcSignatureCustomObjectVtbl {
        unsafe extern "system" fn GetXml<Impl: IOpcSignatureCustomObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetXml(::core::mem::transmute_copy(&xmlmarkup), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcSignatureCustomObject>, base.5, GetXml::<Impl, OFFSET>)
    }
}
pub trait IOpcSignatureCustomObjectEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
impl ::windows::core::RuntimeName for IOpcSignatureCustomObjectEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcSignatureCustomObjectEnumerator";
}
impl IOpcSignatureCustomObjectEnumeratorVtbl {
    pub const fn new<Impl: IOpcSignatureCustomObjectEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcSignatureCustomObjectEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcSignatureCustomObjectEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcSignatureCustomObjectEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MovePrevious(::core::mem::transmute_copy(&hasprevious)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcSignatureCustomObjectEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&customobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcSignatureCustomObjectEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcSignatureCustomObjectEnumerator>, base.5, MoveNext::<Impl, OFFSET>, MovePrevious::<Impl, OFFSET>, GetCurrent::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IOpcSignatureCustomObjectSetImpl: Sized {
    fn Create();
    fn Delete();
    fn GetEnumerator();
}
impl ::windows::core::RuntimeName for IOpcSignatureCustomObjectSet {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcSignatureCustomObjectSet";
}
impl IOpcSignatureCustomObjectSetVtbl {
    pub const fn new<Impl: IOpcSignatureCustomObjectSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcSignatureCustomObjectSetVtbl {
        unsafe extern "system" fn Create<Impl: IOpcSignatureCustomObjectSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xmlmarkup: *const u8, count: u32, customobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(xmlmarkup, count, ::core::mem::transmute_copy(&customobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IOpcSignatureCustomObjectSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&customobject as *const <IOpcSignatureCustomObject as ::windows::core::Abi>::Abi as *const <IOpcSignatureCustomObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcSignatureCustomObjectSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnumerator(::core::mem::transmute_copy(&customobjectenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcSignatureCustomObjectSet>, base.5, Create::<Impl, OFFSET>, Delete::<Impl, OFFSET>, GetEnumerator::<Impl, OFFSET>)
    }
}
pub trait IOpcSignaturePartReferenceImpl: Sized {
    fn GetPartName();
    fn GetContentType();
    fn GetDigestMethod();
    fn GetDigestValue();
    fn GetTransformMethod();
}
impl ::windows::core::RuntimeName for IOpcSignaturePartReference {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcSignaturePartReference";
}
impl IOpcSignaturePartReferenceVtbl {
    pub const fn new<Impl: IOpcSignaturePartReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcSignaturePartReferenceVtbl {
        unsafe extern "system" fn GetPartName<Impl: IOpcSignaturePartReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPartName(::core::mem::transmute_copy(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentType<Impl: IOpcSignaturePartReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetContentType(::core::mem::transmute_copy(&contenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestMethod<Impl: IOpcSignaturePartReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDigestMethod(::core::mem::transmute_copy(&digestmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestValue<Impl: IOpcSignaturePartReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDigestValue(::core::mem::transmute_copy(&digestvalue), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformMethod<Impl: IOpcSignaturePartReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransformMethod(::core::mem::transmute_copy(&transformmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcSignaturePartReference>, base.5, GetPartName::<Impl, OFFSET>, GetContentType::<Impl, OFFSET>, GetDigestMethod::<Impl, OFFSET>, GetDigestValue::<Impl, OFFSET>, GetTransformMethod::<Impl, OFFSET>)
    }
}
pub trait IOpcSignaturePartReferenceEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
impl ::windows::core::RuntimeName for IOpcSignaturePartReferenceEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcSignaturePartReferenceEnumerator";
}
impl IOpcSignaturePartReferenceEnumeratorVtbl {
    pub const fn new<Impl: IOpcSignaturePartReferenceEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcSignaturePartReferenceEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcSignaturePartReferenceEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcSignaturePartReferenceEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MovePrevious(::core::mem::transmute_copy(&hasprevious)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcSignaturePartReferenceEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&partreference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcSignaturePartReferenceEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcSignaturePartReferenceEnumerator>, base.5, MoveNext::<Impl, OFFSET>, MovePrevious::<Impl, OFFSET>, GetCurrent::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IOpcSignaturePartReferenceSetImpl: Sized {
    fn Create();
    fn Delete();
    fn GetEnumerator();
}
impl ::windows::core::RuntimeName for IOpcSignaturePartReferenceSet {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcSignaturePartReferenceSet";
}
impl IOpcSignaturePartReferenceSetVtbl {
    pub const fn new<Impl: IOpcSignaturePartReferenceSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcSignaturePartReferenceSetVtbl {
        unsafe extern "system" fn Create<Impl: IOpcSignaturePartReferenceSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, digestmethod: super::super::super::Foundation::PWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, partreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&parturi as *const <IOpcPartUri as ::windows::core::Abi>::Abi as *const <IOpcPartUri as ::windows::core::DefaultType>::DefaultType), &*(&digestmethod as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), transformmethod, ::core::mem::transmute_copy(&partreference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IOpcSignaturePartReferenceSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partreference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&partreference as *const <IOpcSignaturePartReference as ::windows::core::Abi>::Abi as *const <IOpcSignaturePartReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcSignaturePartReferenceSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnumerator(::core::mem::transmute_copy(&partreferenceenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcSignaturePartReferenceSet>, base.5, Create::<Impl, OFFSET>, Delete::<Impl, OFFSET>, GetEnumerator::<Impl, OFFSET>)
    }
}
pub trait IOpcSignatureReferenceImpl: Sized {
    fn GetId();
    fn GetUri();
    fn GetType();
    fn GetTransformMethod();
    fn GetDigestMethod();
    fn GetDigestValue();
}
impl ::windows::core::RuntimeName for IOpcSignatureReference {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcSignatureReference";
}
impl IOpcSignatureReferenceVtbl {
    pub const fn new<Impl: IOpcSignatureReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcSignatureReferenceVtbl {
        unsafe extern "system" fn GetId<Impl: IOpcSignatureReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referenceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetId(::core::mem::transmute_copy(&referenceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUri<Impl: IOpcSignatureReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referenceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUri(::core::mem::transmute_copy(&referenceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IOpcSignatureReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformMethod<Impl: IOpcSignatureReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransformMethod(::core::mem::transmute_copy(&transformmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestMethod<Impl: IOpcSignatureReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDigestMethod(::core::mem::transmute_copy(&digestmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestValue<Impl: IOpcSignatureReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDigestValue(::core::mem::transmute_copy(&digestvalue), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcSignatureReference>, base.5, GetId::<Impl, OFFSET>, GetUri::<Impl, OFFSET>, GetType::<Impl, OFFSET>, GetTransformMethod::<Impl, OFFSET>, GetDigestMethod::<Impl, OFFSET>, GetDigestValue::<Impl, OFFSET>)
    }
}
pub trait IOpcSignatureReferenceEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
impl ::windows::core::RuntimeName for IOpcSignatureReferenceEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcSignatureReferenceEnumerator";
}
impl IOpcSignatureReferenceEnumeratorVtbl {
    pub const fn new<Impl: IOpcSignatureReferenceEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcSignatureReferenceEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcSignatureReferenceEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcSignatureReferenceEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MovePrevious(::core::mem::transmute_copy(&hasprevious)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcSignatureReferenceEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&reference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcSignatureReferenceEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcSignatureReferenceEnumerator>, base.5, MoveNext::<Impl, OFFSET>, MovePrevious::<Impl, OFFSET>, GetCurrent::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IOpcSignatureReferenceSetImpl: Sized {
    fn Create();
    fn Delete();
    fn GetEnumerator();
}
impl ::windows::core::RuntimeName for IOpcSignatureReferenceSet {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcSignatureReferenceSet";
}
impl IOpcSignatureReferenceSetVtbl {
    pub const fn new<Impl: IOpcSignatureReferenceSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcSignatureReferenceSetVtbl {
        unsafe extern "system" fn Create<Impl: IOpcSignatureReferenceSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referenceuri: ::windows::core::RawPtr, referenceid: super::super::super::Foundation::PWSTR, r#type: super::super::super::Foundation::PWSTR, digestmethod: super::super::super::Foundation::PWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, reference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(
                &*(&referenceuri as *const <super::super::super::System::Com::IUri as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IUri as ::windows::core::DefaultType>::DefaultType),
                &*(&referenceid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&r#type as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&digestmethod as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                transformmethod,
                ::core::mem::transmute_copy(&reference),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IOpcSignatureReferenceSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&reference as *const <IOpcSignatureReference as ::windows::core::Abi>::Abi as *const <IOpcSignatureReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcSignatureReferenceSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnumerator(::core::mem::transmute_copy(&referenceenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcSignatureReferenceSet>, base.5, Create::<Impl, OFFSET>, Delete::<Impl, OFFSET>, GetEnumerator::<Impl, OFFSET>)
    }
}
pub trait IOpcSignatureRelationshipReferenceImpl: Sized {
    fn GetSourceUri();
    fn GetDigestMethod();
    fn GetDigestValue();
    fn GetTransformMethod();
    fn GetRelationshipSigningOption();
    fn GetRelationshipSelectorEnumerator();
}
impl ::windows::core::RuntimeName for IOpcSignatureRelationshipReference {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcSignatureRelationshipReference";
}
impl IOpcSignatureRelationshipReferenceVtbl {
    pub const fn new<Impl: IOpcSignatureRelationshipReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcSignatureRelationshipReferenceVtbl {
        unsafe extern "system" fn GetSourceUri<Impl: IOpcSignatureRelationshipReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceUri(::core::mem::transmute_copy(&sourceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestMethod<Impl: IOpcSignatureRelationshipReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDigestMethod(::core::mem::transmute_copy(&digestmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestValue<Impl: IOpcSignatureRelationshipReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDigestValue(::core::mem::transmute_copy(&digestvalue), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformMethod<Impl: IOpcSignatureRelationshipReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransformMethod(::core::mem::transmute_copy(&transformmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipSigningOption<Impl: IOpcSignatureRelationshipReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipsigningoption: *mut OPC_RELATIONSHIPS_SIGNING_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelationshipSigningOption(::core::mem::transmute_copy(&relationshipsigningoption)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelationshipSelectorEnumerator<Impl: IOpcSignatureRelationshipReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selectorenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelationshipSelectorEnumerator(::core::mem::transmute_copy(&selectorenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcSignatureRelationshipReference>, base.5, GetSourceUri::<Impl, OFFSET>, GetDigestMethod::<Impl, OFFSET>, GetDigestValue::<Impl, OFFSET>, GetTransformMethod::<Impl, OFFSET>, GetRelationshipSigningOption::<Impl, OFFSET>, GetRelationshipSelectorEnumerator::<Impl, OFFSET>)
    }
}
pub trait IOpcSignatureRelationshipReferenceEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
impl ::windows::core::RuntimeName for IOpcSignatureRelationshipReferenceEnumerator {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcSignatureRelationshipReferenceEnumerator";
}
impl IOpcSignatureRelationshipReferenceEnumeratorVtbl {
    pub const fn new<Impl: IOpcSignatureRelationshipReferenceEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcSignatureRelationshipReferenceEnumeratorVtbl {
        unsafe extern "system" fn MoveNext<Impl: IOpcSignatureRelationshipReferenceEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveNext(::core::mem::transmute_copy(&hasnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IOpcSignatureRelationshipReferenceEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MovePrevious(::core::mem::transmute_copy(&hasprevious)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrent<Impl: IOpcSignatureRelationshipReferenceEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrent(::core::mem::transmute_copy(&relationshipreference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IOpcSignatureRelationshipReferenceEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcSignatureRelationshipReferenceEnumerator>, base.5, MoveNext::<Impl, OFFSET>, MovePrevious::<Impl, OFFSET>, GetCurrent::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IOpcSignatureRelationshipReferenceSetImpl: Sized {
    fn Create();
    fn CreateRelationshipSelectorSet();
    fn Delete();
    fn GetEnumerator();
}
impl ::windows::core::RuntimeName for IOpcSignatureRelationshipReferenceSet {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcSignatureRelationshipReferenceSet";
}
impl IOpcSignatureRelationshipReferenceSetVtbl {
    pub const fn new<Impl: IOpcSignatureRelationshipReferenceSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcSignatureRelationshipReferenceSetVtbl {
        unsafe extern "system" fn Create<Impl: IOpcSignatureRelationshipReferenceSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourceuri: ::windows::core::RawPtr, digestmethod: super::super::super::Foundation::PWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: ::windows::core::RawPtr, transformmethod: OPC_CANONICALIZATION_METHOD, relationshipreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(
                &*(&sourceuri as *const <IOpcUri as ::windows::core::Abi>::Abi as *const <IOpcUri as ::windows::core::DefaultType>::DefaultType),
                &*(&digestmethod as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                relationshipsigningoption,
                &*(&selectorset as *const <IOpcRelationshipSelectorSet as ::windows::core::Abi>::Abi as *const <IOpcRelationshipSelectorSet as ::windows::core::DefaultType>::DefaultType),
                transformmethod,
                ::core::mem::transmute_copy(&relationshipreference),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRelationshipSelectorSet<Impl: IOpcSignatureRelationshipReferenceSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selectorset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRelationshipSelectorSet(::core::mem::transmute_copy(&selectorset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IOpcSignatureRelationshipReferenceSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipreference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&relationshipreference as *const <IOpcSignatureRelationshipReference as ::windows::core::Abi>::Abi as *const <IOpcSignatureRelationshipReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Impl: IOpcSignatureRelationshipReferenceSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnumerator(::core::mem::transmute_copy(&relationshipreferenceenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcSignatureRelationshipReferenceSet>, base.5, Create::<Impl, OFFSET>, CreateRelationshipSelectorSet::<Impl, OFFSET>, Delete::<Impl, OFFSET>, GetEnumerator::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IOpcSigningOptions {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcSigningOptions";
}
impl IOpcSigningOptionsVtbl {
    pub const fn new<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcSigningOptionsVtbl {
        unsafe extern "system" fn GetSignatureId<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureId(::core::mem::transmute_copy(&signatureid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureId<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureid: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSignatureId(&*(&signatureid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureMethod<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureMethod(::core::mem::transmute_copy(&signaturemethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureMethod<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturemethod: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSignatureMethod(&*(&signaturemethod as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultDigestMethod<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultDigestMethod(::core::mem::transmute_copy(&digestmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultDigestMethod<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDefaultDigestMethod(&*(&digestmethod as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateEmbeddingOption<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, embeddingoption: *mut OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCertificateEmbeddingOption(::core::mem::transmute_copy(&embeddingoption)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateEmbeddingOption<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCertificateEmbeddingOption(embeddingoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeFormat<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTimeFormat(::core::mem::transmute_copy(&timeformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeFormat<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTimeFormat(timeformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartReferenceSet<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartReferenceSet(::core::mem::transmute_copy(&partreferenceset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureRelationshipReferenceSet<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureRelationshipReferenceSet(::core::mem::transmute_copy(&relationshipreferenceset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectSet<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCustomObjectSet(::core::mem::transmute_copy(&customobjectset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceSet<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCustomReferenceSet(::core::mem::transmute_copy(&customreferenceset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateSet<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificateset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCertificateSet(::core::mem::transmute_copy(&certificateset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartName(::core::mem::transmute_copy(&signaturepartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignaturePartName<Impl: IOpcSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSignaturePartName(&*(&signaturepartname as *const <IOpcPartUri as ::windows::core::Abi>::Abi as *const <IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IOpcSigningOptions>,
            base.5,
            GetSignatureId::<Impl, OFFSET>,
            SetSignatureId::<Impl, OFFSET>,
            GetSignatureMethod::<Impl, OFFSET>,
            SetSignatureMethod::<Impl, OFFSET>,
            GetDefaultDigestMethod::<Impl, OFFSET>,
            SetDefaultDigestMethod::<Impl, OFFSET>,
            GetCertificateEmbeddingOption::<Impl, OFFSET>,
            SetCertificateEmbeddingOption::<Impl, OFFSET>,
            GetTimeFormat::<Impl, OFFSET>,
            SetTimeFormat::<Impl, OFFSET>,
            GetSignaturePartReferenceSet::<Impl, OFFSET>,
            GetSignatureRelationshipReferenceSet::<Impl, OFFSET>,
            GetCustomObjectSet::<Impl, OFFSET>,
            GetCustomReferenceSet::<Impl, OFFSET>,
            GetCertificateSet::<Impl, OFFSET>,
            GetSignaturePartName::<Impl, OFFSET>,
            SetSignaturePartName::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcUriImpl: Sized + IUriImpl {
    fn GetRelationshipsPartUri();
    fn GetRelativeUri();
    fn CombinePartUri();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IOpcUri {
    const NAME: &'static str = "Windows.Win32.Storage.Packaging.Opc.IOpcUri";
}
#[cfg(feature = "Win32_System_Com")]
impl IOpcUriVtbl {
    pub const fn new<Impl: IOpcUriImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOpcUriVtbl {
        unsafe extern "system" fn GetRelationshipsPartUri<Impl: IOpcUriImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relationshipparturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelationshipsPartUri(::core::mem::transmute_copy(&relationshipparturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeUri<Impl: IOpcUriImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetparturi: ::windows::core::RawPtr, relativeuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelativeUri(&*(&targetparturi as *const <IOpcPartUri as ::windows::core::Abi>::Abi as *const <IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&relativeuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CombinePartUri<Impl: IOpcUriImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeuri: ::windows::core::RawPtr, combineduri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CombinePartUri(&*(&relativeuri as *const <super::super::super::System::Com::IUri as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&combineduri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOpcUri>, base.5, GetRelationshipsPartUri::<Impl, OFFSET>, GetRelativeUri::<Impl, OFFSET>, CombinePartUri::<Impl, OFFSET>)
    }
}
