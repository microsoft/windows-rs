#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertSrvSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CAErrorId(&mut self) -> ::windows::core::Result<i32>;
    fn CAErrorString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InitializeDefaults(&mut self, bserver: i16, bclient: i16) -> ::windows::core::Result<()>;
    fn GetCASetupProperty(&mut self, propertyid: CASetupProperty) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetCASetupProperty(&mut self, propertyid: CASetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn IsPropertyEditable(&mut self, propertyid: CASetupProperty) -> ::windows::core::Result<i16>;
    fn GetSupportedCATypes(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetProviderNameList(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetKeyLengthList(&mut self, bstrprovidername: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetHashAlgorithmList(&mut self, bstrprovidername: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetPrivateKeyContainerList(&mut self, bstrprovidername: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetExistingCACertificates(&mut self) -> ::windows::core::Result<ICertSrvSetupKeyInformationCollection>;
    fn CAImportPFX(&mut self, bstrfilename: &super::super::Foundation::BSTR, bstrpasswd: &super::super::Foundation::BSTR, boverwriteexistingkey: i16) -> ::windows::core::Result<ICertSrvSetupKeyInformation>;
    fn SetCADistinguishedName(&mut self, bstrcadn: &super::super::Foundation::BSTR, bignoreunicode: i16, boverwriteexistingkey: i16, boverwriteexistingcainds: i16) -> ::windows::core::Result<()>;
    fn SetDatabaseInformation(&mut self, bstrdbdirectory: &super::super::Foundation::BSTR, bstrlogdirectory: &super::super::Foundation::BSTR, bstrsharedfolder: &super::super::Foundation::BSTR, bforceoverwrite: i16) -> ::windows::core::Result<()>;
    fn SetParentCAInformation(&mut self, bstrcaconfiguration: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetWebCAInformation(&mut self, bstrcaconfiguration: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Install(&mut self) -> ::windows::core::Result<()>;
    fn PreUnInstall(&mut self, bclientonly: i16) -> ::windows::core::Result<()>;
    fn PostUnInstall(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertSrvSetup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>() -> ICertSrvSetup_Vtbl {
        unsafe extern "system" fn CAErrorId<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CAErrorId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAErrorString<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CAErrorString() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeDefaults<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bserver: i16, bclient: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeDefaults(::core::mem::transmute_copy(&bserver), ::core::mem::transmute_copy(&bclient)).into()
        }
        unsafe extern "system" fn GetCASetupProperty<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCASetupProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCASetupProperty<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCASetupProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn IsPropertyEditable<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, pbeditable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPropertyEditable(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbeditable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedCATypes<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcatypes: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedCATypes() {
                ::core::result::Result::Ok(ok__) => {
                    *pcatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderNameList<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProviderNameList() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyLengthList<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetKeyLengthList(::core::mem::transmute_copy(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashAlgorithmList<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHashAlgorithmList(::core::mem::transmute_copy(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivateKeyContainerList<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPrivateKeyContainerList(::core::mem::transmute_copy(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExistingCACertificates<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetExistingCACertificates() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAImportPFX<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpasswd: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwriteexistingkey: i16, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CAImportPFX(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&bstrpasswd), ::core::mem::transmute_copy(&boverwriteexistingkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCADistinguishedName<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcadn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bignoreunicode: i16, boverwriteexistingkey: i16, boverwriteexistingcainds: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCADistinguishedName(::core::mem::transmute_copy(&bstrcadn), ::core::mem::transmute_copy(&bignoreunicode), ::core::mem::transmute_copy(&boverwriteexistingkey), ::core::mem::transmute_copy(&boverwriteexistingcainds)).into()
        }
        unsafe extern "system" fn SetDatabaseInformation<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdbdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlogdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsharedfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bforceoverwrite: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDatabaseInformation(::core::mem::transmute_copy(&bstrdbdirectory), ::core::mem::transmute_copy(&bstrlogdirectory), ::core::mem::transmute_copy(&bstrsharedfolder), ::core::mem::transmute_copy(&bforceoverwrite)).into()
        }
        unsafe extern "system" fn SetParentCAInformation<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParentCAInformation(::core::mem::transmute_copy(&bstrcaconfiguration)).into()
        }
        unsafe extern "system" fn SetWebCAInformation<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWebCAInformation(::core::mem::transmute_copy(&bstrcaconfiguration)).into()
        }
        unsafe extern "system" fn Install<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Install().into()
        }
        unsafe extern "system" fn PreUnInstall<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bclientonly: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PreUnInstall(::core::mem::transmute_copy(&bclientonly)).into()
        }
        unsafe extern "system" fn PostUnInstall<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PostUnInstall().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CAErrorId: CAErrorId::<Identity, Impl, OFFSET>,
            CAErrorString: CAErrorString::<Identity, Impl, OFFSET>,
            InitializeDefaults: InitializeDefaults::<Identity, Impl, OFFSET>,
            GetCASetupProperty: GetCASetupProperty::<Identity, Impl, OFFSET>,
            SetCASetupProperty: SetCASetupProperty::<Identity, Impl, OFFSET>,
            IsPropertyEditable: IsPropertyEditable::<Identity, Impl, OFFSET>,
            GetSupportedCATypes: GetSupportedCATypes::<Identity, Impl, OFFSET>,
            GetProviderNameList: GetProviderNameList::<Identity, Impl, OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Identity, Impl, OFFSET>,
            GetHashAlgorithmList: GetHashAlgorithmList::<Identity, Impl, OFFSET>,
            GetPrivateKeyContainerList: GetPrivateKeyContainerList::<Identity, Impl, OFFSET>,
            GetExistingCACertificates: GetExistingCACertificates::<Identity, Impl, OFFSET>,
            CAImportPFX: CAImportPFX::<Identity, Impl, OFFSET>,
            SetCADistinguishedName: SetCADistinguishedName::<Identity, Impl, OFFSET>,
            SetDatabaseInformation: SetDatabaseInformation::<Identity, Impl, OFFSET>,
            SetParentCAInformation: SetParentCAInformation::<Identity, Impl, OFFSET>,
            SetWebCAInformation: SetWebCAInformation::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            PreUnInstall: PreUnInstall::<Identity, Impl, OFFSET>,
            PostUnInstall: PostUnInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertSrvSetup as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertSrvSetupKeyInformation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ProviderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProviderName(&mut self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn SetLength(&mut self, lval: i32) -> ::windows::core::Result<()>;
    fn Existing(&mut self) -> ::windows::core::Result<i16>;
    fn SetExisting(&mut self, bval: i16) -> ::windows::core::Result<()>;
    fn ContainerName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetContainerName(&mut self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HashAlgorithm(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetHashAlgorithm(&mut self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ExistingCACertificate(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetExistingCACertificate(&mut self, varval: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertSrvSetupKeyInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>() -> ICertSrvSetupKeyInformation_Vtbl {
        unsafe extern "system" fn ProviderName<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderName<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProviderName(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn Length<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLength(::core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn Existing<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Existing() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExisting<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExisting(::core::mem::transmute_copy(&bval)).into()
        }
        unsafe extern "system" fn ContainerName<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContainerName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerName<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContainerName(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn ExistingCACertificate<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExistingCACertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExistingCACertificate<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExistingCACertificate(::core::mem::transmute_copy(&varval)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ProviderName: ProviderName::<Identity, Impl, OFFSET>,
            SetProviderName: SetProviderName::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            SetLength: SetLength::<Identity, Impl, OFFSET>,
            Existing: Existing::<Identity, Impl, OFFSET>,
            SetExisting: SetExisting::<Identity, Impl, OFFSET>,
            ContainerName: ContainerName::<Identity, Impl, OFFSET>,
            SetContainerName: SetContainerName::<Identity, Impl, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, Impl, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, Impl, OFFSET>,
            ExistingCACertificate: ExistingCACertificate::<Identity, Impl, OFFSET>,
            SetExistingCACertificate: SetExistingCACertificate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertSrvSetupKeyInformation as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertSrvSetupKeyInformationCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, pikeyinformation: &::core::option::Option<ICertSrvSetupKeyInformation>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertSrvSetupKeyInformationCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>() -> ICertSrvSetupKeyInformationCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pikeyinformation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&pikeyinformation)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertSrvSetupKeyInformationCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificateEnrollmentPolicyServerSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ErrorString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InitializeInstallDefaults(&mut self) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, propertyid: CEPSetupProperty) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, propertyid: CEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Install(&mut self) -> ::windows::core::Result<()>;
    fn UnInstall(&mut self, pauthkeybasedrenewal: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificateEnrollmentPolicyServerSetup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>() -> ICertificateEnrollmentPolicyServerSetup_Vtbl {
        unsafe extern "system" fn ErrorString<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ErrorString() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeInstallDefaults<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeInstallDefaults().into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn Install<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Install().into()
        }
        unsafe extern "system" fn UnInstall<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthkeybasedrenewal: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnInstall(::core::mem::transmute_copy(&pauthkeybasedrenewal)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ErrorString: ErrorString::<Identity, Impl, OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            UnInstall: UnInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateEnrollmentPolicyServerSetup as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificateEnrollmentServerSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ErrorString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InitializeInstallDefaults(&mut self) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, propertyid: CESSetupProperty) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, propertyid: CESSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetApplicationPoolCredentials(&mut self, bstrusername: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Install(&mut self) -> ::windows::core::Result<()>;
    fn UnInstall(&mut self, pcaconfig: *const super::super::System::Com::VARIANT, pauthentication: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificateEnrollmentServerSetup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>() -> ICertificateEnrollmentServerSetup_Vtbl {
        unsafe extern "system" fn ErrorString<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ErrorString() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeInstallDefaults<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeInstallDefaults().into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn SetApplicationPoolCredentials<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetApplicationPoolCredentials(::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrpassword)).into()
        }
        unsafe extern "system" fn Install<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Install().into()
        }
        unsafe extern "system" fn UnInstall<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaconfig: *const super::super::System::Com::VARIANT, pauthentication: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnInstall(::core::mem::transmute_copy(&pcaconfig), ::core::mem::transmute_copy(&pauthentication)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ErrorString: ErrorString::<Identity, Impl, OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            SetApplicationPoolCredentials: SetApplicationPoolCredentials::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            UnInstall: UnInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateEnrollmentServerSetup as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSCEPSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MSCEPErrorId(&mut self) -> ::windows::core::Result<i32>;
    fn MSCEPErrorString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InitializeDefaults(&mut self) -> ::windows::core::Result<()>;
    fn GetMSCEPSetupProperty(&mut self, propertyid: MSCEPSetupProperty) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetMSCEPSetupProperty(&mut self, propertyid: MSCEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetAccountInformation(&mut self, bstrusername: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsMSCEPStoreEmpty(&mut self) -> ::windows::core::Result<i16>;
    fn GetProviderNameList(&mut self, bexchange: i16) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetKeyLengthList(&mut self, bexchange: i16, bstrprovidername: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Install(&mut self) -> ::windows::core::Result<()>;
    fn PreUnInstall(&mut self) -> ::windows::core::Result<()>;
    fn PostUnInstall(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSCEPSetup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetup_Impl, const OFFSET: isize>() -> IMSCEPSetup_Vtbl {
        unsafe extern "system" fn MSCEPErrorId<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MSCEPErrorId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSCEPErrorString<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MSCEPErrorString() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeDefaults<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeDefaults().into()
        }
        unsafe extern "system" fn GetMSCEPSetupProperty<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: MSCEPSetupProperty, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMSCEPSetupProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMSCEPSetupProperty<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: MSCEPSetupProperty, ppropertyvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMSCEPSetupProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn SetAccountInformation<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAccountInformation(::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrpassword)).into()
        }
        unsafe extern "system" fn IsMSCEPStoreEmpty<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbempty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsMSCEPStoreEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    *pbempty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderNameList<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bexchange: i16, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProviderNameList(::core::mem::transmute_copy(&bexchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyLengthList<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bexchange: i16, bstrprovidername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetKeyLengthList(::core::mem::transmute_copy(&bexchange), ::core::mem::transmute_copy(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Install().into()
        }
        unsafe extern "system" fn PreUnInstall<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PreUnInstall().into()
        }
        unsafe extern "system" fn PostUnInstall<Identity: ::windows::core::IUnknownImpl, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PostUnInstall().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            MSCEPErrorId: MSCEPErrorId::<Identity, Impl, OFFSET>,
            MSCEPErrorString: MSCEPErrorString::<Identity, Impl, OFFSET>,
            InitializeDefaults: InitializeDefaults::<Identity, Impl, OFFSET>,
            GetMSCEPSetupProperty: GetMSCEPSetupProperty::<Identity, Impl, OFFSET>,
            SetMSCEPSetupProperty: SetMSCEPSetupProperty::<Identity, Impl, OFFSET>,
            SetAccountInformation: SetAccountInformation::<Identity, Impl, OFFSET>,
            IsMSCEPStoreEmpty: IsMSCEPStoreEmpty::<Identity, Impl, OFFSET>,
            GetProviderNameList: GetProviderNameList::<Identity, Impl, OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            PreUnInstall: PreUnInstall::<Identity, Impl, OFFSET>,
            PostUnInstall: PostUnInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSCEPSetup as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
