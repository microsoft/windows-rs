#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertSrvSetupImpl: Sized + IDispatchImpl {
    fn CAErrorId();
    fn CAErrorString();
    fn InitializeDefaults();
    fn GetCASetupProperty();
    fn SetCASetupProperty();
    fn IsPropertyEditable();
    fn GetSupportedCATypes();
    fn GetProviderNameList();
    fn GetKeyLengthList();
    fn GetHashAlgorithmList();
    fn GetPrivateKeyContainerList();
    fn GetExistingCACertificates();
    fn CAImportPFX();
    fn SetCADistinguishedName();
    fn SetDatabaseInformation();
    fn SetParentCAInformation();
    fn SetWebCAInformation();
    fn Install();
    fn PreUnInstall();
    fn PostUnInstall();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertSrvSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertSrvSetupVtbl {
        unsafe extern "system" fn CAErrorId<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CAErrorString<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDefaults<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bserver: i16, bclient: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCASetupProperty<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCASetupProperty<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPropertyEditable<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, pbeditable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedCATypes<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcatypes: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProviderNameList<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeyLengthList<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHashAlgorithmList<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateKeyContainerList<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExistingCACertificates<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CAImportPFX<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpasswd: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwriteexistingkey: i16, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCADistinguishedName<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcadn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bignoreunicode: i16, boverwriteexistingkey: i16, boverwriteexistingcainds: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDatabaseInformation<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdbdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlogdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsharedfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bforceoverwrite: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParentCAInformation<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWebCAInformation<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Install<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreUnInstall<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bclientonly: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostUnInstall<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            CAErrorId::<Impl, IMPL_OFFSET>,
            CAErrorString::<Impl, IMPL_OFFSET>,
            InitializeDefaults::<Impl, IMPL_OFFSET>,
            GetCASetupProperty::<Impl, IMPL_OFFSET>,
            SetCASetupProperty::<Impl, IMPL_OFFSET>,
            IsPropertyEditable::<Impl, IMPL_OFFSET>,
            GetSupportedCATypes::<Impl, IMPL_OFFSET>,
            GetProviderNameList::<Impl, IMPL_OFFSET>,
            GetKeyLengthList::<Impl, IMPL_OFFSET>,
            GetHashAlgorithmList::<Impl, IMPL_OFFSET>,
            GetPrivateKeyContainerList::<Impl, IMPL_OFFSET>,
            GetExistingCACertificates::<Impl, IMPL_OFFSET>,
            CAImportPFX::<Impl, IMPL_OFFSET>,
            SetCADistinguishedName::<Impl, IMPL_OFFSET>,
            SetDatabaseInformation::<Impl, IMPL_OFFSET>,
            SetParentCAInformation::<Impl, IMPL_OFFSET>,
            SetWebCAInformation::<Impl, IMPL_OFFSET>,
            Install::<Impl, IMPL_OFFSET>,
            PreUnInstall::<Impl, IMPL_OFFSET>,
            PostUnInstall::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertSrvSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertSrvSetupKeyInformationImpl: Sized + IDispatchImpl {
    fn ProviderName();
    fn SetProviderName();
    fn Length();
    fn SetLength();
    fn Existing();
    fn SetExisting();
    fn ContainerName();
    fn SetContainerName();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn ExistingCACertificate();
    fn SetExistingCACertificate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertSrvSetupKeyInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertSrvSetupKeyInformationVtbl {
        unsafe extern "system" fn ProviderName<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderName<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Length<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLength<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Existing<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExisting<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContainerName<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContainerName<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExistingCACertificate<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExistingCACertificate<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ProviderName::<Impl, IMPL_OFFSET>,
            SetProviderName::<Impl, IMPL_OFFSET>,
            Length::<Impl, IMPL_OFFSET>,
            SetLength::<Impl, IMPL_OFFSET>,
            Existing::<Impl, IMPL_OFFSET>,
            SetExisting::<Impl, IMPL_OFFSET>,
            ContainerName::<Impl, IMPL_OFFSET>,
            SetContainerName::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            ExistingCACertificate::<Impl, IMPL_OFFSET>,
            SetExistingCACertificate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertSrvSetupKeyInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertSrvSetupKeyInformationCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertSrvSetupKeyInformationCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformationCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertSrvSetupKeyInformationCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ICertSrvSetupKeyInformationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ICertSrvSetupKeyInformationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ICertSrvSetupKeyInformationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ICertSrvSetupKeyInformationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pikeyinformation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertSrvSetupKeyInformationCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificateEnrollmentPolicyServerSetupImpl: Sized + IDispatchImpl {
    fn ErrorString();
    fn InitializeInstallDefaults();
    fn GetProperty();
    fn SetProperty();
    fn Install();
    fn UnInstall();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificateEnrollmentPolicyServerSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentPolicyServerSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateEnrollmentPolicyServerSetupVtbl {
        unsafe extern "system" fn ErrorString<Impl: ICertificateEnrollmentPolicyServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeInstallDefaults<Impl: ICertificateEnrollmentPolicyServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: ICertificateEnrollmentPolicyServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: ICertificateEnrollmentPolicyServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Install<Impl: ICertificateEnrollmentPolicyServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnInstall<Impl: ICertificateEnrollmentPolicyServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthkeybasedrenewal: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ErrorString::<Impl, IMPL_OFFSET>,
            InitializeInstallDefaults::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            Install::<Impl, IMPL_OFFSET>,
            UnInstall::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateEnrollmentPolicyServerSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificateEnrollmentServerSetupImpl: Sized + IDispatchImpl {
    fn ErrorString();
    fn InitializeInstallDefaults();
    fn GetProperty();
    fn SetProperty();
    fn SetApplicationPoolCredentials();
    fn Install();
    fn UnInstall();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificateEnrollmentServerSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentServerSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateEnrollmentServerSetupVtbl {
        unsafe extern "system" fn ErrorString<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeInstallDefaults<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetApplicationPoolCredentials<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Install<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnInstall<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaconfig: *const super::super::System::Com::VARIANT, pauthentication: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ErrorString::<Impl, IMPL_OFFSET>,
            InitializeInstallDefaults::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            SetApplicationPoolCredentials::<Impl, IMPL_OFFSET>,
            Install::<Impl, IMPL_OFFSET>,
            UnInstall::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateEnrollmentServerSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSCEPSetupImpl: Sized + IDispatchImpl {
    fn MSCEPErrorId();
    fn MSCEPErrorString();
    fn InitializeDefaults();
    fn GetMSCEPSetupProperty();
    fn SetMSCEPSetupProperty();
    fn SetAccountInformation();
    fn IsMSCEPStoreEmpty();
    fn GetProviderNameList();
    fn GetKeyLengthList();
    fn Install();
    fn PreUnInstall();
    fn PostUnInstall();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSCEPSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSCEPSetupVtbl {
        unsafe extern "system" fn MSCEPErrorId<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MSCEPErrorString<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDefaults<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMSCEPSetupProperty<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: MSCEPSetupProperty, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMSCEPSetupProperty<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: MSCEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAccountInformation<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsMSCEPStoreEmpty<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbempty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProviderNameList<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bexchange: i16, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeyLengthList<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bexchange: i16, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Install<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreUnInstall<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostUnInstall<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            MSCEPErrorId::<Impl, IMPL_OFFSET>,
            MSCEPErrorString::<Impl, IMPL_OFFSET>,
            InitializeDefaults::<Impl, IMPL_OFFSET>,
            GetMSCEPSetupProperty::<Impl, IMPL_OFFSET>,
            SetMSCEPSetupProperty::<Impl, IMPL_OFFSET>,
            SetAccountInformation::<Impl, IMPL_OFFSET>,
            IsMSCEPStoreEmpty::<Impl, IMPL_OFFSET>,
            GetProviderNameList::<Impl, IMPL_OFFSET>,
            GetKeyLengthList::<Impl, IMPL_OFFSET>,
            Install::<Impl, IMPL_OFFSET>,
            PreUnInstall::<Impl, IMPL_OFFSET>,
            PostUnInstall::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSCEPSetup as ::windows::core::Interface>::IID
    }
}
