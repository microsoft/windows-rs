#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAlternativeNameImpl: Sized + IDispatchImpl {
    fn InitializeFromString(&mut self, r#type: AlternativeNameType, strvalue: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeFromRawData(&mut self, r#type: AlternativeNameType, encoding: EncodingType, strrawdata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeFromOtherName(&mut self, pobjectid: ::core::option::Option<IObjectId>, encoding: EncodingType, strrawdata: super::super::super::Foundation::BSTR, tobewrapped: i16) -> ::windows::core::Result<()>;
    fn Type(&mut self) -> ::windows::core::Result<AlternativeNameType>;
    fn StrValue(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ObjectId(&mut self) -> ::windows::core::Result<IObjectId>;
    fn RawData(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAlternativeNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlternativeNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAlternativeNameVtbl {
        unsafe extern "system" fn InitializeFromString<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AlternativeNameType, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromString(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn InitializeFromRawData<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AlternativeNameType, encoding: EncodingType, strrawdata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromRawData(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strrawdata)).into()
        }
        unsafe extern "system" fn InitializeFromOtherName<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, encoding: EncodingType, strrawdata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, tobewrapped: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromOtherName(::core::mem::transmute(&pobjectid), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strrawdata), ::core::mem::transmute_copy(&tobewrapped)).into()
        }
        unsafe extern "system" fn Type<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut AlternativeNameType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrValue<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrValue() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectId<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawData<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawData(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromString: InitializeFromString::<Impl, IMPL_OFFSET>,
            InitializeFromRawData: InitializeFromRawData::<Impl, IMPL_OFFSET>,
            InitializeFromOtherName: InitializeFromOtherName::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            StrValue: StrValue::<Impl, IMPL_OFFSET>,
            ObjectId: ObjectId::<Impl, IMPL_OFFSET>,
            RawData: RawData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlternativeName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAlternativeNamesImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<IAlternativeName>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<IAlternativeName>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAlternativeNamesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlternativeNamesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAlternativeNamesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: IAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: IAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlternativeNames as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBinaryConverterImpl: Sized + IDispatchImpl {
    fn StringToString(&mut self, strencodedin: super::super::super::Foundation::BSTR, encodingin: EncodingType, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn VariantByteArrayToString(&mut self, pvarbytearray: *const super::super::super::System::Com::VARIANT, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn StringToVariantByteArray(&mut self, strencoded: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBinaryConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBinaryConverterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBinaryConverterVtbl {
        unsafe extern "system" fn StringToString<Impl: IBinaryConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodedin: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encodingin: EncodingType, encoding: EncodingType, pstrencoded: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StringToString(::core::mem::transmute_copy(&strencodedin), ::core::mem::transmute_copy(&encodingin), ::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrencoded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VariantByteArrayToString<Impl: IBinaryConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbytearray: *const super::super::super::System::Com::VARIANT, encoding: EncodingType, pstrencoded: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VariantByteArrayToString(::core::mem::transmute_copy(&pvarbytearray), ::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrencoded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StringToVariantByteArray<Impl: IBinaryConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencoded: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, pvarbytearray: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StringToVariantByteArray(::core::mem::transmute_copy(&strencoded), ::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbytearray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StringToString: StringToString::<Impl, IMPL_OFFSET>,
            VariantByteArrayToString: VariantByteArrayToString::<Impl, IMPL_OFFSET>,
            StringToVariantByteArray: StringToVariantByteArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBinaryConverter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBinaryConverter2Impl: Sized + IDispatchImpl + IBinaryConverterImpl {
    fn StringArrayToVariantArray(&mut self, pvarstringarray: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn VariantArrayToStringArray(&mut self, pvarvariantarray: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBinaryConverter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBinaryConverter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBinaryConverter2Vtbl {
        unsafe extern "system" fn StringArrayToVariantArray<Impl: IBinaryConverter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarstringarray: *const super::super::super::System::Com::VARIANT, pvarvariantarray: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StringArrayToVariantArray(::core::mem::transmute_copy(&pvarstringarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvariantarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VariantArrayToStringArray<Impl: IBinaryConverter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvariantarray: *const super::super::super::System::Com::VARIANT, pvarstringarray: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VariantArrayToStringArray(::core::mem::transmute_copy(&pvarvariantarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarstringarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IBinaryConverterVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StringArrayToVariantArray: StringArrayToVariantArray::<Impl, IMPL_OFFSET>,
            VariantArrayToStringArray: VariantArrayToStringArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBinaryConverter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICEnrollImpl: Sized + IDispatchImpl {
    fn createFilePKCS10(&mut self, dnname: super::super::super::Foundation::BSTR, usage: super::super::super::Foundation::BSTR, wszpkcs10filename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn acceptFilePKCS7(&mut self, wszpkcs7filename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn createPKCS10(&mut self, dnname: super::super::super::Foundation::BSTR, usage: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn acceptPKCS7(&mut self, pkcs7: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getCertFromPKCS7(&mut self, wszpkcs7: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn enumProviders(&mut self, dwindex: i32, dwflags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn enumContainers(&mut self, dwindex: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn freeRequestInfo(&mut self, pkcs7orpkcs10: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MyStoreName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetMyStoreName(&mut self, bstrname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MyStoreType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetMyStoreType(&mut self, bstrtype: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MyStoreFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetMyStoreFlags(&mut self, dwflags: i32) -> ::windows::core::Result<()>;
    fn CAStoreName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetCAStoreName(&mut self, bstrname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CAStoreType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetCAStoreType(&mut self, bstrtype: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CAStoreFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetCAStoreFlags(&mut self, dwflags: i32) -> ::windows::core::Result<()>;
    fn RootStoreName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetRootStoreName(&mut self, bstrname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RootStoreType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetRootStoreType(&mut self, bstrtype: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RootStoreFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetRootStoreFlags(&mut self, dwflags: i32) -> ::windows::core::Result<()>;
    fn RequestStoreName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetRequestStoreName(&mut self, bstrname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RequestStoreType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetRequestStoreType(&mut self, bstrtype: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RequestStoreFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetRequestStoreFlags(&mut self, dwflags: i32) -> ::windows::core::Result<()>;
    fn ContainerName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetContainerName(&mut self, bstrcontainer: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ProviderName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetProviderName(&mut self, bstrprovider: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ProviderType(&mut self) -> ::windows::core::Result<i32>;
    fn SetProviderType(&mut self, dwtype: i32) -> ::windows::core::Result<()>;
    fn KeySpec(&mut self) -> ::windows::core::Result<i32>;
    fn SetKeySpec(&mut self, dw: i32) -> ::windows::core::Result<()>;
    fn ProviderFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetProviderFlags(&mut self, dwflags: i32) -> ::windows::core::Result<()>;
    fn UseExistingKeySet(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetUseExistingKeySet(&mut self, fuseexistingkeys: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GenKeyFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetGenKeyFlags(&mut self, dwflags: i32) -> ::windows::core::Result<()>;
    fn DeleteRequestCert(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetDeleteRequestCert(&mut self, fdelete: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteCertToCSP(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetWriteCertToCSP(&mut self, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SPCFileName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetSPCFileName(&mut self, bstr: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PVKFileName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetPVKFileName(&mut self, bstr: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HashAlgorithm(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetHashAlgorithm(&mut self, bstr: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICEnrollVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICEnrollImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICEnrollVtbl {
        unsafe extern "system" fn createFilePKCS10<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usage: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, wszpkcs10filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).createFilePKCS10(::core::mem::transmute_copy(&dnname), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&wszpkcs10filename)).into()
        }
        unsafe extern "system" fn acceptFilePKCS7<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpkcs7filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).acceptFilePKCS7(::core::mem::transmute_copy(&wszpkcs7filename)).into()
        }
        unsafe extern "system" fn createPKCS10<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usage: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppkcs10: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createPKCS10(::core::mem::transmute_copy(&dnname), ::core::mem::transmute_copy(&usage)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppkcs10 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn acceptPKCS7<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkcs7: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).acceptPKCS7(::core::mem::transmute_copy(&pkcs7)).into()
        }
        unsafe extern "system" fn getCertFromPKCS7<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpkcs7: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrcert: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getCertFromPKCS7(::core::mem::transmute_copy(&wszpkcs7)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enumProviders<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, dwflags: i32, pbstrprovname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).enumProviders(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprovname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enumContainers<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).enumContainers(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn freeRequestInfo<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkcs7orpkcs10: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).freeRequestInfo(::core::mem::transmute_copy(&pkcs7orpkcs10)).into()
        }
        unsafe extern "system" fn MyStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MyStoreName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMyStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMyStoreName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn MyStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MyStoreType() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMyStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMyStoreType(::core::mem::transmute_copy(&bstrtype)).into()
        }
        unsafe extern "system" fn MyStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MyStoreFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMyStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMyStoreFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn CAStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CAStoreName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCAStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCAStoreName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn CAStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CAStoreType() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCAStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCAStoreType(::core::mem::transmute_copy(&bstrtype)).into()
        }
        unsafe extern "system" fn CAStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CAStoreFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCAStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCAStoreFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RootStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootStoreName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRootStoreName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn RootStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootStoreType() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRootStoreType(::core::mem::transmute_copy(&bstrtype)).into()
        }
        unsafe extern "system" fn RootStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootStoreFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRootStoreFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestStoreName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn RequestStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreType() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestStoreType(::core::mem::transmute_copy(&bstrtype)).into()
        }
        unsafe extern "system" fn RequestStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestStoreFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ContainerName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontainer: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainerName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcontainer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontainer: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContainerName(::core::mem::transmute_copy(&bstrcontainer)).into()
        }
        unsafe extern "system" fn ProviderName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprovider: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovider: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderName(::core::mem::transmute_copy(&bstrprovider)).into()
        }
        unsafe extern "system" fn ProviderType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderType(::core::mem::transmute_copy(&dwtype)).into()
        }
        unsafe extern "system" fn KeySpec<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdw: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeySpec() {
                ::core::result::Result::Ok(ok__) => {
                    *pdw = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeySpec<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dw: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeySpec(::core::mem::transmute_copy(&dw)).into()
        }
        unsafe extern "system" fn ProviderFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn UseExistingKeySet<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuseexistingkeys: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseExistingKeySet() {
                ::core::result::Result::Ok(ok__) => {
                    *fuseexistingkeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseExistingKeySet<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuseexistingkeys: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseExistingKeySet(::core::mem::transmute_copy(&fuseexistingkeys)).into()
        }
        unsafe extern "system" fn GenKeyFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenKeyFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGenKeyFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGenKeyFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeleteRequestCert<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdelete: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteRequestCert() {
                ::core::result::Result::Ok(ok__) => {
                    *fdelete = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeleteRequestCert<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdelete: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeleteRequestCert(::core::mem::transmute_copy(&fdelete)).into()
        }
        unsafe extern "system" fn WriteCertToCSP<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteCertToCSP() {
                ::core::result::Result::Ok(ok__) => {
                    *fbool = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteCertToCSP<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteCertToCSP(::core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn SPCFileName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SPCFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSPCFileName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSPCFileName(::core::mem::transmute_copy(&bstr)).into()
        }
        unsafe extern "system" fn PVKFileName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PVKFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPVKFileName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPVKFileName(::core::mem::transmute_copy(&bstr)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&bstr)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            createFilePKCS10: createFilePKCS10::<Impl, IMPL_OFFSET>,
            acceptFilePKCS7: acceptFilePKCS7::<Impl, IMPL_OFFSET>,
            createPKCS10: createPKCS10::<Impl, IMPL_OFFSET>,
            acceptPKCS7: acceptPKCS7::<Impl, IMPL_OFFSET>,
            getCertFromPKCS7: getCertFromPKCS7::<Impl, IMPL_OFFSET>,
            enumProviders: enumProviders::<Impl, IMPL_OFFSET>,
            enumContainers: enumContainers::<Impl, IMPL_OFFSET>,
            freeRequestInfo: freeRequestInfo::<Impl, IMPL_OFFSET>,
            MyStoreName: MyStoreName::<Impl, IMPL_OFFSET>,
            SetMyStoreName: SetMyStoreName::<Impl, IMPL_OFFSET>,
            MyStoreType: MyStoreType::<Impl, IMPL_OFFSET>,
            SetMyStoreType: SetMyStoreType::<Impl, IMPL_OFFSET>,
            MyStoreFlags: MyStoreFlags::<Impl, IMPL_OFFSET>,
            SetMyStoreFlags: SetMyStoreFlags::<Impl, IMPL_OFFSET>,
            CAStoreName: CAStoreName::<Impl, IMPL_OFFSET>,
            SetCAStoreName: SetCAStoreName::<Impl, IMPL_OFFSET>,
            CAStoreType: CAStoreType::<Impl, IMPL_OFFSET>,
            SetCAStoreType: SetCAStoreType::<Impl, IMPL_OFFSET>,
            CAStoreFlags: CAStoreFlags::<Impl, IMPL_OFFSET>,
            SetCAStoreFlags: SetCAStoreFlags::<Impl, IMPL_OFFSET>,
            RootStoreName: RootStoreName::<Impl, IMPL_OFFSET>,
            SetRootStoreName: SetRootStoreName::<Impl, IMPL_OFFSET>,
            RootStoreType: RootStoreType::<Impl, IMPL_OFFSET>,
            SetRootStoreType: SetRootStoreType::<Impl, IMPL_OFFSET>,
            RootStoreFlags: RootStoreFlags::<Impl, IMPL_OFFSET>,
            SetRootStoreFlags: SetRootStoreFlags::<Impl, IMPL_OFFSET>,
            RequestStoreName: RequestStoreName::<Impl, IMPL_OFFSET>,
            SetRequestStoreName: SetRequestStoreName::<Impl, IMPL_OFFSET>,
            RequestStoreType: RequestStoreType::<Impl, IMPL_OFFSET>,
            SetRequestStoreType: SetRequestStoreType::<Impl, IMPL_OFFSET>,
            RequestStoreFlags: RequestStoreFlags::<Impl, IMPL_OFFSET>,
            SetRequestStoreFlags: SetRequestStoreFlags::<Impl, IMPL_OFFSET>,
            ContainerName: ContainerName::<Impl, IMPL_OFFSET>,
            SetContainerName: SetContainerName::<Impl, IMPL_OFFSET>,
            ProviderName: ProviderName::<Impl, IMPL_OFFSET>,
            SetProviderName: SetProviderName::<Impl, IMPL_OFFSET>,
            ProviderType: ProviderType::<Impl, IMPL_OFFSET>,
            SetProviderType: SetProviderType::<Impl, IMPL_OFFSET>,
            KeySpec: KeySpec::<Impl, IMPL_OFFSET>,
            SetKeySpec: SetKeySpec::<Impl, IMPL_OFFSET>,
            ProviderFlags: ProviderFlags::<Impl, IMPL_OFFSET>,
            SetProviderFlags: SetProviderFlags::<Impl, IMPL_OFFSET>,
            UseExistingKeySet: UseExistingKeySet::<Impl, IMPL_OFFSET>,
            SetUseExistingKeySet: SetUseExistingKeySet::<Impl, IMPL_OFFSET>,
            GenKeyFlags: GenKeyFlags::<Impl, IMPL_OFFSET>,
            SetGenKeyFlags: SetGenKeyFlags::<Impl, IMPL_OFFSET>,
            DeleteRequestCert: DeleteRequestCert::<Impl, IMPL_OFFSET>,
            SetDeleteRequestCert: SetDeleteRequestCert::<Impl, IMPL_OFFSET>,
            WriteCertToCSP: WriteCertToCSP::<Impl, IMPL_OFFSET>,
            SetWriteCertToCSP: SetWriteCertToCSP::<Impl, IMPL_OFFSET>,
            SPCFileName: SPCFileName::<Impl, IMPL_OFFSET>,
            SetSPCFileName: SetSPCFileName::<Impl, IMPL_OFFSET>,
            PVKFileName: PVKFileName::<Impl, IMPL_OFFSET>,
            SetPVKFileName: SetPVKFileName::<Impl, IMPL_OFFSET>,
            HashAlgorithm: HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICEnroll as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICEnroll2Impl: Sized + IDispatchImpl + ICEnrollImpl {
    fn addCertTypeToRequest(&mut self, certtype: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn addNameValuePairToSignature(&mut self, name: super::super::super::Foundation::BSTR, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn WriteCertToUserDS(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetWriteCertToUserDS(&mut self, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnableT61DNEncoding(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetEnableT61DNEncoding(&mut self, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICEnroll2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICEnroll2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICEnroll2Vtbl {
        unsafe extern "system" fn addCertTypeToRequest<Impl: ICEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addCertTypeToRequest(::core::mem::transmute_copy(&certtype)).into()
        }
        unsafe extern "system" fn addNameValuePairToSignature<Impl: ICEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addNameValuePairToSignature(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn WriteCertToUserDS<Impl: ICEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteCertToUserDS() {
                ::core::result::Result::Ok(ok__) => {
                    *fbool = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteCertToUserDS<Impl: ICEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteCertToUserDS(::core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn EnableT61DNEncoding<Impl: ICEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableT61DNEncoding() {
                ::core::result::Result::Ok(ok__) => {
                    *fbool = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableT61DNEncoding<Impl: ICEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableT61DNEncoding(::core::mem::transmute_copy(&fbool)).into()
        }
        Self {
            base: ICEnrollVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            addCertTypeToRequest: addCertTypeToRequest::<Impl, IMPL_OFFSET>,
            addNameValuePairToSignature: addNameValuePairToSignature::<Impl, IMPL_OFFSET>,
            WriteCertToUserDS: WriteCertToUserDS::<Impl, IMPL_OFFSET>,
            SetWriteCertToUserDS: SetWriteCertToUserDS::<Impl, IMPL_OFFSET>,
            EnableT61DNEncoding: EnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            SetEnableT61DNEncoding: SetEnableT61DNEncoding::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICEnroll2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICEnroll3Impl: Sized + IDispatchImpl + ICEnrollImpl + ICEnroll2Impl {
    fn InstallPKCS7(&mut self, pkcs7: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn GetSupportedKeySpec(&mut self) -> ::windows::core::Result<i32>;
    fn GetKeyLen(&mut self, fmin: super::super::super::Foundation::BOOL, fexchange: super::super::super::Foundation::BOOL) -> ::windows::core::Result<i32>;
    fn EnumAlgs(&mut self, dwindex: i32, algclass: i32) -> ::windows::core::Result<i32>;
    fn GetAlgName(&mut self, algid: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetReuseHardwareKeyIfUnableToGenNew(&mut self, freusehardwarekeyifunabletogennew: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ReuseHardwareKeyIfUnableToGenNew(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetHashAlgID(&mut self, hashalgid: i32) -> ::windows::core::Result<()>;
    fn HashAlgID(&mut self) -> ::windows::core::Result<i32>;
    fn SetLimitExchangeKeyToEncipherment(&mut self, flimitexchangekeytoencipherment: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn LimitExchangeKeyToEncipherment(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetEnableSMIMECapabilities(&mut self, fenablesmimecapabilities: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnableSMIMECapabilities(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICEnroll3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICEnroll3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICEnroll3Vtbl {
        unsafe extern "system" fn InstallPKCS7<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkcs7: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallPKCS7(::core::mem::transmute_copy(&pkcs7)).into()
        }
        unsafe extern "system" fn Reset<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn GetSupportedKeySpec<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwkeyspec: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedKeySpec() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwkeyspec = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyLen<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmin: super::super::super::Foundation::BOOL, fexchange: super::super::super::Foundation::BOOL, pdwkeysize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeyLen(::core::mem::transmute_copy(&fmin), ::core::mem::transmute_copy(&fexchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwkeysize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAlgs<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, algclass: i32, pdwalgid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAlgs(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&algclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwalgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlgName<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, algid: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlgName(::core::mem::transmute_copy(&algid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReuseHardwareKeyIfUnableToGenNew<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freusehardwarekeyifunabletogennew: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReuseHardwareKeyIfUnableToGenNew(::core::mem::transmute_copy(&freusehardwarekeyifunabletogennew)).into()
        }
        unsafe extern "system" fn ReuseHardwareKeyIfUnableToGenNew<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freusehardwarekeyifunabletogennew: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReuseHardwareKeyIfUnableToGenNew() {
                ::core::result::Result::Ok(ok__) => {
                    *freusehardwarekeyifunabletogennew = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgID<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashalgid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgID(::core::mem::transmute_copy(&hashalgid)).into()
        }
        unsafe extern "system" fn HashAlgID<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashalgid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgID() {
                ::core::result::Result::Ok(ok__) => {
                    *hashalgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLimitExchangeKeyToEncipherment<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flimitexchangekeytoencipherment: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLimitExchangeKeyToEncipherment(::core::mem::transmute_copy(&flimitexchangekeytoencipherment)).into()
        }
        unsafe extern "system" fn LimitExchangeKeyToEncipherment<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flimitexchangekeytoencipherment: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LimitExchangeKeyToEncipherment() {
                ::core::result::Result::Ok(ok__) => {
                    *flimitexchangekeytoencipherment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableSMIMECapabilities<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablesmimecapabilities: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableSMIMECapabilities(::core::mem::transmute_copy(&fenablesmimecapabilities)).into()
        }
        unsafe extern "system" fn EnableSMIMECapabilities<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablesmimecapabilities: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableSMIMECapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *fenablesmimecapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICEnroll2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InstallPKCS7: InstallPKCS7::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            GetSupportedKeySpec: GetSupportedKeySpec::<Impl, IMPL_OFFSET>,
            GetKeyLen: GetKeyLen::<Impl, IMPL_OFFSET>,
            EnumAlgs: EnumAlgs::<Impl, IMPL_OFFSET>,
            GetAlgName: GetAlgName::<Impl, IMPL_OFFSET>,
            SetReuseHardwareKeyIfUnableToGenNew: SetReuseHardwareKeyIfUnableToGenNew::<Impl, IMPL_OFFSET>,
            ReuseHardwareKeyIfUnableToGenNew: ReuseHardwareKeyIfUnableToGenNew::<Impl, IMPL_OFFSET>,
            SetHashAlgID: SetHashAlgID::<Impl, IMPL_OFFSET>,
            HashAlgID: HashAlgID::<Impl, IMPL_OFFSET>,
            SetLimitExchangeKeyToEncipherment: SetLimitExchangeKeyToEncipherment::<Impl, IMPL_OFFSET>,
            LimitExchangeKeyToEncipherment: LimitExchangeKeyToEncipherment::<Impl, IMPL_OFFSET>,
            SetEnableSMIMECapabilities: SetEnableSMIMECapabilities::<Impl, IMPL_OFFSET>,
            EnableSMIMECapabilities: EnableSMIMECapabilities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICEnroll3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICEnroll4Impl: Sized + IDispatchImpl + ICEnrollImpl + ICEnroll2Impl + ICEnroll3Impl {
    fn SetPrivateKeyArchiveCertificate(&mut self, bstrcert: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PrivateKeyArchiveCertificate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetThumbPrint(&mut self, bstrthumbprint: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ThumbPrint(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn binaryToString(&mut self, flags: i32, strbinary: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn stringToBinary(&mut self, flags: i32, strencoded: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn addExtensionToRequest(&mut self, flags: i32, strname: super::super::super::Foundation::BSTR, strvalue: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn addAttributeToRequest(&mut self, flags: i32, strname: super::super::super::Foundation::BSTR, strvalue: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn addNameValuePairToRequest(&mut self, flags: i32, strname: super::super::super::Foundation::BSTR, strvalue: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn resetExtensions(&mut self) -> ::windows::core::Result<()>;
    fn resetAttributes(&mut self) -> ::windows::core::Result<()>;
    fn createRequest(&mut self, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: super::super::super::Foundation::BSTR, usage: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn createFileRequest(&mut self, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: super::super::super::Foundation::BSTR, strusage: super::super::super::Foundation::BSTR, strrequestfilename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn acceptResponse(&mut self, strresponse: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn acceptFileResponse(&mut self, strresponsefilename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getCertFromResponse(&mut self, strresponse: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getCertFromFileResponse(&mut self, strresponsefilename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn createPFX(&mut self, strpassword: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn createFilePFX(&mut self, strpassword: super::super::super::Foundation::BSTR, strpfxfilename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setPendingRequestInfo(&mut self, lrequestid: i32, strcadns: super::super::super::Foundation::BSTR, strcaname: super::super::super::Foundation::BSTR, strfriendlyname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn enumPendingRequest(&mut self, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn removePendingRequest(&mut self, strthumbprint: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetKeyLenEx(&mut self, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC) -> ::windows::core::Result<i32>;
    fn InstallPKCS7Ex(&mut self, pkcs7: super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn addCertTypeToRequestEx(&mut self, ltype: ADDED_CERT_TYPE, bstroidorname: super::super::super::Foundation::BSTR, lmajorversion: i32, fminorversion: super::super::super::Foundation::BOOL, lminorversion: i32) -> ::windows::core::Result<()>;
    fn getProviderType(&mut self, strprovname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn SetSignerCertificate(&mut self, bstrcert: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetClientId(&mut self, lclientid: i32) -> ::windows::core::Result<()>;
    fn ClientId(&mut self) -> ::windows::core::Result<i32>;
    fn addBlobPropertyToCertificate(&mut self, lpropertyid: i32, lreserved: i32, bstrproperty: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn resetBlobProperties(&mut self) -> ::windows::core::Result<()>;
    fn SetIncludeSubjectKeyID(&mut self, finclude: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IncludeSubjectKeyID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICEnroll4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICEnroll4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICEnroll4Vtbl {
        unsafe extern "system" fn SetPrivateKeyArchiveCertificate<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcert: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateKeyArchiveCertificate(::core::mem::transmute_copy(&bstrcert)).into()
        }
        unsafe extern "system" fn PrivateKeyArchiveCertificate<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcert: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateKeyArchiveCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbPrint<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrthumbprint: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbPrint(::core::mem::transmute_copy(&bstrthumbprint)).into()
        }
        unsafe extern "system" fn ThumbPrint<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrthumbprint: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThumbPrint() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrthumbprint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn binaryToString<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrencoded: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).binaryToString(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&strbinary)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrencoded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stringToBinary<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, strencoded: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).stringToBinary(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&strencoded)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrbinary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addExtensionToRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addExtensionToRequest(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn addAttributeToRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addAttributeToRequest(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn addNameValuePairToRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addNameValuePairToRequest(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn resetExtensions<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).resetExtensions().into()
        }
        unsafe extern "system" fn resetAttributes<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).resetAttributes().into()
        }
        unsafe extern "system" fn createRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usage: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrrequest: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createRequest(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&strdnname), ::core::mem::transmute_copy(&usage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrrequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createFileRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strusage: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strrequestfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).createFileRequest(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&strdnname), ::core::mem::transmute_copy(&strusage), ::core::mem::transmute_copy(&strrequestfilename)).into()
        }
        unsafe extern "system" fn acceptResponse<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).acceptResponse(::core::mem::transmute_copy(&strresponse)).into()
        }
        unsafe extern "system" fn acceptFileResponse<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresponsefilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).acceptFileResponse(::core::mem::transmute_copy(&strresponsefilename)).into()
        }
        unsafe extern "system" fn getCertFromResponse<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrcert: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getCertFromResponse(::core::mem::transmute_copy(&strresponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrcert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getCertFromFileResponse<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresponsefilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrcert: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getCertFromFileResponse(::core::mem::transmute_copy(&strresponsefilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrcert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createPFX<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrpfx: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createPFX(::core::mem::transmute_copy(&strpassword)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrpfx = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createFilePFX<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpfxfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).createFilePFX(::core::mem::transmute_copy(&strpassword), ::core::mem::transmute_copy(&strpfxfilename)).into()
        }
        unsafe extern "system" fn setPendingRequestInfo<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrequestid: i32, strcadns: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strcaname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strfriendlyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setPendingRequestInfo(::core::mem::transmute_copy(&lrequestid), ::core::mem::transmute_copy(&strcadns), ::core::mem::transmute_copy(&strcaname), ::core::mem::transmute_copy(&strfriendlyname)).into()
        }
        unsafe extern "system" fn enumPendingRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY, pvarproperty: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).enumPendingRequest(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&ldesiredproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removePendingRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strthumbprint: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).removePendingRequest(::core::mem::transmute_copy(&strthumbprint)).into()
        }
        unsafe extern "system" fn GetKeyLenEx<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC, pdwkeysize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeyLenEx(::core::mem::transmute_copy(&lsizespec), ::core::mem::transmute_copy(&lkeyspec)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwkeysize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallPKCS7Ex<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkcs7: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, plcertinstalled: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallPKCS7Ex(::core::mem::transmute_copy(&pkcs7)) {
                ::core::result::Result::Ok(ok__) => {
                    *plcertinstalled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addCertTypeToRequestEx<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltype: ADDED_CERT_TYPE, bstroidorname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, lmajorversion: i32, fminorversion: super::super::super::Foundation::BOOL, lminorversion: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addCertTypeToRequestEx(::core::mem::transmute_copy(&ltype), ::core::mem::transmute_copy(&bstroidorname), ::core::mem::transmute_copy(&lmajorversion), ::core::mem::transmute_copy(&fminorversion), ::core::mem::transmute_copy(&lminorversion)).into()
        }
        unsafe extern "system" fn getProviderType<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprovname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, plprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getProviderType(::core::mem::transmute_copy(&strprovname)) {
                ::core::result::Result::Ok(ok__) => {
                    *plprovtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignerCertificate<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcert: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignerCertificate(::core::mem::transmute_copy(&bstrcert)).into()
        }
        unsafe extern "system" fn SetClientId<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lclientid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientId(::core::mem::transmute_copy(&lclientid)).into()
        }
        unsafe extern "system" fn ClientId<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclientid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientId() {
                ::core::result::Result::Ok(ok__) => {
                    *plclientid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addBlobPropertyToCertificate<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropertyid: i32, lreserved: i32, bstrproperty: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addBlobPropertyToCertificate(::core::mem::transmute_copy(&lpropertyid), ::core::mem::transmute_copy(&lreserved), ::core::mem::transmute_copy(&bstrproperty)).into()
        }
        unsafe extern "system" fn resetBlobProperties<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).resetBlobProperties().into()
        }
        unsafe extern "system" fn SetIncludeSubjectKeyID<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finclude: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeSubjectKeyID(::core::mem::transmute_copy(&finclude)).into()
        }
        unsafe extern "system" fn IncludeSubjectKeyID<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinclude: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeSubjectKeyID() {
                ::core::result::Result::Ok(ok__) => {
                    *pfinclude = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICEnroll3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetPrivateKeyArchiveCertificate: SetPrivateKeyArchiveCertificate::<Impl, IMPL_OFFSET>,
            PrivateKeyArchiveCertificate: PrivateKeyArchiveCertificate::<Impl, IMPL_OFFSET>,
            SetThumbPrint: SetThumbPrint::<Impl, IMPL_OFFSET>,
            ThumbPrint: ThumbPrint::<Impl, IMPL_OFFSET>,
            binaryToString: binaryToString::<Impl, IMPL_OFFSET>,
            stringToBinary: stringToBinary::<Impl, IMPL_OFFSET>,
            addExtensionToRequest: addExtensionToRequest::<Impl, IMPL_OFFSET>,
            addAttributeToRequest: addAttributeToRequest::<Impl, IMPL_OFFSET>,
            addNameValuePairToRequest: addNameValuePairToRequest::<Impl, IMPL_OFFSET>,
            resetExtensions: resetExtensions::<Impl, IMPL_OFFSET>,
            resetAttributes: resetAttributes::<Impl, IMPL_OFFSET>,
            createRequest: createRequest::<Impl, IMPL_OFFSET>,
            createFileRequest: createFileRequest::<Impl, IMPL_OFFSET>,
            acceptResponse: acceptResponse::<Impl, IMPL_OFFSET>,
            acceptFileResponse: acceptFileResponse::<Impl, IMPL_OFFSET>,
            getCertFromResponse: getCertFromResponse::<Impl, IMPL_OFFSET>,
            getCertFromFileResponse: getCertFromFileResponse::<Impl, IMPL_OFFSET>,
            createPFX: createPFX::<Impl, IMPL_OFFSET>,
            createFilePFX: createFilePFX::<Impl, IMPL_OFFSET>,
            setPendingRequestInfo: setPendingRequestInfo::<Impl, IMPL_OFFSET>,
            enumPendingRequest: enumPendingRequest::<Impl, IMPL_OFFSET>,
            removePendingRequest: removePendingRequest::<Impl, IMPL_OFFSET>,
            GetKeyLenEx: GetKeyLenEx::<Impl, IMPL_OFFSET>,
            InstallPKCS7Ex: InstallPKCS7Ex::<Impl, IMPL_OFFSET>,
            addCertTypeToRequestEx: addCertTypeToRequestEx::<Impl, IMPL_OFFSET>,
            getProviderType: getProviderType::<Impl, IMPL_OFFSET>,
            SetSignerCertificate: SetSignerCertificate::<Impl, IMPL_OFFSET>,
            SetClientId: SetClientId::<Impl, IMPL_OFFSET>,
            ClientId: ClientId::<Impl, IMPL_OFFSET>,
            addBlobPropertyToCertificate: addBlobPropertyToCertificate::<Impl, IMPL_OFFSET>,
            resetBlobProperties: resetBlobProperties::<Impl, IMPL_OFFSET>,
            SetIncludeSubjectKeyID: SetIncludeSubjectKeyID::<Impl, IMPL_OFFSET>,
            IncludeSubjectKeyID: IncludeSubjectKeyID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICEnroll4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertAdminImpl: Sized + IDispatchImpl {
    fn IsValidCertificate(&mut self, strconfig: super::super::super::Foundation::BSTR, strserialnumber: super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn GetRevocationReason(&mut self) -> ::windows::core::Result<i32>;
    fn RevokeCertificate(&mut self, strconfig: super::super::super::Foundation::BSTR, strserialnumber: super::super::super::Foundation::BSTR, reason: i32, date: f64) -> ::windows::core::Result<()>;
    fn SetRequestAttributes(&mut self, strconfig: super::super::super::Foundation::BSTR, requestid: i32, strattributes: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetCertificateExtension(&mut self, strconfig: super::super::super::Foundation::BSTR, requestid: i32, strextensionname: super::super::super::Foundation::BSTR, r#type: CERT_PROPERTY_TYPE, flags: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DenyRequest(&mut self, strconfig: super::super::super::Foundation::BSTR, requestid: i32) -> ::windows::core::Result<()>;
    fn ResubmitRequest(&mut self, strconfig: super::super::super::Foundation::BSTR, requestid: i32) -> ::windows::core::Result<i32>;
    fn PublishCRL(&mut self, strconfig: super::super::super::Foundation::BSTR, date: f64) -> ::windows::core::Result<()>;
    fn GetCRL(&mut self, strconfig: super::super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ImportCertificate(&mut self, strconfig: super::super::super::Foundation::BSTR, strcertificate: super::super::super::Foundation::BSTR, flags: CERT_IMPORT_FLAGS) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertAdminVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertAdminImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertAdminVtbl {
        unsafe extern "system" fn IsValidCertificate<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsValidCertificate(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&strserialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdisposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRevocationReason<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preason: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRevocationReason() {
                ::core::result::Result::Ok(ok__) => {
                    *preason = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeCertificate<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, reason: i32, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RevokeCertificate(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&strserialnumber), ::core::mem::transmute_copy(&reason), ::core::mem::transmute_copy(&date)).into()
        }
        unsafe extern "system" fn SetRequestAttributes<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, strattributes: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestAttributes(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&strattributes)).into()
        }
        unsafe extern "system" fn SetCertificateExtension<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, strextensionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: CERT_PROPERTY_TYPE, flags: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCertificateExtension(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&strextensionname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn DenyRequest<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DenyRequest(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn ResubmitRequest<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, pdisposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResubmitRequest(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdisposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublishCRL<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PublishCRL(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&date)).into()
        }
        unsafe extern "system" fn GetCRL<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32, pstrcrl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCRL(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrcrl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportCertificate<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: CERT_IMPORT_FLAGS, prequestid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportCertificate(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&strcertificate), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *prequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsValidCertificate: IsValidCertificate::<Impl, IMPL_OFFSET>,
            GetRevocationReason: GetRevocationReason::<Impl, IMPL_OFFSET>,
            RevokeCertificate: RevokeCertificate::<Impl, IMPL_OFFSET>,
            SetRequestAttributes: SetRequestAttributes::<Impl, IMPL_OFFSET>,
            SetCertificateExtension: SetCertificateExtension::<Impl, IMPL_OFFSET>,
            DenyRequest: DenyRequest::<Impl, IMPL_OFFSET>,
            ResubmitRequest: ResubmitRequest::<Impl, IMPL_OFFSET>,
            PublishCRL: PublishCRL::<Impl, IMPL_OFFSET>,
            GetCRL: GetCRL::<Impl, IMPL_OFFSET>,
            ImportCertificate: ImportCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertAdmin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertAdmin2Impl: Sized + IDispatchImpl + ICertAdminImpl {
    fn PublishCRLs(&mut self, strconfig: super::super::super::Foundation::BSTR, date: f64, crlflags: i32) -> ::windows::core::Result<()>;
    fn GetCAProperty(&mut self, strconfig: super::super::super::Foundation::BSTR, propid: i32, propindex: i32, proptype: i32, flags: i32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn SetCAProperty(&mut self, strconfig: super::super::super::Foundation::BSTR, propid: i32, propindex: i32, proptype: CERT_PROPERTY_TYPE, pvarpropertyvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetCAPropertyFlags(&mut self, strconfig: super::super::super::Foundation::BSTR, propid: i32) -> ::windows::core::Result<i32>;
    fn GetCAPropertyDisplayName(&mut self, strconfig: super::super::super::Foundation::BSTR, propid: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetArchivedKey(&mut self, strconfig: super::super::super::Foundation::BSTR, requestid: i32, flags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetConfigEntry(&mut self, strconfig: super::super::super::Foundation::BSTR, strnodepath: super::super::super::Foundation::BSTR, strentryname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn SetConfigEntry(&mut self, strconfig: super::super::super::Foundation::BSTR, strnodepath: super::super::super::Foundation::BSTR, strentryname: super::super::super::Foundation::BSTR, pvarentry: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ImportKey(&mut self, strconfig: super::super::super::Foundation::BSTR, requestid: i32, strcerthash: super::super::super::Foundation::BSTR, flags: CERT_IMPORT_FLAGS, strkey: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetMyRoles(&mut self, strconfig: super::super::super::Foundation::BSTR) -> ::windows::core::Result<CERTADMIN_GET_ROLES_FLAGS>;
    fn DeleteRow(&mut self, strconfig: super::super::super::Foundation::BSTR, flags: CERT_DELETE_ROW_FLAGS, date: f64, table: CVRC_TABLE, rowid: i32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertAdmin2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertAdmin2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertAdmin2Vtbl {
        unsafe extern "system" fn PublishCRLs<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, date: f64, crlflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PublishCRLs(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&date), ::core::mem::transmute_copy(&crlflags)).into()
        }
        unsafe extern "system" fn GetCAProperty<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, propindex: i32, proptype: i32, flags: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCAProperty(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&propindex), ::core::mem::transmute_copy(&proptype), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarpropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCAProperty<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, propindex: i32, proptype: CERT_PROPERTY_TYPE, pvarpropertyvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCAProperty(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&propindex), ::core::mem::transmute_copy(&proptype), ::core::mem::transmute_copy(&pvarpropertyvalue)).into()
        }
        unsafe extern "system" fn GetCAPropertyFlags<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, ppropflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCAPropertyFlags(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&propid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAPropertyDisplayName<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, pstrdisplayname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCAPropertyDisplayName(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&propid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArchivedKey<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, flags: i32, pstrarchivedkey: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetArchivedKey(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrarchivedkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfigEntry<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strnodepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strentryname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvarentry: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConfigEntry(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&strnodepath), ::core::mem::transmute_copy(&strentryname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfigEntry<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strnodepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strentryname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvarentry: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConfigEntry(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&strnodepath), ::core::mem::transmute_copy(&strentryname), ::core::mem::transmute_copy(&pvarentry)).into()
        }
        unsafe extern "system" fn ImportKey<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, strcerthash: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: CERT_IMPORT_FLAGS, strkey: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImportKey(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&strcerthash), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&strkey)).into()
        }
        unsafe extern "system" fn GetMyRoles<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, proles: *mut CERTADMIN_GET_ROLES_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMyRoles(::core::mem::transmute_copy(&strconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *proles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRow<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: CERT_DELETE_ROW_FLAGS, date: f64, table: CVRC_TABLE, rowid: i32, pcdeleted: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteRow(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&date), ::core::mem::transmute_copy(&table), ::core::mem::transmute_copy(&rowid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcdeleted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertAdminVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PublishCRLs: PublishCRLs::<Impl, IMPL_OFFSET>,
            GetCAProperty: GetCAProperty::<Impl, IMPL_OFFSET>,
            SetCAProperty: SetCAProperty::<Impl, IMPL_OFFSET>,
            GetCAPropertyFlags: GetCAPropertyFlags::<Impl, IMPL_OFFSET>,
            GetCAPropertyDisplayName: GetCAPropertyDisplayName::<Impl, IMPL_OFFSET>,
            GetArchivedKey: GetArchivedKey::<Impl, IMPL_OFFSET>,
            GetConfigEntry: GetConfigEntry::<Impl, IMPL_OFFSET>,
            SetConfigEntry: SetConfigEntry::<Impl, IMPL_OFFSET>,
            ImportKey: ImportKey::<Impl, IMPL_OFFSET>,
            GetMyRoles: GetMyRoles::<Impl, IMPL_OFFSET>,
            DeleteRow: DeleteRow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertAdmin2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertConfigImpl: Sized + IDispatchImpl {
    fn Reset(&mut self, index: i32) -> ::windows::core::Result<i32>;
    fn Next(&mut self) -> ::windows::core::Result<i32>;
    fn GetField(&mut self, strfieldname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetConfig(&mut self, flags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertConfigVtbl {
        unsafe extern "system" fn Reset<Impl: ICertConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: ICertConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *pindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetField<Impl: ICertConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strfieldname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetField(::core::mem::transmute_copy(&strfieldname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfig<Impl: ICertConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConfig(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            GetField: GetField::<Impl, IMPL_OFFSET>,
            GetConfig: GetConfig::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertConfig2Impl: Sized + IDispatchImpl + ICertConfigImpl {
    fn SetSharedFolder(&mut self, strsharedfolder: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertConfig2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertConfig2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertConfig2Vtbl {
        unsafe extern "system" fn SetSharedFolder<Impl: ICertConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsharedfolder: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharedFolder(::core::mem::transmute_copy(&strsharedfolder)).into()
        }
        Self { base: ICertConfigVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetSharedFolder: SetSharedFolder::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertConfig2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeAltNameImpl: Sized + IDispatchImpl {
    fn Decode(&mut self, strbinary: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetNameCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetNameChoice(&mut self, nameindex: i32) -> ::windows::core::Result<i32>;
    fn GetName(&mut self, nameindex: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Reset(&mut self, namecount: i32) -> ::windows::core::Result<()>;
    fn SetNameEntry(&mut self, nameindex: i32, namechoice: CERT_ALT_NAME, strname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Encode(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeAltNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeAltNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeAltNameVtbl {
        unsafe extern "system" fn Decode<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Decode(::core::mem::transmute_copy(&strbinary)).into()
        }
        unsafe extern "system" fn GetNameCount<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pnamecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameChoice<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nameindex: i32, pnamechoice: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameChoice(::core::mem::transmute_copy(&nameindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnamechoice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nameindex: i32, pstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&nameindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namecount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&namecount)).into()
        }
        unsafe extern "system" fn SetNameEntry<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nameindex: i32, namechoice: CERT_ALT_NAME, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNameEntry(::core::mem::transmute_copy(&nameindex), ::core::mem::transmute_copy(&namechoice), ::core::mem::transmute_copy(&strname)).into()
        }
        unsafe extern "system" fn Encode<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Encode() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrbinary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Decode: Decode::<Impl, IMPL_OFFSET>,
            GetNameCount: GetNameCount::<Impl, IMPL_OFFSET>,
            GetNameChoice: GetNameChoice::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            SetNameEntry: SetNameEntry::<Impl, IMPL_OFFSET>,
            Encode: Encode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeAltName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeAltName2Impl: Sized + IDispatchImpl + ICertEncodeAltNameImpl {
    fn DecodeBlob(&mut self, strencodeddata: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
    fn EncodeBlob(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetNameBlob(&mut self, nameindex: i32, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetNameEntryBlob(&mut self, nameindex: i32, namechoice: i32, strname: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeAltName2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeAltName2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeAltName2Vtbl {
        unsafe extern "system" fn DecodeBlob<Impl: ICertEncodeAltName2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecodeBlob(::core::mem::transmute_copy(&strencodeddata), ::core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn EncodeBlob<Impl: ICertEncodeAltName2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodeBlob(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrencodeddata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameBlob<Impl: ICertEncodeAltName2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nameindex: i32, encoding: EncodingType, pstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameBlob(::core::mem::transmute_copy(&nameindex), ::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNameEntryBlob<Impl: ICertEncodeAltName2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nameindex: i32, namechoice: i32, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNameEntryBlob(::core::mem::transmute_copy(&nameindex), ::core::mem::transmute_copy(&namechoice), ::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&encoding)).into()
        }
        Self {
            base: ICertEncodeAltNameVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DecodeBlob: DecodeBlob::<Impl, IMPL_OFFSET>,
            EncodeBlob: EncodeBlob::<Impl, IMPL_OFFSET>,
            GetNameBlob: GetNameBlob::<Impl, IMPL_OFFSET>,
            SetNameEntryBlob: SetNameEntryBlob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeAltName2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeBitStringImpl: Sized + IDispatchImpl {
    fn Decode(&mut self, strbinary: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetBitCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetBitString(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Encode(&mut self, bitcount: i32, strbitstring: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeBitStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeBitStringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeBitStringVtbl {
        unsafe extern "system" fn Decode<Impl: ICertEncodeBitStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Decode(::core::mem::transmute_copy(&strbinary)).into()
        }
        unsafe extern "system" fn GetBitCount<Impl: ICertEncodeBitStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pbitcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitString<Impl: ICertEncodeBitStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrbitstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitString() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrbitstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Encode<Impl: ICertEncodeBitStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitcount: i32, strbitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Encode(::core::mem::transmute_copy(&bitcount), ::core::mem::transmute_copy(&strbitstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrbinary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Decode: Decode::<Impl, IMPL_OFFSET>,
            GetBitCount: GetBitCount::<Impl, IMPL_OFFSET>,
            GetBitString: GetBitString::<Impl, IMPL_OFFSET>,
            Encode: Encode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeBitString as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeBitString2Impl: Sized + IDispatchImpl + ICertEncodeBitStringImpl {
    fn DecodeBlob(&mut self, strencodeddata: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
    fn EncodeBlob(&mut self, bitcount: i32, strbitstring: super::super::super::Foundation::BSTR, encodingin: EncodingType, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetBitStringBlob(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeBitString2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeBitString2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeBitString2Vtbl {
        unsafe extern "system" fn DecodeBlob<Impl: ICertEncodeBitString2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecodeBlob(::core::mem::transmute_copy(&strencodeddata), ::core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn EncodeBlob<Impl: ICertEncodeBitString2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitcount: i32, strbitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encodingin: EncodingType, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodeBlob(::core::mem::transmute_copy(&bitcount), ::core::mem::transmute_copy(&strbitstring), ::core::mem::transmute_copy(&encodingin), ::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrencodeddata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitStringBlob<Impl: ICertEncodeBitString2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrbitstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitStringBlob(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrbitstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertEncodeBitStringVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DecodeBlob: DecodeBlob::<Impl, IMPL_OFFSET>,
            EncodeBlob: EncodeBlob::<Impl, IMPL_OFFSET>,
            GetBitStringBlob: GetBitStringBlob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeBitString2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeCRLDistInfoImpl: Sized + IDispatchImpl {
    fn Decode(&mut self, strbinary: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetDistPointCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetNameCount(&mut self, distpointindex: i32) -> ::windows::core::Result<i32>;
    fn GetNameChoice(&mut self, distpointindex: i32, nameindex: i32) -> ::windows::core::Result<i32>;
    fn GetName(&mut self, distpointindex: i32, nameindex: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Reset(&mut self, distpointcount: i32) -> ::windows::core::Result<()>;
    fn SetNameCount(&mut self, distpointindex: i32, namecount: i32) -> ::windows::core::Result<()>;
    fn SetNameEntry(&mut self, distpointindex: i32, nameindex: i32, namechoice: CERT_ALT_NAME, strname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Encode(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeCRLDistInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeCRLDistInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeCRLDistInfoVtbl {
        unsafe extern "system" fn Decode<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Decode(::core::mem::transmute_copy(&strbinary)).into()
        }
        unsafe extern "system" fn GetDistPointCount<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdistpointcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDistPointCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdistpointcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameCount<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distpointindex: i32, pnamecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameCount(::core::mem::transmute_copy(&distpointindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnamecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameChoice<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distpointindex: i32, nameindex: i32, pnamechoice: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameChoice(::core::mem::transmute_copy(&distpointindex), ::core::mem::transmute_copy(&nameindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnamechoice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distpointindex: i32, nameindex: i32, pstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&distpointindex), ::core::mem::transmute_copy(&nameindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distpointcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&distpointcount)).into()
        }
        unsafe extern "system" fn SetNameCount<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distpointindex: i32, namecount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNameCount(::core::mem::transmute_copy(&distpointindex), ::core::mem::transmute_copy(&namecount)).into()
        }
        unsafe extern "system" fn SetNameEntry<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distpointindex: i32, nameindex: i32, namechoice: CERT_ALT_NAME, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNameEntry(::core::mem::transmute_copy(&distpointindex), ::core::mem::transmute_copy(&nameindex), ::core::mem::transmute_copy(&namechoice), ::core::mem::transmute_copy(&strname)).into()
        }
        unsafe extern "system" fn Encode<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Encode() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrbinary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Decode: Decode::<Impl, IMPL_OFFSET>,
            GetDistPointCount: GetDistPointCount::<Impl, IMPL_OFFSET>,
            GetNameCount: GetNameCount::<Impl, IMPL_OFFSET>,
            GetNameChoice: GetNameChoice::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            SetNameCount: SetNameCount::<Impl, IMPL_OFFSET>,
            SetNameEntry: SetNameEntry::<Impl, IMPL_OFFSET>,
            Encode: Encode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeCRLDistInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeCRLDistInfo2Impl: Sized + IDispatchImpl + ICertEncodeCRLDistInfoImpl {
    fn DecodeBlob(&mut self, strencodeddata: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
    fn EncodeBlob(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeCRLDistInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeCRLDistInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeCRLDistInfo2Vtbl {
        unsafe extern "system" fn DecodeBlob<Impl: ICertEncodeCRLDistInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecodeBlob(::core::mem::transmute_copy(&strencodeddata), ::core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn EncodeBlob<Impl: ICertEncodeCRLDistInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodeBlob(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrencodeddata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertEncodeCRLDistInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DecodeBlob: DecodeBlob::<Impl, IMPL_OFFSET>,
            EncodeBlob: EncodeBlob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeCRLDistInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeDateArrayImpl: Sized + IDispatchImpl {
    fn Decode(&mut self, strbinary: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetValue(&mut self, index: i32) -> ::windows::core::Result<f64>;
    fn Reset(&mut self, count: i32) -> ::windows::core::Result<()>;
    fn SetValue(&mut self, index: i32, value: f64) -> ::windows::core::Result<()>;
    fn Encode(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeDateArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeDateArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeDateArrayVtbl {
        unsafe extern "system" fn Decode<Impl: ICertEncodeDateArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Decode(::core::mem::transmute_copy(&strbinary)).into()
        }
        unsafe extern "system" fn GetCount<Impl: ICertEncodeDateArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: ICertEncodeDateArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ICertEncodeDateArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetValue<Impl: ICertEncodeDateArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Encode<Impl: ICertEncodeDateArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Encode() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrbinary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Decode: Decode::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Encode: Encode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeDateArray as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeDateArray2Impl: Sized + IDispatchImpl + ICertEncodeDateArrayImpl {
    fn DecodeBlob(&mut self, strencodeddata: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
    fn EncodeBlob(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeDateArray2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeDateArray2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeDateArray2Vtbl {
        unsafe extern "system" fn DecodeBlob<Impl: ICertEncodeDateArray2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecodeBlob(::core::mem::transmute_copy(&strencodeddata), ::core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn EncodeBlob<Impl: ICertEncodeDateArray2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodeBlob(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrencodeddata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertEncodeDateArrayVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DecodeBlob: DecodeBlob::<Impl, IMPL_OFFSET>,
            EncodeBlob: EncodeBlob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeDateArray2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeLongArrayImpl: Sized + IDispatchImpl {
    fn Decode(&mut self, strbinary: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetValue(&mut self, index: i32) -> ::windows::core::Result<i32>;
    fn Reset(&mut self, count: i32) -> ::windows::core::Result<()>;
    fn SetValue(&mut self, index: i32, value: i32) -> ::windows::core::Result<()>;
    fn Encode(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeLongArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeLongArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeLongArrayVtbl {
        unsafe extern "system" fn Decode<Impl: ICertEncodeLongArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Decode(::core::mem::transmute_copy(&strbinary)).into()
        }
        unsafe extern "system" fn GetCount<Impl: ICertEncodeLongArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: ICertEncodeLongArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ICertEncodeLongArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetValue<Impl: ICertEncodeLongArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Encode<Impl: ICertEncodeLongArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Encode() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrbinary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Decode: Decode::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Encode: Encode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeLongArray as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeLongArray2Impl: Sized + IDispatchImpl + ICertEncodeLongArrayImpl {
    fn DecodeBlob(&mut self, strencodeddata: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
    fn EncodeBlob(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeLongArray2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeLongArray2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeLongArray2Vtbl {
        unsafe extern "system" fn DecodeBlob<Impl: ICertEncodeLongArray2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecodeBlob(::core::mem::transmute_copy(&strencodeddata), ::core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn EncodeBlob<Impl: ICertEncodeLongArray2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodeBlob(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrencodeddata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertEncodeLongArrayVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DecodeBlob: DecodeBlob::<Impl, IMPL_OFFSET>,
            EncodeBlob: EncodeBlob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeLongArray2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeStringArrayImpl: Sized + IDispatchImpl {
    fn Decode(&mut self, strbinary: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetStringType(&mut self) -> ::windows::core::Result<i32>;
    fn GetCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetValue(&mut self, index: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Reset(&mut self, count: i32, stringtype: super::CERT_RDN_ATTR_VALUE_TYPE) -> ::windows::core::Result<()>;
    fn SetValue(&mut self, index: i32, str: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Encode(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeStringArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeStringArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeStringArrayVtbl {
        unsafe extern "system" fn Decode<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Decode(::core::mem::transmute_copy(&strbinary)).into()
        }
        unsafe extern "system" fn GetStringType<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstringtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringType() {
                ::core::result::Result::Ok(ok__) => {
                    *pstringtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, stringtype: super::CERT_RDN_ATTR_VALUE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&stringtype)).into()
        }
        unsafe extern "system" fn SetValue<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, str: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&str)).into()
        }
        unsafe extern "system" fn Encode<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Encode() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrbinary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Decode: Decode::<Impl, IMPL_OFFSET>,
            GetStringType: GetStringType::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Encode: Encode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeStringArray as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeStringArray2Impl: Sized + IDispatchImpl + ICertEncodeStringArrayImpl {
    fn DecodeBlob(&mut self, strencodeddata: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
    fn EncodeBlob(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeStringArray2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeStringArray2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeStringArray2Vtbl {
        unsafe extern "system" fn DecodeBlob<Impl: ICertEncodeStringArray2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecodeBlob(::core::mem::transmute_copy(&strencodeddata), ::core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn EncodeBlob<Impl: ICertEncodeStringArray2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodeBlob(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrencodeddata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertEncodeStringArrayVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DecodeBlob: DecodeBlob::<Impl, IMPL_OFFSET>,
            EncodeBlob: EncodeBlob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeStringArray2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertExitImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, strconfig: super::super::super::Foundation::BSTR) -> ::windows::core::Result<CERT_EXIT_EVENT_MASK>;
    fn Notify(&mut self, exitevent: i32, context: i32) -> ::windows::core::Result<()>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertExitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertExitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertExitVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, peventmask: *mut CERT_EXIT_EVENT_MASK) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(::core::mem::transmute_copy(&strconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *peventmask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Impl: ICertExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, exitevent: i32, context: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&exitevent), ::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn GetDescription<Impl: ICertExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Notify: Notify::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertExit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertExit2Impl: Sized + IDispatchImpl + ICertExitImpl {
    fn GetManageModule(&mut self) -> ::windows::core::Result<ICertManageModule>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertExit2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertExit2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertExit2Vtbl {
        unsafe extern "system" fn GetManageModule<Impl: ICertExit2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmanagemodule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetManageModule() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmanagemodule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ICertExitVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetManageModule: GetManageModule::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertExit2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertGetConfigImpl: Sized + IDispatchImpl {
    fn GetConfig(&mut self, flags: CERT_GET_CONFIG_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertGetConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertGetConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertGetConfigVtbl {
        unsafe extern "system" fn GetConfig<Impl: ICertGetConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: CERT_GET_CONFIG_FLAGS, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConfig(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetConfig: GetConfig::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertGetConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertManageModuleImpl: Sized + IDispatchImpl {
    fn GetProperty(&mut self, strconfig: super::super::super::Foundation::BSTR, strstoragelocation: super::super::super::Foundation::BSTR, strpropertyname: super::super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, strconfig: super::super::super::Foundation::BSTR, strstoragelocation: super::super::super::Foundation::BSTR, strpropertyname: super::super::super::Foundation::BSTR, flags: i32, pvarproperty: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Configure(&mut self, strconfig: super::super::super::Foundation::BSTR, strstoragelocation: super::super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertManageModuleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertManageModuleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertManageModuleVtbl {
        unsafe extern "system" fn GetProperty<Impl: ICertManageModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strstoragelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32, pvarproperty: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&strstoragelocation), ::core::mem::transmute_copy(&strpropertyname), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: ICertManageModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strstoragelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32, pvarproperty: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&strstoragelocation), ::core::mem::transmute_copy(&strpropertyname), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pvarproperty)).into()
        }
        unsafe extern "system" fn Configure<Impl: ICertManageModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strstoragelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Configure(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&strstoragelocation), ::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            Configure: Configure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertManageModule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPolicyImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, strconfig: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn VerifyRequest(&mut self, strconfig: super::super::super::Foundation::BSTR, context: i32, bnewrequest: i32, flags: i32) -> ::windows::core::Result<i32>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ShutDown(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPolicyVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&strconfig)).into()
        }
        unsafe extern "system" fn VerifyRequest<Impl: ICertPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, context: i32, bnewrequest: i32, flags: i32, pdisposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifyRequest(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&bnewrequest), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdisposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: ICertPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutDown<Impl: ICertPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShutDown().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            VerifyRequest: VerifyRequest::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            ShutDown: ShutDown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPolicy2Impl: Sized + IDispatchImpl + ICertPolicyImpl {
    fn GetManageModule(&mut self) -> ::windows::core::Result<ICertManageModule>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPolicy2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPolicy2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPolicy2Vtbl {
        unsafe extern "system" fn GetManageModule<Impl: ICertPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmanagemodule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetManageModule() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmanagemodule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ICertPolicyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetManageModule: GetManageModule::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPolicy2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertiesImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<ICertProperty>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<ICertProperty>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn InitializeFromCertificate(&mut self, machinecontext: i16, encoding: EncodingType, strcertificate: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertiesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn InitializeFromCertificate<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromCertificate(::core::mem::transmute_copy(&machinecontext), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strcertificate)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate: InitializeFromCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyImpl: Sized + IDispatchImpl {
    fn InitializeFromCertificate(&mut self, machinecontext: i16, encoding: EncodingType, strcertificate: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PropertyId(&mut self) -> ::windows::core::Result<CERTENROLL_PROPERTYID>;
    fn SetPropertyId(&mut self, value: CERTENROLL_PROPERTYID) -> ::windows::core::Result<()>;
    fn RawData(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn RemoveFromCertificate(&mut self, machinecontext: i16, encoding: EncodingType, strcertificate: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetValueOnCertificate(&mut self, machinecontext: i16, encoding: EncodingType, strcertificate: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyVtbl {
        unsafe extern "system" fn InitializeFromCertificate<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromCertificate(::core::mem::transmute_copy(&machinecontext), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strcertificate)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn PropertyId<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut CERTENROLL_PROPERTYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyId<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CERTENROLL_PROPERTYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyId(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn RawData<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawData(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFromCertificate<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFromCertificate(::core::mem::transmute_copy(&machinecontext), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strcertificate)).into()
        }
        unsafe extern "system" fn SetValueOnCertificate<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValueOnCertificate(::core::mem::transmute_copy(&machinecontext), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strcertificate)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromCertificate: InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId: PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId: SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData: RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate: RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate: SetValueOnCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyArchivedImpl: Sized + IDispatchImpl + ICertPropertyImpl {
    fn Initialize(&mut self, archivedvalue: i16) -> ::windows::core::Result<()>;
    fn Archived(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyArchivedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyArchivedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyArchivedVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyArchivedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, archivedvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&archivedvalue)).into()
        }
        unsafe extern "system" fn Archived<Impl: ICertPropertyArchivedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Archived() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertPropertyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Archived: Archived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyArchived as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyArchivedKeyHashImpl: Sized + IDispatchImpl + ICertPropertyImpl {
    fn Initialize(&mut self, encoding: EncodingType, strarchivedkeyhashvalue: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ArchivedKeyHash(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyArchivedKeyHashVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyArchivedKeyHashImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyArchivedKeyHashVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyArchivedKeyHashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strarchivedkeyhashvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strarchivedkeyhashvalue)).into()
        }
        unsafe extern "system" fn ArchivedKeyHash<Impl: ICertPropertyArchivedKeyHashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchivedKeyHash(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertPropertyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            ArchivedKeyHash: ArchivedKeyHash::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyArchivedKeyHash as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyAutoEnrollImpl: Sized + IDispatchImpl + ICertPropertyImpl {
    fn Initialize(&mut self, strtemplatename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TemplateName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyAutoEnrollVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyAutoEnrollImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyAutoEnrollVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyAutoEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&strtemplatename)).into()
        }
        unsafe extern "system" fn TemplateName<Impl: ICertPropertyAutoEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TemplateName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertPropertyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            TemplateName: TemplateName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyAutoEnroll as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyBackedUpImpl: Sized + IDispatchImpl + ICertPropertyImpl {
    fn InitializeFromCurrentTime(&mut self, backedupvalue: i16) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, backedupvalue: i16, date: f64) -> ::windows::core::Result<()>;
    fn BackedUpValue(&mut self) -> ::windows::core::Result<i16>;
    fn BackedUpTime(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyBackedUpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyBackedUpImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyBackedUpVtbl {
        unsafe extern "system" fn InitializeFromCurrentTime<Impl: ICertPropertyBackedUpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backedupvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromCurrentTime(::core::mem::transmute_copy(&backedupvalue)).into()
        }
        unsafe extern "system" fn Initialize<Impl: ICertPropertyBackedUpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backedupvalue: i16, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&backedupvalue), ::core::mem::transmute_copy(&date)).into()
        }
        unsafe extern "system" fn BackedUpValue<Impl: ICertPropertyBackedUpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackedUpValue() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackedUpTime<Impl: ICertPropertyBackedUpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackedUpTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertPropertyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromCurrentTime: InitializeFromCurrentTime::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            BackedUpValue: BackedUpValue::<Impl, IMPL_OFFSET>,
            BackedUpTime: BackedUpTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyBackedUp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyDescriptionImpl: Sized + IDispatchImpl + ICertPropertyImpl {
    fn Initialize(&mut self, strdescription: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyDescriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyDescriptionVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&strdescription)).into()
        }
        unsafe extern "system" fn Description<Impl: ICertPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertPropertyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyDescription as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyEnrollmentImpl: Sized + IDispatchImpl + ICertPropertyImpl {
    fn Initialize(&mut self, requestid: i32, strcadnsname: super::super::super::Foundation::BSTR, strcaname: super::super::super::Foundation::BSTR, strfriendlyname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RequestId(&mut self) -> ::windows::core::Result<i32>;
    fn CADnsName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn CAName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyEnrollmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyEnrollmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyEnrollmentVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: i32, strcadnsname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strcaname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strfriendlyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&strcadnsname), ::core::mem::transmute_copy(&strcaname), ::core::mem::transmute_copy(&strfriendlyname)).into()
        }
        unsafe extern "system" fn RequestId<Impl: ICertPropertyEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CADnsName<Impl: ICertPropertyEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CADnsName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAName<Impl: ICertPropertyEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CAName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Impl: ICertPropertyEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertPropertyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            RequestId: RequestId::<Impl, IMPL_OFFSET>,
            CADnsName: CADnsName::<Impl, IMPL_OFFSET>,
            CAName: CAName::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyEnrollment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyEnrollmentPolicyServerImpl: Sized + IDispatchImpl + ICertPropertyImpl {
    fn Initialize(&mut self, propertyflags: EnrollmentPolicyServerPropertyFlags, authflags: X509EnrollmentAuthFlags, enrollmentserverauthflags: X509EnrollmentAuthFlags, urlflags: PolicyServerUrlFlags, strrequestid: super::super::super::Foundation::BSTR, strurl: super::super::super::Foundation::BSTR, strid: super::super::super::Foundation::BSTR, strenrollmentserverurl: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetPolicyServerUrl(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetPolicyServerId(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetEnrollmentServerUrl(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetRequestIdString(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetPropertyFlags(&mut self) -> ::windows::core::Result<EnrollmentPolicyServerPropertyFlags>;
    fn GetUrlFlags(&mut self) -> ::windows::core::Result<PolicyServerUrlFlags>;
    fn GetAuthentication(&mut self) -> ::windows::core::Result<X509EnrollmentAuthFlags>;
    fn GetEnrollmentServerAuthentication(&mut self) -> ::windows::core::Result<X509EnrollmentAuthFlags>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyEnrollmentPolicyServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyEnrollmentPolicyServerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyEnrollmentPolicyServerVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyflags: EnrollmentPolicyServerPropertyFlags, authflags: X509EnrollmentAuthFlags, enrollmentserverauthflags: X509EnrollmentAuthFlags, urlflags: PolicyServerUrlFlags, strrequestid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strenrollmentserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&propertyflags), ::core::mem::transmute_copy(&authflags), ::core::mem::transmute_copy(&enrollmentserverauthflags), ::core::mem::transmute_copy(&urlflags), ::core::mem::transmute_copy(&strrequestid), ::core::mem::transmute_copy(&strurl), ::core::mem::transmute_copy(&strid), ::core::mem::transmute_copy(&strenrollmentserverurl)).into()
        }
        unsafe extern "system" fn GetPolicyServerUrl<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPolicyServerUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicyServerId<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPolicyServerId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnrollmentServerUrl<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnrollmentServerUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestIdString<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestIdString() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyFlags<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut EnrollmentPolicyServerPropertyFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUrlFlags<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut PolicyServerUrlFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUrlFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthentication<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509EnrollmentAuthFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthentication() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnrollmentServerAuthentication<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509EnrollmentAuthFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnrollmentServerAuthentication() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertPropertyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetPolicyServerUrl: GetPolicyServerUrl::<Impl, IMPL_OFFSET>,
            GetPolicyServerId: GetPolicyServerId::<Impl, IMPL_OFFSET>,
            GetEnrollmentServerUrl: GetEnrollmentServerUrl::<Impl, IMPL_OFFSET>,
            GetRequestIdString: GetRequestIdString::<Impl, IMPL_OFFSET>,
            GetPropertyFlags: GetPropertyFlags::<Impl, IMPL_OFFSET>,
            GetUrlFlags: GetUrlFlags::<Impl, IMPL_OFFSET>,
            GetAuthentication: GetAuthentication::<Impl, IMPL_OFFSET>,
            GetEnrollmentServerAuthentication: GetEnrollmentServerAuthentication::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyEnrollmentPolicyServer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyFriendlyNameImpl: Sized + IDispatchImpl + ICertPropertyImpl {
    fn Initialize(&mut self, strfriendlyname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyFriendlyNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyFriendlyNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyFriendlyNameVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyFriendlyNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strfriendlyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&strfriendlyname)).into()
        }
        unsafe extern "system" fn FriendlyName<Impl: ICertPropertyFriendlyNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertPropertyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyFriendlyName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyKeyProvInfoImpl: Sized + IDispatchImpl + ICertPropertyImpl {
    fn Initialize(&mut self, pvalue: ::core::option::Option<IX509PrivateKey>) -> ::windows::core::Result<()>;
    fn PrivateKey(&mut self) -> ::windows::core::Result<IX509PrivateKey>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyKeyProvInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyKeyProvInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyKeyProvInfoVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyKeyProvInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn PrivateKey<Impl: ICertPropertyKeyProvInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateKey() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertPropertyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            PrivateKey: PrivateKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyKeyProvInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyRenewalImpl: Sized + IDispatchImpl + ICertPropertyImpl {
    fn Initialize(&mut self, encoding: EncodingType, strrenewalvalue: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeFromCertificateHash(&mut self, machinecontext: i16, encoding: EncodingType, strcertificate: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Renewal(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyRenewalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyRenewalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyRenewalVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyRenewalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strrenewalvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strrenewalvalue)).into()
        }
        unsafe extern "system" fn InitializeFromCertificateHash<Impl: ICertPropertyRenewalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromCertificateHash(::core::mem::transmute_copy(&machinecontext), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strcertificate)).into()
        }
        unsafe extern "system" fn Renewal<Impl: ICertPropertyRenewalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Renewal(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertPropertyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            InitializeFromCertificateHash: InitializeFromCertificateHash::<Impl, IMPL_OFFSET>,
            Renewal: Renewal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyRenewal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyRequestOriginatorImpl: Sized + IDispatchImpl + ICertPropertyImpl {
    fn Initialize(&mut self, strrequestoriginator: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeFromLocalRequestOriginator(&mut self) -> ::windows::core::Result<()>;
    fn RequestOriginator(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyRequestOriginatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyRequestOriginatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyRequestOriginatorVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyRequestOriginatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strrequestoriginator: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&strrequestoriginator)).into()
        }
        unsafe extern "system" fn InitializeFromLocalRequestOriginator<Impl: ICertPropertyRequestOriginatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromLocalRequestOriginator().into()
        }
        unsafe extern "system" fn RequestOriginator<Impl: ICertPropertyRequestOriginatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestOriginator() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertPropertyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            InitializeFromLocalRequestOriginator: InitializeFromLocalRequestOriginator::<Impl, IMPL_OFFSET>,
            RequestOriginator: RequestOriginator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyRequestOriginator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertySHA1HashImpl: Sized + IDispatchImpl + ICertPropertyImpl {
    fn Initialize(&mut self, encoding: EncodingType, strrenewalvalue: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SHA1Hash(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertySHA1HashVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertySHA1HashImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertySHA1HashVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertySHA1HashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strrenewalvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strrenewalvalue)).into()
        }
        unsafe extern "system" fn SHA1Hash<Impl: ICertPropertySHA1HashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SHA1Hash(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertPropertyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            SHA1Hash: SHA1Hash::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertySHA1Hash as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertRequestImpl: Sized + IDispatchImpl {
    fn Submit(&mut self, flags: i32, strrequest: super::super::super::Foundation::BSTR, strattributes: super::super::super::Foundation::BSTR, strconfig: super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn RetrievePending(&mut self, requestid: i32, strconfig: super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn GetLastStatus(&mut self) -> ::windows::core::Result<i32>;
    fn GetRequestId(&mut self) -> ::windows::core::Result<i32>;
    fn GetDispositionMessage(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetCACertificate(&mut self, fexchangecertificate: i32, strconfig: super::super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetCertificate(&mut self, flags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertRequestVtbl {
        unsafe extern "system" fn Submit<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, strrequest: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strattributes: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submit(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&strrequest), ::core::mem::transmute_copy(&strattributes), ::core::mem::transmute_copy(&strconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdisposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrievePending<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: i32, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrievePending(::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&strconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdisposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastStatus<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestId<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequestid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *prequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDispositionMessage<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdispositionmessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDispositionMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrdispositionmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCACertificate<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fexchangecertificate: i32, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32, pstrcertificate: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCACertificate(::core::mem::transmute_copy(&fexchangecertificate), ::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrcertificate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificate<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pstrcertificate: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificate(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrcertificate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Submit: Submit::<Impl, IMPL_OFFSET>,
            RetrievePending: RetrievePending::<Impl, IMPL_OFFSET>,
            GetLastStatus: GetLastStatus::<Impl, IMPL_OFFSET>,
            GetRequestId: GetRequestId::<Impl, IMPL_OFFSET>,
            GetDispositionMessage: GetDispositionMessage::<Impl, IMPL_OFFSET>,
            GetCACertificate: GetCACertificate::<Impl, IMPL_OFFSET>,
            GetCertificate: GetCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertRequest2Impl: Sized + IDispatchImpl + ICertRequestImpl {
    fn GetIssuedCertificate(&mut self, strconfig: super::super::super::Foundation::BSTR, requestid: i32, strserialnumber: super::super::super::Foundation::BSTR) -> ::windows::core::Result<CR_DISP>;
    fn GetErrorMessageText(&mut self, hrmessage: i32, flags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetCAProperty(&mut self, strconfig: super::super::super::Foundation::BSTR, propid: i32, propindex: i32, proptype: i32, flags: i32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn GetCAPropertyFlags(&mut self, strconfig: super::super::super::Foundation::BSTR, propid: i32) -> ::windows::core::Result<i32>;
    fn GetCAPropertyDisplayName(&mut self, strconfig: super::super::super::Foundation::BSTR, propid: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetFullResponseProperty(&mut self, propid: FULL_RESPONSE_PROPERTY_ID, propindex: i32, proptype: CERT_PROPERTY_TYPE, flags: CERT_REQUEST_OUT_TYPE) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertRequest2Vtbl {
        unsafe extern "system" fn GetIssuedCertificate<Impl: ICertRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut CR_DISP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIssuedCertificate(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&strserialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdisposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorMessageText<Impl: ICertRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrmessage: i32, flags: i32, pstrerrormessagetext: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorMessageText(::core::mem::transmute_copy(&hrmessage), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrerrormessagetext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAProperty<Impl: ICertRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, propindex: i32, proptype: i32, flags: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCAProperty(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&propindex), ::core::mem::transmute_copy(&proptype), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarpropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAPropertyFlags<Impl: ICertRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, ppropflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCAPropertyFlags(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&propid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAPropertyDisplayName<Impl: ICertRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, pstrdisplayname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCAPropertyDisplayName(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&propid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullResponseProperty<Impl: ICertRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: FULL_RESPONSE_PROPERTY_ID, propindex: i32, proptype: CERT_PROPERTY_TYPE, flags: CERT_REQUEST_OUT_TYPE, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFullResponseProperty(::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&propindex), ::core::mem::transmute_copy(&proptype), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarpropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertRequestVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetIssuedCertificate: GetIssuedCertificate::<Impl, IMPL_OFFSET>,
            GetErrorMessageText: GetErrorMessageText::<Impl, IMPL_OFFSET>,
            GetCAProperty: GetCAProperty::<Impl, IMPL_OFFSET>,
            GetCAPropertyFlags: GetCAPropertyFlags::<Impl, IMPL_OFFSET>,
            GetCAPropertyDisplayName: GetCAPropertyDisplayName::<Impl, IMPL_OFFSET>,
            GetFullResponseProperty: GetFullResponseProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertRequest3Impl: Sized + IDispatchImpl + ICertRequestImpl + ICertRequest2Impl {
    fn SetCredential(&mut self, hwnd: i32, authtype: X509EnrollmentAuthFlags, strcredential: super::super::super::Foundation::BSTR, strpassword: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetRequestIdString(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetIssuedCertificate2(&mut self, strconfig: super::super::super::Foundation::BSTR, strrequestid: super::super::super::Foundation::BSTR, strserialnumber: super::super::super::Foundation::BSTR) -> ::windows::core::Result<CR_DISP>;
    fn GetRefreshPolicy(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertRequest3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertRequest3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertRequest3Vtbl {
        unsafe extern "system" fn SetCredential<Impl: ICertRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: i32, authtype: X509EnrollmentAuthFlags, strcredential: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredential(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&authtype), ::core::mem::transmute_copy(&strcredential), ::core::mem::transmute_copy(&strpassword)).into()
        }
        unsafe extern "system" fn GetRequestIdString<Impl: ICertRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrrequestid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestIdString() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrrequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIssuedCertificate2<Impl: ICertRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strrequestid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut CR_DISP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIssuedCertificate2(::core::mem::transmute_copy(&strconfig), ::core::mem::transmute_copy(&strrequestid), ::core::mem::transmute_copy(&strserialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdisposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRefreshPolicy<Impl: ICertRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRefreshPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICertRequest2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetCredential: SetCredential::<Impl, IMPL_OFFSET>,
            GetRequestIdString: GetRequestIdString::<Impl, IMPL_OFFSET>,
            GetIssuedCertificate2: GetIssuedCertificate2::<Impl, IMPL_OFFSET>,
            GetRefreshPolicy: GetRefreshPolicy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertRequest3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICertRequestDImpl: Sized {
    fn Request(&mut self, dwflags: u32, pwszauthority: super::super::super::Foundation::PWSTR, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: super::super::super::Foundation::PWSTR, pctbrequest: *const CERTTRANSBLOB, pctbcertchain: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> ::windows::core::Result<()>;
    fn GetCACert(&mut self, fchain: u32, pwszauthority: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<CERTTRANSBLOB>;
    fn Ping(&mut self, pwszauthority: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICertRequestDVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertRequestDImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertRequestDVtbl {
        unsafe extern "system" fn Request<Impl: ICertRequestDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pwszauthority: super::super::super::Foundation::PWSTR, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: super::super::super::Foundation::PWSTR, pctbrequest: *const CERTTRANSBLOB, pctbcertchain: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Request(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pwszauthority), ::core::mem::transmute_copy(&pdwrequestid), ::core::mem::transmute_copy(&pdwdisposition), ::core::mem::transmute_copy(&pwszattributes), ::core::mem::transmute_copy(&pctbrequest), ::core::mem::transmute_copy(&pctbcertchain), ::core::mem::transmute_copy(&pctbencodedcert), ::core::mem::transmute_copy(&pctbdispositionmessage)).into()
        }
        unsafe extern "system" fn GetCACert<Impl: ICertRequestDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fchain: u32, pwszauthority: super::super::super::Foundation::PWSTR, pctbout: *mut CERTTRANSBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCACert(::core::mem::transmute_copy(&fchain), ::core::mem::transmute_copy(&pwszauthority)) {
                ::core::result::Result::Ok(ok__) => {
                    *pctbout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ping<Impl: ICertRequestDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszauthority: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Ping(::core::mem::transmute_copy(&pwszauthority)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
            GetCACert: GetCACert::<Impl, IMPL_OFFSET>,
            Ping: Ping::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertRequestD as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICertRequestD2Impl: Sized + ICertRequestDImpl {
    fn Request2(&mut self, pwszauthority: super::super::super::Foundation::PWSTR, dwflags: u32, pwszserialnumber: super::super::super::Foundation::PWSTR, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: super::super::super::Foundation::PWSTR, pctbrequest: *const CERTTRANSBLOB, pctbfullresponse: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> ::windows::core::Result<()>;
    fn GetCAProperty(&mut self, pwszauthority: super::super::super::Foundation::PWSTR, propid: i32, propindex: i32, proptype: i32) -> ::windows::core::Result<CERTTRANSBLOB>;
    fn GetCAPropertyInfo(&mut self, pwszauthority: super::super::super::Foundation::PWSTR, pcproperty: *mut i32, pctbpropinfo: *mut CERTTRANSBLOB) -> ::windows::core::Result<()>;
    fn Ping2(&mut self, pwszauthority: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICertRequestD2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertRequestD2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertRequestD2Vtbl {
        unsafe extern "system" fn Request2<Impl: ICertRequestD2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszauthority: super::super::super::Foundation::PWSTR, dwflags: u32, pwszserialnumber: super::super::super::Foundation::PWSTR, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: super::super::super::Foundation::PWSTR, pctbrequest: *const CERTTRANSBLOB, pctbfullresponse: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .Request2(::core::mem::transmute_copy(&pwszauthority), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pwszserialnumber), ::core::mem::transmute_copy(&pdwrequestid), ::core::mem::transmute_copy(&pdwdisposition), ::core::mem::transmute_copy(&pwszattributes), ::core::mem::transmute_copy(&pctbrequest), ::core::mem::transmute_copy(&pctbfullresponse), ::core::mem::transmute_copy(&pctbencodedcert), ::core::mem::transmute_copy(&pctbdispositionmessage))
                .into()
        }
        unsafe extern "system" fn GetCAProperty<Impl: ICertRequestD2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszauthority: super::super::super::Foundation::PWSTR, propid: i32, propindex: i32, proptype: i32, pctbpropertyvalue: *mut CERTTRANSBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCAProperty(::core::mem::transmute_copy(&pwszauthority), ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&propindex), ::core::mem::transmute_copy(&proptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pctbpropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAPropertyInfo<Impl: ICertRequestD2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszauthority: super::super::super::Foundation::PWSTR, pcproperty: *mut i32, pctbpropinfo: *mut CERTTRANSBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCAPropertyInfo(::core::mem::transmute_copy(&pwszauthority), ::core::mem::transmute_copy(&pcproperty), ::core::mem::transmute_copy(&pctbpropinfo)).into()
        }
        unsafe extern "system" fn Ping2<Impl: ICertRequestD2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszauthority: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Ping2(::core::mem::transmute_copy(&pwszauthority)).into()
        }
        Self {
            base: ICertRequestDVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Request2: Request2::<Impl, IMPL_OFFSET>,
            GetCAProperty: GetCAProperty::<Impl, IMPL_OFFSET>,
            GetCAPropertyInfo: GetCAPropertyInfo::<Impl, IMPL_OFFSET>,
            Ping2: Ping2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertRequestD2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertServerExitImpl: Sized + IDispatchImpl {
    fn SetContext(&mut self, context: i32) -> ::windows::core::Result<()>;
    fn GetRequestProperty(&mut self, strpropertyname: super::super::super::Foundation::BSTR, propertytype: i32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn GetRequestAttribute(&mut self, strattributename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetCertificateProperty(&mut self, strpropertyname: super::super::super::Foundation::BSTR, propertytype: i32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn GetCertificateExtension(&mut self, strextensionname: super::super::super::Foundation::BSTR, r#type: i32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn GetCertificateExtensionFlags(&mut self) -> ::windows::core::Result<i32>;
    fn EnumerateExtensionsSetup(&mut self, flags: i32) -> ::windows::core::Result<()>;
    fn EnumerateExtensions(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn EnumerateExtensionsClose(&mut self) -> ::windows::core::Result<()>;
    fn EnumerateAttributesSetup(&mut self, flags: i32) -> ::windows::core::Result<()>;
    fn EnumerateAttributes(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn EnumerateAttributesClose(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertServerExitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertServerExitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertServerExitVtbl {
        unsafe extern "system" fn SetContext<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContext(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn GetRequestProperty<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestProperty(::core::mem::transmute_copy(&strpropertyname), ::core::mem::transmute_copy(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarpropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestAttribute<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strattributename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrattributevalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestAttribute(::core::mem::transmute_copy(&strattributename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrattributevalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateProperty<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateProperty(::core::mem::transmute_copy(&strpropertyname), ::core::mem::transmute_copy(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarpropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateExtension<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strextensionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: i32, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateExtension(::core::mem::transmute_copy(&strextensionname), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateExtensionFlags<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateExtensionFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pextflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateExtensionsSetup<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateExtensionsSetup(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn EnumerateExtensions<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrextensionname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrextensionname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateExtensionsClose<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateExtensionsClose().into()
        }
        unsafe extern "system" fn EnumerateAttributesSetup<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateAttributesSetup(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn EnumerateAttributes<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrattributename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrattributename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAttributesClose<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateAttributesClose().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetContext: SetContext::<Impl, IMPL_OFFSET>,
            GetRequestProperty: GetRequestProperty::<Impl, IMPL_OFFSET>,
            GetRequestAttribute: GetRequestAttribute::<Impl, IMPL_OFFSET>,
            GetCertificateProperty: GetCertificateProperty::<Impl, IMPL_OFFSET>,
            GetCertificateExtension: GetCertificateExtension::<Impl, IMPL_OFFSET>,
            GetCertificateExtensionFlags: GetCertificateExtensionFlags::<Impl, IMPL_OFFSET>,
            EnumerateExtensionsSetup: EnumerateExtensionsSetup::<Impl, IMPL_OFFSET>,
            EnumerateExtensions: EnumerateExtensions::<Impl, IMPL_OFFSET>,
            EnumerateExtensionsClose: EnumerateExtensionsClose::<Impl, IMPL_OFFSET>,
            EnumerateAttributesSetup: EnumerateAttributesSetup::<Impl, IMPL_OFFSET>,
            EnumerateAttributes: EnumerateAttributes::<Impl, IMPL_OFFSET>,
            EnumerateAttributesClose: EnumerateAttributesClose::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertServerExit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertServerPolicyImpl: Sized + IDispatchImpl {
    fn SetContext(&mut self, context: i32) -> ::windows::core::Result<()>;
    fn GetRequestProperty(&mut self, strpropertyname: super::super::super::Foundation::BSTR, propertytype: i32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn GetRequestAttribute(&mut self, strattributename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetCertificateProperty(&mut self, strpropertyname: super::super::super::Foundation::BSTR, propertytype: CERT_PROPERTY_TYPE) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn SetCertificateProperty(&mut self, strpropertyname: super::super::super::Foundation::BSTR, propertytype: i32, pvarpropertyvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetCertificateExtension(&mut self, strextensionname: super::super::super::Foundation::BSTR, r#type: CERT_PROPERTY_TYPE) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn GetCertificateExtensionFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetCertificateExtension(&mut self, strextensionname: super::super::super::Foundation::BSTR, r#type: i32, extflags: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn EnumerateExtensionsSetup(&mut self, flags: i32) -> ::windows::core::Result<()>;
    fn EnumerateExtensions(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn EnumerateExtensionsClose(&mut self) -> ::windows::core::Result<()>;
    fn EnumerateAttributesSetup(&mut self, flags: i32) -> ::windows::core::Result<()>;
    fn EnumerateAttributes(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn EnumerateAttributesClose(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertServerPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertServerPolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertServerPolicyVtbl {
        unsafe extern "system" fn SetContext<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContext(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn GetRequestProperty<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestProperty(::core::mem::transmute_copy(&strpropertyname), ::core::mem::transmute_copy(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarpropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestAttribute<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strattributename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrattributevalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestAttribute(::core::mem::transmute_copy(&strattributename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrattributevalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateProperty<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: CERT_PROPERTY_TYPE, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateProperty(::core::mem::transmute_copy(&strpropertyname), ::core::mem::transmute_copy(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarpropertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateProperty<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: i32, pvarpropertyvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCertificateProperty(::core::mem::transmute_copy(&strpropertyname), ::core::mem::transmute_copy(&propertytype), ::core::mem::transmute_copy(&pvarpropertyvalue)).into()
        }
        unsafe extern "system" fn GetCertificateExtension<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strextensionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: CERT_PROPERTY_TYPE, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateExtension(::core::mem::transmute_copy(&strextensionname), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateExtensionFlags<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateExtensionFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pextflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateExtension<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strextensionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: i32, extflags: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCertificateExtension(::core::mem::transmute_copy(&strextensionname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&extflags), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn EnumerateExtensionsSetup<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateExtensionsSetup(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn EnumerateExtensions<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrextensionname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrextensionname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateExtensionsClose<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateExtensionsClose().into()
        }
        unsafe extern "system" fn EnumerateAttributesSetup<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateAttributesSetup(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn EnumerateAttributes<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrattributename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrattributename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAttributesClose<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateAttributesClose().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetContext: SetContext::<Impl, IMPL_OFFSET>,
            GetRequestProperty: GetRequestProperty::<Impl, IMPL_OFFSET>,
            GetRequestAttribute: GetRequestAttribute::<Impl, IMPL_OFFSET>,
            GetCertificateProperty: GetCertificateProperty::<Impl, IMPL_OFFSET>,
            SetCertificateProperty: SetCertificateProperty::<Impl, IMPL_OFFSET>,
            GetCertificateExtension: GetCertificateExtension::<Impl, IMPL_OFFSET>,
            GetCertificateExtensionFlags: GetCertificateExtensionFlags::<Impl, IMPL_OFFSET>,
            SetCertificateExtension: SetCertificateExtension::<Impl, IMPL_OFFSET>,
            EnumerateExtensionsSetup: EnumerateExtensionsSetup::<Impl, IMPL_OFFSET>,
            EnumerateExtensions: EnumerateExtensions::<Impl, IMPL_OFFSET>,
            EnumerateExtensionsClose: EnumerateExtensionsClose::<Impl, IMPL_OFFSET>,
            EnumerateAttributesSetup: EnumerateAttributesSetup::<Impl, IMPL_OFFSET>,
            EnumerateAttributes: EnumerateAttributes::<Impl, IMPL_OFFSET>,
            EnumerateAttributesClose: EnumerateAttributesClose::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertServerPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertViewImpl: Sized + IDispatchImpl {
    fn OpenConnection(&mut self, strconfig: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EnumCertViewColumn(&mut self, fresultcolumn: CVRC_COLUMN) -> ::windows::core::Result<IEnumCERTVIEWCOLUMN>;
    fn GetColumnCount(&mut self, fresultcolumn: CVRC_COLUMN, pccolumn: *mut i32) -> ::windows::core::Result<()>;
    fn GetColumnIndex(&mut self, fresultcolumn: CVRC_COLUMN, strcolumnname: super::super::super::Foundation::BSTR, pcolumnindex: *mut i32) -> ::windows::core::Result<()>;
    fn SetResultColumnCount(&mut self, cresultcolumn: i32) -> ::windows::core::Result<()>;
    fn SetResultColumn(&mut self, columnindex: i32) -> ::windows::core::Result<()>;
    fn SetRestriction(&mut self, columnindex: CERT_VIEW_COLUMN_INDEX, seekoperator: CERT_VIEW_SEEK_OPERATOR_FLAGS, sortorder: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn OpenView(&mut self) -> ::windows::core::Result<IEnumCERTVIEWROW>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertViewVtbl {
        unsafe extern "system" fn OpenConnection<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenConnection(::core::mem::transmute_copy(&strconfig)).into()
        }
        unsafe extern "system" fn EnumCertViewColumn<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fresultcolumn: CVRC_COLUMN, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCertViewColumn(::core::mem::transmute_copy(&fresultcolumn)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnCount<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fresultcolumn: CVRC_COLUMN, pccolumn: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColumnCount(::core::mem::transmute_copy(&fresultcolumn), ::core::mem::transmute_copy(&pccolumn)).into()
        }
        unsafe extern "system" fn GetColumnIndex<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fresultcolumn: CVRC_COLUMN, strcolumnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pcolumnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColumnIndex(::core::mem::transmute_copy(&fresultcolumn), ::core::mem::transmute_copy(&strcolumnname), ::core::mem::transmute_copy(&pcolumnindex)).into()
        }
        unsafe extern "system" fn SetResultColumnCount<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cresultcolumn: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResultColumnCount(::core::mem::transmute_copy(&cresultcolumn)).into()
        }
        unsafe extern "system" fn SetResultColumn<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, columnindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResultColumn(::core::mem::transmute_copy(&columnindex)).into()
        }
        unsafe extern "system" fn SetRestriction<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, columnindex: CERT_VIEW_COLUMN_INDEX, seekoperator: CERT_VIEW_SEEK_OPERATOR_FLAGS, sortorder: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRestriction(::core::mem::transmute_copy(&columnindex), ::core::mem::transmute_copy(&seekoperator), ::core::mem::transmute_copy(&sortorder), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn OpenView<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenView() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OpenConnection: OpenConnection::<Impl, IMPL_OFFSET>,
            EnumCertViewColumn: EnumCertViewColumn::<Impl, IMPL_OFFSET>,
            GetColumnCount: GetColumnCount::<Impl, IMPL_OFFSET>,
            GetColumnIndex: GetColumnIndex::<Impl, IMPL_OFFSET>,
            SetResultColumnCount: SetResultColumnCount::<Impl, IMPL_OFFSET>,
            SetResultColumn: SetResultColumn::<Impl, IMPL_OFFSET>,
            SetRestriction: SetRestriction::<Impl, IMPL_OFFSET>,
            OpenView: OpenView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertView2Impl: Sized + IDispatchImpl + ICertViewImpl {
    fn SetTable(&mut self, table: CVRC_TABLE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertView2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertView2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertView2Vtbl {
        unsafe extern "system" fn SetTable<Impl: ICertView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, table: CVRC_TABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTable(::core::mem::transmute_copy(&table)).into()
        }
        Self { base: ICertViewVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetTable: SetTable::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertView2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificateAttestationChallengeImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, encoding: EncodingType, strpendingfullcmcresponsewithchallenge: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DecryptChallenge(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn RequestID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificateAttestationChallengeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateAttestationChallengeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateAttestationChallengeVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertificateAttestationChallengeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strpendingfullcmcresponsewithchallenge: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strpendingfullcmcresponsewithchallenge)).into()
        }
        unsafe extern "system" fn DecryptChallenge<Impl: ICertificateAttestationChallengeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrenvelopedpkcs7reencryptedtoca: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecryptChallenge(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrenvelopedpkcs7reencryptedtoca = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestID<Impl: ICertificateAttestationChallengeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrrequestid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestID() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrrequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            DecryptChallenge: DecryptChallenge::<Impl, IMPL_OFFSET>,
            RequestID: RequestID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateAttestationChallenge as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificateAttestationChallenge2Impl: Sized + IDispatchImpl + ICertificateAttestationChallengeImpl {
    fn SetKeyContainerName(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetKeyBlob(&mut self, encoding: EncodingType, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificateAttestationChallenge2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateAttestationChallenge2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateAttestationChallenge2Vtbl {
        unsafe extern "system" fn SetKeyContainerName<Impl: ICertificateAttestationChallenge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyContainerName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetKeyBlob<Impl: ICertificateAttestationChallenge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyBlob(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: ICertificateAttestationChallengeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetKeyContainerName: SetKeyContainerName::<Impl, IMPL_OFFSET>,
            SetKeyBlob: SetKeyBlob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateAttestationChallenge2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificatePoliciesImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<ICertificatePolicy>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<ICertificatePolicy>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificatePoliciesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificatePoliciesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificatePoliciesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ICertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ICertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: ICertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificatePolicies as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificatePolicyImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, pvalue: ::core::option::Option<IObjectId>) -> ::windows::core::Result<()>;
    fn ObjectId(&mut self) -> ::windows::core::Result<IObjectId>;
    fn PolicyQualifiers(&mut self) -> ::windows::core::Result<IPolicyQualifiers>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificatePolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificatePolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificatePolicyVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertificatePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn ObjectId<Impl: ICertificatePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyQualifiers<Impl: ICertificatePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyQualifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            ObjectId: ObjectId::<Impl, IMPL_OFFSET>,
            PolicyQualifiers: PolicyQualifiers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificatePolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificationAuthoritiesImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<ICertificationAuthority>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<ICertificationAuthority>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn ComputeSiteCosts(&mut self) -> ::windows::core::Result<()>;
    fn ItemByName(&mut self, strname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<ICertificationAuthority>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificationAuthoritiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificationAuthoritiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificationAuthoritiesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn ComputeSiteCosts<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ComputeSiteCosts().into()
        }
        unsafe extern "system" fn ItemByName<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByName(::core::mem::transmute_copy(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            ComputeSiteCosts: ComputeSiteCosts::<Impl, IMPL_OFFSET>,
            ItemByName: ItemByName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificationAuthorities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificationAuthorityImpl: Sized + IDispatchImpl {
    fn Property(&mut self, property: EnrollmentCAProperty) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificationAuthorityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificationAuthorityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificationAuthorityVtbl {
        unsafe extern "system" fn Property<Impl: ICertificationAuthorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: EnrollmentCAProperty, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Property(::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Property: Property::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificationAuthority as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICryptAttributeImpl: Sized + IDispatchImpl {
    fn InitializeFromObjectId(&mut self, pobjectid: ::core::option::Option<IObjectId>) -> ::windows::core::Result<()>;
    fn InitializeFromValues(&mut self, pattributes: ::core::option::Option<IX509Attributes>) -> ::windows::core::Result<()>;
    fn ObjectId(&mut self) -> ::windows::core::Result<IObjectId>;
    fn Values(&mut self) -> ::windows::core::Result<IX509Attributes>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICryptAttributeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICryptAttributeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICryptAttributeVtbl {
        unsafe extern "system" fn InitializeFromObjectId<Impl: ICryptAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromObjectId(::core::mem::transmute(&pobjectid)).into()
        }
        unsafe extern "system" fn InitializeFromValues<Impl: ICryptAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromValues(::core::mem::transmute(&pattributes)).into()
        }
        unsafe extern "system" fn ObjectId<Impl: ICryptAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Values<Impl: ICryptAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Values() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromObjectId: InitializeFromObjectId::<Impl, IMPL_OFFSET>,
            InitializeFromValues: InitializeFromValues::<Impl, IMPL_OFFSET>,
            ObjectId: ObjectId::<Impl, IMPL_OFFSET>,
            Values: Values::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICryptAttribute as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICryptAttributesImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<ICryptAttribute>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<ICryptAttribute>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn IndexByObjectId(&mut self, pobjectid: ::core::option::Option<IObjectId>) -> ::windows::core::Result<i32>;
    fn AddRange(&mut self, pvalue: ::core::option::Option<ICryptAttributes>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICryptAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICryptAttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICryptAttributesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn IndexByObjectId<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexByObjectId(::core::mem::transmute(&pobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRange<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&pvalue)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            IndexByObjectId: IndexByObjectId::<Impl, IMPL_OFFSET>,
            AddRange: AddRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICryptAttributes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICspAlgorithmImpl: Sized + IDispatchImpl {
    fn GetAlgorithmOid(&mut self, length: i32, algflags: AlgorithmFlags) -> ::windows::core::Result<IObjectId>;
    fn DefaultLength(&mut self) -> ::windows::core::Result<i32>;
    fn IncrementLength(&mut self) -> ::windows::core::Result<i32>;
    fn LongName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Valid(&mut self) -> ::windows::core::Result<i16>;
    fn MaxLength(&mut self) -> ::windows::core::Result<i32>;
    fn MinLength(&mut self) -> ::windows::core::Result<i32>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Type(&mut self) -> ::windows::core::Result<AlgorithmType>;
    fn Operations(&mut self) -> ::windows::core::Result<AlgorithmOperationFlags>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICspAlgorithmVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICspAlgorithmImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICspAlgorithmVtbl {
        unsafe extern "system" fn GetAlgorithmOid<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: i32, algflags: AlgorithmFlags, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlgorithmOid(::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&algflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultLength<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncrementLength<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncrementLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongName<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Valid<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Valid() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxLength<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinLength<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut AlgorithmType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Operations<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut AlgorithmOperationFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operations() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetAlgorithmOid: GetAlgorithmOid::<Impl, IMPL_OFFSET>,
            DefaultLength: DefaultLength::<Impl, IMPL_OFFSET>,
            IncrementLength: IncrementLength::<Impl, IMPL_OFFSET>,
            LongName: LongName::<Impl, IMPL_OFFSET>,
            Valid: Valid::<Impl, IMPL_OFFSET>,
            MaxLength: MaxLength::<Impl, IMPL_OFFSET>,
            MinLength: MinLength::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            Operations: Operations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICspAlgorithm as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICspAlgorithmsImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<ICspAlgorithm>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<ICspAlgorithm>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn ItemByName(&mut self, strname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<ICspAlgorithm>;
    fn IndexByObjectId(&mut self, pobjectid: ::core::option::Option<IObjectId>) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICspAlgorithmsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICspAlgorithmsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICspAlgorithmsVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn ItemByName<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByName(::core::mem::transmute_copy(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexByObjectId<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexByObjectId(::core::mem::transmute(&pobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            ItemByName: ItemByName::<Impl, IMPL_OFFSET>,
            IndexByObjectId: IndexByObjectId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICspAlgorithms as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICspInformationImpl: Sized + IDispatchImpl {
    fn InitializeFromName(&mut self, strname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeFromType(&mut self, r#type: X509ProviderType, palgorithm: ::core::option::Option<IObjectId>, machinecontext: i16) -> ::windows::core::Result<()>;
    fn CspAlgorithms(&mut self) -> ::windows::core::Result<ICspAlgorithms>;
    fn HasHardwareRandomNumberGenerator(&mut self) -> ::windows::core::Result<i16>;
    fn IsHardwareDevice(&mut self) -> ::windows::core::Result<i16>;
    fn IsRemovable(&mut self) -> ::windows::core::Result<i16>;
    fn IsSoftwareDevice(&mut self) -> ::windows::core::Result<i16>;
    fn Valid(&mut self) -> ::windows::core::Result<i16>;
    fn MaxKeyContainerNameLength(&mut self) -> ::windows::core::Result<i32>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Type(&mut self) -> ::windows::core::Result<X509ProviderType>;
    fn Version(&mut self) -> ::windows::core::Result<i32>;
    fn KeySpec(&mut self) -> ::windows::core::Result<X509KeySpec>;
    fn IsSmartCard(&mut self) -> ::windows::core::Result<i16>;
    fn GetDefaultSecurityDescriptor(&mut self, machinecontext: i16) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn LegacyCsp(&mut self) -> ::windows::core::Result<i16>;
    fn GetCspStatusFromOperations(&mut self, palgorithm: ::core::option::Option<IObjectId>, operations: AlgorithmOperationFlags) -> ::windows::core::Result<ICspStatus>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICspInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICspInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICspInformationVtbl {
        unsafe extern "system" fn InitializeFromName<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromName(::core::mem::transmute_copy(&strname)).into()
        }
        unsafe extern "system" fn InitializeFromType<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: X509ProviderType, palgorithm: ::windows::core::RawPtr, machinecontext: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromType(::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&palgorithm), ::core::mem::transmute_copy(&machinecontext)).into()
        }
        unsafe extern "system" fn CspAlgorithms<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CspAlgorithms() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasHardwareRandomNumberGenerator<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasHardwareRandomNumberGenerator() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHardwareDevice<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHardwareDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRemovable<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRemovable() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSoftwareDevice<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSoftwareDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Valid<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Valid() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxKeyContainerNameLength<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxKeyContainerNameLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509ProviderType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeySpec<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509KeySpec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeySpec() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSmartCard<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSmartCard() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultSecurityDescriptor<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultSecurityDescriptor(::core::mem::transmute_copy(&machinecontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LegacyCsp<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LegacyCsp() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCspStatusFromOperations<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palgorithm: ::windows::core::RawPtr, operations: AlgorithmOperationFlags, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCspStatusFromOperations(::core::mem::transmute(&palgorithm), ::core::mem::transmute_copy(&operations)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromName: InitializeFromName::<Impl, IMPL_OFFSET>,
            InitializeFromType: InitializeFromType::<Impl, IMPL_OFFSET>,
            CspAlgorithms: CspAlgorithms::<Impl, IMPL_OFFSET>,
            HasHardwareRandomNumberGenerator: HasHardwareRandomNumberGenerator::<Impl, IMPL_OFFSET>,
            IsHardwareDevice: IsHardwareDevice::<Impl, IMPL_OFFSET>,
            IsRemovable: IsRemovable::<Impl, IMPL_OFFSET>,
            IsSoftwareDevice: IsSoftwareDevice::<Impl, IMPL_OFFSET>,
            Valid: Valid::<Impl, IMPL_OFFSET>,
            MaxKeyContainerNameLength: MaxKeyContainerNameLength::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            KeySpec: KeySpec::<Impl, IMPL_OFFSET>,
            IsSmartCard: IsSmartCard::<Impl, IMPL_OFFSET>,
            GetDefaultSecurityDescriptor: GetDefaultSecurityDescriptor::<Impl, IMPL_OFFSET>,
            LegacyCsp: LegacyCsp::<Impl, IMPL_OFFSET>,
            GetCspStatusFromOperations: GetCspStatusFromOperations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICspInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICspInformationsImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<ICspInformation>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<ICspInformation>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn AddAvailableCsps(&mut self) -> ::windows::core::Result<()>;
    fn ItemByName(&mut self, strname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<ICspInformation>;
    fn GetCspStatusFromProviderName(&mut self, strprovidername: super::super::super::Foundation::BSTR, legacykeyspec: X509KeySpec) -> ::windows::core::Result<ICspStatus>;
    fn GetCspStatusesFromOperations(&mut self, operations: AlgorithmOperationFlags, pcspinformation: ::core::option::Option<ICspInformation>) -> ::windows::core::Result<ICspStatuses>;
    fn GetEncryptionCspAlgorithms(&mut self, pcspinformation: ::core::option::Option<ICspInformation>) -> ::windows::core::Result<ICspAlgorithms>;
    fn GetHashAlgorithms(&mut self, pcspinformation: ::core::option::Option<ICspInformation>) -> ::windows::core::Result<IObjectIds>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICspInformationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICspInformationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICspInformationsVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddAvailableCsps<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAvailableCsps().into()
        }
        unsafe extern "system" fn ItemByName<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppcspinformation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByName(::core::mem::transmute_copy(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcspinformation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCspStatusFromProviderName<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, legacykeyspec: X509KeySpec, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCspStatusFromProviderName(::core::mem::transmute_copy(&strprovidername), ::core::mem::transmute_copy(&legacykeyspec)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCspStatusesFromOperations<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operations: AlgorithmOperationFlags, pcspinformation: ::windows::core::RawPtr, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCspStatusesFromOperations(::core::mem::transmute_copy(&operations), ::core::mem::transmute(&pcspinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEncryptionCspAlgorithms<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcspinformation: ::windows::core::RawPtr, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEncryptionCspAlgorithms(::core::mem::transmute(&pcspinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashAlgorithms<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcspinformation: ::windows::core::RawPtr, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHashAlgorithms(::core::mem::transmute(&pcspinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            AddAvailableCsps: AddAvailableCsps::<Impl, IMPL_OFFSET>,
            ItemByName: ItemByName::<Impl, IMPL_OFFSET>,
            GetCspStatusFromProviderName: GetCspStatusFromProviderName::<Impl, IMPL_OFFSET>,
            GetCspStatusesFromOperations: GetCspStatusesFromOperations::<Impl, IMPL_OFFSET>,
            GetEncryptionCspAlgorithms: GetEncryptionCspAlgorithms::<Impl, IMPL_OFFSET>,
            GetHashAlgorithms: GetHashAlgorithms::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICspInformations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICspStatusImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, pcsp: ::core::option::Option<ICspInformation>, palgorithm: ::core::option::Option<ICspAlgorithm>) -> ::windows::core::Result<()>;
    fn Ordinal(&mut self) -> ::windows::core::Result<i32>;
    fn SetOrdinal(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn CspAlgorithm(&mut self) -> ::windows::core::Result<ICspAlgorithm>;
    fn CspInformation(&mut self) -> ::windows::core::Result<ICspInformation>;
    fn EnrollmentStatus(&mut self) -> ::windows::core::Result<IX509EnrollmentStatus>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICspStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICspStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICspStatusVtbl {
        unsafe extern "system" fn Initialize<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsp: ::windows::core::RawPtr, palgorithm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pcsp), ::core::mem::transmute(&palgorithm)).into()
        }
        unsafe extern "system" fn Ordinal<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ordinal() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrdinal<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrdinal(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn CspAlgorithm<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CspAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CspInformation<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CspInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnrollmentStatus<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnrollmentStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Ordinal: Ordinal::<Impl, IMPL_OFFSET>,
            SetOrdinal: SetOrdinal::<Impl, IMPL_OFFSET>,
            CspAlgorithm: CspAlgorithm::<Impl, IMPL_OFFSET>,
            CspInformation: CspInformation::<Impl, IMPL_OFFSET>,
            EnrollmentStatus: EnrollmentStatus::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICspStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICspStatusesImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<ICspStatus>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<ICspStatus>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn ItemByName(&mut self, strcspname: super::super::super::Foundation::BSTR, stralgorithmname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<ICspStatus>;
    fn ItemByOrdinal(&mut self, ordinal: i32) -> ::windows::core::Result<ICspStatus>;
    fn ItemByOperations(&mut self, strcspname: super::super::super::Foundation::BSTR, stralgorithmname: super::super::super::Foundation::BSTR, operations: AlgorithmOperationFlags) -> ::windows::core::Result<ICspStatus>;
    fn ItemByProvider(&mut self, pcspstatus: ::core::option::Option<ICspStatus>) -> ::windows::core::Result<ICspStatus>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICspStatusesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICspStatusesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICspStatusesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn ItemByName<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strcspname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, stralgorithmname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByName(::core::mem::transmute_copy(&strcspname), ::core::mem::transmute_copy(&stralgorithmname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemByOrdinal<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ordinal: i32, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByOrdinal(::core::mem::transmute_copy(&ordinal)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemByOperations<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strcspname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, stralgorithmname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, operations: AlgorithmOperationFlags, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByOperations(::core::mem::transmute_copy(&strcspname), ::core::mem::transmute_copy(&stralgorithmname), ::core::mem::transmute_copy(&operations)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemByProvider<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcspstatus: ::windows::core::RawPtr, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByProvider(::core::mem::transmute(&pcspstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            ItemByName: ItemByName::<Impl, IMPL_OFFSET>,
            ItemByOrdinal: ItemByOrdinal::<Impl, IMPL_OFFSET>,
            ItemByOperations: ItemByOperations::<Impl, IMPL_OFFSET>,
            ItemByProvider: ItemByProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICspStatuses as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnrollImpl: Sized {
    fn createFilePKCS10WStr(&mut self, dnname: super::super::super::Foundation::PWSTR, usage: super::super::super::Foundation::PWSTR, wszpkcs10filename: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn acceptFilePKCS7WStr(&mut self, wszpkcs7filename: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn createPKCS10WStr(&mut self, dnname: super::super::super::Foundation::PWSTR, usage: super::super::super::Foundation::PWSTR, ppkcs10blob: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
    fn acceptPKCS7Blob(&mut self, pblobpkcs7: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
    fn getCertContextFromPKCS7(&mut self, pblobpkcs7: *mut super::CRYPTOAPI_BLOB) -> *mut super::CERT_CONTEXT;
    fn getMyStore(&mut self) -> *mut ::core::ffi::c_void;
    fn getCAStore(&mut self) -> *mut ::core::ffi::c_void;
    fn getROOTHStore(&mut self) -> *mut ::core::ffi::c_void;
    fn enumProvidersWStr(&mut self, dwindex: i32, dwflags: i32, pbstrprovname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn enumContainersWStr(&mut self, dwindex: i32, pbstr: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn freeRequestInfoBlob(&mut self, pkcs7orpkcs10: super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
    fn MyStoreNameWStr(&mut self, szwname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetMyStoreNameWStr(&mut self, szwname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn MyStoreTypeWStr(&mut self, szwtype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetMyStoreTypeWStr(&mut self, szwtype: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn MyStoreFlags(&mut self, pdwflags: *mut i32) -> ::windows::core::Result<()>;
    fn SetMyStoreFlags(&mut self, dwflags: i32) -> ::windows::core::Result<()>;
    fn CAStoreNameWStr(&mut self, szwname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetCAStoreNameWStr(&mut self, szwname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CAStoreTypeWStr(&mut self, szwtype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetCAStoreTypeWStr(&mut self, szwtype: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CAStoreFlags(&mut self, pdwflags: *mut i32) -> ::windows::core::Result<()>;
    fn SetCAStoreFlags(&mut self, dwflags: i32) -> ::windows::core::Result<()>;
    fn RootStoreNameWStr(&mut self, szwname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetRootStoreNameWStr(&mut self, szwname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RootStoreTypeWStr(&mut self, szwtype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetRootStoreTypeWStr(&mut self, szwtype: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RootStoreFlags(&mut self, pdwflags: *mut i32) -> ::windows::core::Result<()>;
    fn SetRootStoreFlags(&mut self, dwflags: i32) -> ::windows::core::Result<()>;
    fn RequestStoreNameWStr(&mut self, szwname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetRequestStoreNameWStr(&mut self, szwname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RequestStoreTypeWStr(&mut self, szwtype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetRequestStoreTypeWStr(&mut self, szwtype: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RequestStoreFlags(&mut self, pdwflags: *mut i32) -> ::windows::core::Result<()>;
    fn SetRequestStoreFlags(&mut self, dwflags: i32) -> ::windows::core::Result<()>;
    fn ContainerNameWStr(&mut self, szwcontainer: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetContainerNameWStr(&mut self, szwcontainer: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ProviderNameWStr(&mut self, szwprovider: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetProviderNameWStr(&mut self, szwprovider: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ProviderType(&mut self, pdwtype: *mut i32) -> ::windows::core::Result<()>;
    fn SetProviderType(&mut self, dwtype: i32) -> ::windows::core::Result<()>;
    fn KeySpec(&mut self, pdw: *mut i32) -> ::windows::core::Result<()>;
    fn SetKeySpec(&mut self, dw: i32) -> ::windows::core::Result<()>;
    fn ProviderFlags(&mut self, pdwflags: *mut i32) -> ::windows::core::Result<()>;
    fn SetProviderFlags(&mut self, dwflags: i32) -> ::windows::core::Result<()>;
    fn UseExistingKeySet(&mut self, fuseexistingkeys: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetUseExistingKeySet(&mut self, fuseexistingkeys: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GenKeyFlags(&mut self, pdwflags: *mut i32) -> ::windows::core::Result<()>;
    fn SetGenKeyFlags(&mut self, dwflags: i32) -> ::windows::core::Result<()>;
    fn DeleteRequestCert(&mut self, fdelete: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetDeleteRequestCert(&mut self, fdelete: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteCertToUserDS(&mut self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetWriteCertToUserDS(&mut self, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnableT61DNEncoding(&mut self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetEnableT61DNEncoding(&mut self, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteCertToCSP(&mut self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetWriteCertToCSP(&mut self, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SPCFileNameWStr(&mut self, szw: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetSPCFileNameWStr(&mut self, szw: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn PVKFileNameWStr(&mut self, szw: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetPVKFileNameWStr(&mut self, szw: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn HashAlgorithmWStr(&mut self, szw: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetHashAlgorithmWStr(&mut self, szw: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RenewalCertificate(&mut self, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows::core::Result<()>;
    fn SetRenewalCertificate(&mut self, pcertcontext: *const super::CERT_CONTEXT) -> ::windows::core::Result<()>;
    fn AddCertTypeToRequestWStr(&mut self, szw: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddNameValuePairToSignatureWStr(&mut self, name: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddExtensionsToRequest(&mut self, pcertextensions: *mut super::CERT_EXTENSIONS) -> ::windows::core::Result<()>;
    fn AddAuthenticatedAttributesToPKCS7Request(&mut self, pattributes: *mut super::CRYPT_ATTRIBUTES) -> ::windows::core::Result<()>;
    fn CreatePKCS7RequestFromRequest(&mut self, prequest: *mut super::CRYPTOAPI_BLOB, psigningcertcontext: *const super::CERT_CONTEXT, ppkcs7blob: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnrollVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnrollImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnrollVtbl {
        unsafe extern "system" fn createFilePKCS10WStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dnname: super::super::super::Foundation::PWSTR, usage: super::super::super::Foundation::PWSTR, wszpkcs10filename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).createFilePKCS10WStr(::core::mem::transmute_copy(&dnname), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&wszpkcs10filename)).into()
        }
        unsafe extern "system" fn acceptFilePKCS7WStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpkcs7filename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).acceptFilePKCS7WStr(::core::mem::transmute_copy(&wszpkcs7filename)).into()
        }
        unsafe extern "system" fn createPKCS10WStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dnname: super::super::super::Foundation::PWSTR, usage: super::super::super::Foundation::PWSTR, ppkcs10blob: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).createPKCS10WStr(::core::mem::transmute_copy(&dnname), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&ppkcs10blob)).into()
        }
        unsafe extern "system" fn acceptPKCS7Blob<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblobpkcs7: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).acceptPKCS7Blob(::core::mem::transmute_copy(&pblobpkcs7)).into()
        }
        unsafe extern "system" fn getCertContextFromPKCS7<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblobpkcs7: *mut super::CRYPTOAPI_BLOB) -> *mut super::CERT_CONTEXT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getCertContextFromPKCS7(::core::mem::transmute_copy(&pblobpkcs7))
        }
        unsafe extern "system" fn getMyStore<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getMyStore()
        }
        unsafe extern "system" fn getCAStore<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getCAStore()
        }
        unsafe extern "system" fn getROOTHStore<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getROOTHStore()
        }
        unsafe extern "system" fn enumProvidersWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, dwflags: i32, pbstrprovname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).enumProvidersWStr(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbstrprovname)).into()
        }
        unsafe extern "system" fn enumContainersWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, pbstr: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).enumContainersWStr(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pbstr)).into()
        }
        unsafe extern "system" fn freeRequestInfoBlob<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkcs7orpkcs10: super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).freeRequestInfoBlob(::core::mem::transmute_copy(&pkcs7orpkcs10)).into()
        }
        unsafe extern "system" fn MyStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MyStoreNameWStr(::core::mem::transmute_copy(&szwname)).into()
        }
        unsafe extern "system" fn SetMyStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMyStoreNameWStr(::core::mem::transmute_copy(&szwname)).into()
        }
        unsafe extern "system" fn MyStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MyStoreTypeWStr(::core::mem::transmute_copy(&szwtype)).into()
        }
        unsafe extern "system" fn SetMyStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMyStoreTypeWStr(::core::mem::transmute_copy(&szwtype)).into()
        }
        unsafe extern "system" fn MyStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MyStoreFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetMyStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMyStoreFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn CAStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CAStoreNameWStr(::core::mem::transmute_copy(&szwname)).into()
        }
        unsafe extern "system" fn SetCAStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCAStoreNameWStr(::core::mem::transmute_copy(&szwname)).into()
        }
        unsafe extern "system" fn CAStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CAStoreTypeWStr(::core::mem::transmute_copy(&szwtype)).into()
        }
        unsafe extern "system" fn SetCAStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCAStoreTypeWStr(::core::mem::transmute_copy(&szwtype)).into()
        }
        unsafe extern "system" fn CAStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CAStoreFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetCAStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCAStoreFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RootStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RootStoreNameWStr(::core::mem::transmute_copy(&szwname)).into()
        }
        unsafe extern "system" fn SetRootStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRootStoreNameWStr(::core::mem::transmute_copy(&szwname)).into()
        }
        unsafe extern "system" fn RootStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RootStoreTypeWStr(::core::mem::transmute_copy(&szwtype)).into()
        }
        unsafe extern "system" fn SetRootStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRootStoreTypeWStr(::core::mem::transmute_copy(&szwtype)).into()
        }
        unsafe extern "system" fn RootStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RootStoreFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetRootStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRootStoreFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestStoreNameWStr(::core::mem::transmute_copy(&szwname)).into()
        }
        unsafe extern "system" fn SetRequestStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestStoreNameWStr(::core::mem::transmute_copy(&szwname)).into()
        }
        unsafe extern "system" fn RequestStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestStoreTypeWStr(::core::mem::transmute_copy(&szwtype)).into()
        }
        unsafe extern "system" fn SetRequestStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestStoreTypeWStr(::core::mem::transmute_copy(&szwtype)).into()
        }
        unsafe extern "system" fn RequestStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestStoreFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetRequestStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestStoreFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ContainerNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwcontainer: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContainerNameWStr(::core::mem::transmute_copy(&szwcontainer)).into()
        }
        unsafe extern "system" fn SetContainerNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwcontainer: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContainerNameWStr(::core::mem::transmute_copy(&szwcontainer)).into()
        }
        unsafe extern "system" fn ProviderNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwprovider: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProviderNameWStr(::core::mem::transmute_copy(&szwprovider)).into()
        }
        unsafe extern "system" fn SetProviderNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwprovider: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderNameWStr(::core::mem::transmute_copy(&szwprovider)).into()
        }
        unsafe extern "system" fn ProviderType<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProviderType(::core::mem::transmute_copy(&pdwtype)).into()
        }
        unsafe extern "system" fn SetProviderType<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderType(::core::mem::transmute_copy(&dwtype)).into()
        }
        unsafe extern "system" fn KeySpec<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdw: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KeySpec(::core::mem::transmute_copy(&pdw)).into()
        }
        unsafe extern "system" fn SetKeySpec<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dw: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeySpec(::core::mem::transmute_copy(&dw)).into()
        }
        unsafe extern "system" fn ProviderFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProviderFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetProviderFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn UseExistingKeySet<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuseexistingkeys: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UseExistingKeySet(::core::mem::transmute_copy(&fuseexistingkeys)).into()
        }
        unsafe extern "system" fn SetUseExistingKeySet<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuseexistingkeys: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseExistingKeySet(::core::mem::transmute_copy(&fuseexistingkeys)).into()
        }
        unsafe extern "system" fn GenKeyFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenKeyFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetGenKeyFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGenKeyFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeleteRequestCert<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdelete: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteRequestCert(::core::mem::transmute_copy(&fdelete)).into()
        }
        unsafe extern "system" fn SetDeleteRequestCert<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdelete: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeleteRequestCert(::core::mem::transmute_copy(&fdelete)).into()
        }
        unsafe extern "system" fn WriteCertToUserDS<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteCertToUserDS(::core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn SetWriteCertToUserDS<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteCertToUserDS(::core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn EnableT61DNEncoding<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableT61DNEncoding(::core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn SetEnableT61DNEncoding<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableT61DNEncoding(::core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn WriteCertToCSP<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteCertToCSP(::core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn SetWriteCertToCSP<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteCertToCSP(::core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn SPCFileNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SPCFileNameWStr(::core::mem::transmute_copy(&szw)).into()
        }
        unsafe extern "system" fn SetSPCFileNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSPCFileNameWStr(::core::mem::transmute_copy(&szw)).into()
        }
        unsafe extern "system" fn PVKFileNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PVKFileNameWStr(::core::mem::transmute_copy(&szw)).into()
        }
        unsafe extern "system" fn SetPVKFileNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPVKFileNameWStr(::core::mem::transmute_copy(&szw)).into()
        }
        unsafe extern "system" fn HashAlgorithmWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HashAlgorithmWStr(::core::mem::transmute_copy(&szw)).into()
        }
        unsafe extern "system" fn SetHashAlgorithmWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithmWStr(::core::mem::transmute_copy(&szw)).into()
        }
        unsafe extern "system" fn RenewalCertificate<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenewalCertificate(::core::mem::transmute_copy(&ppcertcontext)).into()
        }
        unsafe extern "system" fn SetRenewalCertificate<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertcontext: *const super::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRenewalCertificate(::core::mem::transmute_copy(&pcertcontext)).into()
        }
        unsafe extern "system" fn AddCertTypeToRequestWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddCertTypeToRequestWStr(::core::mem::transmute_copy(&szw)).into()
        }
        unsafe extern "system" fn AddNameValuePairToSignatureWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddNameValuePairToSignatureWStr(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AddExtensionsToRequest<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertextensions: *mut super::CERT_EXTENSIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddExtensionsToRequest(::core::mem::transmute_copy(&pcertextensions)).into()
        }
        unsafe extern "system" fn AddAuthenticatedAttributesToPKCS7Request<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: *mut super::CRYPT_ATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAuthenticatedAttributesToPKCS7Request(::core::mem::transmute_copy(&pattributes)).into()
        }
        unsafe extern "system" fn CreatePKCS7RequestFromRequest<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *mut super::CRYPTOAPI_BLOB, psigningcertcontext: *const super::CERT_CONTEXT, ppkcs7blob: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePKCS7RequestFromRequest(::core::mem::transmute_copy(&prequest), ::core::mem::transmute_copy(&psigningcertcontext), ::core::mem::transmute_copy(&ppkcs7blob)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            createFilePKCS10WStr: createFilePKCS10WStr::<Impl, IMPL_OFFSET>,
            acceptFilePKCS7WStr: acceptFilePKCS7WStr::<Impl, IMPL_OFFSET>,
            createPKCS10WStr: createPKCS10WStr::<Impl, IMPL_OFFSET>,
            acceptPKCS7Blob: acceptPKCS7Blob::<Impl, IMPL_OFFSET>,
            getCertContextFromPKCS7: getCertContextFromPKCS7::<Impl, IMPL_OFFSET>,
            getMyStore: getMyStore::<Impl, IMPL_OFFSET>,
            getCAStore: getCAStore::<Impl, IMPL_OFFSET>,
            getROOTHStore: getROOTHStore::<Impl, IMPL_OFFSET>,
            enumProvidersWStr: enumProvidersWStr::<Impl, IMPL_OFFSET>,
            enumContainersWStr: enumContainersWStr::<Impl, IMPL_OFFSET>,
            freeRequestInfoBlob: freeRequestInfoBlob::<Impl, IMPL_OFFSET>,
            MyStoreNameWStr: MyStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetMyStoreNameWStr: SetMyStoreNameWStr::<Impl, IMPL_OFFSET>,
            MyStoreTypeWStr: MyStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetMyStoreTypeWStr: SetMyStoreTypeWStr::<Impl, IMPL_OFFSET>,
            MyStoreFlags: MyStoreFlags::<Impl, IMPL_OFFSET>,
            SetMyStoreFlags: SetMyStoreFlags::<Impl, IMPL_OFFSET>,
            CAStoreNameWStr: CAStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetCAStoreNameWStr: SetCAStoreNameWStr::<Impl, IMPL_OFFSET>,
            CAStoreTypeWStr: CAStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetCAStoreTypeWStr: SetCAStoreTypeWStr::<Impl, IMPL_OFFSET>,
            CAStoreFlags: CAStoreFlags::<Impl, IMPL_OFFSET>,
            SetCAStoreFlags: SetCAStoreFlags::<Impl, IMPL_OFFSET>,
            RootStoreNameWStr: RootStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetRootStoreNameWStr: SetRootStoreNameWStr::<Impl, IMPL_OFFSET>,
            RootStoreTypeWStr: RootStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetRootStoreTypeWStr: SetRootStoreTypeWStr::<Impl, IMPL_OFFSET>,
            RootStoreFlags: RootStoreFlags::<Impl, IMPL_OFFSET>,
            SetRootStoreFlags: SetRootStoreFlags::<Impl, IMPL_OFFSET>,
            RequestStoreNameWStr: RequestStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetRequestStoreNameWStr: SetRequestStoreNameWStr::<Impl, IMPL_OFFSET>,
            RequestStoreTypeWStr: RequestStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetRequestStoreTypeWStr: SetRequestStoreTypeWStr::<Impl, IMPL_OFFSET>,
            RequestStoreFlags: RequestStoreFlags::<Impl, IMPL_OFFSET>,
            SetRequestStoreFlags: SetRequestStoreFlags::<Impl, IMPL_OFFSET>,
            ContainerNameWStr: ContainerNameWStr::<Impl, IMPL_OFFSET>,
            SetContainerNameWStr: SetContainerNameWStr::<Impl, IMPL_OFFSET>,
            ProviderNameWStr: ProviderNameWStr::<Impl, IMPL_OFFSET>,
            SetProviderNameWStr: SetProviderNameWStr::<Impl, IMPL_OFFSET>,
            ProviderType: ProviderType::<Impl, IMPL_OFFSET>,
            SetProviderType: SetProviderType::<Impl, IMPL_OFFSET>,
            KeySpec: KeySpec::<Impl, IMPL_OFFSET>,
            SetKeySpec: SetKeySpec::<Impl, IMPL_OFFSET>,
            ProviderFlags: ProviderFlags::<Impl, IMPL_OFFSET>,
            SetProviderFlags: SetProviderFlags::<Impl, IMPL_OFFSET>,
            UseExistingKeySet: UseExistingKeySet::<Impl, IMPL_OFFSET>,
            SetUseExistingKeySet: SetUseExistingKeySet::<Impl, IMPL_OFFSET>,
            GenKeyFlags: GenKeyFlags::<Impl, IMPL_OFFSET>,
            SetGenKeyFlags: SetGenKeyFlags::<Impl, IMPL_OFFSET>,
            DeleteRequestCert: DeleteRequestCert::<Impl, IMPL_OFFSET>,
            SetDeleteRequestCert: SetDeleteRequestCert::<Impl, IMPL_OFFSET>,
            WriteCertToUserDS: WriteCertToUserDS::<Impl, IMPL_OFFSET>,
            SetWriteCertToUserDS: SetWriteCertToUserDS::<Impl, IMPL_OFFSET>,
            EnableT61DNEncoding: EnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            SetEnableT61DNEncoding: SetEnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            WriteCertToCSP: WriteCertToCSP::<Impl, IMPL_OFFSET>,
            SetWriteCertToCSP: SetWriteCertToCSP::<Impl, IMPL_OFFSET>,
            SPCFileNameWStr: SPCFileNameWStr::<Impl, IMPL_OFFSET>,
            SetSPCFileNameWStr: SetSPCFileNameWStr::<Impl, IMPL_OFFSET>,
            PVKFileNameWStr: PVKFileNameWStr::<Impl, IMPL_OFFSET>,
            SetPVKFileNameWStr: SetPVKFileNameWStr::<Impl, IMPL_OFFSET>,
            HashAlgorithmWStr: HashAlgorithmWStr::<Impl, IMPL_OFFSET>,
            SetHashAlgorithmWStr: SetHashAlgorithmWStr::<Impl, IMPL_OFFSET>,
            RenewalCertificate: RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate: SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            AddCertTypeToRequestWStr: AddCertTypeToRequestWStr::<Impl, IMPL_OFFSET>,
            AddNameValuePairToSignatureWStr: AddNameValuePairToSignatureWStr::<Impl, IMPL_OFFSET>,
            AddExtensionsToRequest: AddExtensionsToRequest::<Impl, IMPL_OFFSET>,
            AddAuthenticatedAttributesToPKCS7Request: AddAuthenticatedAttributesToPKCS7Request::<Impl, IMPL_OFFSET>,
            CreatePKCS7RequestFromRequest: CreatePKCS7RequestFromRequest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnroll as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnroll2Impl: Sized + IEnrollImpl {
    fn InstallPKCS7Blob(&mut self, pblobpkcs7: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn GetSupportedKeySpec(&mut self, pdwkeyspec: *mut i32) -> ::windows::core::Result<()>;
    fn GetKeyLen(&mut self, fmin: super::super::super::Foundation::BOOL, fexchange: super::super::super::Foundation::BOOL, pdwkeysize: *mut i32) -> ::windows::core::Result<()>;
    fn EnumAlgs(&mut self, dwindex: i32, algclass: i32, pdwalgid: *mut i32) -> ::windows::core::Result<()>;
    fn GetAlgNameWStr(&mut self, algid: i32, ppwsz: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetReuseHardwareKeyIfUnableToGenNew(&mut self, freusehardwarekeyifunabletogennew: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ReuseHardwareKeyIfUnableToGenNew(&mut self, freusehardwarekeyifunabletogennew: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetHashAlgID(&mut self, hashalgid: i32) -> ::windows::core::Result<()>;
    fn HashAlgID(&mut self, hashalgid: *mut i32) -> ::windows::core::Result<()>;
    fn SetHStoreMy(&mut self, hstore: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetHStoreCA(&mut self, hstore: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetHStoreROOT(&mut self, hstore: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetHStoreRequest(&mut self, hstore: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetLimitExchangeKeyToEncipherment(&mut self, flimitexchangekeytoencipherment: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn LimitExchangeKeyToEncipherment(&mut self, flimitexchangekeytoencipherment: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetEnableSMIMECapabilities(&mut self, fenablesmimecapabilities: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnableSMIMECapabilities(&mut self, fenablesmimecapabilities: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnroll2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnroll2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnroll2Vtbl {
        unsafe extern "system" fn InstallPKCS7Blob<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblobpkcs7: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallPKCS7Blob(::core::mem::transmute_copy(&pblobpkcs7)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn GetSupportedKeySpec<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwkeyspec: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSupportedKeySpec(::core::mem::transmute_copy(&pdwkeyspec)).into()
        }
        unsafe extern "system" fn GetKeyLen<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmin: super::super::super::Foundation::BOOL, fexchange: super::super::super::Foundation::BOOL, pdwkeysize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKeyLen(::core::mem::transmute_copy(&fmin), ::core::mem::transmute_copy(&fexchange), ::core::mem::transmute_copy(&pdwkeysize)).into()
        }
        unsafe extern "system" fn EnumAlgs<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, algclass: i32, pdwalgid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumAlgs(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&algclass), ::core::mem::transmute_copy(&pdwalgid)).into()
        }
        unsafe extern "system" fn GetAlgNameWStr<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, algid: i32, ppwsz: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAlgNameWStr(::core::mem::transmute_copy(&algid), ::core::mem::transmute_copy(&ppwsz)).into()
        }
        unsafe extern "system" fn SetReuseHardwareKeyIfUnableToGenNew<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freusehardwarekeyifunabletogennew: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReuseHardwareKeyIfUnableToGenNew(::core::mem::transmute_copy(&freusehardwarekeyifunabletogennew)).into()
        }
        unsafe extern "system" fn ReuseHardwareKeyIfUnableToGenNew<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freusehardwarekeyifunabletogennew: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReuseHardwareKeyIfUnableToGenNew(::core::mem::transmute_copy(&freusehardwarekeyifunabletogennew)).into()
        }
        unsafe extern "system" fn SetHashAlgID<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashalgid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgID(::core::mem::transmute_copy(&hashalgid)).into()
        }
        unsafe extern "system" fn HashAlgID<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashalgid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HashAlgID(::core::mem::transmute_copy(&hashalgid)).into()
        }
        unsafe extern "system" fn SetHStoreMy<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHStoreMy(::core::mem::transmute_copy(&hstore)).into()
        }
        unsafe extern "system" fn SetHStoreCA<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHStoreCA(::core::mem::transmute_copy(&hstore)).into()
        }
        unsafe extern "system" fn SetHStoreROOT<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHStoreROOT(::core::mem::transmute_copy(&hstore)).into()
        }
        unsafe extern "system" fn SetHStoreRequest<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHStoreRequest(::core::mem::transmute_copy(&hstore)).into()
        }
        unsafe extern "system" fn SetLimitExchangeKeyToEncipherment<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flimitexchangekeytoencipherment: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLimitExchangeKeyToEncipherment(::core::mem::transmute_copy(&flimitexchangekeytoencipherment)).into()
        }
        unsafe extern "system" fn LimitExchangeKeyToEncipherment<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flimitexchangekeytoencipherment: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LimitExchangeKeyToEncipherment(::core::mem::transmute_copy(&flimitexchangekeytoencipherment)).into()
        }
        unsafe extern "system" fn SetEnableSMIMECapabilities<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablesmimecapabilities: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableSMIMECapabilities(::core::mem::transmute_copy(&fenablesmimecapabilities)).into()
        }
        unsafe extern "system" fn EnableSMIMECapabilities<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablesmimecapabilities: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableSMIMECapabilities(::core::mem::transmute_copy(&fenablesmimecapabilities)).into()
        }
        Self {
            base: IEnrollVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InstallPKCS7Blob: InstallPKCS7Blob::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            GetSupportedKeySpec: GetSupportedKeySpec::<Impl, IMPL_OFFSET>,
            GetKeyLen: GetKeyLen::<Impl, IMPL_OFFSET>,
            EnumAlgs: EnumAlgs::<Impl, IMPL_OFFSET>,
            GetAlgNameWStr: GetAlgNameWStr::<Impl, IMPL_OFFSET>,
            SetReuseHardwareKeyIfUnableToGenNew: SetReuseHardwareKeyIfUnableToGenNew::<Impl, IMPL_OFFSET>,
            ReuseHardwareKeyIfUnableToGenNew: ReuseHardwareKeyIfUnableToGenNew::<Impl, IMPL_OFFSET>,
            SetHashAlgID: SetHashAlgID::<Impl, IMPL_OFFSET>,
            HashAlgID: HashAlgID::<Impl, IMPL_OFFSET>,
            SetHStoreMy: SetHStoreMy::<Impl, IMPL_OFFSET>,
            SetHStoreCA: SetHStoreCA::<Impl, IMPL_OFFSET>,
            SetHStoreROOT: SetHStoreROOT::<Impl, IMPL_OFFSET>,
            SetHStoreRequest: SetHStoreRequest::<Impl, IMPL_OFFSET>,
            SetLimitExchangeKeyToEncipherment: SetLimitExchangeKeyToEncipherment::<Impl, IMPL_OFFSET>,
            LimitExchangeKeyToEncipherment: LimitExchangeKeyToEncipherment::<Impl, IMPL_OFFSET>,
            SetEnableSMIMECapabilities: SetEnableSMIMECapabilities::<Impl, IMPL_OFFSET>,
            EnableSMIMECapabilities: EnableSMIMECapabilities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnroll2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnroll4Impl: Sized + IEnrollImpl + IEnroll2Impl {
    fn SetThumbPrintWStr(&mut self, thumbprintblob: super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
    fn ThumbPrintWStr(&mut self, thumbprintblob: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
    fn SetPrivateKeyArchiveCertificate(&mut self, pprivatekeyarchivecert: *const super::CERT_CONTEXT) -> ::windows::core::Result<()>;
    fn GetPrivateKeyArchiveCertificate(&mut self) -> *mut super::CERT_CONTEXT;
    fn binaryBlobToString(&mut self, flags: i32, pblobbinary: *mut super::CRYPTOAPI_BLOB, ppwszstring: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn stringToBinaryBlob(&mut self, flags: i32, pwszstring: super::super::super::Foundation::PWSTR, pblobbinary: *mut super::CRYPTOAPI_BLOB, pdwskip: *mut i32, pdwflags: *mut i32) -> ::windows::core::Result<()>;
    fn addExtensionToRequestWStr(&mut self, flags: i32, pwszname: super::super::super::Foundation::PWSTR, pblobvalue: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
    fn addAttributeToRequestWStr(&mut self, flags: i32, pwszname: super::super::super::Foundation::PWSTR, pblobvalue: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
    fn addNameValuePairToRequestWStr(&mut self, flags: i32, pwszname: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn resetExtensions(&mut self) -> ::windows::core::Result<()>;
    fn resetAttributes(&mut self) -> ::windows::core::Result<()>;
    fn createRequestWStr(&mut self, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: super::super::super::Foundation::PWSTR, pwszusage: super::super::super::Foundation::PWSTR, pblobrequest: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
    fn createFileRequestWStr(&mut self, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: super::super::super::Foundation::PWSTR, pwszusage: super::super::super::Foundation::PWSTR, pwszrequestfilename: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn acceptResponseBlob(&mut self, pblobresponse: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
    fn acceptFileResponseWStr(&mut self, pwszresponsefilename: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn getCertContextFromResponseBlob(&mut self, pblobresponse: *mut super::CRYPTOAPI_BLOB, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows::core::Result<()>;
    fn getCertContextFromFileResponseWStr(&mut self, pwszresponsefilename: super::super::super::Foundation::PWSTR, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows::core::Result<()>;
    fn createPFXWStr(&mut self, pwszpassword: super::super::super::Foundation::PWSTR, pblobpfx: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
    fn createFilePFXWStr(&mut self, pwszpassword: super::super::super::Foundation::PWSTR, pwszpfxfilename: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn setPendingRequestInfoWStr(&mut self, lrequestid: i32, pwszcadns: super::super::super::Foundation::PWSTR, pwszcaname: super::super::super::Foundation::PWSTR, pwszfriendlyname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn enumPendingRequestWStr(&mut self, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY, ppproperty: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn removePendingRequestWStr(&mut self, thumbprintblob: super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
    fn GetKeyLenEx(&mut self, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC, pdwkeysize: *mut i32) -> ::windows::core::Result<()>;
    fn InstallPKCS7BlobEx(&mut self, pblobpkcs7: *mut super::CRYPTOAPI_BLOB, plcertinstalled: *mut i32) -> ::windows::core::Result<()>;
    fn AddCertTypeToRequestWStrEx(&mut self, ltype: ADDED_CERT_TYPE, pwszoidorname: super::super::super::Foundation::PWSTR, lmajorversion: i32, fminorversion: super::super::super::Foundation::BOOL, lminorversion: i32) -> ::windows::core::Result<()>;
    fn getProviderTypeWStr(&mut self, pwszprovname: super::super::super::Foundation::PWSTR, plprovtype: *mut i32) -> ::windows::core::Result<()>;
    fn addBlobPropertyToCertificateWStr(&mut self, lpropertyid: i32, lreserved: i32, pblobproperty: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::Result<()>;
    fn SetSignerCertificate(&mut self, psignercert: *const super::CERT_CONTEXT) -> ::windows::core::Result<()>;
    fn SetClientId(&mut self, lclientid: i32) -> ::windows::core::Result<()>;
    fn ClientId(&mut self, plclientid: *mut i32) -> ::windows::core::Result<()>;
    fn SetIncludeSubjectKeyID(&mut self, finclude: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IncludeSubjectKeyID(&mut self, pfinclude: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnroll4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnroll4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnroll4Vtbl {
        unsafe extern "system" fn SetThumbPrintWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprintblob: super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbPrintWStr(::core::mem::transmute_copy(&thumbprintblob)).into()
        }
        unsafe extern "system" fn ThumbPrintWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprintblob: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ThumbPrintWStr(::core::mem::transmute_copy(&thumbprintblob)).into()
        }
        unsafe extern "system" fn SetPrivateKeyArchiveCertificate<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprivatekeyarchivecert: *const super::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateKeyArchiveCertificate(::core::mem::transmute_copy(&pprivatekeyarchivecert)).into()
        }
        unsafe extern "system" fn GetPrivateKeyArchiveCertificate<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut super::CERT_CONTEXT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrivateKeyArchiveCertificate()
        }
        unsafe extern "system" fn binaryBlobToString<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pblobbinary: *mut super::CRYPTOAPI_BLOB, ppwszstring: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).binaryBlobToString(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pblobbinary), ::core::mem::transmute_copy(&ppwszstring)).into()
        }
        unsafe extern "system" fn stringToBinaryBlob<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pwszstring: super::super::super::Foundation::PWSTR, pblobbinary: *mut super::CRYPTOAPI_BLOB, pdwskip: *mut i32, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).stringToBinaryBlob(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pwszstring), ::core::mem::transmute_copy(&pblobbinary), ::core::mem::transmute_copy(&pdwskip), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn addExtensionToRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pwszname: super::super::super::Foundation::PWSTR, pblobvalue: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addExtensionToRequestWStr(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pblobvalue)).into()
        }
        unsafe extern "system" fn addAttributeToRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pwszname: super::super::super::Foundation::PWSTR, pblobvalue: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addAttributeToRequestWStr(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pblobvalue)).into()
        }
        unsafe extern "system" fn addNameValuePairToRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pwszname: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addNameValuePairToRequestWStr(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwszvalue)).into()
        }
        unsafe extern "system" fn resetExtensions<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).resetExtensions().into()
        }
        unsafe extern "system" fn resetAttributes<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).resetAttributes().into()
        }
        unsafe extern "system" fn createRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: super::super::super::Foundation::PWSTR, pwszusage: super::super::super::Foundation::PWSTR, pblobrequest: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).createRequestWStr(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pwszdnname), ::core::mem::transmute_copy(&pwszusage), ::core::mem::transmute_copy(&pblobrequest)).into()
        }
        unsafe extern "system" fn createFileRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: super::super::super::Foundation::PWSTR, pwszusage: super::super::super::Foundation::PWSTR, pwszrequestfilename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).createFileRequestWStr(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pwszdnname), ::core::mem::transmute_copy(&pwszusage), ::core::mem::transmute_copy(&pwszrequestfilename)).into()
        }
        unsafe extern "system" fn acceptResponseBlob<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblobresponse: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).acceptResponseBlob(::core::mem::transmute_copy(&pblobresponse)).into()
        }
        unsafe extern "system" fn acceptFileResponseWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszresponsefilename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).acceptFileResponseWStr(::core::mem::transmute_copy(&pwszresponsefilename)).into()
        }
        unsafe extern "system" fn getCertContextFromResponseBlob<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblobresponse: *mut super::CRYPTOAPI_BLOB, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getCertContextFromResponseBlob(::core::mem::transmute_copy(&pblobresponse), ::core::mem::transmute_copy(&ppcertcontext)).into()
        }
        unsafe extern "system" fn getCertContextFromFileResponseWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszresponsefilename: super::super::super::Foundation::PWSTR, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getCertContextFromFileResponseWStr(::core::mem::transmute_copy(&pwszresponsefilename), ::core::mem::transmute_copy(&ppcertcontext)).into()
        }
        unsafe extern "system" fn createPFXWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpassword: super::super::super::Foundation::PWSTR, pblobpfx: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).createPFXWStr(::core::mem::transmute_copy(&pwszpassword), ::core::mem::transmute_copy(&pblobpfx)).into()
        }
        unsafe extern "system" fn createFilePFXWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpassword: super::super::super::Foundation::PWSTR, pwszpfxfilename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).createFilePFXWStr(::core::mem::transmute_copy(&pwszpassword), ::core::mem::transmute_copy(&pwszpfxfilename)).into()
        }
        unsafe extern "system" fn setPendingRequestInfoWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrequestid: i32, pwszcadns: super::super::super::Foundation::PWSTR, pwszcaname: super::super::super::Foundation::PWSTR, pwszfriendlyname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setPendingRequestInfoWStr(::core::mem::transmute_copy(&lrequestid), ::core::mem::transmute_copy(&pwszcadns), ::core::mem::transmute_copy(&pwszcaname), ::core::mem::transmute_copy(&pwszfriendlyname)).into()
        }
        unsafe extern "system" fn enumPendingRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY, ppproperty: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).enumPendingRequestWStr(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&ldesiredproperty), ::core::mem::transmute_copy(&ppproperty)).into()
        }
        unsafe extern "system" fn removePendingRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprintblob: super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).removePendingRequestWStr(::core::mem::transmute_copy(&thumbprintblob)).into()
        }
        unsafe extern "system" fn GetKeyLenEx<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC, pdwkeysize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKeyLenEx(::core::mem::transmute_copy(&lsizespec), ::core::mem::transmute_copy(&lkeyspec), ::core::mem::transmute_copy(&pdwkeysize)).into()
        }
        unsafe extern "system" fn InstallPKCS7BlobEx<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblobpkcs7: *mut super::CRYPTOAPI_BLOB, plcertinstalled: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallPKCS7BlobEx(::core::mem::transmute_copy(&pblobpkcs7), ::core::mem::transmute_copy(&plcertinstalled)).into()
        }
        unsafe extern "system" fn AddCertTypeToRequestWStrEx<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltype: ADDED_CERT_TYPE, pwszoidorname: super::super::super::Foundation::PWSTR, lmajorversion: i32, fminorversion: super::super::super::Foundation::BOOL, lminorversion: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddCertTypeToRequestWStrEx(::core::mem::transmute_copy(&ltype), ::core::mem::transmute_copy(&pwszoidorname), ::core::mem::transmute_copy(&lmajorversion), ::core::mem::transmute_copy(&fminorversion), ::core::mem::transmute_copy(&lminorversion)).into()
        }
        unsafe extern "system" fn getProviderTypeWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprovname: super::super::super::Foundation::PWSTR, plprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getProviderTypeWStr(::core::mem::transmute_copy(&pwszprovname), ::core::mem::transmute_copy(&plprovtype)).into()
        }
        unsafe extern "system" fn addBlobPropertyToCertificateWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropertyid: i32, lreserved: i32, pblobproperty: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addBlobPropertyToCertificateWStr(::core::mem::transmute_copy(&lpropertyid), ::core::mem::transmute_copy(&lreserved), ::core::mem::transmute_copy(&pblobproperty)).into()
        }
        unsafe extern "system" fn SetSignerCertificate<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignercert: *const super::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignerCertificate(::core::mem::transmute_copy(&psignercert)).into()
        }
        unsafe extern "system" fn SetClientId<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lclientid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientId(::core::mem::transmute_copy(&lclientid)).into()
        }
        unsafe extern "system" fn ClientId<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclientid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClientId(::core::mem::transmute_copy(&plclientid)).into()
        }
        unsafe extern "system" fn SetIncludeSubjectKeyID<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finclude: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeSubjectKeyID(::core::mem::transmute_copy(&finclude)).into()
        }
        unsafe extern "system" fn IncludeSubjectKeyID<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinclude: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IncludeSubjectKeyID(::core::mem::transmute_copy(&pfinclude)).into()
        }
        Self {
            base: IEnroll2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetThumbPrintWStr: SetThumbPrintWStr::<Impl, IMPL_OFFSET>,
            ThumbPrintWStr: ThumbPrintWStr::<Impl, IMPL_OFFSET>,
            SetPrivateKeyArchiveCertificate: SetPrivateKeyArchiveCertificate::<Impl, IMPL_OFFSET>,
            GetPrivateKeyArchiveCertificate: GetPrivateKeyArchiveCertificate::<Impl, IMPL_OFFSET>,
            binaryBlobToString: binaryBlobToString::<Impl, IMPL_OFFSET>,
            stringToBinaryBlob: stringToBinaryBlob::<Impl, IMPL_OFFSET>,
            addExtensionToRequestWStr: addExtensionToRequestWStr::<Impl, IMPL_OFFSET>,
            addAttributeToRequestWStr: addAttributeToRequestWStr::<Impl, IMPL_OFFSET>,
            addNameValuePairToRequestWStr: addNameValuePairToRequestWStr::<Impl, IMPL_OFFSET>,
            resetExtensions: resetExtensions::<Impl, IMPL_OFFSET>,
            resetAttributes: resetAttributes::<Impl, IMPL_OFFSET>,
            createRequestWStr: createRequestWStr::<Impl, IMPL_OFFSET>,
            createFileRequestWStr: createFileRequestWStr::<Impl, IMPL_OFFSET>,
            acceptResponseBlob: acceptResponseBlob::<Impl, IMPL_OFFSET>,
            acceptFileResponseWStr: acceptFileResponseWStr::<Impl, IMPL_OFFSET>,
            getCertContextFromResponseBlob: getCertContextFromResponseBlob::<Impl, IMPL_OFFSET>,
            getCertContextFromFileResponseWStr: getCertContextFromFileResponseWStr::<Impl, IMPL_OFFSET>,
            createPFXWStr: createPFXWStr::<Impl, IMPL_OFFSET>,
            createFilePFXWStr: createFilePFXWStr::<Impl, IMPL_OFFSET>,
            setPendingRequestInfoWStr: setPendingRequestInfoWStr::<Impl, IMPL_OFFSET>,
            enumPendingRequestWStr: enumPendingRequestWStr::<Impl, IMPL_OFFSET>,
            removePendingRequestWStr: removePendingRequestWStr::<Impl, IMPL_OFFSET>,
            GetKeyLenEx: GetKeyLenEx::<Impl, IMPL_OFFSET>,
            InstallPKCS7BlobEx: InstallPKCS7BlobEx::<Impl, IMPL_OFFSET>,
            AddCertTypeToRequestWStrEx: AddCertTypeToRequestWStrEx::<Impl, IMPL_OFFSET>,
            getProviderTypeWStr: getProviderTypeWStr::<Impl, IMPL_OFFSET>,
            addBlobPropertyToCertificateWStr: addBlobPropertyToCertificateWStr::<Impl, IMPL_OFFSET>,
            SetSignerCertificate: SetSignerCertificate::<Impl, IMPL_OFFSET>,
            SetClientId: SetClientId::<Impl, IMPL_OFFSET>,
            ClientId: ClientId::<Impl, IMPL_OFFSET>,
            SetIncludeSubjectKeyID: SetIncludeSubjectKeyID::<Impl, IMPL_OFFSET>,
            IncludeSubjectKeyID: IncludeSubjectKeyID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnroll4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumCERTVIEWATTRIBUTEImpl: Sized + IDispatchImpl {
    fn Next(&mut self, pindex: *mut i32) -> ::windows::core::Result<()>;
    fn GetName(&mut self, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetValue(&mut self, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: i32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumCERTVIEWATTRIBUTE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumCERTVIEWATTRIBUTEVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCERTVIEWATTRIBUTEImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCERTVIEWATTRIBUTEVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCERTVIEWATTRIBUTEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&pindex)).into()
        }
        unsafe extern "system" fn GetName<Impl: IEnumCERTVIEWATTRIBUTEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pstrout)).into()
        }
        unsafe extern "system" fn GetValue<Impl: IEnumCERTVIEWATTRIBUTEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&pstrout)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCERTVIEWATTRIBUTEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCERTVIEWATTRIBUTEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCERTVIEWATTRIBUTEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCERTVIEWATTRIBUTE as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumCERTVIEWCOLUMNImpl: Sized + IDispatchImpl {
    fn Next(&mut self, pindex: *mut i32) -> ::windows::core::Result<()>;
    fn GetName(&mut self, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetDisplayName(&mut self, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetType(&mut self, ptype: *mut i32) -> ::windows::core::Result<()>;
    fn IsIndexed(&mut self, pindexed: *mut i32) -> ::windows::core::Result<()>;
    fn GetMaxLength(&mut self, pmaxlength: *mut i32) -> ::windows::core::Result<()>;
    fn GetValue(&mut self, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: i32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumCERTVIEWCOLUMN>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumCERTVIEWCOLUMNVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCERTVIEWCOLUMNImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCERTVIEWCOLUMNVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&pindex)).into()
        }
        unsafe extern "system" fn GetName<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pstrout)).into()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayName(::core::mem::transmute_copy(&pstrout)).into()
        }
        unsafe extern "system" fn GetType<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetType(::core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn IsIndexed<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexed: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsIndexed(::core::mem::transmute_copy(&pindexed)).into()
        }
        unsafe extern "system" fn GetMaxLength<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMaxLength(::core::mem::transmute_copy(&pmaxlength)).into()
        }
        unsafe extern "system" fn GetValue<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetDisplayName: GetDisplayName::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            IsIndexed: IsIndexed::<Impl, IMPL_OFFSET>,
            GetMaxLength: GetMaxLength::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCERTVIEWCOLUMN as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumCERTVIEWEXTENSIONImpl: Sized + IDispatchImpl {
    fn Next(&mut self, pindex: *mut i32) -> ::windows::core::Result<()>;
    fn GetName(&mut self, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetFlags(&mut self, pflags: *mut i32) -> ::windows::core::Result<()>;
    fn GetValue(&mut self, r#type: CERT_PROPERTY_TYPE, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: i32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumCERTVIEWEXTENSION>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumCERTVIEWEXTENSIONVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCERTVIEWEXTENSIONImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCERTVIEWEXTENSIONVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&pindex)).into()
        }
        unsafe extern "system" fn GetName<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pstrout)).into()
        }
        unsafe extern "system" fn GetFlags<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFlags(::core::mem::transmute_copy(&pflags)).into()
        }
        unsafe extern "system" fn GetValue<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: CERT_PROPERTY_TYPE, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCERTVIEWEXTENSION as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumCERTVIEWROWImpl: Sized + IDispatchImpl {
    fn Next(&mut self, pindex: *mut i32) -> ::windows::core::Result<()>;
    fn EnumCertViewColumn(&mut self) -> ::windows::core::Result<IEnumCERTVIEWCOLUMN>;
    fn EnumCertViewAttribute(&mut self, flags: i32) -> ::windows::core::Result<IEnumCERTVIEWATTRIBUTE>;
    fn EnumCertViewExtension(&mut self, flags: i32) -> ::windows::core::Result<IEnumCERTVIEWEXTENSION>;
    fn Skip(&mut self, celt: i32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumCERTVIEWROW>;
    fn GetMaxIndex(&mut self, pindex: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumCERTVIEWROWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCERTVIEWROWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCERTVIEWROWVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&pindex)).into()
        }
        unsafe extern "system" fn EnumCertViewColumn<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCertViewColumn() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCertViewAttribute<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCertViewAttribute(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCertViewExtension<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCertViewExtension(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxIndex<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMaxIndex(::core::mem::transmute_copy(&pindex)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            EnumCertViewColumn: EnumCertViewColumn::<Impl, IMPL_OFFSET>,
            EnumCertViewAttribute: EnumCertViewAttribute::<Impl, IMPL_OFFSET>,
            EnumCertViewExtension: EnumCertViewExtension::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetMaxIndex: GetMaxIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCERTVIEWROW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INDESPolicyImpl: Sized {
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
    fn Uninitialize(&mut self) -> ::windows::core::Result<()>;
    fn GenerateChallenge(&mut self, pwsztemplate: super::super::super::Foundation::PWSTR, pwszparams: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn VerifyRequest(&mut self, pctbrequest: *mut CERTTRANSBLOB, pctbsigningcertencoded: *mut CERTTRANSBLOB, pwsztemplate: super::super::super::Foundation::PWSTR, pwsztransactionid: super::super::super::Foundation::PWSTR, pfverified: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Notify(&mut self, pwszchallenge: super::super::super::Foundation::PWSTR, pwsztransactionid: super::super::super::Foundation::PWSTR, disposition: X509SCEPDisposition, lasthresult: i32, pctbissuedcertencoded: *mut CERTTRANSBLOB) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INDESPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDESPolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDESPolicyVtbl {
        unsafe extern "system" fn Initialize<Impl: INDESPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn Uninitialize<Impl: INDESPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Uninitialize().into()
        }
        unsafe extern "system" fn GenerateChallenge<Impl: INDESPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztemplate: super::super::super::Foundation::PWSTR, pwszparams: super::super::super::Foundation::PWSTR, ppwszresponse: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateChallenge(::core::mem::transmute_copy(&pwsztemplate), ::core::mem::transmute_copy(&pwszparams)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifyRequest<Impl: INDESPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctbrequest: *mut CERTTRANSBLOB, pctbsigningcertencoded: *mut CERTTRANSBLOB, pwsztemplate: super::super::super::Foundation::PWSTR, pwsztransactionid: super::super::super::Foundation::PWSTR, pfverified: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VerifyRequest(::core::mem::transmute_copy(&pctbrequest), ::core::mem::transmute_copy(&pctbsigningcertencoded), ::core::mem::transmute_copy(&pwsztemplate), ::core::mem::transmute_copy(&pwsztransactionid), ::core::mem::transmute_copy(&pfverified)).into()
        }
        unsafe extern "system" fn Notify<Impl: INDESPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszchallenge: super::super::super::Foundation::PWSTR, pwsztransactionid: super::super::super::Foundation::PWSTR, disposition: X509SCEPDisposition, lasthresult: i32, pctbissuedcertencoded: *mut CERTTRANSBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&pwszchallenge), ::core::mem::transmute_copy(&pwsztransactionid), ::core::mem::transmute_copy(&disposition), ::core::mem::transmute_copy(&lasthresult), ::core::mem::transmute_copy(&pctbissuedcertencoded)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Uninitialize: Uninitialize::<Impl, IMPL_OFFSET>,
            GenerateChallenge: GenerateChallenge::<Impl, IMPL_OFFSET>,
            VerifyRequest: VerifyRequest::<Impl, IMPL_OFFSET>,
            Notify: Notify::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDESPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOCSPAdminImpl: Sized + IDispatchImpl {
    fn OCSPServiceProperties(&mut self) -> ::windows::core::Result<IOCSPPropertyCollection>;
    fn OCSPCAConfigurationCollection(&mut self) -> ::windows::core::Result<IOCSPCAConfigurationCollection>;
    fn GetConfiguration(&mut self, bstrservername: super::super::super::Foundation::BSTR, bforce: i16) -> ::windows::core::Result<()>;
    fn SetConfiguration(&mut self, bstrservername: super::super::super::Foundation::BSTR, bforce: i16) -> ::windows::core::Result<()>;
    fn GetMyRoles(&mut self, bstrservername: super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn Ping(&mut self, bstrservername: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSecurity(&mut self, bstrservername: super::super::super::Foundation::BSTR, bstrval: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetSecurity(&mut self, bstrservername: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetSigningCertificates(&mut self, bstrservername: super::super::super::Foundation::BSTR, pcacertvar: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn GetHashAlgorithms(&mut self, bstrservername: super::super::super::Foundation::BSTR, bstrcaid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOCSPAdminVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOCSPAdminImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOCSPAdminVtbl {
        unsafe extern "system" fn OCSPServiceProperties<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OCSPServiceProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OCSPCAConfigurationCollection<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OCSPCAConfigurationCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfiguration<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bforce: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConfiguration(::core::mem::transmute_copy(&bstrservername), ::core::mem::transmute_copy(&bforce)).into()
        }
        unsafe extern "system" fn SetConfiguration<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bforce: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConfiguration(::core::mem::transmute_copy(&bstrservername), ::core::mem::transmute_copy(&bforce)).into()
        }
        unsafe extern "system" fn GetMyRoles<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, proles: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMyRoles(::core::mem::transmute_copy(&bstrservername)) {
                ::core::result::Result::Ok(ok__) => {
                    *proles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ping<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Ping(::core::mem::transmute_copy(&bstrservername)).into()
        }
        unsafe extern "system" fn SetSecurity<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute_copy(&bstrservername), ::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn GetSecurity<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurity(::core::mem::transmute_copy(&bstrservername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningCertificates<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pcacertvar: *const super::super::super::System::Com::VARIANT, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSigningCertificates(::core::mem::transmute_copy(&bstrservername), ::core::mem::transmute_copy(&pcacertvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashAlgorithms<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcaid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHashAlgorithms(::core::mem::transmute_copy(&bstrservername), ::core::mem::transmute_copy(&bstrcaid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OCSPServiceProperties: OCSPServiceProperties::<Impl, IMPL_OFFSET>,
            OCSPCAConfigurationCollection: OCSPCAConfigurationCollection::<Impl, IMPL_OFFSET>,
            GetConfiguration: GetConfiguration::<Impl, IMPL_OFFSET>,
            SetConfiguration: SetConfiguration::<Impl, IMPL_OFFSET>,
            GetMyRoles: GetMyRoles::<Impl, IMPL_OFFSET>,
            Ping: Ping::<Impl, IMPL_OFFSET>,
            SetSecurity: SetSecurity::<Impl, IMPL_OFFSET>,
            GetSecurity: GetSecurity::<Impl, IMPL_OFFSET>,
            GetSigningCertificates: GetSigningCertificates::<Impl, IMPL_OFFSET>,
            GetHashAlgorithms: GetHashAlgorithms::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOCSPAdmin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOCSPCAConfigurationImpl: Sized + IDispatchImpl {
    fn Identifier(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn CACertificate(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn HashAlgorithm(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetHashAlgorithm(&mut self, newval: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SigningFlags(&mut self) -> ::windows::core::Result<u32>;
    fn SetSigningFlags(&mut self, newval: u32) -> ::windows::core::Result<()>;
    fn SigningCertificate(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn SetSigningCertificate(&mut self, newval: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ReminderDuration(&mut self) -> ::windows::core::Result<u32>;
    fn SetReminderDuration(&mut self, newval: u32) -> ::windows::core::Result<()>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<u32>;
    fn CSPName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn KeySpec(&mut self) -> ::windows::core::Result<u32>;
    fn ProviderCLSID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetProviderCLSID(&mut self, newval: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ProviderProperties(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn SetProviderProperties(&mut self, newval: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Modified(&mut self) -> ::windows::core::Result<i16>;
    fn LocalRevocationInformation(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn SetLocalRevocationInformation(&mut self, newval: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SigningCertificateTemplate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetSigningCertificateTemplate(&mut self, newval: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CAConfig(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetCAConfig(&mut self, newval: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOCSPCAConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOCSPCAConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOCSPCAConfigurationVtbl {
        unsafe extern "system" fn Identifier<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identifier() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CACertificate<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CACertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SigningFlags<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SigningFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningFlags<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSigningFlags(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SigningCertificate<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SigningCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningCertificate<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSigningCertificate(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ReminderDuration<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReminderDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReminderDuration<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReminderDuration(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ErrorCode<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSPName<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSPName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeySpec<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeySpec() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderCLSID<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderCLSID<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderCLSID(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ProviderProperties<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderProperties<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderProperties(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Modified<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modified() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalRevocationInformation<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalRevocationInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalRevocationInformation<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalRevocationInformation(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SigningCertificateTemplate<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SigningCertificateTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningCertificateTemplate<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSigningCertificateTemplate(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn CAConfig<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CAConfig() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCAConfig<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCAConfig(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Identifier: Identifier::<Impl, IMPL_OFFSET>,
            CACertificate: CACertificate::<Impl, IMPL_OFFSET>,
            HashAlgorithm: HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            SigningFlags: SigningFlags::<Impl, IMPL_OFFSET>,
            SetSigningFlags: SetSigningFlags::<Impl, IMPL_OFFSET>,
            SigningCertificate: SigningCertificate::<Impl, IMPL_OFFSET>,
            SetSigningCertificate: SetSigningCertificate::<Impl, IMPL_OFFSET>,
            ReminderDuration: ReminderDuration::<Impl, IMPL_OFFSET>,
            SetReminderDuration: SetReminderDuration::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
            CSPName: CSPName::<Impl, IMPL_OFFSET>,
            KeySpec: KeySpec::<Impl, IMPL_OFFSET>,
            ProviderCLSID: ProviderCLSID::<Impl, IMPL_OFFSET>,
            SetProviderCLSID: SetProviderCLSID::<Impl, IMPL_OFFSET>,
            ProviderProperties: ProviderProperties::<Impl, IMPL_OFFSET>,
            SetProviderProperties: SetProviderProperties::<Impl, IMPL_OFFSET>,
            Modified: Modified::<Impl, IMPL_OFFSET>,
            LocalRevocationInformation: LocalRevocationInformation::<Impl, IMPL_OFFSET>,
            SetLocalRevocationInformation: SetLocalRevocationInformation::<Impl, IMPL_OFFSET>,
            SigningCertificateTemplate: SigningCertificateTemplate::<Impl, IMPL_OFFSET>,
            SetSigningCertificateTemplate: SetSigningCertificateTemplate::<Impl, IMPL_OFFSET>,
            CAConfig: CAConfig::<Impl, IMPL_OFFSET>,
            SetCAConfig: SetCAConfig::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOCSPCAConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOCSPCAConfigurationCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn ItemByName(&mut self, bstridentifier: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn CreateCAConfiguration(&mut self, bstridentifier: super::super::super::Foundation::BSTR, varcacert: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IOCSPCAConfiguration>;
    fn DeleteCAConfiguration(&mut self, bstridentifier: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOCSPCAConfigurationCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOCSPCAConfigurationCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOCSPCAConfigurationCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IOCSPCAConfigurationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IOCSPCAConfigurationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IOCSPCAConfigurationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemByName<Impl: IOCSPCAConfigurationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstridentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByName(::core::mem::transmute_copy(&bstridentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCAConfiguration<Impl: IOCSPCAConfigurationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstridentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varcacert: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCAConfiguration(::core::mem::transmute_copy(&bstridentifier), ::core::mem::transmute_copy(&varcacert)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCAConfiguration<Impl: IOCSPCAConfigurationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstridentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteCAConfiguration(::core::mem::transmute_copy(&bstridentifier)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            ItemByName: ItemByName::<Impl, IMPL_OFFSET>,
            CreateCAConfiguration: CreateCAConfiguration::<Impl, IMPL_OFFSET>,
            DeleteCAConfiguration: DeleteCAConfiguration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOCSPCAConfigurationCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOCSPPropertyImpl: Sized + IDispatchImpl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn SetValue(&mut self, newval: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Modified(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOCSPPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOCSPPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOCSPPropertyVtbl {
        unsafe extern "system" fn Name<Impl: IOCSPPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IOCSPPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IOCSPPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Modified<Impl: IOCSPPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modified() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Modified: Modified::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOCSPProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOCSPPropertyCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn ItemByName(&mut self, bstrpropname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn CreateProperty(&mut self, bstrpropname: super::super::super::Foundation::BSTR, pvarpropvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IOCSPProperty>;
    fn DeleteProperty(&mut self, bstrpropname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeFromProperties(&mut self, pvarproperties: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetAllProperties(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOCSPPropertyCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOCSPPropertyCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOCSPPropertyCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemByName<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByName(::core::mem::transmute_copy(&bstrpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProperty<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvarpropvalue: *const super::super::super::System::Com::VARIANT, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProperty(::core::mem::transmute_copy(&bstrpropname), ::core::mem::transmute_copy(&pvarpropvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProperty<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteProperty(::core::mem::transmute_copy(&bstrpropname)).into()
        }
        unsafe extern "system" fn InitializeFromProperties<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarproperties: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromProperties(::core::mem::transmute_copy(&pvarproperties)).into()
        }
        unsafe extern "system" fn GetAllProperties<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarproperties: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            ItemByName: ItemByName::<Impl, IMPL_OFFSET>,
            CreateProperty: CreateProperty::<Impl, IMPL_OFFSET>,
            DeleteProperty: DeleteProperty::<Impl, IMPL_OFFSET>,
            InitializeFromProperties: InitializeFromProperties::<Impl, IMPL_OFFSET>,
            GetAllProperties: GetAllProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOCSPPropertyCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IObjectIdImpl: Sized + IDispatchImpl {
    fn InitializeFromName(&mut self, name: CERTENROLL_OBJECTID) -> ::windows::core::Result<()>;
    fn InitializeFromValue(&mut self, strvalue: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeFromAlgorithmName(&mut self, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags, algflags: AlgorithmFlags, stralgorithmname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<CERTENROLL_OBJECTID>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetFriendlyName(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetAlgorithmName(&mut self, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IObjectIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectIdVtbl {
        unsafe extern "system" fn InitializeFromName<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: CERTENROLL_OBJECTID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn InitializeFromValue<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromValue(::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn InitializeFromAlgorithmName<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags, algflags: AlgorithmFlags, stralgorithmname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromAlgorithmName(::core::mem::transmute_copy(&groupid), ::core::mem::transmute_copy(&keyflags), ::core::mem::transmute_copy(&algflags), ::core::mem::transmute_copy(&stralgorithmname)).into()
        }
        unsafe extern "system" fn Name<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut CERTENROLL_OBJECTID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Value<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlgorithmName<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags, pstralgorithmname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlgorithmName(::core::mem::transmute_copy(&groupid), ::core::mem::transmute_copy(&keyflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstralgorithmname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromName: InitializeFromName::<Impl, IMPL_OFFSET>,
            InitializeFromValue: InitializeFromValue::<Impl, IMPL_OFFSET>,
            InitializeFromAlgorithmName: InitializeFromAlgorithmName::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            SetFriendlyName: SetFriendlyName::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            GetAlgorithmName: GetAlgorithmName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IObjectIdsImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<IObjectId>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<IObjectId>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn AddRange(&mut self, pvalue: ::core::option::Option<IObjectIds>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IObjectIdsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectIdsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectIdsVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddRange<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&pvalue)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            AddRange: AddRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectIds as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPolicyQualifierImpl: Sized + IDispatchImpl {
    fn InitializeEncode(&mut self, strqualifier: super::super::super::Foundation::BSTR, r#type: PolicyQualifierType) -> ::windows::core::Result<()>;
    fn ObjectId(&mut self) -> ::windows::core::Result<IObjectId>;
    fn Qualifier(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Type(&mut self) -> ::windows::core::Result<PolicyQualifierType>;
    fn RawData(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPolicyQualifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolicyQualifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolicyQualifierVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IPolicyQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqualifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: PolicyQualifierType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute_copy(&strqualifier), ::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn ObjectId<Impl: IPolicyQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifier<Impl: IPolicyQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Qualifier() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IPolicyQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut PolicyQualifierType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawData<Impl: IPolicyQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawData(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            ObjectId: ObjectId::<Impl, IMPL_OFFSET>,
            Qualifier: Qualifier::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            RawData: RawData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolicyQualifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPolicyQualifiersImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<IPolicyQualifier>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<IPolicyQualifier>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPolicyQualifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolicyQualifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolicyQualifiersVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IPolicyQualifiersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IPolicyQualifiersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IPolicyQualifiersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IPolicyQualifiersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: IPolicyQualifiersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: IPolicyQualifiersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolicyQualifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISignerCertificateImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, machinecontext: i16, verifytype: X509PrivateKeyVerify, encoding: EncodingType, strcertificate: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Certificate(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn PrivateKey(&mut self) -> ::windows::core::Result<IX509PrivateKey>;
    fn Silent(&mut self) -> ::windows::core::Result<i16>;
    fn SetSilent(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn ParentWindow(&mut self) -> ::windows::core::Result<i32>;
    fn SetParentWindow(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn UIContextMessage(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetUIContextMessage(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetPin(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SignatureInformation(&mut self) -> ::windows::core::Result<IX509SignatureInformation>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISignerCertificateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISignerCertificateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISignerCertificateVtbl {
        unsafe extern "system" fn Initialize<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, verifytype: X509PrivateKeyVerify, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&machinecontext), ::core::mem::transmute_copy(&verifytype), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strcertificate)).into()
        }
        unsafe extern "system" fn Certificate<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Certificate(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateKey<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateKey() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Silent<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Silent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSilent<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSilent(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ParentWindow<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentWindow<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParentWindow(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UIContextMessage<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIContextMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUIContextMessage<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUIContextMessage(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetPin<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPin(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SignatureInformation<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignatureInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Certificate: Certificate::<Impl, IMPL_OFFSET>,
            PrivateKey: PrivateKey::<Impl, IMPL_OFFSET>,
            Silent: Silent::<Impl, IMPL_OFFSET>,
            SetSilent: SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow: ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow: SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage: UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage: SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SetPin: SetPin::<Impl, IMPL_OFFSET>,
            SignatureInformation: SignatureInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISignerCertificate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISignerCertificatesImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<ISignerCertificate>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<ISignerCertificate>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn Find(&mut self, psignercert: ::core::option::Option<ISignerCertificate>) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISignerCertificatesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISignerCertificatesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISignerCertificatesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn Find<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignercert: ::windows::core::RawPtr, pisignercert: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Find(::core::mem::transmute(&psignercert)) {
                ::core::result::Result::Ok(ok__) => {
                    *pisignercert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            Find: Find::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISignerCertificates as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISmimeCapabilitiesImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<ISmimeCapability>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<ISmimeCapability>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn AddFromCsp(&mut self, pvalue: ::core::option::Option<ICspInformation>) -> ::windows::core::Result<()>;
    fn AddAvailableSmimeCapabilities(&mut self, machinecontext: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISmimeCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmimeCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmimeCapabilitiesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddFromCsp<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFromCsp(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn AddAvailableSmimeCapabilities<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAvailableSmimeCapabilities(::core::mem::transmute_copy(&machinecontext)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            AddFromCsp: AddFromCsp::<Impl, IMPL_OFFSET>,
            AddAvailableSmimeCapabilities: AddAvailableSmimeCapabilities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmimeCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISmimeCapabilityImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, pobjectid: ::core::option::Option<IObjectId>, bitcount: i32) -> ::windows::core::Result<()>;
    fn ObjectId(&mut self) -> ::windows::core::Result<IObjectId>;
    fn BitCount(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISmimeCapabilityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmimeCapabilityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmimeCapabilityVtbl {
        unsafe extern "system" fn Initialize<Impl: ISmimeCapabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, bitcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pobjectid), ::core::mem::transmute_copy(&bitcount)).into()
        }
        unsafe extern "system" fn ObjectId<Impl: ISmimeCapabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitCount<Impl: ISmimeCapabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            ObjectId: ObjectId::<Impl, IMPL_OFFSET>,
            BitCount: BitCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmimeCapability as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX500DistinguishedNameImpl: Sized + IDispatchImpl {
    fn Decode(&mut self, strencodedname: super::super::super::Foundation::BSTR, encoding: EncodingType, nameflags: X500NameFlags) -> ::windows::core::Result<()>;
    fn Encode(&mut self, strname: super::super::super::Foundation::BSTR, nameflags: X500NameFlags) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn EncodedName(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX500DistinguishedNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX500DistinguishedNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX500DistinguishedNameVtbl {
        unsafe extern "system" fn Decode<Impl: IX500DistinguishedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodedname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, nameflags: X500NameFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Decode(::core::mem::transmute_copy(&strencodedname), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&nameflags)).into()
        }
        unsafe extern "system" fn Encode<Impl: IX500DistinguishedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nameflags: X500NameFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Encode(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&nameflags)).into()
        }
        unsafe extern "system" fn Name<Impl: IX500DistinguishedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodedName<Impl: IX500DistinguishedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodedName(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Decode: Decode::<Impl, IMPL_OFFSET>,
            Encode: Encode::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            EncodedName: EncodedName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX500DistinguishedName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, pobjectid: ::core::option::Option<IObjectId>, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ObjectId(&mut self) -> ::windows::core::Result<IObjectId>;
    fn RawData(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509AttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pobjectid), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn ObjectId<Impl: IX509AttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawData<Impl: IX509AttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawData(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            ObjectId: ObjectId::<Impl, IMPL_OFFSET>,
            RawData: RawData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509Attribute as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeArchiveKeyImpl: Sized + IDispatchImpl + IX509AttributeImpl {
    fn InitializeEncode(&mut self, pkey: ::core::option::Option<IX509PrivateKey>, encoding: EncodingType, strcaxcert: super::super::super::Foundation::BSTR, palgorithm: ::core::option::Option<IObjectId>, encryptionstrength: i32) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EncryptedKeyBlob(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn EncryptionAlgorithm(&mut self) -> ::windows::core::Result<IObjectId>;
    fn EncryptionStrength(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeArchiveKeyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeArchiveKeyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeArchiveKeyVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509AttributeArchiveKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: ::windows::core::RawPtr, encoding: EncodingType, strcaxcert: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, palgorithm: ::windows::core::RawPtr, encryptionstrength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute(&pkey), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strcaxcert), ::core::mem::transmute(&palgorithm), ::core::mem::transmute_copy(&encryptionstrength)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeArchiveKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn EncryptedKeyBlob<Impl: IX509AttributeArchiveKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptedKeyBlob(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptionAlgorithm<Impl: IX509AttributeArchiveKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptionAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptionStrength<Impl: IX509AttributeArchiveKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptionStrength() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509AttributeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            EncryptedKeyBlob: EncryptedKeyBlob::<Impl, IMPL_OFFSET>,
            EncryptionAlgorithm: EncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptionStrength: EncryptionStrength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeArchiveKey as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeArchiveKeyHashImpl: Sized + IDispatchImpl + IX509AttributeImpl {
    fn InitializeEncodeFromEncryptedKeyBlob(&mut self, encoding: EncodingType, strencryptedkeyblob: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EncryptedKeyHashBlob(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeArchiveKeyHashVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeArchiveKeyHashImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeArchiveKeyHashVtbl {
        unsafe extern "system" fn InitializeEncodeFromEncryptedKeyBlob<Impl: IX509AttributeArchiveKeyHashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencryptedkeyblob: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncodeFromEncryptedKeyBlob(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencryptedkeyblob)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeArchiveKeyHashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn EncryptedKeyHashBlob<Impl: IX509AttributeArchiveKeyHashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptedKeyHashBlob(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509AttributeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncodeFromEncryptedKeyBlob: InitializeEncodeFromEncryptedKeyBlob::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            EncryptedKeyHashBlob: EncryptedKeyHashBlob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeArchiveKeyHash as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeClientIdImpl: Sized + IDispatchImpl + IX509AttributeImpl {
    fn InitializeEncode(&mut self, clientid: RequestClientInfoClientId, strmachinednsname: super::super::super::Foundation::BSTR, strusersamname: super::super::super::Foundation::BSTR, strprocessname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClientId(&mut self) -> ::windows::core::Result<RequestClientInfoClientId>;
    fn MachineDnsName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn UserSamName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ProcessName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeClientIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeClientIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeClientIdVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509AttributeClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientid: RequestClientInfoClientId, strmachinednsname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strusersamname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strprocessname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute_copy(&clientid), ::core::mem::transmute_copy(&strmachinednsname), ::core::mem::transmute_copy(&strusersamname), ::core::mem::transmute_copy(&strprocessname)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn ClientId<Impl: IX509AttributeClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut RequestClientInfoClientId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MachineDnsName<Impl: IX509AttributeClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MachineDnsName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSamName<Impl: IX509AttributeClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserSamName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessName<Impl: IX509AttributeClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509AttributeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            ClientId: ClientId::<Impl, IMPL_OFFSET>,
            MachineDnsName: MachineDnsName::<Impl, IMPL_OFFSET>,
            UserSamName: UserSamName::<Impl, IMPL_OFFSET>,
            ProcessName: ProcessName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeClientId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeCspProviderImpl: Sized + IDispatchImpl + IX509AttributeImpl {
    fn InitializeEncode(&mut self, keyspec: X509KeySpec, strprovidername: super::super::super::Foundation::BSTR, encoding: EncodingType, strsignature: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn KeySpec(&mut self) -> ::windows::core::Result<X509KeySpec>;
    fn ProviderName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Signature(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeCspProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeCspProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeCspProviderVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509AttributeCspProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keyspec: X509KeySpec, strprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, strsignature: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute_copy(&keyspec), ::core::mem::transmute_copy(&strprovidername), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strsignature)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeCspProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn KeySpec<Impl: IX509AttributeCspProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509KeySpec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeySpec() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderName<Impl: IX509AttributeCspProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Signature<Impl: IX509AttributeCspProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509AttributeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            KeySpec: KeySpec::<Impl, IMPL_OFFSET>,
            ProviderName: ProviderName::<Impl, IMPL_OFFSET>,
            Signature: Signature::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeCspProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeExtensionsImpl: Sized + IDispatchImpl + IX509AttributeImpl {
    fn InitializeEncode(&mut self, pextensions: ::core::option::Option<IX509Extensions>) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn X509Extensions(&mut self) -> ::windows::core::Result<IX509Extensions>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeExtensionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeExtensionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeExtensionsVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509AttributeExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextensions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute(&pextensions)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn X509Extensions<Impl: IX509AttributeExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X509Extensions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509AttributeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            X509Extensions: X509Extensions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeExtensions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeOSVersionImpl: Sized + IDispatchImpl + IX509AttributeImpl {
    fn InitializeEncode(&mut self, strosversion: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OSVersion(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeOSVersionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeOSVersionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeOSVersionVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509AttributeOSVersionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strosversion: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute_copy(&strosversion)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeOSVersionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn OSVersion<Impl: IX509AttributeOSVersionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OSVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509AttributeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            OSVersion: OSVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeOSVersion as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeRenewalCertificateImpl: Sized + IDispatchImpl + IX509AttributeImpl {
    fn InitializeEncode(&mut self, encoding: EncodingType, strcert: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RenewalCertificate(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeRenewalCertificateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeRenewalCertificateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeRenewalCertificateVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509AttributeRenewalCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strcert: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strcert)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeRenewalCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn RenewalCertificate<Impl: IX509AttributeRenewalCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenewalCertificate(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509AttributeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            RenewalCertificate: RenewalCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeRenewalCertificate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributesImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<IX509Attribute>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<IX509Attribute>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IX509AttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IX509AttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IX509AttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IX509AttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: IX509AttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: IX509AttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509Attributes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()>;
    fn Encode(&mut self) -> ::windows::core::Result<()>;
    fn ResetForEncode(&mut self) -> ::windows::core::Result<()>;
    fn GetInnerRequest(&mut self, level: InnerRequestLevel) -> ::windows::core::Result<IX509CertificateRequest>;
    fn Type(&mut self) -> ::windows::core::Result<X509RequestType>;
    fn EnrollmentContext(&mut self) -> ::windows::core::Result<X509CertificateEnrollmentContext>;
    fn Silent(&mut self) -> ::windows::core::Result<i16>;
    fn SetSilent(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn ParentWindow(&mut self) -> ::windows::core::Result<i32>;
    fn SetParentWindow(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn UIContextMessage(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetUIContextMessage(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SuppressDefaults(&mut self) -> ::windows::core::Result<i16>;
    fn SetSuppressDefaults(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn RenewalCertificate(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetRenewalCertificate(&mut self, encoding: EncodingType, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClientId(&mut self) -> ::windows::core::Result<RequestClientInfoClientId>;
    fn SetClientId(&mut self, value: RequestClientInfoClientId) -> ::windows::core::Result<()>;
    fn CspInformations(&mut self) -> ::windows::core::Result<ICspInformations>;
    fn SetCspInformations(&mut self, pvalue: ::core::option::Option<ICspInformations>) -> ::windows::core::Result<()>;
    fn HashAlgorithm(&mut self) -> ::windows::core::Result<IObjectId>;
    fn SetHashAlgorithm(&mut self, pvalue: ::core::option::Option<IObjectId>) -> ::windows::core::Result<()>;
    fn AlternateSignatureAlgorithm(&mut self) -> ::windows::core::Result<i16>;
    fn SetAlternateSignatureAlgorithm(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn RawData(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn Encode<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Encode().into()
        }
        unsafe extern "system" fn ResetForEncode<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetForEncode().into()
        }
        unsafe extern "system" fn GetInnerRequest<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: InnerRequestLevel, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInnerRequest(::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509RequestType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnrollmentContext<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnrollmentContext() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Silent<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Silent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSilent<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSilent(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ParentWindow<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentWindow<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParentWindow(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UIContextMessage<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIContextMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUIContextMessage<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUIContextMessage(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SuppressDefaults<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuppressDefaults() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuppressDefaults<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuppressDefaults(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn RenewalCertificate<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenewalCertificate(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenewalCertificate<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRenewalCertificate(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ClientId<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut RequestClientInfoClientId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientId<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: RequestClientInfoClientId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientId(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn CspInformations<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CspInformations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCspInformations<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCspInformations(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn AlternateSignatureAlgorithm<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateSignatureAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlternateSignatureAlgorithm<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlternateSignatureAlgorithm(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn RawData<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawData(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Encode: Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode: ResetForEncode::<Impl, IMPL_OFFSET>,
            GetInnerRequest: GetInnerRequest::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            EnrollmentContext: EnrollmentContext::<Impl, IMPL_OFFSET>,
            Silent: Silent::<Impl, IMPL_OFFSET>,
            SetSilent: SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow: ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow: SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage: UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage: SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SuppressDefaults: SuppressDefaults::<Impl, IMPL_OFFSET>,
            SetSuppressDefaults: SetSuppressDefaults::<Impl, IMPL_OFFSET>,
            RenewalCertificate: RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate: SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            ClientId: ClientId::<Impl, IMPL_OFFSET>,
            SetClientId: SetClientId::<Impl, IMPL_OFFSET>,
            CspInformations: CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations: SetCspInformations::<Impl, IMPL_OFFSET>,
            HashAlgorithm: HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm: AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm: SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            RawData: RawData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestCertificateImpl: Sized + IDispatchImpl + IX509CertificateRequestImpl + IX509CertificateRequestPkcs10Impl {
    fn CheckPublicKeySignature(&mut self, ppublickey: ::core::option::Option<IX509PublicKey>) -> ::windows::core::Result<()>;
    fn Issuer(&mut self) -> ::windows::core::Result<IX500DistinguishedName>;
    fn SetIssuer(&mut self, pvalue: ::core::option::Option<IX500DistinguishedName>) -> ::windows::core::Result<()>;
    fn NotBefore(&mut self) -> ::windows::core::Result<f64>;
    fn SetNotBefore(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn NotAfter(&mut self) -> ::windows::core::Result<f64>;
    fn SetNotAfter(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn SerialNumber(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetSerialNumber(&mut self, encoding: EncodingType, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SignerCertificate(&mut self) -> ::windows::core::Result<ISignerCertificate>;
    fn SetSignerCertificate(&mut self, pvalue: ::core::option::Option<ISignerCertificate>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestCertificateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestCertificateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestCertificateVtbl {
        unsafe extern "system" fn CheckPublicKeySignature<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppublickey: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckPublicKeySignature(::core::mem::transmute(&ppublickey)).into()
        }
        unsafe extern "system" fn Issuer<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Issuer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIssuer<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIssuer(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn NotBefore<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotBefore() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotBefore<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotBefore(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn NotAfter<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotAfter() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotAfter<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotAfter(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SerialNumber<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerialNumber(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSerialNumber<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSerialNumber(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SignerCertificate<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignerCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignerCertificate<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignerCertificate(::core::mem::transmute(&pvalue)).into()
        }
        Self {
            base: IX509CertificateRequestPkcs10Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CheckPublicKeySignature: CheckPublicKeySignature::<Impl, IMPL_OFFSET>,
            Issuer: Issuer::<Impl, IMPL_OFFSET>,
            SetIssuer: SetIssuer::<Impl, IMPL_OFFSET>,
            NotBefore: NotBefore::<Impl, IMPL_OFFSET>,
            SetNotBefore: SetNotBefore::<Impl, IMPL_OFFSET>,
            NotAfter: NotAfter::<Impl, IMPL_OFFSET>,
            SetNotAfter: SetNotAfter::<Impl, IMPL_OFFSET>,
            SerialNumber: SerialNumber::<Impl, IMPL_OFFSET>,
            SetSerialNumber: SetSerialNumber::<Impl, IMPL_OFFSET>,
            SignerCertificate: SignerCertificate::<Impl, IMPL_OFFSET>,
            SetSignerCertificate: SetSignerCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestCertificate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestCertificate2Impl: Sized + IDispatchImpl + IX509CertificateRequestImpl + IX509CertificateRequestPkcs10Impl + IX509CertificateRequestCertificateImpl {
    fn InitializeFromTemplate(&mut self, context: X509CertificateEnrollmentContext, ppolicyserver: ::core::option::Option<IX509EnrollmentPolicyServer>, ptemplate: ::core::option::Option<IX509CertificateTemplate>) -> ::windows::core::Result<()>;
    fn InitializeFromPrivateKeyTemplate(&mut self, context: X509CertificateEnrollmentContext, pprivatekey: ::core::option::Option<IX509PrivateKey>, ppolicyserver: ::core::option::Option<IX509EnrollmentPolicyServer>, ptemplate: ::core::option::Option<IX509CertificateTemplate>) -> ::windows::core::Result<()>;
    fn PolicyServer(&mut self) -> ::windows::core::Result<IX509EnrollmentPolicyServer>;
    fn Template(&mut self) -> ::windows::core::Result<IX509CertificateTemplate>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestCertificate2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestCertificate2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestCertificate2Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Impl: IX509CertificateRequestCertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromTemplate(::core::mem::transmute_copy(&context), ::core::mem::transmute(&ppolicyserver), ::core::mem::transmute(&ptemplate)).into()
        }
        unsafe extern "system" fn InitializeFromPrivateKeyTemplate<Impl: IX509CertificateRequestCertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, pprivatekey: ::windows::core::RawPtr, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromPrivateKeyTemplate(::core::mem::transmute_copy(&context), ::core::mem::transmute(&pprivatekey), ::core::mem::transmute(&ppolicyserver), ::core::mem::transmute(&ptemplate)).into()
        }
        unsafe extern "system" fn PolicyServer<Impl: IX509CertificateRequestCertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppolicyserver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyServer() {
                ::core::result::Result::Ok(ok__) => {
                    *pppolicyserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Template<Impl: IX509CertificateRequestCertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Template() {
                ::core::result::Result::Ok(ok__) => {
                    *pptemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509CertificateRequestCertificateVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromTemplate: InitializeFromTemplate::<Impl, IMPL_OFFSET>,
            InitializeFromPrivateKeyTemplate: InitializeFromPrivateKeyTemplate::<Impl, IMPL_OFFSET>,
            PolicyServer: PolicyServer::<Impl, IMPL_OFFSET>,
            Template: Template::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestCertificate2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestCmcImpl: Sized + IDispatchImpl + IX509CertificateRequestImpl + IX509CertificateRequestPkcs7Impl {
    fn InitializeFromInnerRequestTemplateName(&mut self, pinnerrequest: ::core::option::Option<IX509CertificateRequest>, strtemplatename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TemplateObjectId(&mut self) -> ::windows::core::Result<IObjectId>;
    fn NullSigned(&mut self) -> ::windows::core::Result<i16>;
    fn CryptAttributes(&mut self) -> ::windows::core::Result<ICryptAttributes>;
    fn NameValuePairs(&mut self) -> ::windows::core::Result<IX509NameValuePairs>;
    fn X509Extensions(&mut self) -> ::windows::core::Result<IX509Extensions>;
    fn CriticalExtensions(&mut self) -> ::windows::core::Result<IObjectIds>;
    fn SuppressOids(&mut self) -> ::windows::core::Result<IObjectIds>;
    fn TransactionId(&mut self) -> ::windows::core::Result<i32>;
    fn SetTransactionId(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn SenderNonce(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetSenderNonce(&mut self, encoding: EncodingType, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SignatureInformation(&mut self) -> ::windows::core::Result<IX509SignatureInformation>;
    fn ArchivePrivateKey(&mut self) -> ::windows::core::Result<i16>;
    fn SetArchivePrivateKey(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn KeyArchivalCertificate(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetKeyArchivalCertificate(&mut self, encoding: EncodingType, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EncryptionAlgorithm(&mut self) -> ::windows::core::Result<IObjectId>;
    fn SetEncryptionAlgorithm(&mut self, pvalue: ::core::option::Option<IObjectId>) -> ::windows::core::Result<()>;
    fn EncryptionStrength(&mut self) -> ::windows::core::Result<i32>;
    fn SetEncryptionStrength(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn EncryptedKeyHash(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SignerCertificates(&mut self) -> ::windows::core::Result<ISignerCertificates>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestCmcVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestCmcImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestCmcVtbl {
        unsafe extern "system" fn InitializeFromInnerRequestTemplateName<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinnerrequest: ::windows::core::RawPtr, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromInnerRequestTemplateName(::core::mem::transmute(&pinnerrequest), ::core::mem::transmute_copy(&strtemplatename)).into()
        }
        unsafe extern "system" fn TemplateObjectId<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TemplateObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NullSigned<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NullSigned() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CryptAttributes<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CryptAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NameValuePairs<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameValuePairs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X509Extensions<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X509Extensions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CriticalExtensions<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CriticalExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuppressOids<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuppressOids() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransactionId<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransactionId(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SenderNonce<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderNonce(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderNonce<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderNonce(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SignatureInformation<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignatureInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArchivePrivateKey<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchivePrivateKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchivePrivateKey<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetArchivePrivateKey(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn KeyArchivalCertificate<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyArchivalCertificate(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyArchivalCertificate<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyArchivalCertificate(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn EncryptionAlgorithm<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptionAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptionAlgorithm<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncryptionAlgorithm(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn EncryptionStrength<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptionStrength() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptionStrength<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncryptionStrength(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn EncryptedKeyHash<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptedKeyHash(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignerCertificates<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignerCertificates() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509CertificateRequestPkcs7Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromInnerRequestTemplateName: InitializeFromInnerRequestTemplateName::<Impl, IMPL_OFFSET>,
            TemplateObjectId: TemplateObjectId::<Impl, IMPL_OFFSET>,
            NullSigned: NullSigned::<Impl, IMPL_OFFSET>,
            CryptAttributes: CryptAttributes::<Impl, IMPL_OFFSET>,
            NameValuePairs: NameValuePairs::<Impl, IMPL_OFFSET>,
            X509Extensions: X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions: CriticalExtensions::<Impl, IMPL_OFFSET>,
            SuppressOids: SuppressOids::<Impl, IMPL_OFFSET>,
            TransactionId: TransactionId::<Impl, IMPL_OFFSET>,
            SetTransactionId: SetTransactionId::<Impl, IMPL_OFFSET>,
            SenderNonce: SenderNonce::<Impl, IMPL_OFFSET>,
            SetSenderNonce: SetSenderNonce::<Impl, IMPL_OFFSET>,
            SignatureInformation: SignatureInformation::<Impl, IMPL_OFFSET>,
            ArchivePrivateKey: ArchivePrivateKey::<Impl, IMPL_OFFSET>,
            SetArchivePrivateKey: SetArchivePrivateKey::<Impl, IMPL_OFFSET>,
            KeyArchivalCertificate: KeyArchivalCertificate::<Impl, IMPL_OFFSET>,
            SetKeyArchivalCertificate: SetKeyArchivalCertificate::<Impl, IMPL_OFFSET>,
            EncryptionAlgorithm: EncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptionAlgorithm: SetEncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptionStrength: EncryptionStrength::<Impl, IMPL_OFFSET>,
            SetEncryptionStrength: SetEncryptionStrength::<Impl, IMPL_OFFSET>,
            EncryptedKeyHash: EncryptedKeyHash::<Impl, IMPL_OFFSET>,
            SignerCertificates: SignerCertificates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestCmc as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestCmc2Impl: Sized + IDispatchImpl + IX509CertificateRequestImpl + IX509CertificateRequestPkcs7Impl + IX509CertificateRequestCmcImpl {
    fn InitializeFromTemplate(&mut self, context: X509CertificateEnrollmentContext, ppolicyserver: ::core::option::Option<IX509EnrollmentPolicyServer>, ptemplate: ::core::option::Option<IX509CertificateTemplate>) -> ::windows::core::Result<()>;
    fn InitializeFromInnerRequestTemplate(&mut self, pinnerrequest: ::core::option::Option<IX509CertificateRequest>, ppolicyserver: ::core::option::Option<IX509EnrollmentPolicyServer>, ptemplate: ::core::option::Option<IX509CertificateTemplate>) -> ::windows::core::Result<()>;
    fn PolicyServer(&mut self) -> ::windows::core::Result<IX509EnrollmentPolicyServer>;
    fn Template(&mut self) -> ::windows::core::Result<IX509CertificateTemplate>;
    fn CheckSignature(&mut self, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> ::windows::core::Result<()>;
    fn CheckCertificateSignature(&mut self, psignercertificate: ::core::option::Option<ISignerCertificate>, validatecertificatechain: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestCmc2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestCmc2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestCmc2Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Impl: IX509CertificateRequestCmc2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromTemplate(::core::mem::transmute_copy(&context), ::core::mem::transmute(&ppolicyserver), ::core::mem::transmute(&ptemplate)).into()
        }
        unsafe extern "system" fn InitializeFromInnerRequestTemplate<Impl: IX509CertificateRequestCmc2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinnerrequest: ::windows::core::RawPtr, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromInnerRequestTemplate(::core::mem::transmute(&pinnerrequest), ::core::mem::transmute(&ppolicyserver), ::core::mem::transmute(&ptemplate)).into()
        }
        unsafe extern "system" fn PolicyServer<Impl: IX509CertificateRequestCmc2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppolicyserver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyServer() {
                ::core::result::Result::Ok(ok__) => {
                    *pppolicyserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Template<Impl: IX509CertificateRequestCmc2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Template() {
                ::core::result::Result::Ok(ok__) => {
                    *pptemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckSignature<Impl: IX509CertificateRequestCmc2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckSignature(::core::mem::transmute_copy(&allowedsignaturetypes)).into()
        }
        unsafe extern "system" fn CheckCertificateSignature<Impl: IX509CertificateRequestCmc2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignercertificate: ::windows::core::RawPtr, validatecertificatechain: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckCertificateSignature(::core::mem::transmute(&psignercertificate), ::core::mem::transmute_copy(&validatecertificatechain)).into()
        }
        Self {
            base: IX509CertificateRequestCmcVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromTemplate: InitializeFromTemplate::<Impl, IMPL_OFFSET>,
            InitializeFromInnerRequestTemplate: InitializeFromInnerRequestTemplate::<Impl, IMPL_OFFSET>,
            PolicyServer: PolicyServer::<Impl, IMPL_OFFSET>,
            Template: Template::<Impl, IMPL_OFFSET>,
            CheckSignature: CheckSignature::<Impl, IMPL_OFFSET>,
            CheckCertificateSignature: CheckCertificateSignature::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestCmc2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestPkcs10Impl: Sized + IDispatchImpl + IX509CertificateRequestImpl {
    fn InitializeFromTemplateName(&mut self, context: X509CertificateEnrollmentContext, strtemplatename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeFromPrivateKey(&mut self, context: X509CertificateEnrollmentContext, pprivatekey: ::core::option::Option<IX509PrivateKey>, strtemplatename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeFromPublicKey(&mut self, context: X509CertificateEnrollmentContext, ppublickey: ::core::option::Option<IX509PublicKey>, strtemplatename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeFromCertificate(&mut self, context: X509CertificateEnrollmentContext, strcertificate: super::super::super::Foundation::BSTR, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, strencodeddata: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
    fn CheckSignature(&mut self, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> ::windows::core::Result<()>;
    fn IsSmartCard(&mut self) -> ::windows::core::Result<i16>;
    fn TemplateObjectId(&mut self) -> ::windows::core::Result<IObjectId>;
    fn PublicKey(&mut self) -> ::windows::core::Result<IX509PublicKey>;
    fn PrivateKey(&mut self) -> ::windows::core::Result<IX509PrivateKey>;
    fn NullSigned(&mut self) -> ::windows::core::Result<i16>;
    fn ReuseKey(&mut self) -> ::windows::core::Result<i16>;
    fn OldCertificate(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Subject(&mut self) -> ::windows::core::Result<IX500DistinguishedName>;
    fn SetSubject(&mut self, pvalue: ::core::option::Option<IX500DistinguishedName>) -> ::windows::core::Result<()>;
    fn CspStatuses(&mut self) -> ::windows::core::Result<ICspStatuses>;
    fn SmimeCapabilities(&mut self) -> ::windows::core::Result<i16>;
    fn SetSmimeCapabilities(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn SignatureInformation(&mut self) -> ::windows::core::Result<IX509SignatureInformation>;
    fn KeyContainerNamePrefix(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetKeyContainerNamePrefix(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CryptAttributes(&mut self) -> ::windows::core::Result<ICryptAttributes>;
    fn X509Extensions(&mut self) -> ::windows::core::Result<IX509Extensions>;
    fn CriticalExtensions(&mut self) -> ::windows::core::Result<IObjectIds>;
    fn SuppressOids(&mut self) -> ::windows::core::Result<IObjectIds>;
    fn RawDataToBeSigned(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Signature(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetCspStatuses(&mut self, keyspec: X509KeySpec) -> ::windows::core::Result<ICspStatuses>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestPkcs10Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestPkcs10Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestPkcs10Vtbl {
        unsafe extern "system" fn InitializeFromTemplateName<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromTemplateName(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&strtemplatename)).into()
        }
        unsafe extern "system" fn InitializeFromPrivateKey<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, pprivatekey: ::windows::core::RawPtr, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromPrivateKey(::core::mem::transmute_copy(&context), ::core::mem::transmute(&pprivatekey), ::core::mem::transmute_copy(&strtemplatename)).into()
        }
        unsafe extern "system" fn InitializeFromPublicKey<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppublickey: ::windows::core::RawPtr, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromPublicKey(::core::mem::transmute_copy(&context), ::core::mem::transmute(&ppublickey), ::core::mem::transmute_copy(&strtemplatename)).into()
        }
        unsafe extern "system" fn InitializeFromCertificate<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromCertificate(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&strcertificate), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&inheritoptions)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&strencodeddata), ::core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn CheckSignature<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckSignature(::core::mem::transmute_copy(&allowedsignaturetypes)).into()
        }
        unsafe extern "system" fn IsSmartCard<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSmartCard() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TemplateObjectId<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TemplateObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublicKey<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublicKey() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateKey<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateKey() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NullSigned<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NullSigned() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReuseKey<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReuseKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldCertificate<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldCertificate(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubject(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn CspStatuses<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CspStatuses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmimeCapabilities<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmimeCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmimeCapabilities<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmimeCapabilities(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SignatureInformation<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignatureInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyContainerNamePrefix<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyContainerNamePrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyContainerNamePrefix<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyContainerNamePrefix(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn CryptAttributes<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CryptAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X509Extensions<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X509Extensions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CriticalExtensions<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CriticalExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuppressOids<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuppressOids() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawDataToBeSigned<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawDataToBeSigned(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Signature<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCspStatuses<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keyspec: X509KeySpec, ppcspstatuses: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCspStatuses(::core::mem::transmute_copy(&keyspec)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcspstatuses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509CertificateRequestVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromTemplateName: InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromPrivateKey: InitializeFromPrivateKey::<Impl, IMPL_OFFSET>,
            InitializeFromPublicKey: InitializeFromPublicKey::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate: InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            CheckSignature: CheckSignature::<Impl, IMPL_OFFSET>,
            IsSmartCard: IsSmartCard::<Impl, IMPL_OFFSET>,
            TemplateObjectId: TemplateObjectId::<Impl, IMPL_OFFSET>,
            PublicKey: PublicKey::<Impl, IMPL_OFFSET>,
            PrivateKey: PrivateKey::<Impl, IMPL_OFFSET>,
            NullSigned: NullSigned::<Impl, IMPL_OFFSET>,
            ReuseKey: ReuseKey::<Impl, IMPL_OFFSET>,
            OldCertificate: OldCertificate::<Impl, IMPL_OFFSET>,
            Subject: Subject::<Impl, IMPL_OFFSET>,
            SetSubject: SetSubject::<Impl, IMPL_OFFSET>,
            CspStatuses: CspStatuses::<Impl, IMPL_OFFSET>,
            SmimeCapabilities: SmimeCapabilities::<Impl, IMPL_OFFSET>,
            SetSmimeCapabilities: SetSmimeCapabilities::<Impl, IMPL_OFFSET>,
            SignatureInformation: SignatureInformation::<Impl, IMPL_OFFSET>,
            KeyContainerNamePrefix: KeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            SetKeyContainerNamePrefix: SetKeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            CryptAttributes: CryptAttributes::<Impl, IMPL_OFFSET>,
            X509Extensions: X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions: CriticalExtensions::<Impl, IMPL_OFFSET>,
            SuppressOids: SuppressOids::<Impl, IMPL_OFFSET>,
            RawDataToBeSigned: RawDataToBeSigned::<Impl, IMPL_OFFSET>,
            Signature: Signature::<Impl, IMPL_OFFSET>,
            GetCspStatuses: GetCspStatuses::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs10 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestPkcs10V2Impl: Sized + IDispatchImpl + IX509CertificateRequestImpl + IX509CertificateRequestPkcs10Impl {
    fn InitializeFromTemplate(&mut self, context: X509CertificateEnrollmentContext, ppolicyserver: ::core::option::Option<IX509EnrollmentPolicyServer>, ptemplate: ::core::option::Option<IX509CertificateTemplate>) -> ::windows::core::Result<()>;
    fn InitializeFromPrivateKeyTemplate(&mut self, context: X509CertificateEnrollmentContext, pprivatekey: ::core::option::Option<IX509PrivateKey>, ppolicyserver: ::core::option::Option<IX509EnrollmentPolicyServer>, ptemplate: ::core::option::Option<IX509CertificateTemplate>) -> ::windows::core::Result<()>;
    fn InitializeFromPublicKeyTemplate(&mut self, context: X509CertificateEnrollmentContext, ppublickey: ::core::option::Option<IX509PublicKey>, ppolicyserver: ::core::option::Option<IX509EnrollmentPolicyServer>, ptemplate: ::core::option::Option<IX509CertificateTemplate>) -> ::windows::core::Result<()>;
    fn PolicyServer(&mut self) -> ::windows::core::Result<IX509EnrollmentPolicyServer>;
    fn Template(&mut self) -> ::windows::core::Result<IX509CertificateTemplate>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestPkcs10V2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestPkcs10V2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestPkcs10V2Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Impl: IX509CertificateRequestPkcs10V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromTemplate(::core::mem::transmute_copy(&context), ::core::mem::transmute(&ppolicyserver), ::core::mem::transmute(&ptemplate)).into()
        }
        unsafe extern "system" fn InitializeFromPrivateKeyTemplate<Impl: IX509CertificateRequestPkcs10V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, pprivatekey: ::windows::core::RawPtr, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromPrivateKeyTemplate(::core::mem::transmute_copy(&context), ::core::mem::transmute(&pprivatekey), ::core::mem::transmute(&ppolicyserver), ::core::mem::transmute(&ptemplate)).into()
        }
        unsafe extern "system" fn InitializeFromPublicKeyTemplate<Impl: IX509CertificateRequestPkcs10V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppublickey: ::windows::core::RawPtr, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromPublicKeyTemplate(::core::mem::transmute_copy(&context), ::core::mem::transmute(&ppublickey), ::core::mem::transmute(&ppolicyserver), ::core::mem::transmute(&ptemplate)).into()
        }
        unsafe extern "system" fn PolicyServer<Impl: IX509CertificateRequestPkcs10V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppolicyserver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyServer() {
                ::core::result::Result::Ok(ok__) => {
                    *pppolicyserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Template<Impl: IX509CertificateRequestPkcs10V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Template() {
                ::core::result::Result::Ok(ok__) => {
                    *pptemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509CertificateRequestPkcs10Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromTemplate: InitializeFromTemplate::<Impl, IMPL_OFFSET>,
            InitializeFromPrivateKeyTemplate: InitializeFromPrivateKeyTemplate::<Impl, IMPL_OFFSET>,
            InitializeFromPublicKeyTemplate: InitializeFromPublicKeyTemplate::<Impl, IMPL_OFFSET>,
            PolicyServer: PolicyServer::<Impl, IMPL_OFFSET>,
            Template: Template::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs10V2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestPkcs10V3Impl: Sized + IDispatchImpl + IX509CertificateRequestImpl + IX509CertificateRequestPkcs10Impl + IX509CertificateRequestPkcs10V2Impl {
    fn AttestPrivateKey(&mut self) -> ::windows::core::Result<i16>;
    fn SetAttestPrivateKey(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn AttestationEncryptionCertificate(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetAttestationEncryptionCertificate(&mut self, encoding: EncodingType, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EncryptionAlgorithm(&mut self) -> ::windows::core::Result<IObjectId>;
    fn SetEncryptionAlgorithm(&mut self, pvalue: ::core::option::Option<IObjectId>) -> ::windows::core::Result<()>;
    fn EncryptionStrength(&mut self) -> ::windows::core::Result<i32>;
    fn SetEncryptionStrength(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn ChallengePassword(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetChallengePassword(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NameValuePairs(&mut self) -> ::windows::core::Result<IX509NameValuePairs>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestPkcs10V3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestPkcs10V3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestPkcs10V3Vtbl {
        unsafe extern "system" fn AttestPrivateKey<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttestPrivateKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttestPrivateKey<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttestPrivateKey(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AttestationEncryptionCertificate<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttestationEncryptionCertificate(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttestationEncryptionCertificate<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttestationEncryptionCertificate(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn EncryptionAlgorithm<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptionAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptionAlgorithm<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncryptionAlgorithm(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn EncryptionStrength<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptionStrength() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptionStrength<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncryptionStrength(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ChallengePassword<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChallengePassword() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChallengePassword<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChallengePassword(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn NameValuePairs<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameValuePairs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509CertificateRequestPkcs10V2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AttestPrivateKey: AttestPrivateKey::<Impl, IMPL_OFFSET>,
            SetAttestPrivateKey: SetAttestPrivateKey::<Impl, IMPL_OFFSET>,
            AttestationEncryptionCertificate: AttestationEncryptionCertificate::<Impl, IMPL_OFFSET>,
            SetAttestationEncryptionCertificate: SetAttestationEncryptionCertificate::<Impl, IMPL_OFFSET>,
            EncryptionAlgorithm: EncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptionAlgorithm: SetEncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptionStrength: EncryptionStrength::<Impl, IMPL_OFFSET>,
            SetEncryptionStrength: SetEncryptionStrength::<Impl, IMPL_OFFSET>,
            ChallengePassword: ChallengePassword::<Impl, IMPL_OFFSET>,
            SetChallengePassword: SetChallengePassword::<Impl, IMPL_OFFSET>,
            NameValuePairs: NameValuePairs::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs10V3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestPkcs10V4Impl: Sized + IDispatchImpl + IX509CertificateRequestImpl + IX509CertificateRequestPkcs10Impl + IX509CertificateRequestPkcs10V2Impl + IX509CertificateRequestPkcs10V3Impl {
    fn ClaimType(&mut self) -> ::windows::core::Result<KeyAttestationClaimType>;
    fn SetClaimType(&mut self, value: KeyAttestationClaimType) -> ::windows::core::Result<()>;
    fn AttestPrivateKeyPreferred(&mut self) -> ::windows::core::Result<i16>;
    fn SetAttestPrivateKeyPreferred(&mut self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestPkcs10V4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestPkcs10V4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestPkcs10V4Vtbl {
        unsafe extern "system" fn ClaimType<Impl: IX509CertificateRequestPkcs10V4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut KeyAttestationClaimType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClaimType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClaimType<Impl: IX509CertificateRequestPkcs10V4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: KeyAttestationClaimType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClaimType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AttestPrivateKeyPreferred<Impl: IX509CertificateRequestPkcs10V4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttestPrivateKeyPreferred() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttestPrivateKeyPreferred<Impl: IX509CertificateRequestPkcs10V4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttestPrivateKeyPreferred(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IX509CertificateRequestPkcs10V3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ClaimType: ClaimType::<Impl, IMPL_OFFSET>,
            SetClaimType: SetClaimType::<Impl, IMPL_OFFSET>,
            AttestPrivateKeyPreferred: AttestPrivateKeyPreferred::<Impl, IMPL_OFFSET>,
            SetAttestPrivateKeyPreferred: SetAttestPrivateKeyPreferred::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs10V4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestPkcs7Impl: Sized + IDispatchImpl + IX509CertificateRequestImpl {
    fn InitializeFromTemplateName(&mut self, context: X509CertificateEnrollmentContext, strtemplatename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeFromCertificate(&mut self, context: X509CertificateEnrollmentContext, renewalrequest: i16, strcertificate: super::super::super::Foundation::BSTR, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::Result<()>;
    fn InitializeFromInnerRequest(&mut self, pinnerrequest: ::core::option::Option<IX509CertificateRequest>) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, strencodeddata: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
    fn RequesterName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetRequesterName(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SignerCertificate(&mut self) -> ::windows::core::Result<ISignerCertificate>;
    fn SetSignerCertificate(&mut self, pvalue: ::core::option::Option<ISignerCertificate>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestPkcs7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestPkcs7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestPkcs7Vtbl {
        unsafe extern "system" fn InitializeFromTemplateName<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromTemplateName(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&strtemplatename)).into()
        }
        unsafe extern "system" fn InitializeFromCertificate<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, renewalrequest: i16, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromCertificate(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&renewalrequest), ::core::mem::transmute_copy(&strcertificate), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&inheritoptions)).into()
        }
        unsafe extern "system" fn InitializeFromInnerRequest<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinnerrequest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromInnerRequest(::core::mem::transmute(&pinnerrequest)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&strencodeddata), ::core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn RequesterName<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequesterName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequesterName<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequesterName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SignerCertificate<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignerCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignerCertificate<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignerCertificate(::core::mem::transmute(&pvalue)).into()
        }
        Self {
            base: IX509CertificateRequestVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromTemplateName: InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate: InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeFromInnerRequest: InitializeFromInnerRequest::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            RequesterName: RequesterName::<Impl, IMPL_OFFSET>,
            SetRequesterName: SetRequesterName::<Impl, IMPL_OFFSET>,
            SignerCertificate: SignerCertificate::<Impl, IMPL_OFFSET>,
            SetSignerCertificate: SetSignerCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestPkcs7V2Impl: Sized + IDispatchImpl + IX509CertificateRequestImpl + IX509CertificateRequestPkcs7Impl {
    fn InitializeFromTemplate(&mut self, context: X509CertificateEnrollmentContext, ppolicyserver: ::core::option::Option<IX509EnrollmentPolicyServer>, ptemplate: ::core::option::Option<IX509CertificateTemplate>) -> ::windows::core::Result<()>;
    fn PolicyServer(&mut self) -> ::windows::core::Result<IX509EnrollmentPolicyServer>;
    fn Template(&mut self) -> ::windows::core::Result<IX509CertificateTemplate>;
    fn CheckCertificateSignature(&mut self, validatecertificatechain: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestPkcs7V2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestPkcs7V2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestPkcs7V2Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Impl: IX509CertificateRequestPkcs7V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromTemplate(::core::mem::transmute_copy(&context), ::core::mem::transmute(&ppolicyserver), ::core::mem::transmute(&ptemplate)).into()
        }
        unsafe extern "system" fn PolicyServer<Impl: IX509CertificateRequestPkcs7V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppolicyserver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyServer() {
                ::core::result::Result::Ok(ok__) => {
                    *pppolicyserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Template<Impl: IX509CertificateRequestPkcs7V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Template() {
                ::core::result::Result::Ok(ok__) => {
                    *pptemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCertificateSignature<Impl: IX509CertificateRequestPkcs7V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, validatecertificatechain: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckCertificateSignature(::core::mem::transmute_copy(&validatecertificatechain)).into()
        }
        Self {
            base: IX509CertificateRequestPkcs7Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromTemplate: InitializeFromTemplate::<Impl, IMPL_OFFSET>,
            PolicyServer: PolicyServer::<Impl, IMPL_OFFSET>,
            Template: Template::<Impl, IMPL_OFFSET>,
            CheckCertificateSignature: CheckCertificateSignature::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs7V2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRevocationListImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, strencodeddata: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
    fn Encode(&mut self) -> ::windows::core::Result<()>;
    fn ResetForEncode(&mut self) -> ::windows::core::Result<()>;
    fn CheckPublicKeySignature(&mut self, ppublickey: ::core::option::Option<IX509PublicKey>) -> ::windows::core::Result<()>;
    fn CheckSignature(&mut self) -> ::windows::core::Result<()>;
    fn Issuer(&mut self) -> ::windows::core::Result<IX500DistinguishedName>;
    fn SetIssuer(&mut self, pvalue: ::core::option::Option<IX500DistinguishedName>) -> ::windows::core::Result<()>;
    fn ThisUpdate(&mut self) -> ::windows::core::Result<f64>;
    fn SetThisUpdate(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn NextUpdate(&mut self) -> ::windows::core::Result<f64>;
    fn SetNextUpdate(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn X509CRLEntries(&mut self) -> ::windows::core::Result<IX509CertificateRevocationListEntries>;
    fn X509Extensions(&mut self) -> ::windows::core::Result<IX509Extensions>;
    fn CriticalExtensions(&mut self) -> ::windows::core::Result<IObjectIds>;
    fn SignerCertificate(&mut self) -> ::windows::core::Result<ISignerCertificate>;
    fn SetSignerCertificate(&mut self, pvalue: ::core::option::Option<ISignerCertificate>) -> ::windows::core::Result<()>;
    fn CRLNumber(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetCRLNumber(&mut self, encoding: EncodingType, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CAVersion(&mut self) -> ::windows::core::Result<i32>;
    fn SetCAVersion(&mut self, pvalue: i32) -> ::windows::core::Result<()>;
    fn BaseCRL(&mut self) -> ::windows::core::Result<i16>;
    fn NullSigned(&mut self) -> ::windows::core::Result<i16>;
    fn HashAlgorithm(&mut self) -> ::windows::core::Result<IObjectId>;
    fn SetHashAlgorithm(&mut self, pvalue: ::core::option::Option<IObjectId>) -> ::windows::core::Result<()>;
    fn AlternateSignatureAlgorithm(&mut self) -> ::windows::core::Result<i16>;
    fn SetAlternateSignatureAlgorithm(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn SignatureInformation(&mut self) -> ::windows::core::Result<IX509SignatureInformation>;
    fn RawData(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn RawDataToBeSigned(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Signature(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRevocationListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRevocationListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRevocationListVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&strencodeddata), ::core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn Encode<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Encode().into()
        }
        unsafe extern "system" fn ResetForEncode<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetForEncode().into()
        }
        unsafe extern "system" fn CheckPublicKeySignature<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppublickey: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckPublicKeySignature(::core::mem::transmute(&ppublickey)).into()
        }
        unsafe extern "system" fn CheckSignature<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckSignature().into()
        }
        unsafe extern "system" fn Issuer<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Issuer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIssuer<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIssuer(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn ThisUpdate<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThisUpdate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThisUpdate<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThisUpdate(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn NextUpdate<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextUpdate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNextUpdate<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNextUpdate(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn X509CRLEntries<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X509CRLEntries() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X509Extensions<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X509Extensions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CriticalExtensions<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CriticalExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignerCertificate<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignerCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignerCertificate<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignerCertificate(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn CRLNumber<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CRLNumber(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCRLNumber<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCRLNumber(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn CAVersion<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CAVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCAVersion<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCAVersion(::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn BaseCRL<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseCRL() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NullSigned<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NullSigned() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn AlternateSignatureAlgorithm<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateSignatureAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlternateSignatureAlgorithm<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlternateSignatureAlgorithm(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SignatureInformation<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignatureInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawData<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawData(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawDataToBeSigned<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawDataToBeSigned(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Signature<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            Encode: Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode: ResetForEncode::<Impl, IMPL_OFFSET>,
            CheckPublicKeySignature: CheckPublicKeySignature::<Impl, IMPL_OFFSET>,
            CheckSignature: CheckSignature::<Impl, IMPL_OFFSET>,
            Issuer: Issuer::<Impl, IMPL_OFFSET>,
            SetIssuer: SetIssuer::<Impl, IMPL_OFFSET>,
            ThisUpdate: ThisUpdate::<Impl, IMPL_OFFSET>,
            SetThisUpdate: SetThisUpdate::<Impl, IMPL_OFFSET>,
            NextUpdate: NextUpdate::<Impl, IMPL_OFFSET>,
            SetNextUpdate: SetNextUpdate::<Impl, IMPL_OFFSET>,
            X509CRLEntries: X509CRLEntries::<Impl, IMPL_OFFSET>,
            X509Extensions: X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions: CriticalExtensions::<Impl, IMPL_OFFSET>,
            SignerCertificate: SignerCertificate::<Impl, IMPL_OFFSET>,
            SetSignerCertificate: SetSignerCertificate::<Impl, IMPL_OFFSET>,
            CRLNumber: CRLNumber::<Impl, IMPL_OFFSET>,
            SetCRLNumber: SetCRLNumber::<Impl, IMPL_OFFSET>,
            CAVersion: CAVersion::<Impl, IMPL_OFFSET>,
            SetCAVersion: SetCAVersion::<Impl, IMPL_OFFSET>,
            BaseCRL: BaseCRL::<Impl, IMPL_OFFSET>,
            NullSigned: NullSigned::<Impl, IMPL_OFFSET>,
            HashAlgorithm: HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm: AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm: SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SignatureInformation: SignatureInformation::<Impl, IMPL_OFFSET>,
            RawData: RawData::<Impl, IMPL_OFFSET>,
            RawDataToBeSigned: RawDataToBeSigned::<Impl, IMPL_OFFSET>,
            Signature: Signature::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRevocationList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRevocationListEntriesImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<IX509CertificateRevocationListEntry>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<IX509CertificateRevocationListEntry>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn IndexBySerialNumber(&mut self, encoding: EncodingType, serialnumber: super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn AddRange(&mut self, pvalue: ::core::option::Option<IX509CertificateRevocationListEntries>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRevocationListEntriesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRevocationListEntriesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRevocationListEntriesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn IndexBySerialNumber<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, serialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexBySerialNumber(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&serialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRange<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&pvalue)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            IndexBySerialNumber: IndexBySerialNumber::<Impl, IMPL_OFFSET>,
            AddRange: AddRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRevocationListEntries as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRevocationListEntryImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, encoding: EncodingType, serialnumber: super::super::super::Foundation::BSTR, revocationdate: f64) -> ::windows::core::Result<()>;
    fn SerialNumber(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn RevocationDate(&mut self) -> ::windows::core::Result<f64>;
    fn RevocationReason(&mut self) -> ::windows::core::Result<CRLRevocationReason>;
    fn SetRevocationReason(&mut self, value: CRLRevocationReason) -> ::windows::core::Result<()>;
    fn X509Extensions(&mut self) -> ::windows::core::Result<IX509Extensions>;
    fn CriticalExtensions(&mut self) -> ::windows::core::Result<IObjectIds>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRevocationListEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRevocationListEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRevocationListEntryVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, serialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, revocationdate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&serialnumber), ::core::mem::transmute_copy(&revocationdate)).into()
        }
        unsafe extern "system" fn SerialNumber<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerialNumber(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevocationDate<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevocationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevocationReason<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut CRLRevocationReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevocationReason() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevocationReason<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CRLRevocationReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevocationReason(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn X509Extensions<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X509Extensions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CriticalExtensions<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CriticalExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            SerialNumber: SerialNumber::<Impl, IMPL_OFFSET>,
            RevocationDate: RevocationDate::<Impl, IMPL_OFFSET>,
            RevocationReason: RevocationReason::<Impl, IMPL_OFFSET>,
            SetRevocationReason: SetRevocationReason::<Impl, IMPL_OFFSET>,
            X509Extensions: X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions: CriticalExtensions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRevocationListEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateTemplateImpl: Sized + IDispatchImpl {
    fn Property(&mut self, property: EnrollmentTemplateProperty) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateTemplateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateTemplateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateTemplateVtbl {
        unsafe extern "system" fn Property<Impl: IX509CertificateTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: EnrollmentTemplateProperty, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Property(::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Property: Property::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateTemplateWritableImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, pvalue: ::core::option::Option<IX509CertificateTemplate>) -> ::windows::core::Result<()>;
    fn Commit(&mut self, commitflags: CommitTemplateFlags, strservercontext: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Property(&mut self, property: EnrollmentTemplateProperty) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, property: EnrollmentTemplateProperty, value: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Template(&mut self) -> ::windows::core::Result<IX509CertificateTemplate>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateTemplateWritableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateTemplateWritableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateTemplateWritableVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509CertificateTemplateWritableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn Commit<Impl: IX509CertificateTemplateWritableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commitflags: CommitTemplateFlags, strservercontext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&commitflags), ::core::mem::transmute_copy(&strservercontext)).into()
        }
        unsafe extern "system" fn Property<Impl: IX509CertificateTemplateWritableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: EnrollmentTemplateProperty, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Property(::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IX509CertificateTemplateWritableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: EnrollmentTemplateProperty, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&property), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Template<Impl: IX509CertificateTemplateWritableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Template() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
            Property: Property::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            Template: Template::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateTemplateWritable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateTemplatesImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<IX509CertificateTemplate>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<IX509CertificateTemplate>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn ItemByName(&mut self, bstrname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<IX509CertificateTemplate>;
    fn ItemByOid(&mut self, poid: ::core::option::Option<IObjectId>) -> ::windows::core::Result<IX509CertificateTemplate>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateTemplatesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateTemplatesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateTemplatesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn ItemByName<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByName(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemByOid<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poid: ::windows::core::RawPtr, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByOid(::core::mem::transmute(&poid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            ItemByName: ItemByName::<Impl, IMPL_OFFSET>,
            ItemByOid: ItemByOid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateTemplates as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509EndorsementKeyImpl: Sized + IDispatchImpl {
    fn ProviderName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetProviderName(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn Opened(&mut self) -> ::windows::core::Result<i16>;
    fn AddCertificate(&mut self, encoding: EncodingType, strcertificate: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveCertificate(&mut self, encoding: EncodingType, strcertificate: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetCertificateByIndex(&mut self, manufactureronly: i16, dwindex: i32, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetCertificateCount(&mut self, manufactureronly: i16) -> ::windows::core::Result<i32>;
    fn ExportPublicKey(&mut self) -> ::windows::core::Result<IX509PublicKey>;
    fn Open(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509EndorsementKeyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509EndorsementKeyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509EndorsementKeyVtbl {
        unsafe extern "system" fn ProviderName<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderName<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Length<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Opened<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opened() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCertificate<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddCertificate(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strcertificate)).into()
        }
        unsafe extern "system" fn RemoveCertificate<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCertificate(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strcertificate)).into()
        }
        unsafe extern "system" fn GetCertificateByIndex<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manufactureronly: i16, dwindex: i32, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateByIndex(::core::mem::transmute_copy(&manufactureronly), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateCount<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manufactureronly: i16, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateCount(::core::mem::transmute_copy(&manufactureronly)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportPublicKey<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppublickey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportPublicKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pppublickey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn Close<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ProviderName: ProviderName::<Impl, IMPL_OFFSET>,
            SetProviderName: SetProviderName::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            Opened: Opened::<Impl, IMPL_OFFSET>,
            AddCertificate: AddCertificate::<Impl, IMPL_OFFSET>,
            RemoveCertificate: RemoveCertificate::<Impl, IMPL_OFFSET>,
            GetCertificateByIndex: GetCertificateByIndex::<Impl, IMPL_OFFSET>,
            GetCertificateCount: GetCertificateCount::<Impl, IMPL_OFFSET>,
            ExportPublicKey: ExportPublicKey::<Impl, IMPL_OFFSET>,
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509EndorsementKey as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509EnrollmentImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()>;
    fn InitializeFromTemplateName(&mut self, context: X509CertificateEnrollmentContext, strtemplatename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeFromRequest(&mut self, prequest: ::core::option::Option<IX509CertificateRequest>) -> ::windows::core::Result<()>;
    fn CreateRequest(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Enroll(&mut self) -> ::windows::core::Result<()>;
    fn InstallResponse(&mut self, restrictions: InstallResponseRestrictionFlags, strresponse: super::super::super::Foundation::BSTR, encoding: EncodingType, strpassword: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CreatePFX(&mut self, strpassword: super::super::super::Foundation::BSTR, exportoptions: PFXExportOptions, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Request(&mut self) -> ::windows::core::Result<IX509CertificateRequest>;
    fn Silent(&mut self) -> ::windows::core::Result<i16>;
    fn SetSilent(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn ParentWindow(&mut self) -> ::windows::core::Result<i32>;
    fn SetParentWindow(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn NameValuePairs(&mut self) -> ::windows::core::Result<IX509NameValuePairs>;
    fn EnrollmentContext(&mut self) -> ::windows::core::Result<X509CertificateEnrollmentContext>;
    fn Status(&mut self) -> ::windows::core::Result<IX509EnrollmentStatus>;
    fn Certificate(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Response(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn CertificateFriendlyName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetCertificateFriendlyName(&mut self, strvalue: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CertificateDescription(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetCertificateDescription(&mut self, strvalue: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RequestId(&mut self) -> ::windows::core::Result<i32>;
    fn CAConfigString(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509EnrollmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509EnrollmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509EnrollmentVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn InitializeFromTemplateName<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromTemplateName(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&strtemplatename)).into()
        }
        unsafe extern "system" fn InitializeFromRequest<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromRequest(::core::mem::transmute(&prequest)).into()
        }
        unsafe extern "system" fn CreateRequest<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRequest(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enroll<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enroll().into()
        }
        unsafe extern "system" fn InstallResponse<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictions: InstallResponseRestrictionFlags, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallResponse(::core::mem::transmute_copy(&restrictions), ::core::mem::transmute_copy(&strresponse), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strpassword)).into()
        }
        unsafe extern "system" fn CreatePFX<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, exportoptions: PFXExportOptions, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePFX(::core::mem::transmute_copy(&strpassword), ::core::mem::transmute_copy(&exportoptions), ::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Silent<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Silent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSilent<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSilent(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ParentWindow<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentWindow<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParentWindow(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn NameValuePairs<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameValuePairs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnrollmentContext<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnrollmentContext() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Certificate<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Certificate(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Response<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Response(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CertificateFriendlyName<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CertificateFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateFriendlyName<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCertificateFriendlyName(::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn CertificateDescription<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CertificateDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateDescription<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCertificateDescription(::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn RequestId<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAConfigString<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CAConfigString() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            InitializeFromTemplateName: InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromRequest: InitializeFromRequest::<Impl, IMPL_OFFSET>,
            CreateRequest: CreateRequest::<Impl, IMPL_OFFSET>,
            Enroll: Enroll::<Impl, IMPL_OFFSET>,
            InstallResponse: InstallResponse::<Impl, IMPL_OFFSET>,
            CreatePFX: CreatePFX::<Impl, IMPL_OFFSET>,
            Request: Request::<Impl, IMPL_OFFSET>,
            Silent: Silent::<Impl, IMPL_OFFSET>,
            SetSilent: SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow: ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow: SetParentWindow::<Impl, IMPL_OFFSET>,
            NameValuePairs: NameValuePairs::<Impl, IMPL_OFFSET>,
            EnrollmentContext: EnrollmentContext::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Certificate: Certificate::<Impl, IMPL_OFFSET>,
            Response: Response::<Impl, IMPL_OFFSET>,
            CertificateFriendlyName: CertificateFriendlyName::<Impl, IMPL_OFFSET>,
            SetCertificateFriendlyName: SetCertificateFriendlyName::<Impl, IMPL_OFFSET>,
            CertificateDescription: CertificateDescription::<Impl, IMPL_OFFSET>,
            SetCertificateDescription: SetCertificateDescription::<Impl, IMPL_OFFSET>,
            RequestId: RequestId::<Impl, IMPL_OFFSET>,
            CAConfigString: CAConfigString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509Enrollment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509Enrollment2Impl: Sized + IDispatchImpl + IX509EnrollmentImpl {
    fn InitializeFromTemplate(&mut self, context: X509CertificateEnrollmentContext, ppolicyserver: ::core::option::Option<IX509EnrollmentPolicyServer>, ptemplate: ::core::option::Option<IX509CertificateTemplate>) -> ::windows::core::Result<()>;
    fn InstallResponse2(&mut self, restrictions: InstallResponseRestrictionFlags, strresponse: super::super::super::Foundation::BSTR, encoding: EncodingType, strpassword: super::super::super::Foundation::BSTR, strenrollmentpolicyserverurl: super::super::super::Foundation::BSTR, strenrollmentpolicyserverid: super::super::super::Foundation::BSTR, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags) -> ::windows::core::Result<()>;
    fn PolicyServer(&mut self) -> ::windows::core::Result<IX509EnrollmentPolicyServer>;
    fn Template(&mut self) -> ::windows::core::Result<IX509CertificateTemplate>;
    fn RequestIdString(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509Enrollment2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509Enrollment2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509Enrollment2Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Impl: IX509Enrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromTemplate(::core::mem::transmute_copy(&context), ::core::mem::transmute(&ppolicyserver), ::core::mem::transmute(&ptemplate)).into()
        }
        unsafe extern "system" fn InstallResponse2<Impl: IX509Enrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictions: InstallResponseRestrictionFlags, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strenrollmentpolicyserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strenrollmentpolicyserverid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallResponse2(::core::mem::transmute_copy(&restrictions), ::core::mem::transmute_copy(&strresponse), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strpassword), ::core::mem::transmute_copy(&strenrollmentpolicyserverurl), ::core::mem::transmute_copy(&strenrollmentpolicyserverid), ::core::mem::transmute_copy(&enrollmentpolicyserverflags), ::core::mem::transmute_copy(&authflags)).into()
        }
        unsafe extern "system" fn PolicyServer<Impl: IX509Enrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppolicyserver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyServer() {
                ::core::result::Result::Ok(ok__) => {
                    *pppolicyserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Template<Impl: IX509Enrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Template() {
                ::core::result::Result::Ok(ok__) => {
                    *pptemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestIdString<Impl: IX509Enrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestIdString() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509EnrollmentVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeFromTemplate: InitializeFromTemplate::<Impl, IMPL_OFFSET>,
            InstallResponse2: InstallResponse2::<Impl, IMPL_OFFSET>,
            PolicyServer: PolicyServer::<Impl, IMPL_OFFSET>,
            Template: Template::<Impl, IMPL_OFFSET>,
            RequestIdString: RequestIdString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509Enrollment2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509EnrollmentHelperImpl: Sized + IDispatchImpl {
    fn AddPolicyServer(&mut self, strenrollmentpolicyserveruri: super::super::super::Foundation::BSTR, strenrollmentpolicyid: super::super::super::Foundation::BSTR, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags, strcredential: super::super::super::Foundation::BSTR, strpassword: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddEnrollmentServer(&mut self, strenrollmentserveruri: super::super::super::Foundation::BSTR, authflags: X509EnrollmentAuthFlags, strcredential: super::super::super::Foundation::BSTR, strpassword: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enroll(&mut self, strenrollmentpolicyserveruri: super::super::super::Foundation::BSTR, strtemplatename: super::super::super::Foundation::BSTR, encoding: EncodingType, enrollflags: WebEnrollmentFlags) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Initialize(&mut self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509EnrollmentHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509EnrollmentHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509EnrollmentHelperVtbl {
        unsafe extern "system" fn AddPolicyServer<Impl: IX509EnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strenrollmentpolicyserveruri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strenrollmentpolicyid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags, strcredential: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPolicyServer(::core::mem::transmute_copy(&strenrollmentpolicyserveruri), ::core::mem::transmute_copy(&strenrollmentpolicyid), ::core::mem::transmute_copy(&enrollmentpolicyserverflags), ::core::mem::transmute_copy(&authflags), ::core::mem::transmute_copy(&strcredential), ::core::mem::transmute_copy(&strpassword)).into()
        }
        unsafe extern "system" fn AddEnrollmentServer<Impl: IX509EnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strenrollmentserveruri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, authflags: X509EnrollmentAuthFlags, strcredential: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddEnrollmentServer(::core::mem::transmute_copy(&strenrollmentserveruri), ::core::mem::transmute_copy(&authflags), ::core::mem::transmute_copy(&strcredential), ::core::mem::transmute_copy(&strpassword)).into()
        }
        unsafe extern "system" fn Enroll<Impl: IX509EnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strenrollmentpolicyserveruri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, enrollflags: WebEnrollmentFlags, pstrcertificate: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enroll(::core::mem::transmute_copy(&strenrollmentpolicyserveruri), ::core::mem::transmute_copy(&strtemplatename), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&enrollflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrcertificate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IX509EnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&context)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddPolicyServer: AddPolicyServer::<Impl, IMPL_OFFSET>,
            AddEnrollmentServer: AddEnrollmentServer::<Impl, IMPL_OFFSET>,
            Enroll: Enroll::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509EnrollmentHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509EnrollmentPolicyServerImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, bstrpolicyserverurl: super::super::super::Foundation::BSTR, bstrpolicyserverid: super::super::super::Foundation::BSTR, authflags: X509EnrollmentAuthFlags, fisuntrusted: i16, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()>;
    fn LoadPolicy(&mut self, option: X509EnrollmentPolicyLoadOption) -> ::windows::core::Result<()>;
    fn GetTemplates(&mut self) -> ::windows::core::Result<IX509CertificateTemplates>;
    fn GetCAsForTemplate(&mut self, ptemplate: ::core::option::Option<IX509CertificateTemplate>) -> ::windows::core::Result<ICertificationAuthorities>;
    fn GetCAs(&mut self) -> ::windows::core::Result<ICertificationAuthorities>;
    fn Validate(&mut self) -> ::windows::core::Result<()>;
    fn GetCustomOids(&mut self) -> ::windows::core::Result<IObjectIds>;
    fn GetNextUpdateTime(&mut self) -> ::windows::core::Result<f64>;
    fn GetLastUpdateTime(&mut self) -> ::windows::core::Result<f64>;
    fn GetPolicyServerUrl(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetPolicyServerId(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetFriendlyName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetIsDefaultCEP(&mut self) -> ::windows::core::Result<i16>;
    fn GetUseClientId(&mut self) -> ::windows::core::Result<i16>;
    fn GetAllowUnTrustedCA(&mut self) -> ::windows::core::Result<i16>;
    fn GetCachePath(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetCacheDir(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetAuthFlags(&mut self) -> ::windows::core::Result<X509EnrollmentAuthFlags>;
    fn SetCredential(&mut self, hwndparent: i32, flag: X509EnrollmentAuthFlags, strcredential: super::super::super::Foundation::BSTR, strpassword: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn QueryChanges(&mut self) -> ::windows::core::Result<i16>;
    fn InitializeImport(&mut self, val: super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Export(&mut self, exportflags: X509EnrollmentPolicyExportFlags) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn Cost(&mut self) -> ::windows::core::Result<u32>;
    fn SetCost(&mut self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509EnrollmentPolicyServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509EnrollmentPolicyServerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509EnrollmentPolicyServerVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpolicyserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrpolicyserverid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, authflags: X509EnrollmentAuthFlags, fisuntrusted: i16, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&bstrpolicyserverurl), ::core::mem::transmute_copy(&bstrpolicyserverid), ::core::mem::transmute_copy(&authflags), ::core::mem::transmute_copy(&fisuntrusted), ::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn LoadPolicy<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: X509EnrollmentPolicyLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadPolicy(::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn GetTemplates<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTemplates() {
                ::core::result::Result::Ok(ok__) => {
                    *ptemplates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAsForTemplate<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptemplate: ::windows::core::RawPtr, ppcas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCAsForTemplate(::core::mem::transmute(&ptemplate)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcas = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAs<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCAs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcas = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Validate().into()
        }
        unsafe extern "system" fn GetCustomOids<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjectids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomOids() {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjectids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextUpdateTime<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextUpdateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastUpdateTime<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastUpdateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicyServerUrl<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPolicyServerUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicyServerId<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPolicyServerId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFriendlyName<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsDefaultCEP<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsDefaultCEP() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUseClientId<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUseClientId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllowUnTrustedCA<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllowUnTrustedCA() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachePath<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachePath() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCacheDir<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCacheDir() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthFlags<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509EnrollmentAuthFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredential<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: i32, flag: X509EnrollmentAuthFlags, strcredential: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredential(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&flag), ::core::mem::transmute_copy(&strcredential), ::core::mem::transmute_copy(&strpassword)).into()
        }
        unsafe extern "system" fn QueryChanges<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryChanges() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeImport<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeImport(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn Export<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, exportflags: X509EnrollmentPolicyExportFlags, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Export(::core::mem::transmute_copy(&exportflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cost<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cost() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCost<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCost(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            LoadPolicy: LoadPolicy::<Impl, IMPL_OFFSET>,
            GetTemplates: GetTemplates::<Impl, IMPL_OFFSET>,
            GetCAsForTemplate: GetCAsForTemplate::<Impl, IMPL_OFFSET>,
            GetCAs: GetCAs::<Impl, IMPL_OFFSET>,
            Validate: Validate::<Impl, IMPL_OFFSET>,
            GetCustomOids: GetCustomOids::<Impl, IMPL_OFFSET>,
            GetNextUpdateTime: GetNextUpdateTime::<Impl, IMPL_OFFSET>,
            GetLastUpdateTime: GetLastUpdateTime::<Impl, IMPL_OFFSET>,
            GetPolicyServerUrl: GetPolicyServerUrl::<Impl, IMPL_OFFSET>,
            GetPolicyServerId: GetPolicyServerId::<Impl, IMPL_OFFSET>,
            GetFriendlyName: GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetIsDefaultCEP: GetIsDefaultCEP::<Impl, IMPL_OFFSET>,
            GetUseClientId: GetUseClientId::<Impl, IMPL_OFFSET>,
            GetAllowUnTrustedCA: GetAllowUnTrustedCA::<Impl, IMPL_OFFSET>,
            GetCachePath: GetCachePath::<Impl, IMPL_OFFSET>,
            GetCacheDir: GetCacheDir::<Impl, IMPL_OFFSET>,
            GetAuthFlags: GetAuthFlags::<Impl, IMPL_OFFSET>,
            SetCredential: SetCredential::<Impl, IMPL_OFFSET>,
            QueryChanges: QueryChanges::<Impl, IMPL_OFFSET>,
            InitializeImport: InitializeImport::<Impl, IMPL_OFFSET>,
            Export: Export::<Impl, IMPL_OFFSET>,
            Cost: Cost::<Impl, IMPL_OFFSET>,
            SetCost: SetCost::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509EnrollmentPolicyServer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509EnrollmentStatusImpl: Sized + IDispatchImpl {
    fn AppendText(&mut self, strtext: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Text(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetText(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Selected(&mut self) -> ::windows::core::Result<EnrollmentSelectionStatus>;
    fn SetSelected(&mut self, value: EnrollmentSelectionStatus) -> ::windows::core::Result<()>;
    fn Display(&mut self) -> ::windows::core::Result<EnrollmentDisplayStatus>;
    fn SetDisplay(&mut self, value: EnrollmentDisplayStatus) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<EnrollmentEnrollStatus>;
    fn SetStatus(&mut self, value: EnrollmentEnrollStatus) -> ::windows::core::Result<()>;
    fn Error(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn SetError(&mut self, value: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn ErrorText(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509EnrollmentStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509EnrollmentStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509EnrollmentStatusVtbl {
        unsafe extern "system" fn AppendText<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strtext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AppendText(::core::mem::transmute_copy(&strtext)).into()
        }
        unsafe extern "system" fn Text<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Selected<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut EnrollmentSelectionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selected() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelected<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EnrollmentSelectionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelected(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Display<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut EnrollmentDisplayStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Display() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplay<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EnrollmentDisplayStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplay(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Status<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut EnrollmentEnrollStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EnrollmentEnrollStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Error<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetError<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetError(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ErrorText<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorText() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AppendText: AppendText::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            Selected: Selected::<Impl, IMPL_OFFSET>,
            SetSelected: SetSelected::<Impl, IMPL_OFFSET>,
            Display: Display::<Impl, IMPL_OFFSET>,
            SetDisplay: SetDisplay::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
            SetError: SetError::<Impl, IMPL_OFFSET>,
            ErrorText: ErrorText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509EnrollmentStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509EnrollmentWebClassFactoryImpl: Sized + IDispatchImpl {
    fn CreateObject(&mut self, strprogid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509EnrollmentWebClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509EnrollmentWebClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509EnrollmentWebClassFactoryVtbl {
        unsafe extern "system" fn CreateObject<Impl: IX509EnrollmentWebClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprogid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppiunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateObject(::core::mem::transmute_copy(&strprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreateObject: CreateObject::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509EnrollmentWebClassFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, pobjectid: ::core::option::Option<IObjectId>, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ObjectId(&mut self) -> ::windows::core::Result<IObjectId>;
    fn RawData(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Critical(&mut self) -> ::windows::core::Result<i16>;
    fn SetCritical(&mut self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pobjectid), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn ObjectId<Impl: IX509ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawData<Impl: IX509ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawData(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Critical<Impl: IX509ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Critical() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCritical<Impl: IX509ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCritical(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            ObjectId: ObjectId::<Impl, IMPL_OFFSET>,
            RawData: RawData::<Impl, IMPL_OFFSET>,
            Critical: Critical::<Impl, IMPL_OFFSET>,
            SetCritical: SetCritical::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509Extension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionAlternativeNamesImpl: Sized + IDispatchImpl + IX509ExtensionImpl {
    fn InitializeEncode(&mut self, pvalue: ::core::option::Option<IAlternativeNames>) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AlternativeNames(&mut self) -> ::windows::core::Result<IAlternativeNames>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionAlternativeNamesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionAlternativeNamesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionAlternativeNamesVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn AlternativeNames<Impl: IX509ExtensionAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternativeNames() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509ExtensionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            AlternativeNames: AlternativeNames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionAlternativeNames as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionAuthorityKeyIdentifierImpl: Sized + IDispatchImpl + IX509ExtensionImpl {
    fn InitializeEncode(&mut self, encoding: EncodingType, strkeyidentifier: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AuthorityKeyIdentifier(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionAuthorityKeyIdentifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionAuthorityKeyIdentifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionAuthorityKeyIdentifierVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionAuthorityKeyIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strkeyidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strkeyidentifier)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionAuthorityKeyIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn AuthorityKeyIdentifier<Impl: IX509ExtensionAuthorityKeyIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthorityKeyIdentifier(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509ExtensionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            AuthorityKeyIdentifier: AuthorityKeyIdentifier::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionAuthorityKeyIdentifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionBasicConstraintsImpl: Sized + IDispatchImpl + IX509ExtensionImpl {
    fn InitializeEncode(&mut self, isca: i16, pathlenconstraint: i32) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsCA(&mut self) -> ::windows::core::Result<i16>;
    fn PathLenConstraint(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionBasicConstraintsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionBasicConstraintsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionBasicConstraintsVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionBasicConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isca: i16, pathlenconstraint: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute_copy(&isca), ::core::mem::transmute_copy(&pathlenconstraint)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionBasicConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn IsCA<Impl: IX509ExtensionBasicConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCA() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathLenConstraint<Impl: IX509ExtensionBasicConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathLenConstraint() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509ExtensionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            IsCA: IsCA::<Impl, IMPL_OFFSET>,
            PathLenConstraint: PathLenConstraint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionBasicConstraints as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionCertificatePoliciesImpl: Sized + IDispatchImpl + IX509ExtensionImpl {
    fn InitializeEncode(&mut self, pvalue: ::core::option::Option<ICertificatePolicies>) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Policies(&mut self) -> ::windows::core::Result<ICertificatePolicies>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionCertificatePoliciesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionCertificatePoliciesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionCertificatePoliciesVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionCertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionCertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn Policies<Impl: IX509ExtensionCertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Policies() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509ExtensionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            Policies: Policies::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionCertificatePolicies as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionEnhancedKeyUsageImpl: Sized + IDispatchImpl + IX509ExtensionImpl {
    fn InitializeEncode(&mut self, pvalue: ::core::option::Option<IObjectIds>) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EnhancedKeyUsage(&mut self) -> ::windows::core::Result<IObjectIds>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionEnhancedKeyUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionEnhancedKeyUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionEnhancedKeyUsageVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionEnhancedKeyUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionEnhancedKeyUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn EnhancedKeyUsage<Impl: IX509ExtensionEnhancedKeyUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnhancedKeyUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509ExtensionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            EnhancedKeyUsage: EnhancedKeyUsage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionEnhancedKeyUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionKeyUsageImpl: Sized + IDispatchImpl + IX509ExtensionImpl {
    fn InitializeEncode(&mut self, usageflags: X509KeyUsageFlags) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn KeyUsage(&mut self) -> ::windows::core::Result<X509KeyUsageFlags>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionKeyUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionKeyUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionKeyUsageVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionKeyUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usageflags: X509KeyUsageFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute_copy(&usageflags)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionKeyUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn KeyUsage<Impl: IX509ExtensionKeyUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509KeyUsageFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509ExtensionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            KeyUsage: KeyUsage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionKeyUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionMSApplicationPoliciesImpl: Sized + IDispatchImpl + IX509ExtensionImpl {
    fn InitializeEncode(&mut self, pvalue: ::core::option::Option<ICertificatePolicies>) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Policies(&mut self) -> ::windows::core::Result<ICertificatePolicies>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionMSApplicationPoliciesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionMSApplicationPoliciesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionMSApplicationPoliciesVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionMSApplicationPoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionMSApplicationPoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn Policies<Impl: IX509ExtensionMSApplicationPoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Policies() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509ExtensionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            Policies: Policies::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionMSApplicationPolicies as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionSmimeCapabilitiesImpl: Sized + IDispatchImpl + IX509ExtensionImpl {
    fn InitializeEncode(&mut self, pvalue: ::core::option::Option<ISmimeCapabilities>) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SmimeCapabilities(&mut self) -> ::windows::core::Result<ISmimeCapabilities>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionSmimeCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionSmimeCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionSmimeCapabilitiesVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionSmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionSmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn SmimeCapabilities<Impl: IX509ExtensionSmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmimeCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509ExtensionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            SmimeCapabilities: SmimeCapabilities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionSmimeCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionSubjectKeyIdentifierImpl: Sized + IDispatchImpl + IX509ExtensionImpl {
    fn InitializeEncode(&mut self, encoding: EncodingType, strkeyidentifier: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SubjectKeyIdentifier(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionSubjectKeyIdentifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionSubjectKeyIdentifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionSubjectKeyIdentifierVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionSubjectKeyIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strkeyidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strkeyidentifier)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionSubjectKeyIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn SubjectKeyIdentifier<Impl: IX509ExtensionSubjectKeyIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubjectKeyIdentifier(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509ExtensionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            SubjectKeyIdentifier: SubjectKeyIdentifier::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionSubjectKeyIdentifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionTemplateImpl: Sized + IDispatchImpl + IX509ExtensionImpl {
    fn InitializeEncode(&mut self, ptemplateoid: ::core::option::Option<IObjectId>, majorversion: i32, minorversion: i32) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TemplateOid(&mut self) -> ::windows::core::Result<IObjectId>;
    fn MajorVersion(&mut self) -> ::windows::core::Result<i32>;
    fn MinorVersion(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionTemplateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionTemplateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionTemplateVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptemplateoid: ::windows::core::RawPtr, majorversion: i32, minorversion: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute(&ptemplateoid), ::core::mem::transmute_copy(&majorversion), ::core::mem::transmute_copy(&minorversion)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn TemplateOid<Impl: IX509ExtensionTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TemplateOid() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: IX509ExtensionTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: IX509ExtensionTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509ExtensionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            TemplateOid: TemplateOid::<Impl, IMPL_OFFSET>,
            MajorVersion: MajorVersion::<Impl, IMPL_OFFSET>,
            MinorVersion: MinorVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionTemplateNameImpl: Sized + IDispatchImpl + IX509ExtensionImpl {
    fn InitializeEncode(&mut self, strtemplatename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeDecode(&mut self, encoding: EncodingType, strencodeddata: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TemplateName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionTemplateNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionTemplateNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionTemplateNameVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionTemplateNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEncode(::core::mem::transmute_copy(&strtemplatename)).into()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionTemplateNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDecode(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&strencodeddata)).into()
        }
        unsafe extern "system" fn TemplateName<Impl: IX509ExtensionTemplateNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TemplateName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IX509ExtensionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeEncode: InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode: InitializeDecode::<Impl, IMPL_OFFSET>,
            TemplateName: TemplateName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionTemplateName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionsImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<IX509Extension>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<IX509Extension>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn IndexByObjectId(&mut self, pobjectid: ::core::option::Option<IObjectId>) -> ::windows::core::Result<i32>;
    fn AddRange(&mut self, pvalue: ::core::option::Option<IX509Extensions>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionsVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn IndexByObjectId<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexByObjectId(::core::mem::transmute(&pobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRange<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&pvalue)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            IndexByObjectId: IndexByObjectId::<Impl, IMPL_OFFSET>,
            AddRange: AddRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509Extensions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509MachineEnrollmentFactoryImpl: Sized + IDispatchImpl {
    fn CreateObject(&mut self, strprogid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<IX509EnrollmentHelper>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509MachineEnrollmentFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509MachineEnrollmentFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509MachineEnrollmentFactoryVtbl {
        unsafe extern "system" fn CreateObject<Impl: IX509MachineEnrollmentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprogid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppihelper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateObject(::core::mem::transmute_copy(&strprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppihelper = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreateObject: CreateObject::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509MachineEnrollmentFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509NameValuePairImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, strname: super::super::super::Foundation::BSTR, strvalue: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509NameValuePairVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509NameValuePairImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509NameValuePairVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509NameValuePairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn Value<Impl: IX509NameValuePairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IX509NameValuePairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509NameValuePair as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509NameValuePairsImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<IX509NameValuePair>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<IX509NameValuePair>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509NameValuePairsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509NameValuePairsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509NameValuePairsVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IX509NameValuePairsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IX509NameValuePairsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IX509NameValuePairsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IX509NameValuePairsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: IX509NameValuePairsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: IX509NameValuePairsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509NameValuePairs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509PolicyServerListManagerImpl: Sized + IDispatchImpl {
    fn ItemByIndex(&mut self, index: i32) -> ::windows::core::Result<IX509PolicyServerUrl>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pval: ::core::option::Option<IX509PolicyServerUrl>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, context: X509CertificateEnrollmentContext, flags: PolicyServerUrlFlags) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509PolicyServerListManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509PolicyServerListManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509PolicyServerListManagerVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn Remove<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn Initialize<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, flags: PolicyServerUrlFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemByIndex: ItemByIndex::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509PolicyServerListManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509PolicyServerUrlImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()>;
    fn Url(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetUrl(&mut self, pvalue: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Default(&mut self) -> ::windows::core::Result<i16>;
    fn SetDefault(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn Flags(&mut self) -> ::windows::core::Result<PolicyServerUrlFlags>;
    fn SetFlags(&mut self, flags: PolicyServerUrlFlags) -> ::windows::core::Result<()>;
    fn AuthFlags(&mut self) -> ::windows::core::Result<X509EnrollmentAuthFlags>;
    fn SetAuthFlags(&mut self, flags: X509EnrollmentAuthFlags) -> ::windows::core::Result<()>;
    fn Cost(&mut self) -> ::windows::core::Result<u32>;
    fn SetCost(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn GetStringProperty(&mut self, propertyid: PolicyServerUrlPropertyID) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetStringProperty(&mut self, propertyid: PolicyServerUrlPropertyID, pvalue: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UpdateRegistry(&mut self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()>;
    fn RemoveFromRegistry(&mut self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509PolicyServerUrlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509PolicyServerUrlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509PolicyServerUrlVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn Url<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUrl(::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn Default<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Default() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefault<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefault(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Flags<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut PolicyServerUrlFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: PolicyServerUrlFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn AuthFlags<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509EnrollmentAuthFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthFlags<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: X509EnrollmentAuthFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Cost<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cost() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCost<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCost(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStringProperty<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: PolicyServerUrlPropertyID, ppvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringProperty(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringProperty<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: PolicyServerUrlPropertyID, pvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStringProperty(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn UpdateRegistry<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateRegistry(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn RemoveFromRegistry<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFromRegistry(::core::mem::transmute_copy(&context)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Url: Url::<Impl, IMPL_OFFSET>,
            SetUrl: SetUrl::<Impl, IMPL_OFFSET>,
            Default: Default::<Impl, IMPL_OFFSET>,
            SetDefault: SetDefault::<Impl, IMPL_OFFSET>,
            Flags: Flags::<Impl, IMPL_OFFSET>,
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
            AuthFlags: AuthFlags::<Impl, IMPL_OFFSET>,
            SetAuthFlags: SetAuthFlags::<Impl, IMPL_OFFSET>,
            Cost: Cost::<Impl, IMPL_OFFSET>,
            SetCost: SetCost::<Impl, IMPL_OFFSET>,
            GetStringProperty: GetStringProperty::<Impl, IMPL_OFFSET>,
            SetStringProperty: SetStringProperty::<Impl, IMPL_OFFSET>,
            UpdateRegistry: UpdateRegistry::<Impl, IMPL_OFFSET>,
            RemoveFromRegistry: RemoveFromRegistry::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509PolicyServerUrl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509PrivateKeyImpl: Sized + IDispatchImpl {
    fn Open(&mut self) -> ::windows::core::Result<()>;
    fn Create(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Verify(&mut self, verifytype: X509PrivateKeyVerify) -> ::windows::core::Result<()>;
    fn Import(&mut self, strexporttype: super::super::super::Foundation::BSTR, strencodedkey: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
    fn Export(&mut self, strexporttype: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ExportPublicKey(&mut self) -> ::windows::core::Result<IX509PublicKey>;
    fn ContainerName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetContainerName(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ContainerNamePrefix(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetContainerNamePrefix(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReaderName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetReaderName(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CspInformations(&mut self) -> ::windows::core::Result<ICspInformations>;
    fn SetCspInformations(&mut self, pvalue: ::core::option::Option<ICspInformations>) -> ::windows::core::Result<()>;
    fn CspStatus(&mut self) -> ::windows::core::Result<ICspStatus>;
    fn SetCspStatus(&mut self, pvalue: ::core::option::Option<ICspStatus>) -> ::windows::core::Result<()>;
    fn ProviderName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetProviderName(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ProviderType(&mut self) -> ::windows::core::Result<X509ProviderType>;
    fn SetProviderType(&mut self, value: X509ProviderType) -> ::windows::core::Result<()>;
    fn LegacyCsp(&mut self) -> ::windows::core::Result<i16>;
    fn SetLegacyCsp(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn Algorithm(&mut self) -> ::windows::core::Result<IObjectId>;
    fn SetAlgorithm(&mut self, pvalue: ::core::option::Option<IObjectId>) -> ::windows::core::Result<()>;
    fn KeySpec(&mut self) -> ::windows::core::Result<X509KeySpec>;
    fn SetKeySpec(&mut self, value: X509KeySpec) -> ::windows::core::Result<()>;
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn SetLength(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn ExportPolicy(&mut self) -> ::windows::core::Result<X509PrivateKeyExportFlags>;
    fn SetExportPolicy(&mut self, value: X509PrivateKeyExportFlags) -> ::windows::core::Result<()>;
    fn KeyUsage(&mut self) -> ::windows::core::Result<X509PrivateKeyUsageFlags>;
    fn SetKeyUsage(&mut self, value: X509PrivateKeyUsageFlags) -> ::windows::core::Result<()>;
    fn KeyProtection(&mut self) -> ::windows::core::Result<X509PrivateKeyProtection>;
    fn SetKeyProtection(&mut self, value: X509PrivateKeyProtection) -> ::windows::core::Result<()>;
    fn MachineContext(&mut self) -> ::windows::core::Result<i16>;
    fn SetMachineContext(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn SecurityDescriptor(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetSecurityDescriptor(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Certificate(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetCertificate(&mut self, encoding: EncodingType, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UniqueContainerName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Opened(&mut self) -> ::windows::core::Result<i16>;
    fn DefaultContainer(&mut self) -> ::windows::core::Result<i16>;
    fn Existing(&mut self) -> ::windows::core::Result<i16>;
    fn SetExisting(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn Silent(&mut self) -> ::windows::core::Result<i16>;
    fn SetSilent(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn ParentWindow(&mut self) -> ::windows::core::Result<i32>;
    fn SetParentWindow(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn UIContextMessage(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetUIContextMessage(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetPin(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetFriendlyName(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509PrivateKeyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509PrivateKeyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509PrivateKeyVtbl {
        unsafe extern "system" fn Open<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn Create<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create().into()
        }
        unsafe extern "system" fn Close<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Delete<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Verify<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, verifytype: X509PrivateKeyVerify) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Verify(::core::mem::transmute_copy(&verifytype)).into()
        }
        unsafe extern "system" fn Import<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strexporttype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strencodedkey: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Import(::core::mem::transmute_copy(&strexporttype), ::core::mem::transmute_copy(&strencodedkey), ::core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn Export<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strexporttype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, pstrencodedkey: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Export(::core::mem::transmute_copy(&strexporttype), ::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrencodedkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportPublicKey<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppublickey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportPublicKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pppublickey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainerName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContainerName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ContainerNamePrefix<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainerNamePrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerNamePrefix<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContainerNamePrefix(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReaderName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReaderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReaderName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReaderName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn CspInformations<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CspInformations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCspInformations<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCspInformations(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn CspStatus<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CspStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCspStatus<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCspStatus(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn ProviderName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ProviderType<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509ProviderType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderType<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509ProviderType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn LegacyCsp<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LegacyCsp() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLegacyCsp<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLegacyCsp(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Algorithm<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Algorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlgorithm<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlgorithm(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn KeySpec<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509KeySpec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeySpec() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeySpec<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509KeySpec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeySpec(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Length<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLength(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ExportPolicy<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509PrivateKeyExportFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExportPolicy<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509PrivateKeyExportFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExportPolicy(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn KeyUsage<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509PrivateKeyUsageFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyUsage<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509PrivateKeyUsageFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyUsage(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn KeyProtection<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509PrivateKeyProtection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyProtection() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyProtection<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509PrivateKeyProtection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyProtection(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn MachineContext<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MachineContext() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachineContext<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMachineContext(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SecurityDescriptor<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecurityDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Certificate<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Certificate(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificate<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCertificate(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UniqueContainerName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueContainerName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Opened<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opened() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultContainer<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Existing<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Existing() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExisting<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExisting(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Silent<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Silent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSilent<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSilent(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ParentWindow<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentWindow<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParentWindow(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UIContextMessage<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIContextMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUIContextMessage<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUIContextMessage(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetPin<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPin(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn FriendlyName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Description<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Verify: Verify::<Impl, IMPL_OFFSET>,
            Import: Import::<Impl, IMPL_OFFSET>,
            Export: Export::<Impl, IMPL_OFFSET>,
            ExportPublicKey: ExportPublicKey::<Impl, IMPL_OFFSET>,
            ContainerName: ContainerName::<Impl, IMPL_OFFSET>,
            SetContainerName: SetContainerName::<Impl, IMPL_OFFSET>,
            ContainerNamePrefix: ContainerNamePrefix::<Impl, IMPL_OFFSET>,
            SetContainerNamePrefix: SetContainerNamePrefix::<Impl, IMPL_OFFSET>,
            ReaderName: ReaderName::<Impl, IMPL_OFFSET>,
            SetReaderName: SetReaderName::<Impl, IMPL_OFFSET>,
            CspInformations: CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations: SetCspInformations::<Impl, IMPL_OFFSET>,
            CspStatus: CspStatus::<Impl, IMPL_OFFSET>,
            SetCspStatus: SetCspStatus::<Impl, IMPL_OFFSET>,
            ProviderName: ProviderName::<Impl, IMPL_OFFSET>,
            SetProviderName: SetProviderName::<Impl, IMPL_OFFSET>,
            ProviderType: ProviderType::<Impl, IMPL_OFFSET>,
            SetProviderType: SetProviderType::<Impl, IMPL_OFFSET>,
            LegacyCsp: LegacyCsp::<Impl, IMPL_OFFSET>,
            SetLegacyCsp: SetLegacyCsp::<Impl, IMPL_OFFSET>,
            Algorithm: Algorithm::<Impl, IMPL_OFFSET>,
            SetAlgorithm: SetAlgorithm::<Impl, IMPL_OFFSET>,
            KeySpec: KeySpec::<Impl, IMPL_OFFSET>,
            SetKeySpec: SetKeySpec::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            SetLength: SetLength::<Impl, IMPL_OFFSET>,
            ExportPolicy: ExportPolicy::<Impl, IMPL_OFFSET>,
            SetExportPolicy: SetExportPolicy::<Impl, IMPL_OFFSET>,
            KeyUsage: KeyUsage::<Impl, IMPL_OFFSET>,
            SetKeyUsage: SetKeyUsage::<Impl, IMPL_OFFSET>,
            KeyProtection: KeyProtection::<Impl, IMPL_OFFSET>,
            SetKeyProtection: SetKeyProtection::<Impl, IMPL_OFFSET>,
            MachineContext: MachineContext::<Impl, IMPL_OFFSET>,
            SetMachineContext: SetMachineContext::<Impl, IMPL_OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            Certificate: Certificate::<Impl, IMPL_OFFSET>,
            SetCertificate: SetCertificate::<Impl, IMPL_OFFSET>,
            UniqueContainerName: UniqueContainerName::<Impl, IMPL_OFFSET>,
            Opened: Opened::<Impl, IMPL_OFFSET>,
            DefaultContainer: DefaultContainer::<Impl, IMPL_OFFSET>,
            Existing: Existing::<Impl, IMPL_OFFSET>,
            SetExisting: SetExisting::<Impl, IMPL_OFFSET>,
            Silent: Silent::<Impl, IMPL_OFFSET>,
            SetSilent: SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow: ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow: SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage: UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage: SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SetPin: SetPin::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            SetFriendlyName: SetFriendlyName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509PrivateKey as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509PrivateKey2Impl: Sized + IDispatchImpl + IX509PrivateKeyImpl {
    fn HardwareKeyUsage(&mut self) -> ::windows::core::Result<X509HardwareKeyUsageFlags>;
    fn SetHardwareKeyUsage(&mut self, value: X509HardwareKeyUsageFlags) -> ::windows::core::Result<()>;
    fn AlternateStorageLocation(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetAlternateStorageLocation(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AlgorithmName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetAlgorithmName(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AlgorithmParameters(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetAlgorithmParameters(&mut self, encoding: EncodingType, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ParametersExportType(&mut self) -> ::windows::core::Result<X509KeyParametersExportType>;
    fn SetParametersExportType(&mut self, value: X509KeyParametersExportType) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509PrivateKey2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509PrivateKey2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509PrivateKey2Vtbl {
        unsafe extern "system" fn HardwareKeyUsage<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509HardwareKeyUsageFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareKeyUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHardwareKeyUsage<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509HardwareKeyUsageFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHardwareKeyUsage(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AlternateStorageLocation<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateStorageLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlternateStorageLocation<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlternateStorageLocation(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AlgorithmName<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlgorithmName<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlgorithmName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AlgorithmParameters<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlgorithmParameters(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlgorithmParameters<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlgorithmParameters(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ParametersExportType<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509KeyParametersExportType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParametersExportType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParametersExportType<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509KeyParametersExportType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParametersExportType(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IX509PrivateKeyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            HardwareKeyUsage: HardwareKeyUsage::<Impl, IMPL_OFFSET>,
            SetHardwareKeyUsage: SetHardwareKeyUsage::<Impl, IMPL_OFFSET>,
            AlternateStorageLocation: AlternateStorageLocation::<Impl, IMPL_OFFSET>,
            SetAlternateStorageLocation: SetAlternateStorageLocation::<Impl, IMPL_OFFSET>,
            AlgorithmName: AlgorithmName::<Impl, IMPL_OFFSET>,
            SetAlgorithmName: SetAlgorithmName::<Impl, IMPL_OFFSET>,
            AlgorithmParameters: AlgorithmParameters::<Impl, IMPL_OFFSET>,
            SetAlgorithmParameters: SetAlgorithmParameters::<Impl, IMPL_OFFSET>,
            ParametersExportType: ParametersExportType::<Impl, IMPL_OFFSET>,
            SetParametersExportType: SetParametersExportType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509PrivateKey2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509PublicKeyImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, pobjectid: ::core::option::Option<IObjectId>, strencodedkey: super::super::super::Foundation::BSTR, strencodedparameters: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
    fn InitializeFromEncodedPublicKeyInfo(&mut self, strencodedpublickeyinfo: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
    fn Algorithm(&mut self) -> ::windows::core::Result<IObjectId>;
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn EncodedKey(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn EncodedParameters(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ComputeKeyIdentifier(&mut self, algorithm: KeyIdentifierHashAlgorithm, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509PublicKeyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509PublicKeyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509PublicKeyVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, strencodedkey: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strencodedparameters: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pobjectid), ::core::mem::transmute_copy(&strencodedkey), ::core::mem::transmute_copy(&strencodedparameters), ::core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn InitializeFromEncodedPublicKeyInfo<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodedpublickeyinfo: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromEncodedPublicKeyInfo(::core::mem::transmute_copy(&strencodedpublickeyinfo), ::core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn Algorithm<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Algorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodedKey<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodedKey(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodedParameters<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodedParameters(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeKeyIdentifier<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, algorithm: KeyIdentifierHashAlgorithm, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputeKeyIdentifier(::core::mem::transmute_copy(&algorithm), ::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            InitializeFromEncodedPublicKeyInfo: InitializeFromEncodedPublicKeyInfo::<Impl, IMPL_OFFSET>,
            Algorithm: Algorithm::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            EncodedKey: EncodedKey::<Impl, IMPL_OFFSET>,
            EncodedParameters: EncodedParameters::<Impl, IMPL_OFFSET>,
            ComputeKeyIdentifier: ComputeKeyIdentifier::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509PublicKey as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509SCEPEnrollmentImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, prequest: ::core::option::Option<IX509CertificateRequestPkcs10>, strthumbprint: super::super::super::Foundation::BSTR, thumprintencoding: EncodingType, strservercertificates: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>;
    fn InitializeForPending(&mut self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()>;
    fn CreateRequestMessage(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn CreateRetrievePendingMessage(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn CreateRetrieveCertificateMessage(&mut self, context: X509CertificateEnrollmentContext, strissuer: super::super::super::Foundation::BSTR, issuerencoding: EncodingType, strserialnumber: super::super::super::Foundation::BSTR, serialnumberencoding: EncodingType, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ProcessResponseMessage(&mut self, strresponse: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<X509SCEPDisposition>;
    fn SetServerCapabilities(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FailInfo(&mut self) -> ::windows::core::Result<X509SCEPFailInfo>;
    fn SignerCertificate(&mut self) -> ::windows::core::Result<ISignerCertificate>;
    fn SetSignerCertificate(&mut self, pvalue: ::core::option::Option<ISignerCertificate>) -> ::windows::core::Result<()>;
    fn OldCertificate(&mut self) -> ::windows::core::Result<ISignerCertificate>;
    fn SetOldCertificate(&mut self, pvalue: ::core::option::Option<ISignerCertificate>) -> ::windows::core::Result<()>;
    fn TransactionId(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetTransactionId(&mut self, encoding: EncodingType, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Request(&mut self) -> ::windows::core::Result<IX509CertificateRequestPkcs10>;
    fn CertificateFriendlyName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetCertificateFriendlyName(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<IX509EnrollmentStatus>;
    fn Certificate(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Silent(&mut self) -> ::windows::core::Result<i16>;
    fn SetSilent(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn DeleteRequest(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509SCEPEnrollmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509SCEPEnrollmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509SCEPEnrollmentVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: ::windows::core::RawPtr, strthumbprint: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, thumprintencoding: EncodingType, strservercertificates: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&prequest), ::core::mem::transmute_copy(&strthumbprint), ::core::mem::transmute_copy(&thumprintencoding), ::core::mem::transmute_copy(&strservercertificates), ::core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn InitializeForPending<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeForPending(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn CreateRequestMessage<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRequestMessage(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRetrievePendingMessage<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRetrievePendingMessage(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRetrieveCertificateMessage<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, strissuer: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, issuerencoding: EncodingType, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, serialnumberencoding: EncodingType, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRetrieveCertificateMessage(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&strissuer), ::core::mem::transmute_copy(&issuerencoding), ::core::mem::transmute_copy(&strserialnumber), ::core::mem::transmute_copy(&serialnumberencoding), ::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessResponseMessage<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, pdisposition: *mut X509SCEPDisposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessResponseMessage(::core::mem::transmute_copy(&strresponse), ::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdisposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerCapabilities<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerCapabilities(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn FailInfo<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509SCEPFailInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FailInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignerCertificate<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignerCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignerCertificate<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignerCertificate(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn OldCertificate<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOldCertificate<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOldCertificate(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn TransactionId<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionId(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransactionId<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransactionId(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Request<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CertificateFriendlyName<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CertificateFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateFriendlyName<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCertificateFriendlyName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Status<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Certificate<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Certificate(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Silent<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Silent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSilent<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSilent(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DeleteRequest<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteRequest().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            InitializeForPending: InitializeForPending::<Impl, IMPL_OFFSET>,
            CreateRequestMessage: CreateRequestMessage::<Impl, IMPL_OFFSET>,
            CreateRetrievePendingMessage: CreateRetrievePendingMessage::<Impl, IMPL_OFFSET>,
            CreateRetrieveCertificateMessage: CreateRetrieveCertificateMessage::<Impl, IMPL_OFFSET>,
            ProcessResponseMessage: ProcessResponseMessage::<Impl, IMPL_OFFSET>,
            SetServerCapabilities: SetServerCapabilities::<Impl, IMPL_OFFSET>,
            FailInfo: FailInfo::<Impl, IMPL_OFFSET>,
            SignerCertificate: SignerCertificate::<Impl, IMPL_OFFSET>,
            SetSignerCertificate: SetSignerCertificate::<Impl, IMPL_OFFSET>,
            OldCertificate: OldCertificate::<Impl, IMPL_OFFSET>,
            SetOldCertificate: SetOldCertificate::<Impl, IMPL_OFFSET>,
            TransactionId: TransactionId::<Impl, IMPL_OFFSET>,
            SetTransactionId: SetTransactionId::<Impl, IMPL_OFFSET>,
            Request: Request::<Impl, IMPL_OFFSET>,
            CertificateFriendlyName: CertificateFriendlyName::<Impl, IMPL_OFFSET>,
            SetCertificateFriendlyName: SetCertificateFriendlyName::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Certificate: Certificate::<Impl, IMPL_OFFSET>,
            Silent: Silent::<Impl, IMPL_OFFSET>,
            SetSilent: SetSilent::<Impl, IMPL_OFFSET>,
            DeleteRequest: DeleteRequest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509SCEPEnrollment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509SCEPEnrollment2Impl: Sized + IDispatchImpl + IX509SCEPEnrollmentImpl {
    fn CreateChallengeAnswerMessage(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn ProcessResponseMessage2(&mut self, flags: X509SCEPProcessMessageFlags, strresponse: super::super::super::Foundation::BSTR, encoding: EncodingType) -> ::windows::core::Result<X509SCEPDisposition>;
    fn ResultMessageText(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn DelayRetry(&mut self) -> ::windows::core::Result<DelayRetryAction>;
    fn ActivityId(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetActivityId(&mut self, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509SCEPEnrollment2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509SCEPEnrollment2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509SCEPEnrollment2Vtbl {
        unsafe extern "system" fn CreateChallengeAnswerMessage<Impl: IX509SCEPEnrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateChallengeAnswerMessage(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessResponseMessage2<Impl: IX509SCEPEnrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: X509SCEPProcessMessageFlags, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, pdisposition: *mut X509SCEPDisposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessResponseMessage2(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&strresponse), ::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdisposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultMessageText<Impl: IX509SCEPEnrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultMessageText() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DelayRetry<Impl: IX509SCEPEnrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut DelayRetryAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelayRetry() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityId<Impl: IX509SCEPEnrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityId<Impl: IX509SCEPEnrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActivityId(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IX509SCEPEnrollmentVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateChallengeAnswerMessage: CreateChallengeAnswerMessage::<Impl, IMPL_OFFSET>,
            ProcessResponseMessage2: ProcessResponseMessage2::<Impl, IMPL_OFFSET>,
            ResultMessageText: ResultMessageText::<Impl, IMPL_OFFSET>,
            DelayRetry: DelayRetry::<Impl, IMPL_OFFSET>,
            ActivityId: ActivityId::<Impl, IMPL_OFFSET>,
            SetActivityId: SetActivityId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509SCEPEnrollment2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509SCEPEnrollmentHelperImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self, strserverurl: super::super::super::Foundation::BSTR, strrequestheaders: super::super::super::Foundation::BSTR, prequest: ::core::option::Option<IX509CertificateRequestPkcs10>, strcacertificatethumbprint: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeForPending(&mut self, strserverurl: super::super::super::Foundation::BSTR, strrequestheaders: super::super::super::Foundation::BSTR, context: X509CertificateEnrollmentContext, strtransactionid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enroll(&mut self, processflags: X509SCEPProcessMessageFlags) -> ::windows::core::Result<X509SCEPDisposition>;
    fn FetchPending(&mut self, processflags: X509SCEPProcessMessageFlags) -> ::windows::core::Result<X509SCEPDisposition>;
    fn X509SCEPEnrollment(&mut self) -> ::windows::core::Result<IX509SCEPEnrollment>;
    fn ResultMessageText(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509SCEPEnrollmentHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509SCEPEnrollmentHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509SCEPEnrollmentHelperVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509SCEPEnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strrequestheaders: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, prequest: ::windows::core::RawPtr, strcacertificatethumbprint: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&strserverurl), ::core::mem::transmute_copy(&strrequestheaders), ::core::mem::transmute(&prequest), ::core::mem::transmute_copy(&strcacertificatethumbprint)).into()
        }
        unsafe extern "system" fn InitializeForPending<Impl: IX509SCEPEnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strrequestheaders: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, context: X509CertificateEnrollmentContext, strtransactionid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeForPending(::core::mem::transmute_copy(&strserverurl), ::core::mem::transmute_copy(&strrequestheaders), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&strtransactionid)).into()
        }
        unsafe extern "system" fn Enroll<Impl: IX509SCEPEnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processflags: X509SCEPProcessMessageFlags, pdisposition: *mut X509SCEPDisposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enroll(::core::mem::transmute_copy(&processflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdisposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FetchPending<Impl: IX509SCEPEnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processflags: X509SCEPProcessMessageFlags, pdisposition: *mut X509SCEPDisposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FetchPending(::core::mem::transmute_copy(&processflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdisposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X509SCEPEnrollment<Impl: IX509SCEPEnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X509SCEPEnrollment() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultMessageText<Impl: IX509SCEPEnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultMessageText() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            InitializeForPending: InitializeForPending::<Impl, IMPL_OFFSET>,
            Enroll: Enroll::<Impl, IMPL_OFFSET>,
            FetchPending: FetchPending::<Impl, IMPL_OFFSET>,
            X509SCEPEnrollment: X509SCEPEnrollment::<Impl, IMPL_OFFSET>,
            ResultMessageText: ResultMessageText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509SCEPEnrollmentHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509SignatureInformationImpl: Sized + IDispatchImpl {
    fn HashAlgorithm(&mut self) -> ::windows::core::Result<IObjectId>;
    fn SetHashAlgorithm(&mut self, pvalue: ::core::option::Option<IObjectId>) -> ::windows::core::Result<()>;
    fn PublicKeyAlgorithm(&mut self) -> ::windows::core::Result<IObjectId>;
    fn SetPublicKeyAlgorithm(&mut self, pvalue: ::core::option::Option<IObjectId>) -> ::windows::core::Result<()>;
    fn Parameters(&mut self, encoding: EncodingType) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetParameters(&mut self, encoding: EncodingType, value: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AlternateSignatureAlgorithm(&mut self) -> ::windows::core::Result<i16>;
    fn SetAlternateSignatureAlgorithm(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn AlternateSignatureAlgorithmSet(&mut self) -> ::windows::core::Result<i16>;
    fn NullSigned(&mut self) -> ::windows::core::Result<i16>;
    fn SetNullSigned(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn GetSignatureAlgorithm(&mut self, pkcs7signature: i16, signaturekey: i16) -> ::windows::core::Result<IObjectId>;
    fn SetDefaultValues(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509SignatureInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509SignatureInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509SignatureInformationVtbl {
        unsafe extern "system" fn HashAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn PublicKeyAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublicKeyAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublicKeyAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPublicKeyAlgorithm(::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameters(::core::mem::transmute_copy(&encoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AlternateSignatureAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateSignatureAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlternateSignatureAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlternateSignatureAlgorithm(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AlternateSignatureAlgorithmSet<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateSignatureAlgorithmSet() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NullSigned<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NullSigned() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNullSigned<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNullSigned(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSignatureAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkcs7signature: i16, signaturekey: i16, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureAlgorithm(::core::mem::transmute_copy(&pkcs7signature), ::core::mem::transmute_copy(&signaturekey)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultValues<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultValues().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            HashAlgorithm: HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            PublicKeyAlgorithm: PublicKeyAlgorithm::<Impl, IMPL_OFFSET>,
            SetPublicKeyAlgorithm: SetPublicKeyAlgorithm::<Impl, IMPL_OFFSET>,
            Parameters: Parameters::<Impl, IMPL_OFFSET>,
            SetParameters: SetParameters::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm: AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm: SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithmSet: AlternateSignatureAlgorithmSet::<Impl, IMPL_OFFSET>,
            NullSigned: NullSigned::<Impl, IMPL_OFFSET>,
            SetNullSigned: SetNullSigned::<Impl, IMPL_OFFSET>,
            GetSignatureAlgorithm: GetSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetDefaultValues: SetDefaultValues::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509SignatureInformation as ::windows::core::Interface>::IID
    }
}
