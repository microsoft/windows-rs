#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertSrvSetupImpl: Sized + IDispatchImpl {
    fn CAErrorId(&mut self) -> ::windows::core::Result<i32>;
    fn CAErrorString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InitializeDefaults(&mut self, bserver: i16, bclient: i16) -> ::windows::core::Result<()>;
    fn GetCASetupProperty(&mut self, propertyid: CASetupProperty) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetCASetupProperty(&mut self, propertyid: CASetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn IsPropertyEditable(&mut self, propertyid: CASetupProperty) -> ::windows::core::Result<i16>;
    fn GetSupportedCATypes(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetProviderNameList(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetKeyLengthList(&mut self, bstrprovidername: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetHashAlgorithmList(&mut self, bstrprovidername: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetPrivateKeyContainerList(&mut self, bstrprovidername: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetExistingCACertificates(&mut self) -> ::windows::core::Result<ICertSrvSetupKeyInformationCollection>;
    fn CAImportPFX(&mut self, bstrfilename: super::super::Foundation::BSTR, bstrpasswd: super::super::Foundation::BSTR, boverwriteexistingkey: i16) -> ::windows::core::Result<ICertSrvSetupKeyInformation>;
    fn SetCADistinguishedName(&mut self, bstrcadn: super::super::Foundation::BSTR, bignoreunicode: i16, boverwriteexistingkey: i16, boverwriteexistingcainds: i16) -> ::windows::core::Result<()>;
    fn SetDatabaseInformation(&mut self, bstrdbdirectory: super::super::Foundation::BSTR, bstrlogdirectory: super::super::Foundation::BSTR, bstrsharedfolder: super::super::Foundation::BSTR, bforceoverwrite: i16) -> ::windows::core::Result<()>;
    fn SetParentCAInformation(&mut self, bstrcaconfiguration: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetWebCAInformation(&mut self, bstrcaconfiguration: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Install(&mut self) -> ::windows::core::Result<()>;
    fn PreUnInstall(&mut self, bclientonly: i16) -> ::windows::core::Result<()>;
    fn PostUnInstall(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertSrvSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertSrvSetupVtbl {
        unsafe extern "system" fn CAErrorId<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CAErrorId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAErrorString<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CAErrorString() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeDefaults<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bserver: i16, bclient: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDefaults(::core::mem::transmute_copy(&bserver), ::core::mem::transmute_copy(&bclient)).into()
        }
        unsafe extern "system" fn GetCASetupProperty<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCASetupProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCASetupProperty<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCASetupProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn IsPropertyEditable<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, pbeditable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPropertyEditable(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbeditable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedCATypes<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcatypes: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedCATypes() {
                ::core::result::Result::Ok(ok__) => {
                    *pcatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderNameList<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderNameList() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyLengthList<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeyLengthList(::core::mem::transmute_copy(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashAlgorithmList<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHashAlgorithmList(::core::mem::transmute_copy(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivateKeyContainerList<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrivateKeyContainerList(::core::mem::transmute_copy(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExistingCACertificates<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExistingCACertificates() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAImportPFX<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpasswd: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwriteexistingkey: i16, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CAImportPFX(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&bstrpasswd), ::core::mem::transmute_copy(&boverwriteexistingkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCADistinguishedName<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcadn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bignoreunicode: i16, boverwriteexistingkey: i16, boverwriteexistingcainds: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCADistinguishedName(::core::mem::transmute_copy(&bstrcadn), ::core::mem::transmute_copy(&bignoreunicode), ::core::mem::transmute_copy(&boverwriteexistingkey), ::core::mem::transmute_copy(&boverwriteexistingcainds)).into()
        }
        unsafe extern "system" fn SetDatabaseInformation<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdbdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlogdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsharedfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bforceoverwrite: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDatabaseInformation(::core::mem::transmute_copy(&bstrdbdirectory), ::core::mem::transmute_copy(&bstrlogdirectory), ::core::mem::transmute_copy(&bstrsharedfolder), ::core::mem::transmute_copy(&bforceoverwrite)).into()
        }
        unsafe extern "system" fn SetParentCAInformation<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParentCAInformation(::core::mem::transmute_copy(&bstrcaconfiguration)).into()
        }
        unsafe extern "system" fn SetWebCAInformation<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWebCAInformation(::core::mem::transmute_copy(&bstrcaconfiguration)).into()
        }
        unsafe extern "system" fn Install<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Install().into()
        }
        unsafe extern "system" fn PreUnInstall<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bclientonly: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreUnInstall(::core::mem::transmute_copy(&bclientonly)).into()
        }
        unsafe extern "system" fn PostUnInstall<Impl: ICertSrvSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PostUnInstall().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CAErrorId: CAErrorId::<Impl, IMPL_OFFSET>,
            CAErrorString: CAErrorString::<Impl, IMPL_OFFSET>,
            InitializeDefaults: InitializeDefaults::<Impl, IMPL_OFFSET>,
            GetCASetupProperty: GetCASetupProperty::<Impl, IMPL_OFFSET>,
            SetCASetupProperty: SetCASetupProperty::<Impl, IMPL_OFFSET>,
            IsPropertyEditable: IsPropertyEditable::<Impl, IMPL_OFFSET>,
            GetSupportedCATypes: GetSupportedCATypes::<Impl, IMPL_OFFSET>,
            GetProviderNameList: GetProviderNameList::<Impl, IMPL_OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Impl, IMPL_OFFSET>,
            GetHashAlgorithmList: GetHashAlgorithmList::<Impl, IMPL_OFFSET>,
            GetPrivateKeyContainerList: GetPrivateKeyContainerList::<Impl, IMPL_OFFSET>,
            GetExistingCACertificates: GetExistingCACertificates::<Impl, IMPL_OFFSET>,
            CAImportPFX: CAImportPFX::<Impl, IMPL_OFFSET>,
            SetCADistinguishedName: SetCADistinguishedName::<Impl, IMPL_OFFSET>,
            SetDatabaseInformation: SetDatabaseInformation::<Impl, IMPL_OFFSET>,
            SetParentCAInformation: SetParentCAInformation::<Impl, IMPL_OFFSET>,
            SetWebCAInformation: SetWebCAInformation::<Impl, IMPL_OFFSET>,
            Install: Install::<Impl, IMPL_OFFSET>,
            PreUnInstall: PreUnInstall::<Impl, IMPL_OFFSET>,
            PostUnInstall: PostUnInstall::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertSrvSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertSrvSetupKeyInformationImpl: Sized + IDispatchImpl {
    fn ProviderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProviderName(&mut self, bstrval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn SetLength(&mut self, lval: i32) -> ::windows::core::Result<()>;
    fn Existing(&mut self) -> ::windows::core::Result<i16>;
    fn SetExisting(&mut self, bval: i16) -> ::windows::core::Result<()>;
    fn ContainerName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetContainerName(&mut self, bstrval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HashAlgorithm(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetHashAlgorithm(&mut self, bstrval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ExistingCACertificate(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetExistingCACertificate(&mut self, varval: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertSrvSetupKeyInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertSrvSetupKeyInformationVtbl {
        unsafe extern "system" fn ProviderName<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderName<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderName(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn Length<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLength(::core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn Existing<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Existing() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExisting<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExisting(::core::mem::transmute_copy(&bval)).into()
        }
        unsafe extern "system" fn ContainerName<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainerName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerName<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContainerName(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn ExistingCACertificate<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistingCACertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExistingCACertificate<Impl: ICertSrvSetupKeyInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExistingCACertificate(::core::mem::transmute_copy(&varval)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ProviderName: ProviderName::<Impl, IMPL_OFFSET>,
            SetProviderName: SetProviderName::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            SetLength: SetLength::<Impl, IMPL_OFFSET>,
            Existing: Existing::<Impl, IMPL_OFFSET>,
            SetExisting: SetExisting::<Impl, IMPL_OFFSET>,
            ContainerName: ContainerName::<Impl, IMPL_OFFSET>,
            SetContainerName: SetContainerName::<Impl, IMPL_OFFSET>,
            HashAlgorithm: HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            ExistingCACertificate: ExistingCACertificate::<Impl, IMPL_OFFSET>,
            SetExistingCACertificate: SetExistingCACertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertSrvSetupKeyInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertSrvSetupKeyInformationCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, pikeyinformation: ::core::option::Option<ICertSrvSetupKeyInformation>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertSrvSetupKeyInformationCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformationCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertSrvSetupKeyInformationCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ICertSrvSetupKeyInformationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ICertSrvSetupKeyInformationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICertSrvSetupKeyInformationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ICertSrvSetupKeyInformationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pikeyinformation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pikeyinformation)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertSrvSetupKeyInformationCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificateEnrollmentPolicyServerSetupImpl: Sized + IDispatchImpl {
    fn ErrorString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InitializeInstallDefaults(&mut self) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, propertyid: CEPSetupProperty) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, propertyid: CEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Install(&mut self) -> ::windows::core::Result<()>;
    fn UnInstall(&mut self, pauthkeybasedrenewal: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificateEnrollmentPolicyServerSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentPolicyServerSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateEnrollmentPolicyServerSetupVtbl {
        unsafe extern "system" fn ErrorString<Impl: ICertificateEnrollmentPolicyServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorString() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeInstallDefaults<Impl: ICertificateEnrollmentPolicyServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeInstallDefaults().into()
        }
        unsafe extern "system" fn GetProperty<Impl: ICertificateEnrollmentPolicyServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: ICertificateEnrollmentPolicyServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn Install<Impl: ICertificateEnrollmentPolicyServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Install().into()
        }
        unsafe extern "system" fn UnInstall<Impl: ICertificateEnrollmentPolicyServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthkeybasedrenewal: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnInstall(::core::mem::transmute_copy(&pauthkeybasedrenewal)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ErrorString: ErrorString::<Impl, IMPL_OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            Install: Install::<Impl, IMPL_OFFSET>,
            UnInstall: UnInstall::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateEnrollmentPolicyServerSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificateEnrollmentServerSetupImpl: Sized + IDispatchImpl {
    fn ErrorString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InitializeInstallDefaults(&mut self) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, propertyid: CESSetupProperty) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, propertyid: CESSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetApplicationPoolCredentials(&mut self, bstrusername: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Install(&mut self) -> ::windows::core::Result<()>;
    fn UnInstall(&mut self, pcaconfig: *const super::super::System::Com::VARIANT, pauthentication: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificateEnrollmentServerSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentServerSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateEnrollmentServerSetupVtbl {
        unsafe extern "system" fn ErrorString<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorString() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeInstallDefaults<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeInstallDefaults().into()
        }
        unsafe extern "system" fn GetProperty<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn SetApplicationPoolCredentials<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationPoolCredentials(::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrpassword)).into()
        }
        unsafe extern "system" fn Install<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Install().into()
        }
        unsafe extern "system" fn UnInstall<Impl: ICertificateEnrollmentServerSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaconfig: *const super::super::System::Com::VARIANT, pauthentication: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnInstall(::core::mem::transmute_copy(&pcaconfig), ::core::mem::transmute_copy(&pauthentication)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ErrorString: ErrorString::<Impl, IMPL_OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            SetApplicationPoolCredentials: SetApplicationPoolCredentials::<Impl, IMPL_OFFSET>,
            Install: Install::<Impl, IMPL_OFFSET>,
            UnInstall: UnInstall::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateEnrollmentServerSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSCEPSetupImpl: Sized + IDispatchImpl {
    fn MSCEPErrorId(&mut self) -> ::windows::core::Result<i32>;
    fn MSCEPErrorString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InitializeDefaults(&mut self) -> ::windows::core::Result<()>;
    fn GetMSCEPSetupProperty(&mut self, propertyid: MSCEPSetupProperty) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetMSCEPSetupProperty(&mut self, propertyid: MSCEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetAccountInformation(&mut self, bstrusername: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsMSCEPStoreEmpty(&mut self) -> ::windows::core::Result<i16>;
    fn GetProviderNameList(&mut self, bexchange: i16) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetKeyLengthList(&mut self, bexchange: i16, bstrprovidername: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Install(&mut self) -> ::windows::core::Result<()>;
    fn PreUnInstall(&mut self) -> ::windows::core::Result<()>;
    fn PostUnInstall(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSCEPSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSCEPSetupVtbl {
        unsafe extern "system" fn MSCEPErrorId<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MSCEPErrorId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSCEPErrorString<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MSCEPErrorString() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeDefaults<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDefaults().into()
        }
        unsafe extern "system" fn GetMSCEPSetupProperty<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: MSCEPSetupProperty, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMSCEPSetupProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMSCEPSetupProperty<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: MSCEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMSCEPSetupProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn SetAccountInformation<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccountInformation(::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrpassword)).into()
        }
        unsafe extern "system" fn IsMSCEPStoreEmpty<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbempty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMSCEPStoreEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    *pbempty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderNameList<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bexchange: i16, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderNameList(::core::mem::transmute_copy(&bexchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyLengthList<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bexchange: i16, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeyLengthList(::core::mem::transmute_copy(&bexchange), ::core::mem::transmute_copy(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Install().into()
        }
        unsafe extern "system" fn PreUnInstall<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreUnInstall().into()
        }
        unsafe extern "system" fn PostUnInstall<Impl: IMSCEPSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PostUnInstall().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MSCEPErrorId: MSCEPErrorId::<Impl, IMPL_OFFSET>,
            MSCEPErrorString: MSCEPErrorString::<Impl, IMPL_OFFSET>,
            InitializeDefaults: InitializeDefaults::<Impl, IMPL_OFFSET>,
            GetMSCEPSetupProperty: GetMSCEPSetupProperty::<Impl, IMPL_OFFSET>,
            SetMSCEPSetupProperty: SetMSCEPSetupProperty::<Impl, IMPL_OFFSET>,
            SetAccountInformation: SetAccountInformation::<Impl, IMPL_OFFSET>,
            IsMSCEPStoreEmpty: IsMSCEPStoreEmpty::<Impl, IMPL_OFFSET>,
            GetProviderNameList: GetProviderNameList::<Impl, IMPL_OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Impl, IMPL_OFFSET>,
            Install: Install::<Impl, IMPL_OFFSET>,
            PreUnInstall: PreUnInstall::<Impl, IMPL_OFFSET>,
            PostUnInstall: PostUnInstall::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSCEPSetup as ::windows::core::Interface>::IID
    }
}
