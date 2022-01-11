#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAlternativeNameImpl: Sized + IDispatchImpl {
    fn InitializeFromString();
    fn InitializeFromRawData();
    fn InitializeFromOtherName();
    fn Type();
    fn StrValue();
    fn ObjectId();
    fn RawData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAlternativeNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlternativeNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAlternativeNameVtbl {
        unsafe extern "system" fn InitializeFromString<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AlternativeNameType, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromRawData<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AlternativeNameType, encoding: EncodingType, strrawdata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromOtherName<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, encoding: EncodingType, strrawdata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, tobewrapped: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut AlternativeNameType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StrValue<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ObjectId<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RawData<Impl: IAlternativeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            InitializeFromString::<Impl, IMPL_OFFSET>,
            InitializeFromRawData::<Impl, IMPL_OFFSET>,
            InitializeFromOtherName::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            StrValue::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlternativeName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAlternativeNamesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAlternativeNamesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlternativeNamesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAlternativeNamesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, ItemByIndex::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlternativeNames as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBinaryConverterImpl: Sized + IDispatchImpl {
    fn StringToString();
    fn VariantByteArrayToString();
    fn StringToVariantByteArray();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBinaryConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBinaryConverterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBinaryConverterVtbl {
        unsafe extern "system" fn StringToString<Impl: IBinaryConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodedin: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encodingin: EncodingType, encoding: EncodingType, pstrencoded: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VariantByteArrayToString<Impl: IBinaryConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbytearray: *const super::super::super::System::Com::VARIANT, encoding: EncodingType, pstrencoded: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StringToVariantByteArray<Impl: IBinaryConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencoded: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, pvarbytearray: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, StringToString::<Impl, IMPL_OFFSET>, VariantByteArrayToString::<Impl, IMPL_OFFSET>, StringToVariantByteArray::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBinaryConverter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBinaryConverter2Impl: Sized + IBinaryConverterImpl + IDispatchImpl {
    fn StringArrayToVariantArray();
    fn VariantArrayToStringArray();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBinaryConverter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBinaryConverter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBinaryConverter2Vtbl {
        unsafe extern "system" fn StringArrayToVariantArray<Impl: IBinaryConverter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarstringarray: *const super::super::super::System::Com::VARIANT, pvarvariantarray: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VariantArrayToStringArray<Impl: IBinaryConverter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvariantarray: *const super::super::super::System::Com::VARIANT, pvarstringarray: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            StringToString::<Impl, IMPL_OFFSET>,
            VariantByteArrayToString::<Impl, IMPL_OFFSET>,
            StringToVariantByteArray::<Impl, IMPL_OFFSET>,
            StringArrayToVariantArray::<Impl, IMPL_OFFSET>,
            VariantArrayToStringArray::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBinaryConverter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICEnrollImpl: Sized + IDispatchImpl {
    fn createFilePKCS10();
    fn acceptFilePKCS7();
    fn createPKCS10();
    fn acceptPKCS7();
    fn getCertFromPKCS7();
    fn enumProviders();
    fn enumContainers();
    fn freeRequestInfo();
    fn MyStoreName();
    fn SetMyStoreName();
    fn MyStoreType();
    fn SetMyStoreType();
    fn MyStoreFlags();
    fn SetMyStoreFlags();
    fn CAStoreName();
    fn SetCAStoreName();
    fn CAStoreType();
    fn SetCAStoreType();
    fn CAStoreFlags();
    fn SetCAStoreFlags();
    fn RootStoreName();
    fn SetRootStoreName();
    fn RootStoreType();
    fn SetRootStoreType();
    fn RootStoreFlags();
    fn SetRootStoreFlags();
    fn RequestStoreName();
    fn SetRequestStoreName();
    fn RequestStoreType();
    fn SetRequestStoreType();
    fn RequestStoreFlags();
    fn SetRequestStoreFlags();
    fn ContainerName();
    fn SetContainerName();
    fn ProviderName();
    fn SetProviderName();
    fn ProviderType();
    fn SetProviderType();
    fn KeySpec();
    fn SetKeySpec();
    fn ProviderFlags();
    fn SetProviderFlags();
    fn UseExistingKeySet();
    fn SetUseExistingKeySet();
    fn GenKeyFlags();
    fn SetGenKeyFlags();
    fn DeleteRequestCert();
    fn SetDeleteRequestCert();
    fn WriteCertToCSP();
    fn SetWriteCertToCSP();
    fn SPCFileName();
    fn SetSPCFileName();
    fn PVKFileName();
    fn SetPVKFileName();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICEnrollVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICEnrollImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICEnrollVtbl {
        unsafe extern "system" fn createFilePKCS10<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usage: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, wszpkcs10filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn acceptFilePKCS7<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpkcs7filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createPKCS10<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usage: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppkcs10: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn acceptPKCS7<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkcs7: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getCertFromPKCS7<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpkcs7: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrcert: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn enumProviders<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, dwflags: i32, pbstrprovname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn enumContainers<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn freeRequestInfo<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkcs7orpkcs10: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MyStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMyStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MyStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMyStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MyStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMyStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CAStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCAStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CAStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCAStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CAStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCAStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RootStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRootStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RootStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRootStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RootStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRootStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRequestStoreName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRequestStoreType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRequestStoreFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContainerName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontainer: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContainerName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontainer: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprovider: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprovider: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderType<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeySpec<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdw: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeySpec<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dw: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UseExistingKeySet<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuseexistingkeys: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUseExistingKeySet<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuseexistingkeys: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenKeyFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGenKeyFlags<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteRequestCert<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdelete: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDeleteRequestCert<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdelete: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteCertToCSP<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWriteCertToCSP<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SPCFileName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSPCFileName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PVKFileName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPVKFileName<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: ICEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            createFilePKCS10::<Impl, IMPL_OFFSET>,
            acceptFilePKCS7::<Impl, IMPL_OFFSET>,
            createPKCS10::<Impl, IMPL_OFFSET>,
            acceptPKCS7::<Impl, IMPL_OFFSET>,
            getCertFromPKCS7::<Impl, IMPL_OFFSET>,
            enumProviders::<Impl, IMPL_OFFSET>,
            enumContainers::<Impl, IMPL_OFFSET>,
            freeRequestInfo::<Impl, IMPL_OFFSET>,
            MyStoreName::<Impl, IMPL_OFFSET>,
            SetMyStoreName::<Impl, IMPL_OFFSET>,
            MyStoreType::<Impl, IMPL_OFFSET>,
            SetMyStoreType::<Impl, IMPL_OFFSET>,
            MyStoreFlags::<Impl, IMPL_OFFSET>,
            SetMyStoreFlags::<Impl, IMPL_OFFSET>,
            CAStoreName::<Impl, IMPL_OFFSET>,
            SetCAStoreName::<Impl, IMPL_OFFSET>,
            CAStoreType::<Impl, IMPL_OFFSET>,
            SetCAStoreType::<Impl, IMPL_OFFSET>,
            CAStoreFlags::<Impl, IMPL_OFFSET>,
            SetCAStoreFlags::<Impl, IMPL_OFFSET>,
            RootStoreName::<Impl, IMPL_OFFSET>,
            SetRootStoreName::<Impl, IMPL_OFFSET>,
            RootStoreType::<Impl, IMPL_OFFSET>,
            SetRootStoreType::<Impl, IMPL_OFFSET>,
            RootStoreFlags::<Impl, IMPL_OFFSET>,
            SetRootStoreFlags::<Impl, IMPL_OFFSET>,
            RequestStoreName::<Impl, IMPL_OFFSET>,
            SetRequestStoreName::<Impl, IMPL_OFFSET>,
            RequestStoreType::<Impl, IMPL_OFFSET>,
            SetRequestStoreType::<Impl, IMPL_OFFSET>,
            RequestStoreFlags::<Impl, IMPL_OFFSET>,
            SetRequestStoreFlags::<Impl, IMPL_OFFSET>,
            ContainerName::<Impl, IMPL_OFFSET>,
            SetContainerName::<Impl, IMPL_OFFSET>,
            ProviderName::<Impl, IMPL_OFFSET>,
            SetProviderName::<Impl, IMPL_OFFSET>,
            ProviderType::<Impl, IMPL_OFFSET>,
            SetProviderType::<Impl, IMPL_OFFSET>,
            KeySpec::<Impl, IMPL_OFFSET>,
            SetKeySpec::<Impl, IMPL_OFFSET>,
            ProviderFlags::<Impl, IMPL_OFFSET>,
            SetProviderFlags::<Impl, IMPL_OFFSET>,
            UseExistingKeySet::<Impl, IMPL_OFFSET>,
            SetUseExistingKeySet::<Impl, IMPL_OFFSET>,
            GenKeyFlags::<Impl, IMPL_OFFSET>,
            SetGenKeyFlags::<Impl, IMPL_OFFSET>,
            DeleteRequestCert::<Impl, IMPL_OFFSET>,
            SetDeleteRequestCert::<Impl, IMPL_OFFSET>,
            WriteCertToCSP::<Impl, IMPL_OFFSET>,
            SetWriteCertToCSP::<Impl, IMPL_OFFSET>,
            SPCFileName::<Impl, IMPL_OFFSET>,
            SetSPCFileName::<Impl, IMPL_OFFSET>,
            PVKFileName::<Impl, IMPL_OFFSET>,
            SetPVKFileName::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICEnroll as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICEnroll2Impl: Sized + ICEnrollImpl + IDispatchImpl {
    fn addCertTypeToRequest();
    fn addNameValuePairToSignature();
    fn WriteCertToUserDS();
    fn SetWriteCertToUserDS();
    fn EnableT61DNEncoding();
    fn SetEnableT61DNEncoding();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICEnroll2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICEnroll2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICEnroll2Vtbl {
        unsafe extern "system" fn addCertTypeToRequest<Impl: ICEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addNameValuePairToSignature<Impl: ICEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteCertToUserDS<Impl: ICEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWriteCertToUserDS<Impl: ICEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableT61DNEncoding<Impl: ICEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableT61DNEncoding<Impl: ICEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
            createFilePKCS10::<Impl, IMPL_OFFSET>,
            acceptFilePKCS7::<Impl, IMPL_OFFSET>,
            createPKCS10::<Impl, IMPL_OFFSET>,
            acceptPKCS7::<Impl, IMPL_OFFSET>,
            getCertFromPKCS7::<Impl, IMPL_OFFSET>,
            enumProviders::<Impl, IMPL_OFFSET>,
            enumContainers::<Impl, IMPL_OFFSET>,
            freeRequestInfo::<Impl, IMPL_OFFSET>,
            MyStoreName::<Impl, IMPL_OFFSET>,
            SetMyStoreName::<Impl, IMPL_OFFSET>,
            MyStoreType::<Impl, IMPL_OFFSET>,
            SetMyStoreType::<Impl, IMPL_OFFSET>,
            MyStoreFlags::<Impl, IMPL_OFFSET>,
            SetMyStoreFlags::<Impl, IMPL_OFFSET>,
            CAStoreName::<Impl, IMPL_OFFSET>,
            SetCAStoreName::<Impl, IMPL_OFFSET>,
            CAStoreType::<Impl, IMPL_OFFSET>,
            SetCAStoreType::<Impl, IMPL_OFFSET>,
            CAStoreFlags::<Impl, IMPL_OFFSET>,
            SetCAStoreFlags::<Impl, IMPL_OFFSET>,
            RootStoreName::<Impl, IMPL_OFFSET>,
            SetRootStoreName::<Impl, IMPL_OFFSET>,
            RootStoreType::<Impl, IMPL_OFFSET>,
            SetRootStoreType::<Impl, IMPL_OFFSET>,
            RootStoreFlags::<Impl, IMPL_OFFSET>,
            SetRootStoreFlags::<Impl, IMPL_OFFSET>,
            RequestStoreName::<Impl, IMPL_OFFSET>,
            SetRequestStoreName::<Impl, IMPL_OFFSET>,
            RequestStoreType::<Impl, IMPL_OFFSET>,
            SetRequestStoreType::<Impl, IMPL_OFFSET>,
            RequestStoreFlags::<Impl, IMPL_OFFSET>,
            SetRequestStoreFlags::<Impl, IMPL_OFFSET>,
            ContainerName::<Impl, IMPL_OFFSET>,
            SetContainerName::<Impl, IMPL_OFFSET>,
            ProviderName::<Impl, IMPL_OFFSET>,
            SetProviderName::<Impl, IMPL_OFFSET>,
            ProviderType::<Impl, IMPL_OFFSET>,
            SetProviderType::<Impl, IMPL_OFFSET>,
            KeySpec::<Impl, IMPL_OFFSET>,
            SetKeySpec::<Impl, IMPL_OFFSET>,
            ProviderFlags::<Impl, IMPL_OFFSET>,
            SetProviderFlags::<Impl, IMPL_OFFSET>,
            UseExistingKeySet::<Impl, IMPL_OFFSET>,
            SetUseExistingKeySet::<Impl, IMPL_OFFSET>,
            GenKeyFlags::<Impl, IMPL_OFFSET>,
            SetGenKeyFlags::<Impl, IMPL_OFFSET>,
            DeleteRequestCert::<Impl, IMPL_OFFSET>,
            SetDeleteRequestCert::<Impl, IMPL_OFFSET>,
            WriteCertToCSP::<Impl, IMPL_OFFSET>,
            SetWriteCertToCSP::<Impl, IMPL_OFFSET>,
            SPCFileName::<Impl, IMPL_OFFSET>,
            SetSPCFileName::<Impl, IMPL_OFFSET>,
            PVKFileName::<Impl, IMPL_OFFSET>,
            SetPVKFileName::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            addCertTypeToRequest::<Impl, IMPL_OFFSET>,
            addNameValuePairToSignature::<Impl, IMPL_OFFSET>,
            WriteCertToUserDS::<Impl, IMPL_OFFSET>,
            SetWriteCertToUserDS::<Impl, IMPL_OFFSET>,
            EnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            SetEnableT61DNEncoding::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICEnroll2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICEnroll3Impl: Sized + ICEnroll2Impl + ICEnrollImpl + IDispatchImpl {
    fn InstallPKCS7();
    fn Reset();
    fn GetSupportedKeySpec();
    fn GetKeyLen();
    fn EnumAlgs();
    fn GetAlgName();
    fn SetReuseHardwareKeyIfUnableToGenNew();
    fn ReuseHardwareKeyIfUnableToGenNew();
    fn SetHashAlgID();
    fn HashAlgID();
    fn SetLimitExchangeKeyToEncipherment();
    fn LimitExchangeKeyToEncipherment();
    fn SetEnableSMIMECapabilities();
    fn EnableSMIMECapabilities();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICEnroll3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICEnroll3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICEnroll3Vtbl {
        unsafe extern "system" fn InstallPKCS7<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkcs7: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedKeySpec<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwkeyspec: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeyLen<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmin: super::super::super::Foundation::BOOL, fexchange: super::super::super::Foundation::BOOL, pdwkeysize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAlgs<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, algclass: i32, pdwalgid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAlgName<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, algid: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReuseHardwareKeyIfUnableToGenNew<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freusehardwarekeyifunabletogennew: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReuseHardwareKeyIfUnableToGenNew<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freusehardwarekeyifunabletogennew: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashAlgID<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashalgid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HashAlgID<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashalgid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLimitExchangeKeyToEncipherment<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flimitexchangekeytoencipherment: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LimitExchangeKeyToEncipherment<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flimitexchangekeytoencipherment: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableSMIMECapabilities<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablesmimecapabilities: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableSMIMECapabilities<Impl: ICEnroll3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablesmimecapabilities: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
            createFilePKCS10::<Impl, IMPL_OFFSET>,
            acceptFilePKCS7::<Impl, IMPL_OFFSET>,
            createPKCS10::<Impl, IMPL_OFFSET>,
            acceptPKCS7::<Impl, IMPL_OFFSET>,
            getCertFromPKCS7::<Impl, IMPL_OFFSET>,
            enumProviders::<Impl, IMPL_OFFSET>,
            enumContainers::<Impl, IMPL_OFFSET>,
            freeRequestInfo::<Impl, IMPL_OFFSET>,
            MyStoreName::<Impl, IMPL_OFFSET>,
            SetMyStoreName::<Impl, IMPL_OFFSET>,
            MyStoreType::<Impl, IMPL_OFFSET>,
            SetMyStoreType::<Impl, IMPL_OFFSET>,
            MyStoreFlags::<Impl, IMPL_OFFSET>,
            SetMyStoreFlags::<Impl, IMPL_OFFSET>,
            CAStoreName::<Impl, IMPL_OFFSET>,
            SetCAStoreName::<Impl, IMPL_OFFSET>,
            CAStoreType::<Impl, IMPL_OFFSET>,
            SetCAStoreType::<Impl, IMPL_OFFSET>,
            CAStoreFlags::<Impl, IMPL_OFFSET>,
            SetCAStoreFlags::<Impl, IMPL_OFFSET>,
            RootStoreName::<Impl, IMPL_OFFSET>,
            SetRootStoreName::<Impl, IMPL_OFFSET>,
            RootStoreType::<Impl, IMPL_OFFSET>,
            SetRootStoreType::<Impl, IMPL_OFFSET>,
            RootStoreFlags::<Impl, IMPL_OFFSET>,
            SetRootStoreFlags::<Impl, IMPL_OFFSET>,
            RequestStoreName::<Impl, IMPL_OFFSET>,
            SetRequestStoreName::<Impl, IMPL_OFFSET>,
            RequestStoreType::<Impl, IMPL_OFFSET>,
            SetRequestStoreType::<Impl, IMPL_OFFSET>,
            RequestStoreFlags::<Impl, IMPL_OFFSET>,
            SetRequestStoreFlags::<Impl, IMPL_OFFSET>,
            ContainerName::<Impl, IMPL_OFFSET>,
            SetContainerName::<Impl, IMPL_OFFSET>,
            ProviderName::<Impl, IMPL_OFFSET>,
            SetProviderName::<Impl, IMPL_OFFSET>,
            ProviderType::<Impl, IMPL_OFFSET>,
            SetProviderType::<Impl, IMPL_OFFSET>,
            KeySpec::<Impl, IMPL_OFFSET>,
            SetKeySpec::<Impl, IMPL_OFFSET>,
            ProviderFlags::<Impl, IMPL_OFFSET>,
            SetProviderFlags::<Impl, IMPL_OFFSET>,
            UseExistingKeySet::<Impl, IMPL_OFFSET>,
            SetUseExistingKeySet::<Impl, IMPL_OFFSET>,
            GenKeyFlags::<Impl, IMPL_OFFSET>,
            SetGenKeyFlags::<Impl, IMPL_OFFSET>,
            DeleteRequestCert::<Impl, IMPL_OFFSET>,
            SetDeleteRequestCert::<Impl, IMPL_OFFSET>,
            WriteCertToCSP::<Impl, IMPL_OFFSET>,
            SetWriteCertToCSP::<Impl, IMPL_OFFSET>,
            SPCFileName::<Impl, IMPL_OFFSET>,
            SetSPCFileName::<Impl, IMPL_OFFSET>,
            PVKFileName::<Impl, IMPL_OFFSET>,
            SetPVKFileName::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            addCertTypeToRequest::<Impl, IMPL_OFFSET>,
            addNameValuePairToSignature::<Impl, IMPL_OFFSET>,
            WriteCertToUserDS::<Impl, IMPL_OFFSET>,
            SetWriteCertToUserDS::<Impl, IMPL_OFFSET>,
            EnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            SetEnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            InstallPKCS7::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            GetSupportedKeySpec::<Impl, IMPL_OFFSET>,
            GetKeyLen::<Impl, IMPL_OFFSET>,
            EnumAlgs::<Impl, IMPL_OFFSET>,
            GetAlgName::<Impl, IMPL_OFFSET>,
            SetReuseHardwareKeyIfUnableToGenNew::<Impl, IMPL_OFFSET>,
            ReuseHardwareKeyIfUnableToGenNew::<Impl, IMPL_OFFSET>,
            SetHashAlgID::<Impl, IMPL_OFFSET>,
            HashAlgID::<Impl, IMPL_OFFSET>,
            SetLimitExchangeKeyToEncipherment::<Impl, IMPL_OFFSET>,
            LimitExchangeKeyToEncipherment::<Impl, IMPL_OFFSET>,
            SetEnableSMIMECapabilities::<Impl, IMPL_OFFSET>,
            EnableSMIMECapabilities::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICEnroll3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICEnroll4Impl: Sized + ICEnroll3Impl + ICEnroll2Impl + ICEnrollImpl + IDispatchImpl {
    fn SetPrivateKeyArchiveCertificate();
    fn PrivateKeyArchiveCertificate();
    fn SetThumbPrint();
    fn ThumbPrint();
    fn binaryToString();
    fn stringToBinary();
    fn addExtensionToRequest();
    fn addAttributeToRequest();
    fn addNameValuePairToRequest();
    fn resetExtensions();
    fn resetAttributes();
    fn createRequest();
    fn createFileRequest();
    fn acceptResponse();
    fn acceptFileResponse();
    fn getCertFromResponse();
    fn getCertFromFileResponse();
    fn createPFX();
    fn createFilePFX();
    fn setPendingRequestInfo();
    fn enumPendingRequest();
    fn removePendingRequest();
    fn GetKeyLenEx();
    fn InstallPKCS7Ex();
    fn addCertTypeToRequestEx();
    fn getProviderType();
    fn SetSignerCertificate();
    fn SetClientId();
    fn ClientId();
    fn addBlobPropertyToCertificate();
    fn resetBlobProperties();
    fn SetIncludeSubjectKeyID();
    fn IncludeSubjectKeyID();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICEnroll4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICEnroll4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICEnroll4Vtbl {
        unsafe extern "system" fn SetPrivateKeyArchiveCertificate<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcert: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivateKeyArchiveCertificate<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcert: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetThumbPrint<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrthumbprint: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ThumbPrint<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrthumbprint: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn binaryToString<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrencoded: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn stringToBinary<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, strencoded: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addExtensionToRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addAttributeToRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addNameValuePairToRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn resetExtensions<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn resetAttributes<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usage: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrrequest: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createFileRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strusage: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strrequestfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn acceptResponse<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn acceptFileResponse<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresponsefilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getCertFromResponse<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrcert: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getCertFromFileResponse<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresponsefilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrcert: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createPFX<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrpfx: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createFilePFX<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpfxfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setPendingRequestInfo<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrequestid: i32, strcadns: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strcaname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strfriendlyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn enumPendingRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY, pvarproperty: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removePendingRequest<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strthumbprint: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeyLenEx<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC, pdwkeysize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallPKCS7Ex<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkcs7: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, plcertinstalled: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addCertTypeToRequestEx<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltype: ADDED_CERT_TYPE, bstroidorname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, lmajorversion: i32, fminorversion: super::super::super::Foundation::BOOL, lminorversion: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getProviderType<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprovname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, plprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignerCertificate<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcert: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientId<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lclientid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientId<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclientid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addBlobPropertyToCertificate<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropertyid: i32, lreserved: i32, bstrproperty: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn resetBlobProperties<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIncludeSubjectKeyID<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finclude: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IncludeSubjectKeyID<Impl: ICEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinclude: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
            createFilePKCS10::<Impl, IMPL_OFFSET>,
            acceptFilePKCS7::<Impl, IMPL_OFFSET>,
            createPKCS10::<Impl, IMPL_OFFSET>,
            acceptPKCS7::<Impl, IMPL_OFFSET>,
            getCertFromPKCS7::<Impl, IMPL_OFFSET>,
            enumProviders::<Impl, IMPL_OFFSET>,
            enumContainers::<Impl, IMPL_OFFSET>,
            freeRequestInfo::<Impl, IMPL_OFFSET>,
            MyStoreName::<Impl, IMPL_OFFSET>,
            SetMyStoreName::<Impl, IMPL_OFFSET>,
            MyStoreType::<Impl, IMPL_OFFSET>,
            SetMyStoreType::<Impl, IMPL_OFFSET>,
            MyStoreFlags::<Impl, IMPL_OFFSET>,
            SetMyStoreFlags::<Impl, IMPL_OFFSET>,
            CAStoreName::<Impl, IMPL_OFFSET>,
            SetCAStoreName::<Impl, IMPL_OFFSET>,
            CAStoreType::<Impl, IMPL_OFFSET>,
            SetCAStoreType::<Impl, IMPL_OFFSET>,
            CAStoreFlags::<Impl, IMPL_OFFSET>,
            SetCAStoreFlags::<Impl, IMPL_OFFSET>,
            RootStoreName::<Impl, IMPL_OFFSET>,
            SetRootStoreName::<Impl, IMPL_OFFSET>,
            RootStoreType::<Impl, IMPL_OFFSET>,
            SetRootStoreType::<Impl, IMPL_OFFSET>,
            RootStoreFlags::<Impl, IMPL_OFFSET>,
            SetRootStoreFlags::<Impl, IMPL_OFFSET>,
            RequestStoreName::<Impl, IMPL_OFFSET>,
            SetRequestStoreName::<Impl, IMPL_OFFSET>,
            RequestStoreType::<Impl, IMPL_OFFSET>,
            SetRequestStoreType::<Impl, IMPL_OFFSET>,
            RequestStoreFlags::<Impl, IMPL_OFFSET>,
            SetRequestStoreFlags::<Impl, IMPL_OFFSET>,
            ContainerName::<Impl, IMPL_OFFSET>,
            SetContainerName::<Impl, IMPL_OFFSET>,
            ProviderName::<Impl, IMPL_OFFSET>,
            SetProviderName::<Impl, IMPL_OFFSET>,
            ProviderType::<Impl, IMPL_OFFSET>,
            SetProviderType::<Impl, IMPL_OFFSET>,
            KeySpec::<Impl, IMPL_OFFSET>,
            SetKeySpec::<Impl, IMPL_OFFSET>,
            ProviderFlags::<Impl, IMPL_OFFSET>,
            SetProviderFlags::<Impl, IMPL_OFFSET>,
            UseExistingKeySet::<Impl, IMPL_OFFSET>,
            SetUseExistingKeySet::<Impl, IMPL_OFFSET>,
            GenKeyFlags::<Impl, IMPL_OFFSET>,
            SetGenKeyFlags::<Impl, IMPL_OFFSET>,
            DeleteRequestCert::<Impl, IMPL_OFFSET>,
            SetDeleteRequestCert::<Impl, IMPL_OFFSET>,
            WriteCertToCSP::<Impl, IMPL_OFFSET>,
            SetWriteCertToCSP::<Impl, IMPL_OFFSET>,
            SPCFileName::<Impl, IMPL_OFFSET>,
            SetSPCFileName::<Impl, IMPL_OFFSET>,
            PVKFileName::<Impl, IMPL_OFFSET>,
            SetPVKFileName::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            addCertTypeToRequest::<Impl, IMPL_OFFSET>,
            addNameValuePairToSignature::<Impl, IMPL_OFFSET>,
            WriteCertToUserDS::<Impl, IMPL_OFFSET>,
            SetWriteCertToUserDS::<Impl, IMPL_OFFSET>,
            EnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            SetEnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            InstallPKCS7::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            GetSupportedKeySpec::<Impl, IMPL_OFFSET>,
            GetKeyLen::<Impl, IMPL_OFFSET>,
            EnumAlgs::<Impl, IMPL_OFFSET>,
            GetAlgName::<Impl, IMPL_OFFSET>,
            SetReuseHardwareKeyIfUnableToGenNew::<Impl, IMPL_OFFSET>,
            ReuseHardwareKeyIfUnableToGenNew::<Impl, IMPL_OFFSET>,
            SetHashAlgID::<Impl, IMPL_OFFSET>,
            HashAlgID::<Impl, IMPL_OFFSET>,
            SetLimitExchangeKeyToEncipherment::<Impl, IMPL_OFFSET>,
            LimitExchangeKeyToEncipherment::<Impl, IMPL_OFFSET>,
            SetEnableSMIMECapabilities::<Impl, IMPL_OFFSET>,
            EnableSMIMECapabilities::<Impl, IMPL_OFFSET>,
            SetPrivateKeyArchiveCertificate::<Impl, IMPL_OFFSET>,
            PrivateKeyArchiveCertificate::<Impl, IMPL_OFFSET>,
            SetThumbPrint::<Impl, IMPL_OFFSET>,
            ThumbPrint::<Impl, IMPL_OFFSET>,
            binaryToString::<Impl, IMPL_OFFSET>,
            stringToBinary::<Impl, IMPL_OFFSET>,
            addExtensionToRequest::<Impl, IMPL_OFFSET>,
            addAttributeToRequest::<Impl, IMPL_OFFSET>,
            addNameValuePairToRequest::<Impl, IMPL_OFFSET>,
            resetExtensions::<Impl, IMPL_OFFSET>,
            resetAttributes::<Impl, IMPL_OFFSET>,
            createRequest::<Impl, IMPL_OFFSET>,
            createFileRequest::<Impl, IMPL_OFFSET>,
            acceptResponse::<Impl, IMPL_OFFSET>,
            acceptFileResponse::<Impl, IMPL_OFFSET>,
            getCertFromResponse::<Impl, IMPL_OFFSET>,
            getCertFromFileResponse::<Impl, IMPL_OFFSET>,
            createPFX::<Impl, IMPL_OFFSET>,
            createFilePFX::<Impl, IMPL_OFFSET>,
            setPendingRequestInfo::<Impl, IMPL_OFFSET>,
            enumPendingRequest::<Impl, IMPL_OFFSET>,
            removePendingRequest::<Impl, IMPL_OFFSET>,
            GetKeyLenEx::<Impl, IMPL_OFFSET>,
            InstallPKCS7Ex::<Impl, IMPL_OFFSET>,
            addCertTypeToRequestEx::<Impl, IMPL_OFFSET>,
            getProviderType::<Impl, IMPL_OFFSET>,
            SetSignerCertificate::<Impl, IMPL_OFFSET>,
            SetClientId::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            addBlobPropertyToCertificate::<Impl, IMPL_OFFSET>,
            resetBlobProperties::<Impl, IMPL_OFFSET>,
            SetIncludeSubjectKeyID::<Impl, IMPL_OFFSET>,
            IncludeSubjectKeyID::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICEnroll4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertAdminImpl: Sized + IDispatchImpl {
    fn IsValidCertificate();
    fn GetRevocationReason();
    fn RevokeCertificate();
    fn SetRequestAttributes();
    fn SetCertificateExtension();
    fn DenyRequest();
    fn ResubmitRequest();
    fn PublishCRL();
    fn GetCRL();
    fn ImportCertificate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertAdminVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertAdminImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertAdminVtbl {
        unsafe extern "system" fn IsValidCertificate<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRevocationReason<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preason: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RevokeCertificate<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, reason: i32, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRequestAttributes<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, strattributes: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCertificateExtension<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, strextensionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: CERT_PROPERTY_TYPE, flags: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DenyRequest<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResubmitRequest<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, pdisposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PublishCRL<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCRL<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32, pstrcrl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportCertificate<Impl: ICertAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: CERT_IMPORT_FLAGS, prequestid: *mut i32) -> ::windows::core::HRESULT {
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
            IsValidCertificate::<Impl, IMPL_OFFSET>,
            GetRevocationReason::<Impl, IMPL_OFFSET>,
            RevokeCertificate::<Impl, IMPL_OFFSET>,
            SetRequestAttributes::<Impl, IMPL_OFFSET>,
            SetCertificateExtension::<Impl, IMPL_OFFSET>,
            DenyRequest::<Impl, IMPL_OFFSET>,
            ResubmitRequest::<Impl, IMPL_OFFSET>,
            PublishCRL::<Impl, IMPL_OFFSET>,
            GetCRL::<Impl, IMPL_OFFSET>,
            ImportCertificate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertAdmin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertAdmin2Impl: Sized + ICertAdminImpl + IDispatchImpl {
    fn PublishCRLs();
    fn GetCAProperty();
    fn SetCAProperty();
    fn GetCAPropertyFlags();
    fn GetCAPropertyDisplayName();
    fn GetArchivedKey();
    fn GetConfigEntry();
    fn SetConfigEntry();
    fn ImportKey();
    fn GetMyRoles();
    fn DeleteRow();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertAdmin2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertAdmin2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertAdmin2Vtbl {
        unsafe extern "system" fn PublishCRLs<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, date: f64, crlflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCAProperty<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, propindex: i32, proptype: i32, flags: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCAProperty<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, propindex: i32, proptype: CERT_PROPERTY_TYPE, pvarpropertyvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCAPropertyFlags<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, ppropflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCAPropertyDisplayName<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, pstrdisplayname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetArchivedKey<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, flags: i32, pstrarchivedkey: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConfigEntry<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strnodepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strentryname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvarentry: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConfigEntry<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strnodepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strentryname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvarentry: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportKey<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, strcerthash: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: CERT_IMPORT_FLAGS, strkey: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMyRoles<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, proles: *mut CERTADMIN_GET_ROLES_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteRow<Impl: ICertAdmin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: CERT_DELETE_ROW_FLAGS, date: f64, table: CVRC_TABLE, rowid: i32, pcdeleted: *mut i32) -> ::windows::core::HRESULT {
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
            IsValidCertificate::<Impl, IMPL_OFFSET>,
            GetRevocationReason::<Impl, IMPL_OFFSET>,
            RevokeCertificate::<Impl, IMPL_OFFSET>,
            SetRequestAttributes::<Impl, IMPL_OFFSET>,
            SetCertificateExtension::<Impl, IMPL_OFFSET>,
            DenyRequest::<Impl, IMPL_OFFSET>,
            ResubmitRequest::<Impl, IMPL_OFFSET>,
            PublishCRL::<Impl, IMPL_OFFSET>,
            GetCRL::<Impl, IMPL_OFFSET>,
            ImportCertificate::<Impl, IMPL_OFFSET>,
            PublishCRLs::<Impl, IMPL_OFFSET>,
            GetCAProperty::<Impl, IMPL_OFFSET>,
            SetCAProperty::<Impl, IMPL_OFFSET>,
            GetCAPropertyFlags::<Impl, IMPL_OFFSET>,
            GetCAPropertyDisplayName::<Impl, IMPL_OFFSET>,
            GetArchivedKey::<Impl, IMPL_OFFSET>,
            GetConfigEntry::<Impl, IMPL_OFFSET>,
            SetConfigEntry::<Impl, IMPL_OFFSET>,
            ImportKey::<Impl, IMPL_OFFSET>,
            GetMyRoles::<Impl, IMPL_OFFSET>,
            DeleteRow::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertAdmin2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertConfigImpl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
    fn GetField();
    fn GetConfig();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertConfigVtbl {
        unsafe extern "system" fn Reset<Impl: ICertConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: ICertConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetField<Impl: ICertConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strfieldname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConfig<Impl: ICertConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Next::<Impl, IMPL_OFFSET>, GetField::<Impl, IMPL_OFFSET>, GetConfig::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertConfig2Impl: Sized + ICertConfigImpl + IDispatchImpl {
    fn SetSharedFolder();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertConfig2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertConfig2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertConfig2Vtbl {
        unsafe extern "system" fn SetSharedFolder<Impl: ICertConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsharedfolder: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Next::<Impl, IMPL_OFFSET>, GetField::<Impl, IMPL_OFFSET>, GetConfig::<Impl, IMPL_OFFSET>, SetSharedFolder::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertConfig2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeAltNameImpl: Sized + IDispatchImpl {
    fn Decode();
    fn GetNameCount();
    fn GetNameChoice();
    fn GetName();
    fn Reset();
    fn SetNameEntry();
    fn Encode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeAltNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeAltNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeAltNameVtbl {
        unsafe extern "system" fn Decode<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNameCount<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNameChoice<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nameindex: i32, pnamechoice: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nameindex: i32, pstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namecount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNameEntry<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nameindex: i32, namechoice: CERT_ALT_NAME, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Encode<Impl: ICertEncodeAltNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Decode::<Impl, IMPL_OFFSET>,
            GetNameCount::<Impl, IMPL_OFFSET>,
            GetNameChoice::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            SetNameEntry::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeAltName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeAltName2Impl: Sized + ICertEncodeAltNameImpl + IDispatchImpl {
    fn DecodeBlob();
    fn EncodeBlob();
    fn GetNameBlob();
    fn SetNameEntryBlob();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeAltName2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeAltName2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeAltName2Vtbl {
        unsafe extern "system" fn DecodeBlob<Impl: ICertEncodeAltName2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncodeBlob<Impl: ICertEncodeAltName2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNameBlob<Impl: ICertEncodeAltName2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nameindex: i32, encoding: EncodingType, pstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNameEntryBlob<Impl: ICertEncodeAltName2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nameindex: i32, namechoice: i32, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
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
            Decode::<Impl, IMPL_OFFSET>,
            GetNameCount::<Impl, IMPL_OFFSET>,
            GetNameChoice::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            SetNameEntry::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            DecodeBlob::<Impl, IMPL_OFFSET>,
            EncodeBlob::<Impl, IMPL_OFFSET>,
            GetNameBlob::<Impl, IMPL_OFFSET>,
            SetNameEntryBlob::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeAltName2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeBitStringImpl: Sized + IDispatchImpl {
    fn Decode();
    fn GetBitCount();
    fn GetBitString();
    fn Encode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeBitStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeBitStringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeBitStringVtbl {
        unsafe extern "system" fn Decode<Impl: ICertEncodeBitStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBitCount<Impl: ICertEncodeBitStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBitString<Impl: ICertEncodeBitStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrbitstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Encode<Impl: ICertEncodeBitStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitcount: i32, strbitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Decode::<Impl, IMPL_OFFSET>, GetBitCount::<Impl, IMPL_OFFSET>, GetBitString::<Impl, IMPL_OFFSET>, Encode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeBitString as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeBitString2Impl: Sized + ICertEncodeBitStringImpl + IDispatchImpl {
    fn DecodeBlob();
    fn EncodeBlob();
    fn GetBitStringBlob();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeBitString2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeBitString2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeBitString2Vtbl {
        unsafe extern "system" fn DecodeBlob<Impl: ICertEncodeBitString2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncodeBlob<Impl: ICertEncodeBitString2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitcount: i32, strbitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encodingin: EncodingType, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBitStringBlob<Impl: ICertEncodeBitString2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrbitstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Decode::<Impl, IMPL_OFFSET>,
            GetBitCount::<Impl, IMPL_OFFSET>,
            GetBitString::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            DecodeBlob::<Impl, IMPL_OFFSET>,
            EncodeBlob::<Impl, IMPL_OFFSET>,
            GetBitStringBlob::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeBitString2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeCRLDistInfoImpl: Sized + IDispatchImpl {
    fn Decode();
    fn GetDistPointCount();
    fn GetNameCount();
    fn GetNameChoice();
    fn GetName();
    fn Reset();
    fn SetNameCount();
    fn SetNameEntry();
    fn Encode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeCRLDistInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeCRLDistInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeCRLDistInfoVtbl {
        unsafe extern "system" fn Decode<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDistPointCount<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdistpointcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNameCount<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distpointindex: i32, pnamecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNameChoice<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distpointindex: i32, nameindex: i32, pnamechoice: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distpointindex: i32, nameindex: i32, pstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distpointcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNameCount<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distpointindex: i32, namecount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNameEntry<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distpointindex: i32, nameindex: i32, namechoice: CERT_ALT_NAME, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Encode<Impl: ICertEncodeCRLDistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Decode::<Impl, IMPL_OFFSET>,
            GetDistPointCount::<Impl, IMPL_OFFSET>,
            GetNameCount::<Impl, IMPL_OFFSET>,
            GetNameChoice::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            SetNameCount::<Impl, IMPL_OFFSET>,
            SetNameEntry::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeCRLDistInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeCRLDistInfo2Impl: Sized + ICertEncodeCRLDistInfoImpl + IDispatchImpl {
    fn DecodeBlob();
    fn EncodeBlob();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeCRLDistInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeCRLDistInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeCRLDistInfo2Vtbl {
        unsafe extern "system" fn DecodeBlob<Impl: ICertEncodeCRLDistInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncodeBlob<Impl: ICertEncodeCRLDistInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Decode::<Impl, IMPL_OFFSET>,
            GetDistPointCount::<Impl, IMPL_OFFSET>,
            GetNameCount::<Impl, IMPL_OFFSET>,
            GetNameChoice::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            SetNameCount::<Impl, IMPL_OFFSET>,
            SetNameEntry::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            DecodeBlob::<Impl, IMPL_OFFSET>,
            EncodeBlob::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeCRLDistInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeDateArrayImpl: Sized + IDispatchImpl {
    fn Decode();
    fn GetCount();
    fn GetValue();
    fn Reset();
    fn SetValue();
    fn Encode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeDateArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeDateArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeDateArrayVtbl {
        unsafe extern "system" fn Decode<Impl: ICertEncodeDateArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: ICertEncodeDateArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: ICertEncodeDateArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ICertEncodeDateArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ICertEncodeDateArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Encode<Impl: ICertEncodeDateArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Decode::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetValue::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>, Encode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeDateArray as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeDateArray2Impl: Sized + ICertEncodeDateArrayImpl + IDispatchImpl {
    fn DecodeBlob();
    fn EncodeBlob();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeDateArray2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeDateArray2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeDateArray2Vtbl {
        unsafe extern "system" fn DecodeBlob<Impl: ICertEncodeDateArray2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncodeBlob<Impl: ICertEncodeDateArray2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Decode::<Impl, IMPL_OFFSET>,
            GetCount::<Impl, IMPL_OFFSET>,
            GetValue::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            DecodeBlob::<Impl, IMPL_OFFSET>,
            EncodeBlob::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeDateArray2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeLongArrayImpl: Sized + IDispatchImpl {
    fn Decode();
    fn GetCount();
    fn GetValue();
    fn Reset();
    fn SetValue();
    fn Encode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeLongArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeLongArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeLongArrayVtbl {
        unsafe extern "system" fn Decode<Impl: ICertEncodeLongArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: ICertEncodeLongArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: ICertEncodeLongArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ICertEncodeLongArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ICertEncodeLongArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Encode<Impl: ICertEncodeLongArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Decode::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetValue::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>, Encode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeLongArray as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeLongArray2Impl: Sized + ICertEncodeLongArrayImpl + IDispatchImpl {
    fn DecodeBlob();
    fn EncodeBlob();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeLongArray2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeLongArray2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeLongArray2Vtbl {
        unsafe extern "system" fn DecodeBlob<Impl: ICertEncodeLongArray2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncodeBlob<Impl: ICertEncodeLongArray2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Decode::<Impl, IMPL_OFFSET>,
            GetCount::<Impl, IMPL_OFFSET>,
            GetValue::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            DecodeBlob::<Impl, IMPL_OFFSET>,
            EncodeBlob::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeLongArray2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeStringArrayImpl: Sized + IDispatchImpl {
    fn Decode();
    fn GetStringType();
    fn GetCount();
    fn GetValue();
    fn Reset();
    fn SetValue();
    fn Encode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeStringArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeStringArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeStringArrayVtbl {
        unsafe extern "system" fn Decode<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStringType<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstringtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, stringtype: super::CERT_RDN_ATTR_VALUE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, str: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Encode<Impl: ICertEncodeStringArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Decode::<Impl, IMPL_OFFSET>,
            GetStringType::<Impl, IMPL_OFFSET>,
            GetCount::<Impl, IMPL_OFFSET>,
            GetValue::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeStringArray as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertEncodeStringArray2Impl: Sized + ICertEncodeStringArrayImpl + IDispatchImpl {
    fn DecodeBlob();
    fn EncodeBlob();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertEncodeStringArray2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertEncodeStringArray2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertEncodeStringArray2Vtbl {
        unsafe extern "system" fn DecodeBlob<Impl: ICertEncodeStringArray2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncodeBlob<Impl: ICertEncodeStringArray2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Decode::<Impl, IMPL_OFFSET>,
            GetStringType::<Impl, IMPL_OFFSET>,
            GetCount::<Impl, IMPL_OFFSET>,
            GetValue::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            DecodeBlob::<Impl, IMPL_OFFSET>,
            EncodeBlob::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertEncodeStringArray2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertExitImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Notify();
    fn GetDescription();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertExitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertExitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertExitVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, peventmask: *mut CERT_EXIT_EVENT_MASK) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notify<Impl: ICertExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, exitevent: i32, context: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: ICertExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Notify::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertExit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertExit2Impl: Sized + ICertExitImpl + IDispatchImpl {
    fn GetManageModule();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertExit2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertExit2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertExit2Vtbl {
        unsafe extern "system" fn GetManageModule<Impl: ICertExit2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmanagemodule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Notify::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>, GetManageModule::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertExit2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertGetConfigImpl: Sized + IDispatchImpl {
    fn GetConfig();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertGetConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertGetConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertGetConfigVtbl {
        unsafe extern "system" fn GetConfig<Impl: ICertGetConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: CERT_GET_CONFIG_FLAGS, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, GetConfig::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertGetConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertManageModuleImpl: Sized + IDispatchImpl {
    fn GetProperty();
    fn SetProperty();
    fn Configure();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertManageModuleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertManageModuleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertManageModuleVtbl {
        unsafe extern "system" fn GetProperty<Impl: ICertManageModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strstoragelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32, pvarproperty: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: ICertManageModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strstoragelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32, pvarproperty: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Configure<Impl: ICertManageModuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strstoragelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, GetProperty::<Impl, IMPL_OFFSET>, SetProperty::<Impl, IMPL_OFFSET>, Configure::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertManageModule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPolicyImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn VerifyRequest();
    fn GetDescription();
    fn ShutDown();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPolicyVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VerifyRequest<Impl: ICertPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, context: i32, bnewrequest: i32, flags: i32, pdisposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: ICertPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShutDown<Impl: ICertPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, VerifyRequest::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>, ShutDown::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPolicy2Impl: Sized + ICertPolicyImpl + IDispatchImpl {
    fn GetManageModule();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPolicy2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPolicy2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPolicy2Vtbl {
        unsafe extern "system" fn GetManageModule<Impl: ICertPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmanagemodule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, VerifyRequest::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>, ShutDown::<Impl, IMPL_OFFSET>, GetManageModule::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPolicy2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertiesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn InitializeFromCertificate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertiesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromCertificate<Impl: ICertPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            ItemByIndex::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyImpl: Sized + IDispatchImpl {
    fn InitializeFromCertificate();
    fn InitializeDecode();
    fn PropertyId();
    fn SetPropertyId();
    fn RawData();
    fn RemoveFromCertificate();
    fn SetValueOnCertificate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyVtbl {
        unsafe extern "system" fn InitializeFromCertificate<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropertyId<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut CERTENROLL_PROPERTYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPropertyId<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CERTENROLL_PROPERTYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RawData<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveFromCertificate<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValueOnCertificate<Impl: ICertPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyArchivedImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn Archived();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyArchivedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyArchivedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyArchivedVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyArchivedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, archivedvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Archived<Impl: ICertPropertyArchivedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
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
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            Archived::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyArchived as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyArchivedKeyHashImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn ArchivedKeyHash();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyArchivedKeyHashVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyArchivedKeyHashImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyArchivedKeyHashVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyArchivedKeyHashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strarchivedkeyhashvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ArchivedKeyHash<Impl: ICertPropertyArchivedKeyHashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            ArchivedKeyHash::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyArchivedKeyHash as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyAutoEnrollImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn TemplateName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyAutoEnrollVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyAutoEnrollImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyAutoEnrollVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyAutoEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TemplateName<Impl: ICertPropertyAutoEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            TemplateName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyAutoEnroll as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyBackedUpImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn InitializeFromCurrentTime();
    fn Initialize();
    fn BackedUpValue();
    fn BackedUpTime();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyBackedUpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyBackedUpImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyBackedUpVtbl {
        unsafe extern "system" fn InitializeFromCurrentTime<Impl: ICertPropertyBackedUpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backedupvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: ICertPropertyBackedUpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backedupvalue: i16, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackedUpValue<Impl: ICertPropertyBackedUpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackedUpTime<Impl: ICertPropertyBackedUpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
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
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate::<Impl, IMPL_OFFSET>,
            InitializeFromCurrentTime::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            BackedUpValue::<Impl, IMPL_OFFSET>,
            BackedUpTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyBackedUp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyDescriptionImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn Description();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyDescriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyDescriptionVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: ICertPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyDescription as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyEnrollmentImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn RequestId();
    fn CADnsName();
    fn CAName();
    fn FriendlyName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyEnrollmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyEnrollmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyEnrollmentVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: i32, strcadnsname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strcaname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strfriendlyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestId<Impl: ICertPropertyEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CADnsName<Impl: ICertPropertyEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CAName<Impl: ICertPropertyEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FriendlyName<Impl: ICertPropertyEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            RequestId::<Impl, IMPL_OFFSET>,
            CADnsName::<Impl, IMPL_OFFSET>,
            CAName::<Impl, IMPL_OFFSET>,
            FriendlyName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyEnrollment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyEnrollmentPolicyServerImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn GetPolicyServerUrl();
    fn GetPolicyServerId();
    fn GetEnrollmentServerUrl();
    fn GetRequestIdString();
    fn GetPropertyFlags();
    fn GetUrlFlags();
    fn GetAuthentication();
    fn GetEnrollmentServerAuthentication();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyEnrollmentPolicyServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyEnrollmentPolicyServerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyEnrollmentPolicyServerVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyflags: EnrollmentPolicyServerPropertyFlags, authflags: X509EnrollmentAuthFlags, enrollmentserverauthflags: X509EnrollmentAuthFlags, urlflags: PolicyServerUrlFlags, strrequestid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strenrollmentserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPolicyServerUrl<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPolicyServerId<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnrollmentServerUrl<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRequestIdString<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyFlags<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut EnrollmentPolicyServerPropertyFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUrlFlags<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut PolicyServerUrlFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAuthentication<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509EnrollmentAuthFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnrollmentServerAuthentication<Impl: ICertPropertyEnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509EnrollmentAuthFlags) -> ::windows::core::HRESULT {
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
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            GetPolicyServerUrl::<Impl, IMPL_OFFSET>,
            GetPolicyServerId::<Impl, IMPL_OFFSET>,
            GetEnrollmentServerUrl::<Impl, IMPL_OFFSET>,
            GetRequestIdString::<Impl, IMPL_OFFSET>,
            GetPropertyFlags::<Impl, IMPL_OFFSET>,
            GetUrlFlags::<Impl, IMPL_OFFSET>,
            GetAuthentication::<Impl, IMPL_OFFSET>,
            GetEnrollmentServerAuthentication::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyEnrollmentPolicyServer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyFriendlyNameImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn FriendlyName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyFriendlyNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyFriendlyNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyFriendlyNameVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyFriendlyNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strfriendlyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FriendlyName<Impl: ICertPropertyFriendlyNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            FriendlyName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyFriendlyName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyKeyProvInfoImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn PrivateKey();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyKeyProvInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyKeyProvInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyKeyProvInfoVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyKeyProvInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivateKey<Impl: ICertPropertyKeyProvInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            PrivateKey::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyKeyProvInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyRenewalImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn InitializeFromCertificateHash();
    fn Renewal();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyRenewalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyRenewalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyRenewalVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyRenewalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strrenewalvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromCertificateHash<Impl: ICertPropertyRenewalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Renewal<Impl: ICertPropertyRenewalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            InitializeFromCertificateHash::<Impl, IMPL_OFFSET>,
            Renewal::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyRenewal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertyRequestOriginatorImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn InitializeFromLocalRequestOriginator();
    fn RequestOriginator();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertyRequestOriginatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertyRequestOriginatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertyRequestOriginatorVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertyRequestOriginatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strrequestoriginator: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromLocalRequestOriginator<Impl: ICertPropertyRequestOriginatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestOriginator<Impl: ICertPropertyRequestOriginatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            InitializeFromLocalRequestOriginator::<Impl, IMPL_OFFSET>,
            RequestOriginator::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertyRequestOriginator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertPropertySHA1HashImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn SHA1Hash();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertPropertySHA1HashVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertPropertySHA1HashImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertPropertySHA1HashVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertPropertySHA1HashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strrenewalvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SHA1Hash<Impl: ICertPropertySHA1HashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RemoveFromCertificate::<Impl, IMPL_OFFSET>,
            SetValueOnCertificate::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            SHA1Hash::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertPropertySHA1Hash as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertRequestImpl: Sized + IDispatchImpl {
    fn Submit();
    fn RetrievePending();
    fn GetLastStatus();
    fn GetRequestId();
    fn GetDispositionMessage();
    fn GetCACertificate();
    fn GetCertificate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertRequestVtbl {
        unsafe extern "system" fn Submit<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, strrequest: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strattributes: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RetrievePending<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: i32, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastStatus<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRequestId<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequestid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDispositionMessage<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdispositionmessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCACertificate<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fexchangecertificate: i32, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32, pstrcertificate: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificate<Impl: ICertRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pstrcertificate: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Submit::<Impl, IMPL_OFFSET>,
            RetrievePending::<Impl, IMPL_OFFSET>,
            GetLastStatus::<Impl, IMPL_OFFSET>,
            GetRequestId::<Impl, IMPL_OFFSET>,
            GetDispositionMessage::<Impl, IMPL_OFFSET>,
            GetCACertificate::<Impl, IMPL_OFFSET>,
            GetCertificate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertRequest2Impl: Sized + ICertRequestImpl + IDispatchImpl {
    fn GetIssuedCertificate();
    fn GetErrorMessageText();
    fn GetCAProperty();
    fn GetCAPropertyFlags();
    fn GetCAPropertyDisplayName();
    fn GetFullResponseProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertRequest2Vtbl {
        unsafe extern "system" fn GetIssuedCertificate<Impl: ICertRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut CR_DISP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorMessageText<Impl: ICertRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrmessage: i32, flags: i32, pstrerrormessagetext: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCAProperty<Impl: ICertRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, propindex: i32, proptype: i32, flags: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCAPropertyFlags<Impl: ICertRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, ppropflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCAPropertyDisplayName<Impl: ICertRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, pstrdisplayname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFullResponseProperty<Impl: ICertRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propid: FULL_RESPONSE_PROPERTY_ID, propindex: i32, proptype: CERT_PROPERTY_TYPE, flags: CERT_REQUEST_OUT_TYPE, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            Submit::<Impl, IMPL_OFFSET>,
            RetrievePending::<Impl, IMPL_OFFSET>,
            GetLastStatus::<Impl, IMPL_OFFSET>,
            GetRequestId::<Impl, IMPL_OFFSET>,
            GetDispositionMessage::<Impl, IMPL_OFFSET>,
            GetCACertificate::<Impl, IMPL_OFFSET>,
            GetCertificate::<Impl, IMPL_OFFSET>,
            GetIssuedCertificate::<Impl, IMPL_OFFSET>,
            GetErrorMessageText::<Impl, IMPL_OFFSET>,
            GetCAProperty::<Impl, IMPL_OFFSET>,
            GetCAPropertyFlags::<Impl, IMPL_OFFSET>,
            GetCAPropertyDisplayName::<Impl, IMPL_OFFSET>,
            GetFullResponseProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertRequest3Impl: Sized + ICertRequest2Impl + ICertRequestImpl + IDispatchImpl {
    fn SetCredential();
    fn GetRequestIdString();
    fn GetIssuedCertificate2();
    fn GetRefreshPolicy();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertRequest3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertRequest3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertRequest3Vtbl {
        unsafe extern "system" fn SetCredential<Impl: ICertRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: i32, authtype: X509EnrollmentAuthFlags, strcredential: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRequestIdString<Impl: ICertRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrrequestid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIssuedCertificate2<Impl: ICertRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strrequestid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut CR_DISP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRefreshPolicy<Impl: ICertRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
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
            Submit::<Impl, IMPL_OFFSET>,
            RetrievePending::<Impl, IMPL_OFFSET>,
            GetLastStatus::<Impl, IMPL_OFFSET>,
            GetRequestId::<Impl, IMPL_OFFSET>,
            GetDispositionMessage::<Impl, IMPL_OFFSET>,
            GetCACertificate::<Impl, IMPL_OFFSET>,
            GetCertificate::<Impl, IMPL_OFFSET>,
            GetIssuedCertificate::<Impl, IMPL_OFFSET>,
            GetErrorMessageText::<Impl, IMPL_OFFSET>,
            GetCAProperty::<Impl, IMPL_OFFSET>,
            GetCAPropertyFlags::<Impl, IMPL_OFFSET>,
            GetCAPropertyDisplayName::<Impl, IMPL_OFFSET>,
            GetFullResponseProperty::<Impl, IMPL_OFFSET>,
            SetCredential::<Impl, IMPL_OFFSET>,
            GetRequestIdString::<Impl, IMPL_OFFSET>,
            GetIssuedCertificate2::<Impl, IMPL_OFFSET>,
            GetRefreshPolicy::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertRequest3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICertRequestDImpl: Sized {
    fn Request();
    fn GetCACert();
    fn Ping();
}
#[cfg(feature = "Win32_Foundation")]
impl ICertRequestDVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertRequestDImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertRequestDVtbl {
        unsafe extern "system" fn Request<Impl: ICertRequestDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pwszauthority: super::super::super::Foundation::PWSTR, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: super::super::super::Foundation::PWSTR, pctbrequest: *const CERTTRANSBLOB, pctbcertchain: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCACert<Impl: ICertRequestDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fchain: u32, pwszauthority: super::super::super::Foundation::PWSTR, pctbout: *mut CERTTRANSBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ping<Impl: ICertRequestDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszauthority: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Request::<Impl, IMPL_OFFSET>, GetCACert::<Impl, IMPL_OFFSET>, Ping::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertRequestD as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICertRequestD2Impl: Sized + ICertRequestDImpl {
    fn Request2();
    fn GetCAProperty();
    fn GetCAPropertyInfo();
    fn Ping2();
}
#[cfg(feature = "Win32_Foundation")]
impl ICertRequestD2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertRequestD2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertRequestD2Vtbl {
        unsafe extern "system" fn Request2<Impl: ICertRequestD2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszauthority: super::super::super::Foundation::PWSTR, dwflags: u32, pwszserialnumber: super::super::super::Foundation::PWSTR, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: super::super::super::Foundation::PWSTR, pctbrequest: *const CERTTRANSBLOB, pctbfullresponse: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCAProperty<Impl: ICertRequestD2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszauthority: super::super::super::Foundation::PWSTR, propid: i32, propindex: i32, proptype: i32, pctbpropertyvalue: *mut CERTTRANSBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCAPropertyInfo<Impl: ICertRequestD2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszauthority: super::super::super::Foundation::PWSTR, pcproperty: *mut i32, pctbpropinfo: *mut CERTTRANSBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ping2<Impl: ICertRequestD2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszauthority: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Request::<Impl, IMPL_OFFSET>, GetCACert::<Impl, IMPL_OFFSET>, Ping::<Impl, IMPL_OFFSET>, Request2::<Impl, IMPL_OFFSET>, GetCAProperty::<Impl, IMPL_OFFSET>, GetCAPropertyInfo::<Impl, IMPL_OFFSET>, Ping2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertRequestD2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertServerExitImpl: Sized + IDispatchImpl {
    fn SetContext();
    fn GetRequestProperty();
    fn GetRequestAttribute();
    fn GetCertificateProperty();
    fn GetCertificateExtension();
    fn GetCertificateExtensionFlags();
    fn EnumerateExtensionsSetup();
    fn EnumerateExtensions();
    fn EnumerateExtensionsClose();
    fn EnumerateAttributesSetup();
    fn EnumerateAttributes();
    fn EnumerateAttributesClose();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertServerExitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertServerExitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertServerExitVtbl {
        unsafe extern "system" fn SetContext<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRequestProperty<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRequestAttribute<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strattributename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrattributevalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateProperty<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateExtension<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strextensionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: i32, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateExtensionFlags<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateExtensionsSetup<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateExtensions<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrextensionname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateExtensionsClose<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateAttributesSetup<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateAttributes<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrattributename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateAttributesClose<Impl: ICertServerExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            SetContext::<Impl, IMPL_OFFSET>,
            GetRequestProperty::<Impl, IMPL_OFFSET>,
            GetRequestAttribute::<Impl, IMPL_OFFSET>,
            GetCertificateProperty::<Impl, IMPL_OFFSET>,
            GetCertificateExtension::<Impl, IMPL_OFFSET>,
            GetCertificateExtensionFlags::<Impl, IMPL_OFFSET>,
            EnumerateExtensionsSetup::<Impl, IMPL_OFFSET>,
            EnumerateExtensions::<Impl, IMPL_OFFSET>,
            EnumerateExtensionsClose::<Impl, IMPL_OFFSET>,
            EnumerateAttributesSetup::<Impl, IMPL_OFFSET>,
            EnumerateAttributes::<Impl, IMPL_OFFSET>,
            EnumerateAttributesClose::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertServerExit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertServerPolicyImpl: Sized + IDispatchImpl {
    fn SetContext();
    fn GetRequestProperty();
    fn GetRequestAttribute();
    fn GetCertificateProperty();
    fn SetCertificateProperty();
    fn GetCertificateExtension();
    fn GetCertificateExtensionFlags();
    fn SetCertificateExtension();
    fn EnumerateExtensionsSetup();
    fn EnumerateExtensions();
    fn EnumerateExtensionsClose();
    fn EnumerateAttributesSetup();
    fn EnumerateAttributes();
    fn EnumerateAttributesClose();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertServerPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertServerPolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertServerPolicyVtbl {
        unsafe extern "system" fn SetContext<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRequestProperty<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRequestAttribute<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strattributename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrattributevalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateProperty<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: CERT_PROPERTY_TYPE, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCertificateProperty<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: i32, pvarpropertyvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateExtension<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strextensionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: CERT_PROPERTY_TYPE, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateExtensionFlags<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCertificateExtension<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strextensionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: i32, extflags: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateExtensionsSetup<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateExtensions<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrextensionname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateExtensionsClose<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateAttributesSetup<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateAttributes<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrattributename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateAttributesClose<Impl: ICertServerPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            SetContext::<Impl, IMPL_OFFSET>,
            GetRequestProperty::<Impl, IMPL_OFFSET>,
            GetRequestAttribute::<Impl, IMPL_OFFSET>,
            GetCertificateProperty::<Impl, IMPL_OFFSET>,
            SetCertificateProperty::<Impl, IMPL_OFFSET>,
            GetCertificateExtension::<Impl, IMPL_OFFSET>,
            GetCertificateExtensionFlags::<Impl, IMPL_OFFSET>,
            SetCertificateExtension::<Impl, IMPL_OFFSET>,
            EnumerateExtensionsSetup::<Impl, IMPL_OFFSET>,
            EnumerateExtensions::<Impl, IMPL_OFFSET>,
            EnumerateExtensionsClose::<Impl, IMPL_OFFSET>,
            EnumerateAttributesSetup::<Impl, IMPL_OFFSET>,
            EnumerateAttributes::<Impl, IMPL_OFFSET>,
            EnumerateAttributesClose::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertServerPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertViewImpl: Sized + IDispatchImpl {
    fn OpenConnection();
    fn EnumCertViewColumn();
    fn GetColumnCount();
    fn GetColumnIndex();
    fn SetResultColumnCount();
    fn SetResultColumn();
    fn SetRestriction();
    fn OpenView();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertViewVtbl {
        unsafe extern "system" fn OpenConnection<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumCertViewColumn<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fresultcolumn: CVRC_COLUMN, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnCount<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fresultcolumn: CVRC_COLUMN, pccolumn: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnIndex<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fresultcolumn: CVRC_COLUMN, strcolumnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pcolumnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetResultColumnCount<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cresultcolumn: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetResultColumn<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, columnindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRestriction<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, columnindex: CERT_VIEW_COLUMN_INDEX, seekoperator: CERT_VIEW_SEEK_OPERATOR_FLAGS, sortorder: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenView<Impl: ICertViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            OpenConnection::<Impl, IMPL_OFFSET>,
            EnumCertViewColumn::<Impl, IMPL_OFFSET>,
            GetColumnCount::<Impl, IMPL_OFFSET>,
            GetColumnIndex::<Impl, IMPL_OFFSET>,
            SetResultColumnCount::<Impl, IMPL_OFFSET>,
            SetResultColumn::<Impl, IMPL_OFFSET>,
            SetRestriction::<Impl, IMPL_OFFSET>,
            OpenView::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertView2Impl: Sized + ICertViewImpl + IDispatchImpl {
    fn SetTable();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertView2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertView2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertView2Vtbl {
        unsafe extern "system" fn SetTable<Impl: ICertView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, table: CVRC_TABLE) -> ::windows::core::HRESULT {
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
            OpenConnection::<Impl, IMPL_OFFSET>,
            EnumCertViewColumn::<Impl, IMPL_OFFSET>,
            GetColumnCount::<Impl, IMPL_OFFSET>,
            GetColumnIndex::<Impl, IMPL_OFFSET>,
            SetResultColumnCount::<Impl, IMPL_OFFSET>,
            SetResultColumn::<Impl, IMPL_OFFSET>,
            SetRestriction::<Impl, IMPL_OFFSET>,
            OpenView::<Impl, IMPL_OFFSET>,
            SetTable::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertView2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificateAttestationChallengeImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn DecryptChallenge();
    fn RequestID();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificateAttestationChallengeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateAttestationChallengeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateAttestationChallengeVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertificateAttestationChallengeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strpendingfullcmcresponsewithchallenge: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DecryptChallenge<Impl: ICertificateAttestationChallengeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pstrenvelopedpkcs7reencryptedtoca: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestID<Impl: ICertificateAttestationChallengeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrrequestid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, DecryptChallenge::<Impl, IMPL_OFFSET>, RequestID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateAttestationChallenge as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificateAttestationChallenge2Impl: Sized + ICertificateAttestationChallengeImpl + IDispatchImpl {
    fn SetKeyContainerName();
    fn SetKeyBlob();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificateAttestationChallenge2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateAttestationChallenge2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateAttestationChallenge2Vtbl {
        unsafe extern "system" fn SetKeyContainerName<Impl: ICertificateAttestationChallenge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeyBlob<Impl: ICertificateAttestationChallenge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, DecryptChallenge::<Impl, IMPL_OFFSET>, RequestID::<Impl, IMPL_OFFSET>, SetKeyContainerName::<Impl, IMPL_OFFSET>, SetKeyBlob::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateAttestationChallenge2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificatePoliciesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificatePoliciesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificatePoliciesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificatePoliciesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ICertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ICertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ICertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ICertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ICertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, ItemByIndex::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificatePolicies as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificatePolicyImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn ObjectId();
    fn PolicyQualifiers();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificatePolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificatePolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificatePolicyVtbl {
        unsafe extern "system" fn Initialize<Impl: ICertificatePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ObjectId<Impl: ICertificatePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PolicyQualifiers<Impl: ICertificatePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, ObjectId::<Impl, IMPL_OFFSET>, PolicyQualifiers::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificatePolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificationAuthoritiesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn ComputeSiteCosts();
    fn ItemByName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificationAuthoritiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificationAuthoritiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificationAuthoritiesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputeSiteCosts<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemByName<Impl: ICertificationAuthoritiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            ItemByIndex::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            ComputeSiteCosts::<Impl, IMPL_OFFSET>,
            ItemByName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificationAuthorities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICertificationAuthorityImpl: Sized + IDispatchImpl {
    fn Property();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICertificationAuthorityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificationAuthorityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificationAuthorityVtbl {
        unsafe extern "system" fn Property<Impl: ICertificationAuthorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: EnrollmentCAProperty, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Property::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificationAuthority as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICryptAttributeImpl: Sized + IDispatchImpl {
    fn InitializeFromObjectId();
    fn InitializeFromValues();
    fn ObjectId();
    fn Values();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICryptAttributeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICryptAttributeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICryptAttributeVtbl {
        unsafe extern "system" fn InitializeFromObjectId<Impl: ICryptAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromValues<Impl: ICryptAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ObjectId<Impl: ICryptAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Values<Impl: ICryptAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, InitializeFromObjectId::<Impl, IMPL_OFFSET>, InitializeFromValues::<Impl, IMPL_OFFSET>, ObjectId::<Impl, IMPL_OFFSET>, Values::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICryptAttribute as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICryptAttributesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn IndexByObjectId();
    fn AddRange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICryptAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICryptAttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICryptAttributesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IndexByObjectId<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRange<Impl: ICryptAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            ItemByIndex::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            IndexByObjectId::<Impl, IMPL_OFFSET>,
            AddRange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICryptAttributes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICspAlgorithmImpl: Sized + IDispatchImpl {
    fn GetAlgorithmOid();
    fn DefaultLength();
    fn IncrementLength();
    fn LongName();
    fn Valid();
    fn MaxLength();
    fn MinLength();
    fn Name();
    fn Type();
    fn Operations();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICspAlgorithmVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICspAlgorithmImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICspAlgorithmVtbl {
        unsafe extern "system" fn GetAlgorithmOid<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: i32, algflags: AlgorithmFlags, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultLength<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IncrementLength<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LongName<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Valid<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxLength<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MinLength<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut AlgorithmType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Operations<Impl: ICspAlgorithmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut AlgorithmOperationFlags) -> ::windows::core::HRESULT {
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
            GetAlgorithmOid::<Impl, IMPL_OFFSET>,
            DefaultLength::<Impl, IMPL_OFFSET>,
            IncrementLength::<Impl, IMPL_OFFSET>,
            LongName::<Impl, IMPL_OFFSET>,
            Valid::<Impl, IMPL_OFFSET>,
            MaxLength::<Impl, IMPL_OFFSET>,
            MinLength::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            Operations::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICspAlgorithm as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICspAlgorithmsImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn ItemByName();
    fn IndexByObjectId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICspAlgorithmsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICspAlgorithmsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICspAlgorithmsVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemByName<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IndexByObjectId<Impl: ICspAlgorithmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, pindex: *mut i32) -> ::windows::core::HRESULT {
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
            ItemByIndex::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            ItemByName::<Impl, IMPL_OFFSET>,
            IndexByObjectId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICspAlgorithms as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICspInformationImpl: Sized + IDispatchImpl {
    fn InitializeFromName();
    fn InitializeFromType();
    fn CspAlgorithms();
    fn HasHardwareRandomNumberGenerator();
    fn IsHardwareDevice();
    fn IsRemovable();
    fn IsSoftwareDevice();
    fn Valid();
    fn MaxKeyContainerNameLength();
    fn Name();
    fn Type();
    fn Version();
    fn KeySpec();
    fn IsSmartCard();
    fn GetDefaultSecurityDescriptor();
    fn LegacyCsp();
    fn GetCspStatusFromOperations();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICspInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICspInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICspInformationVtbl {
        unsafe extern "system" fn InitializeFromName<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromType<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: X509ProviderType, palgorithm: ::windows::core::RawPtr, machinecontext: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CspAlgorithms<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasHardwareRandomNumberGenerator<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsHardwareDevice<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRemovable<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSoftwareDevice<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Valid<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxKeyContainerNameLength<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509ProviderType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Version<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeySpec<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509KeySpec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSmartCard<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultSecurityDescriptor<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LegacyCsp<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCspStatusFromOperations<Impl: ICspInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palgorithm: ::windows::core::RawPtr, operations: AlgorithmOperationFlags, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            InitializeFromName::<Impl, IMPL_OFFSET>,
            InitializeFromType::<Impl, IMPL_OFFSET>,
            CspAlgorithms::<Impl, IMPL_OFFSET>,
            HasHardwareRandomNumberGenerator::<Impl, IMPL_OFFSET>,
            IsHardwareDevice::<Impl, IMPL_OFFSET>,
            IsRemovable::<Impl, IMPL_OFFSET>,
            IsSoftwareDevice::<Impl, IMPL_OFFSET>,
            Valid::<Impl, IMPL_OFFSET>,
            MaxKeyContainerNameLength::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            Version::<Impl, IMPL_OFFSET>,
            KeySpec::<Impl, IMPL_OFFSET>,
            IsSmartCard::<Impl, IMPL_OFFSET>,
            GetDefaultSecurityDescriptor::<Impl, IMPL_OFFSET>,
            LegacyCsp::<Impl, IMPL_OFFSET>,
            GetCspStatusFromOperations::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICspInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICspInformationsImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn AddAvailableCsps();
    fn ItemByName();
    fn GetCspStatusFromProviderName();
    fn GetCspStatusesFromOperations();
    fn GetEncryptionCspAlgorithms();
    fn GetHashAlgorithms();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICspInformationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICspInformationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICspInformationsVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAvailableCsps<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemByName<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppcspinformation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCspStatusFromProviderName<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, legacykeyspec: X509KeySpec, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCspStatusesFromOperations<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operations: AlgorithmOperationFlags, pcspinformation: ::windows::core::RawPtr, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEncryptionCspAlgorithms<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcspinformation: ::windows::core::RawPtr, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHashAlgorithms<Impl: ICspInformationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcspinformation: ::windows::core::RawPtr, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            ItemByIndex::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            AddAvailableCsps::<Impl, IMPL_OFFSET>,
            ItemByName::<Impl, IMPL_OFFSET>,
            GetCspStatusFromProviderName::<Impl, IMPL_OFFSET>,
            GetCspStatusesFromOperations::<Impl, IMPL_OFFSET>,
            GetEncryptionCspAlgorithms::<Impl, IMPL_OFFSET>,
            GetHashAlgorithms::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICspInformations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICspStatusImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Ordinal();
    fn SetOrdinal();
    fn CspAlgorithm();
    fn CspInformation();
    fn EnrollmentStatus();
    fn DisplayName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICspStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICspStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICspStatusVtbl {
        unsafe extern "system" fn Initialize<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsp: ::windows::core::RawPtr, palgorithm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ordinal<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOrdinal<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CspAlgorithm<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CspInformation<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnrollmentStatus<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayName<Impl: ICspStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Ordinal::<Impl, IMPL_OFFSET>,
            SetOrdinal::<Impl, IMPL_OFFSET>,
            CspAlgorithm::<Impl, IMPL_OFFSET>,
            CspInformation::<Impl, IMPL_OFFSET>,
            EnrollmentStatus::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICspStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICspStatusesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn ItemByName();
    fn ItemByOrdinal();
    fn ItemByOperations();
    fn ItemByProvider();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICspStatusesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICspStatusesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICspStatusesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemByName<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strcspname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, stralgorithmname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemByOrdinal<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ordinal: i32, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemByOperations<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strcspname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, stralgorithmname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, operations: AlgorithmOperationFlags, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemByProvider<Impl: ICspStatusesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcspstatus: ::windows::core::RawPtr, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            ItemByIndex::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            ItemByName::<Impl, IMPL_OFFSET>,
            ItemByOrdinal::<Impl, IMPL_OFFSET>,
            ItemByOperations::<Impl, IMPL_OFFSET>,
            ItemByProvider::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICspStatuses as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnrollImpl: Sized {
    fn createFilePKCS10WStr();
    fn acceptFilePKCS7WStr();
    fn createPKCS10WStr();
    fn acceptPKCS7Blob();
    fn getCertContextFromPKCS7();
    fn getMyStore();
    fn getCAStore();
    fn getROOTHStore();
    fn enumProvidersWStr();
    fn enumContainersWStr();
    fn freeRequestInfoBlob();
    fn MyStoreNameWStr();
    fn SetMyStoreNameWStr();
    fn MyStoreTypeWStr();
    fn SetMyStoreTypeWStr();
    fn MyStoreFlags();
    fn SetMyStoreFlags();
    fn CAStoreNameWStr();
    fn SetCAStoreNameWStr();
    fn CAStoreTypeWStr();
    fn SetCAStoreTypeWStr();
    fn CAStoreFlags();
    fn SetCAStoreFlags();
    fn RootStoreNameWStr();
    fn SetRootStoreNameWStr();
    fn RootStoreTypeWStr();
    fn SetRootStoreTypeWStr();
    fn RootStoreFlags();
    fn SetRootStoreFlags();
    fn RequestStoreNameWStr();
    fn SetRequestStoreNameWStr();
    fn RequestStoreTypeWStr();
    fn SetRequestStoreTypeWStr();
    fn RequestStoreFlags();
    fn SetRequestStoreFlags();
    fn ContainerNameWStr();
    fn SetContainerNameWStr();
    fn ProviderNameWStr();
    fn SetProviderNameWStr();
    fn ProviderType();
    fn SetProviderType();
    fn KeySpec();
    fn SetKeySpec();
    fn ProviderFlags();
    fn SetProviderFlags();
    fn UseExistingKeySet();
    fn SetUseExistingKeySet();
    fn GenKeyFlags();
    fn SetGenKeyFlags();
    fn DeleteRequestCert();
    fn SetDeleteRequestCert();
    fn WriteCertToUserDS();
    fn SetWriteCertToUserDS();
    fn EnableT61DNEncoding();
    fn SetEnableT61DNEncoding();
    fn WriteCertToCSP();
    fn SetWriteCertToCSP();
    fn SPCFileNameWStr();
    fn SetSPCFileNameWStr();
    fn PVKFileNameWStr();
    fn SetPVKFileNameWStr();
    fn HashAlgorithmWStr();
    fn SetHashAlgorithmWStr();
    fn RenewalCertificate();
    fn SetRenewalCertificate();
    fn AddCertTypeToRequestWStr();
    fn AddNameValuePairToSignatureWStr();
    fn AddExtensionsToRequest();
    fn AddAuthenticatedAttributesToPKCS7Request();
    fn CreatePKCS7RequestFromRequest();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnrollVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnrollImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnrollVtbl {
        unsafe extern "system" fn createFilePKCS10WStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dnname: super::super::super::Foundation::PWSTR, usage: super::super::super::Foundation::PWSTR, wszpkcs10filename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn acceptFilePKCS7WStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpkcs7filename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createPKCS10WStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dnname: super::super::super::Foundation::PWSTR, usage: super::super::super::Foundation::PWSTR, ppkcs10blob: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn acceptPKCS7Blob<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblobpkcs7: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getCertContextFromPKCS7<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblobpkcs7: *mut super::CRYPTOAPI_BLOB) -> *mut super::CERT_CONTEXT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getMyStore<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getCAStore<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getROOTHStore<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn enumProvidersWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, dwflags: i32, pbstrprovname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn enumContainersWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, pbstr: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn freeRequestInfoBlob<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkcs7orpkcs10: super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MyStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMyStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MyStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMyStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MyStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMyStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CAStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCAStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CAStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCAStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CAStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCAStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RootStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRootStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RootStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRootStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RootStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRootStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRequestStoreNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRequestStoreTypeWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwtype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRequestStoreFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContainerNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwcontainer: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContainerNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwcontainer: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwprovider: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szwprovider: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderType<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderType<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeySpec<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdw: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeySpec<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dw: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UseExistingKeySet<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuseexistingkeys: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUseExistingKeySet<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuseexistingkeys: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenKeyFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGenKeyFlags<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteRequestCert<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdelete: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDeleteRequestCert<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdelete: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteCertToUserDS<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWriteCertToUserDS<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableT61DNEncoding<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableT61DNEncoding<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteCertToCSP<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWriteCertToCSP<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SPCFileNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSPCFileNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PVKFileNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPVKFileNameWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HashAlgorithmWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashAlgorithmWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RenewalCertificate<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRenewalCertificate<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertcontext: *const super::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddCertTypeToRequestWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szw: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddNameValuePairToSignatureWStr<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddExtensionsToRequest<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcertextensions: *mut super::CERT_EXTENSIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAuthenticatedAttributesToPKCS7Request<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: *mut super::CRYPT_ATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePKCS7RequestFromRequest<Impl: IEnrollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *mut super::CRYPTOAPI_BLOB, psigningcertcontext: *const super::CERT_CONTEXT, ppkcs7blob: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            createFilePKCS10WStr::<Impl, IMPL_OFFSET>,
            acceptFilePKCS7WStr::<Impl, IMPL_OFFSET>,
            createPKCS10WStr::<Impl, IMPL_OFFSET>,
            acceptPKCS7Blob::<Impl, IMPL_OFFSET>,
            getCertContextFromPKCS7::<Impl, IMPL_OFFSET>,
            getMyStore::<Impl, IMPL_OFFSET>,
            getCAStore::<Impl, IMPL_OFFSET>,
            getROOTHStore::<Impl, IMPL_OFFSET>,
            enumProvidersWStr::<Impl, IMPL_OFFSET>,
            enumContainersWStr::<Impl, IMPL_OFFSET>,
            freeRequestInfoBlob::<Impl, IMPL_OFFSET>,
            MyStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetMyStoreNameWStr::<Impl, IMPL_OFFSET>,
            MyStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetMyStoreTypeWStr::<Impl, IMPL_OFFSET>,
            MyStoreFlags::<Impl, IMPL_OFFSET>,
            SetMyStoreFlags::<Impl, IMPL_OFFSET>,
            CAStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetCAStoreNameWStr::<Impl, IMPL_OFFSET>,
            CAStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetCAStoreTypeWStr::<Impl, IMPL_OFFSET>,
            CAStoreFlags::<Impl, IMPL_OFFSET>,
            SetCAStoreFlags::<Impl, IMPL_OFFSET>,
            RootStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetRootStoreNameWStr::<Impl, IMPL_OFFSET>,
            RootStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetRootStoreTypeWStr::<Impl, IMPL_OFFSET>,
            RootStoreFlags::<Impl, IMPL_OFFSET>,
            SetRootStoreFlags::<Impl, IMPL_OFFSET>,
            RequestStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetRequestStoreNameWStr::<Impl, IMPL_OFFSET>,
            RequestStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetRequestStoreTypeWStr::<Impl, IMPL_OFFSET>,
            RequestStoreFlags::<Impl, IMPL_OFFSET>,
            SetRequestStoreFlags::<Impl, IMPL_OFFSET>,
            ContainerNameWStr::<Impl, IMPL_OFFSET>,
            SetContainerNameWStr::<Impl, IMPL_OFFSET>,
            ProviderNameWStr::<Impl, IMPL_OFFSET>,
            SetProviderNameWStr::<Impl, IMPL_OFFSET>,
            ProviderType::<Impl, IMPL_OFFSET>,
            SetProviderType::<Impl, IMPL_OFFSET>,
            KeySpec::<Impl, IMPL_OFFSET>,
            SetKeySpec::<Impl, IMPL_OFFSET>,
            ProviderFlags::<Impl, IMPL_OFFSET>,
            SetProviderFlags::<Impl, IMPL_OFFSET>,
            UseExistingKeySet::<Impl, IMPL_OFFSET>,
            SetUseExistingKeySet::<Impl, IMPL_OFFSET>,
            GenKeyFlags::<Impl, IMPL_OFFSET>,
            SetGenKeyFlags::<Impl, IMPL_OFFSET>,
            DeleteRequestCert::<Impl, IMPL_OFFSET>,
            SetDeleteRequestCert::<Impl, IMPL_OFFSET>,
            WriteCertToUserDS::<Impl, IMPL_OFFSET>,
            SetWriteCertToUserDS::<Impl, IMPL_OFFSET>,
            EnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            SetEnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            WriteCertToCSP::<Impl, IMPL_OFFSET>,
            SetWriteCertToCSP::<Impl, IMPL_OFFSET>,
            SPCFileNameWStr::<Impl, IMPL_OFFSET>,
            SetSPCFileNameWStr::<Impl, IMPL_OFFSET>,
            PVKFileNameWStr::<Impl, IMPL_OFFSET>,
            SetPVKFileNameWStr::<Impl, IMPL_OFFSET>,
            HashAlgorithmWStr::<Impl, IMPL_OFFSET>,
            SetHashAlgorithmWStr::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            AddCertTypeToRequestWStr::<Impl, IMPL_OFFSET>,
            AddNameValuePairToSignatureWStr::<Impl, IMPL_OFFSET>,
            AddExtensionsToRequest::<Impl, IMPL_OFFSET>,
            AddAuthenticatedAttributesToPKCS7Request::<Impl, IMPL_OFFSET>,
            CreatePKCS7RequestFromRequest::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnroll as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnroll2Impl: Sized + IEnrollImpl {
    fn InstallPKCS7Blob();
    fn Reset();
    fn GetSupportedKeySpec();
    fn GetKeyLen();
    fn EnumAlgs();
    fn GetAlgNameWStr();
    fn SetReuseHardwareKeyIfUnableToGenNew();
    fn ReuseHardwareKeyIfUnableToGenNew();
    fn SetHashAlgID();
    fn HashAlgID();
    fn SetHStoreMy();
    fn SetHStoreCA();
    fn SetHStoreROOT();
    fn SetHStoreRequest();
    fn SetLimitExchangeKeyToEncipherment();
    fn LimitExchangeKeyToEncipherment();
    fn SetEnableSMIMECapabilities();
    fn EnableSMIMECapabilities();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnroll2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnroll2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnroll2Vtbl {
        unsafe extern "system" fn InstallPKCS7Blob<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblobpkcs7: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedKeySpec<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwkeyspec: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeyLen<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmin: super::super::super::Foundation::BOOL, fexchange: super::super::super::Foundation::BOOL, pdwkeysize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAlgs<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, algclass: i32, pdwalgid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAlgNameWStr<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, algid: i32, ppwsz: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReuseHardwareKeyIfUnableToGenNew<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freusehardwarekeyifunabletogennew: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReuseHardwareKeyIfUnableToGenNew<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freusehardwarekeyifunabletogennew: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashAlgID<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashalgid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HashAlgID<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashalgid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHStoreMy<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHStoreCA<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHStoreROOT<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHStoreRequest<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLimitExchangeKeyToEncipherment<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flimitexchangekeytoencipherment: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LimitExchangeKeyToEncipherment<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flimitexchangekeytoencipherment: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableSMIMECapabilities<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablesmimecapabilities: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableSMIMECapabilities<Impl: IEnroll2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablesmimecapabilities: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            createFilePKCS10WStr::<Impl, IMPL_OFFSET>,
            acceptFilePKCS7WStr::<Impl, IMPL_OFFSET>,
            createPKCS10WStr::<Impl, IMPL_OFFSET>,
            acceptPKCS7Blob::<Impl, IMPL_OFFSET>,
            getCertContextFromPKCS7::<Impl, IMPL_OFFSET>,
            getMyStore::<Impl, IMPL_OFFSET>,
            getCAStore::<Impl, IMPL_OFFSET>,
            getROOTHStore::<Impl, IMPL_OFFSET>,
            enumProvidersWStr::<Impl, IMPL_OFFSET>,
            enumContainersWStr::<Impl, IMPL_OFFSET>,
            freeRequestInfoBlob::<Impl, IMPL_OFFSET>,
            MyStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetMyStoreNameWStr::<Impl, IMPL_OFFSET>,
            MyStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetMyStoreTypeWStr::<Impl, IMPL_OFFSET>,
            MyStoreFlags::<Impl, IMPL_OFFSET>,
            SetMyStoreFlags::<Impl, IMPL_OFFSET>,
            CAStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetCAStoreNameWStr::<Impl, IMPL_OFFSET>,
            CAStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetCAStoreTypeWStr::<Impl, IMPL_OFFSET>,
            CAStoreFlags::<Impl, IMPL_OFFSET>,
            SetCAStoreFlags::<Impl, IMPL_OFFSET>,
            RootStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetRootStoreNameWStr::<Impl, IMPL_OFFSET>,
            RootStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetRootStoreTypeWStr::<Impl, IMPL_OFFSET>,
            RootStoreFlags::<Impl, IMPL_OFFSET>,
            SetRootStoreFlags::<Impl, IMPL_OFFSET>,
            RequestStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetRequestStoreNameWStr::<Impl, IMPL_OFFSET>,
            RequestStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetRequestStoreTypeWStr::<Impl, IMPL_OFFSET>,
            RequestStoreFlags::<Impl, IMPL_OFFSET>,
            SetRequestStoreFlags::<Impl, IMPL_OFFSET>,
            ContainerNameWStr::<Impl, IMPL_OFFSET>,
            SetContainerNameWStr::<Impl, IMPL_OFFSET>,
            ProviderNameWStr::<Impl, IMPL_OFFSET>,
            SetProviderNameWStr::<Impl, IMPL_OFFSET>,
            ProviderType::<Impl, IMPL_OFFSET>,
            SetProviderType::<Impl, IMPL_OFFSET>,
            KeySpec::<Impl, IMPL_OFFSET>,
            SetKeySpec::<Impl, IMPL_OFFSET>,
            ProviderFlags::<Impl, IMPL_OFFSET>,
            SetProviderFlags::<Impl, IMPL_OFFSET>,
            UseExistingKeySet::<Impl, IMPL_OFFSET>,
            SetUseExistingKeySet::<Impl, IMPL_OFFSET>,
            GenKeyFlags::<Impl, IMPL_OFFSET>,
            SetGenKeyFlags::<Impl, IMPL_OFFSET>,
            DeleteRequestCert::<Impl, IMPL_OFFSET>,
            SetDeleteRequestCert::<Impl, IMPL_OFFSET>,
            WriteCertToUserDS::<Impl, IMPL_OFFSET>,
            SetWriteCertToUserDS::<Impl, IMPL_OFFSET>,
            EnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            SetEnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            WriteCertToCSP::<Impl, IMPL_OFFSET>,
            SetWriteCertToCSP::<Impl, IMPL_OFFSET>,
            SPCFileNameWStr::<Impl, IMPL_OFFSET>,
            SetSPCFileNameWStr::<Impl, IMPL_OFFSET>,
            PVKFileNameWStr::<Impl, IMPL_OFFSET>,
            SetPVKFileNameWStr::<Impl, IMPL_OFFSET>,
            HashAlgorithmWStr::<Impl, IMPL_OFFSET>,
            SetHashAlgorithmWStr::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            AddCertTypeToRequestWStr::<Impl, IMPL_OFFSET>,
            AddNameValuePairToSignatureWStr::<Impl, IMPL_OFFSET>,
            AddExtensionsToRequest::<Impl, IMPL_OFFSET>,
            AddAuthenticatedAttributesToPKCS7Request::<Impl, IMPL_OFFSET>,
            CreatePKCS7RequestFromRequest::<Impl, IMPL_OFFSET>,
            InstallPKCS7Blob::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            GetSupportedKeySpec::<Impl, IMPL_OFFSET>,
            GetKeyLen::<Impl, IMPL_OFFSET>,
            EnumAlgs::<Impl, IMPL_OFFSET>,
            GetAlgNameWStr::<Impl, IMPL_OFFSET>,
            SetReuseHardwareKeyIfUnableToGenNew::<Impl, IMPL_OFFSET>,
            ReuseHardwareKeyIfUnableToGenNew::<Impl, IMPL_OFFSET>,
            SetHashAlgID::<Impl, IMPL_OFFSET>,
            HashAlgID::<Impl, IMPL_OFFSET>,
            SetHStoreMy::<Impl, IMPL_OFFSET>,
            SetHStoreCA::<Impl, IMPL_OFFSET>,
            SetHStoreROOT::<Impl, IMPL_OFFSET>,
            SetHStoreRequest::<Impl, IMPL_OFFSET>,
            SetLimitExchangeKeyToEncipherment::<Impl, IMPL_OFFSET>,
            LimitExchangeKeyToEncipherment::<Impl, IMPL_OFFSET>,
            SetEnableSMIMECapabilities::<Impl, IMPL_OFFSET>,
            EnableSMIMECapabilities::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnroll2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnroll4Impl: Sized + IEnroll2Impl + IEnrollImpl {
    fn SetThumbPrintWStr();
    fn ThumbPrintWStr();
    fn SetPrivateKeyArchiveCertificate();
    fn GetPrivateKeyArchiveCertificate();
    fn binaryBlobToString();
    fn stringToBinaryBlob();
    fn addExtensionToRequestWStr();
    fn addAttributeToRequestWStr();
    fn addNameValuePairToRequestWStr();
    fn resetExtensions();
    fn resetAttributes();
    fn createRequestWStr();
    fn createFileRequestWStr();
    fn acceptResponseBlob();
    fn acceptFileResponseWStr();
    fn getCertContextFromResponseBlob();
    fn getCertContextFromFileResponseWStr();
    fn createPFXWStr();
    fn createFilePFXWStr();
    fn setPendingRequestInfoWStr();
    fn enumPendingRequestWStr();
    fn removePendingRequestWStr();
    fn GetKeyLenEx();
    fn InstallPKCS7BlobEx();
    fn AddCertTypeToRequestWStrEx();
    fn getProviderTypeWStr();
    fn addBlobPropertyToCertificateWStr();
    fn SetSignerCertificate();
    fn SetClientId();
    fn ClientId();
    fn SetIncludeSubjectKeyID();
    fn IncludeSubjectKeyID();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnroll4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnroll4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnroll4Vtbl {
        unsafe extern "system" fn SetThumbPrintWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprintblob: super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ThumbPrintWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprintblob: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateKeyArchiveCertificate<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprivatekeyarchivecert: *const super::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateKeyArchiveCertificate<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut super::CERT_CONTEXT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn binaryBlobToString<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pblobbinary: *mut super::CRYPTOAPI_BLOB, ppwszstring: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn stringToBinaryBlob<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pwszstring: super::super::super::Foundation::PWSTR, pblobbinary: *mut super::CRYPTOAPI_BLOB, pdwskip: *mut i32, pdwflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addExtensionToRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pwszname: super::super::super::Foundation::PWSTR, pblobvalue: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addAttributeToRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pwszname: super::super::super::Foundation::PWSTR, pblobvalue: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addNameValuePairToRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pwszname: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn resetExtensions<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn resetAttributes<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: super::super::super::Foundation::PWSTR, pwszusage: super::super::super::Foundation::PWSTR, pblobrequest: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createFileRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: super::super::super::Foundation::PWSTR, pwszusage: super::super::super::Foundation::PWSTR, pwszrequestfilename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn acceptResponseBlob<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblobresponse: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn acceptFileResponseWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszresponsefilename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getCertContextFromResponseBlob<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblobresponse: *mut super::CRYPTOAPI_BLOB, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getCertContextFromFileResponseWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszresponsefilename: super::super::super::Foundation::PWSTR, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createPFXWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpassword: super::super::super::Foundation::PWSTR, pblobpfx: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createFilePFXWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpassword: super::super::super::Foundation::PWSTR, pwszpfxfilename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setPendingRequestInfoWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrequestid: i32, pwszcadns: super::super::super::Foundation::PWSTR, pwszcaname: super::super::super::Foundation::PWSTR, pwszfriendlyname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn enumPendingRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY, ppproperty: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removePendingRequestWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprintblob: super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeyLenEx<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC, pdwkeysize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallPKCS7BlobEx<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblobpkcs7: *mut super::CRYPTOAPI_BLOB, plcertinstalled: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddCertTypeToRequestWStrEx<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltype: ADDED_CERT_TYPE, pwszoidorname: super::super::super::Foundation::PWSTR, lmajorversion: i32, fminorversion: super::super::super::Foundation::BOOL, lminorversion: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getProviderTypeWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprovname: super::super::super::Foundation::PWSTR, plprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addBlobPropertyToCertificateWStr<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropertyid: i32, lreserved: i32, pblobproperty: *mut super::CRYPTOAPI_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignerCertificate<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignercert: *const super::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientId<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lclientid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientId<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclientid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIncludeSubjectKeyID<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finclude: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IncludeSubjectKeyID<Impl: IEnroll4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinclude: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            createFilePKCS10WStr::<Impl, IMPL_OFFSET>,
            acceptFilePKCS7WStr::<Impl, IMPL_OFFSET>,
            createPKCS10WStr::<Impl, IMPL_OFFSET>,
            acceptPKCS7Blob::<Impl, IMPL_OFFSET>,
            getCertContextFromPKCS7::<Impl, IMPL_OFFSET>,
            getMyStore::<Impl, IMPL_OFFSET>,
            getCAStore::<Impl, IMPL_OFFSET>,
            getROOTHStore::<Impl, IMPL_OFFSET>,
            enumProvidersWStr::<Impl, IMPL_OFFSET>,
            enumContainersWStr::<Impl, IMPL_OFFSET>,
            freeRequestInfoBlob::<Impl, IMPL_OFFSET>,
            MyStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetMyStoreNameWStr::<Impl, IMPL_OFFSET>,
            MyStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetMyStoreTypeWStr::<Impl, IMPL_OFFSET>,
            MyStoreFlags::<Impl, IMPL_OFFSET>,
            SetMyStoreFlags::<Impl, IMPL_OFFSET>,
            CAStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetCAStoreNameWStr::<Impl, IMPL_OFFSET>,
            CAStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetCAStoreTypeWStr::<Impl, IMPL_OFFSET>,
            CAStoreFlags::<Impl, IMPL_OFFSET>,
            SetCAStoreFlags::<Impl, IMPL_OFFSET>,
            RootStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetRootStoreNameWStr::<Impl, IMPL_OFFSET>,
            RootStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetRootStoreTypeWStr::<Impl, IMPL_OFFSET>,
            RootStoreFlags::<Impl, IMPL_OFFSET>,
            SetRootStoreFlags::<Impl, IMPL_OFFSET>,
            RequestStoreNameWStr::<Impl, IMPL_OFFSET>,
            SetRequestStoreNameWStr::<Impl, IMPL_OFFSET>,
            RequestStoreTypeWStr::<Impl, IMPL_OFFSET>,
            SetRequestStoreTypeWStr::<Impl, IMPL_OFFSET>,
            RequestStoreFlags::<Impl, IMPL_OFFSET>,
            SetRequestStoreFlags::<Impl, IMPL_OFFSET>,
            ContainerNameWStr::<Impl, IMPL_OFFSET>,
            SetContainerNameWStr::<Impl, IMPL_OFFSET>,
            ProviderNameWStr::<Impl, IMPL_OFFSET>,
            SetProviderNameWStr::<Impl, IMPL_OFFSET>,
            ProviderType::<Impl, IMPL_OFFSET>,
            SetProviderType::<Impl, IMPL_OFFSET>,
            KeySpec::<Impl, IMPL_OFFSET>,
            SetKeySpec::<Impl, IMPL_OFFSET>,
            ProviderFlags::<Impl, IMPL_OFFSET>,
            SetProviderFlags::<Impl, IMPL_OFFSET>,
            UseExistingKeySet::<Impl, IMPL_OFFSET>,
            SetUseExistingKeySet::<Impl, IMPL_OFFSET>,
            GenKeyFlags::<Impl, IMPL_OFFSET>,
            SetGenKeyFlags::<Impl, IMPL_OFFSET>,
            DeleteRequestCert::<Impl, IMPL_OFFSET>,
            SetDeleteRequestCert::<Impl, IMPL_OFFSET>,
            WriteCertToUserDS::<Impl, IMPL_OFFSET>,
            SetWriteCertToUserDS::<Impl, IMPL_OFFSET>,
            EnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            SetEnableT61DNEncoding::<Impl, IMPL_OFFSET>,
            WriteCertToCSP::<Impl, IMPL_OFFSET>,
            SetWriteCertToCSP::<Impl, IMPL_OFFSET>,
            SPCFileNameWStr::<Impl, IMPL_OFFSET>,
            SetSPCFileNameWStr::<Impl, IMPL_OFFSET>,
            PVKFileNameWStr::<Impl, IMPL_OFFSET>,
            SetPVKFileNameWStr::<Impl, IMPL_OFFSET>,
            HashAlgorithmWStr::<Impl, IMPL_OFFSET>,
            SetHashAlgorithmWStr::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            AddCertTypeToRequestWStr::<Impl, IMPL_OFFSET>,
            AddNameValuePairToSignatureWStr::<Impl, IMPL_OFFSET>,
            AddExtensionsToRequest::<Impl, IMPL_OFFSET>,
            AddAuthenticatedAttributesToPKCS7Request::<Impl, IMPL_OFFSET>,
            CreatePKCS7RequestFromRequest::<Impl, IMPL_OFFSET>,
            InstallPKCS7Blob::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            GetSupportedKeySpec::<Impl, IMPL_OFFSET>,
            GetKeyLen::<Impl, IMPL_OFFSET>,
            EnumAlgs::<Impl, IMPL_OFFSET>,
            GetAlgNameWStr::<Impl, IMPL_OFFSET>,
            SetReuseHardwareKeyIfUnableToGenNew::<Impl, IMPL_OFFSET>,
            ReuseHardwareKeyIfUnableToGenNew::<Impl, IMPL_OFFSET>,
            SetHashAlgID::<Impl, IMPL_OFFSET>,
            HashAlgID::<Impl, IMPL_OFFSET>,
            SetHStoreMy::<Impl, IMPL_OFFSET>,
            SetHStoreCA::<Impl, IMPL_OFFSET>,
            SetHStoreROOT::<Impl, IMPL_OFFSET>,
            SetHStoreRequest::<Impl, IMPL_OFFSET>,
            SetLimitExchangeKeyToEncipherment::<Impl, IMPL_OFFSET>,
            LimitExchangeKeyToEncipherment::<Impl, IMPL_OFFSET>,
            SetEnableSMIMECapabilities::<Impl, IMPL_OFFSET>,
            EnableSMIMECapabilities::<Impl, IMPL_OFFSET>,
            SetThumbPrintWStr::<Impl, IMPL_OFFSET>,
            ThumbPrintWStr::<Impl, IMPL_OFFSET>,
            SetPrivateKeyArchiveCertificate::<Impl, IMPL_OFFSET>,
            GetPrivateKeyArchiveCertificate::<Impl, IMPL_OFFSET>,
            binaryBlobToString::<Impl, IMPL_OFFSET>,
            stringToBinaryBlob::<Impl, IMPL_OFFSET>,
            addExtensionToRequestWStr::<Impl, IMPL_OFFSET>,
            addAttributeToRequestWStr::<Impl, IMPL_OFFSET>,
            addNameValuePairToRequestWStr::<Impl, IMPL_OFFSET>,
            resetExtensions::<Impl, IMPL_OFFSET>,
            resetAttributes::<Impl, IMPL_OFFSET>,
            createRequestWStr::<Impl, IMPL_OFFSET>,
            createFileRequestWStr::<Impl, IMPL_OFFSET>,
            acceptResponseBlob::<Impl, IMPL_OFFSET>,
            acceptFileResponseWStr::<Impl, IMPL_OFFSET>,
            getCertContextFromResponseBlob::<Impl, IMPL_OFFSET>,
            getCertContextFromFileResponseWStr::<Impl, IMPL_OFFSET>,
            createPFXWStr::<Impl, IMPL_OFFSET>,
            createFilePFXWStr::<Impl, IMPL_OFFSET>,
            setPendingRequestInfoWStr::<Impl, IMPL_OFFSET>,
            enumPendingRequestWStr::<Impl, IMPL_OFFSET>,
            removePendingRequestWStr::<Impl, IMPL_OFFSET>,
            GetKeyLenEx::<Impl, IMPL_OFFSET>,
            InstallPKCS7BlobEx::<Impl, IMPL_OFFSET>,
            AddCertTypeToRequestWStrEx::<Impl, IMPL_OFFSET>,
            getProviderTypeWStr::<Impl, IMPL_OFFSET>,
            addBlobPropertyToCertificateWStr::<Impl, IMPL_OFFSET>,
            SetSignerCertificate::<Impl, IMPL_OFFSET>,
            SetClientId::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            SetIncludeSubjectKeyID::<Impl, IMPL_OFFSET>,
            IncludeSubjectKeyID::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnroll4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumCERTVIEWATTRIBUTEImpl: Sized + IDispatchImpl {
    fn Next();
    fn GetName();
    fn GetValue();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumCERTVIEWATTRIBUTEVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCERTVIEWATTRIBUTEImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCERTVIEWATTRIBUTEVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCERTVIEWATTRIBUTEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: IEnumCERTVIEWATTRIBUTEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IEnumCERTVIEWATTRIBUTEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCERTVIEWATTRIBUTEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCERTVIEWATTRIBUTEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCERTVIEWATTRIBUTEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Next::<Impl, IMPL_OFFSET>, GetName::<Impl, IMPL_OFFSET>, GetValue::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCERTVIEWATTRIBUTE as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumCERTVIEWCOLUMNImpl: Sized + IDispatchImpl {
    fn Next();
    fn GetName();
    fn GetDisplayName();
    fn GetType();
    fn IsIndexed();
    fn GetMaxLength();
    fn GetValue();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumCERTVIEWCOLUMNVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCERTVIEWCOLUMNImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCERTVIEWCOLUMNVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsIndexed<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindexed: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxLength<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCERTVIEWCOLUMNImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Next::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            IsIndexed::<Impl, IMPL_OFFSET>,
            GetMaxLength::<Impl, IMPL_OFFSET>,
            GetValue::<Impl, IMPL_OFFSET>,
            Skip::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCERTVIEWCOLUMN as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumCERTVIEWEXTENSIONImpl: Sized + IDispatchImpl {
    fn Next();
    fn GetName();
    fn GetFlags();
    fn GetValue();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumCERTVIEWEXTENSIONVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCERTVIEWEXTENSIONImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCERTVIEWEXTENSIONVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: CERT_PROPERTY_TYPE, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCERTVIEWEXTENSIONImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Next::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            GetFlags::<Impl, IMPL_OFFSET>,
            GetValue::<Impl, IMPL_OFFSET>,
            Skip::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCERTVIEWEXTENSION as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumCERTVIEWROWImpl: Sized + IDispatchImpl {
    fn Next();
    fn EnumCertViewColumn();
    fn EnumCertViewAttribute();
    fn EnumCertViewExtension();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetMaxIndex();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumCERTVIEWROWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCERTVIEWROWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCERTVIEWROWVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumCertViewColumn<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumCertViewAttribute<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumCertViewExtension<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxIndex<Impl: IEnumCERTVIEWROWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows::core::HRESULT {
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
            Next::<Impl, IMPL_OFFSET>,
            EnumCertViewColumn::<Impl, IMPL_OFFSET>,
            EnumCertViewAttribute::<Impl, IMPL_OFFSET>,
            EnumCertViewExtension::<Impl, IMPL_OFFSET>,
            Skip::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            GetMaxIndex::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCERTVIEWROW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INDESPolicyImpl: Sized {
    fn Initialize();
    fn Uninitialize();
    fn GenerateChallenge();
    fn VerifyRequest();
    fn Notify();
}
#[cfg(feature = "Win32_Foundation")]
impl INDESPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INDESPolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INDESPolicyVtbl {
        unsafe extern "system" fn Initialize<Impl: INDESPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Uninitialize<Impl: INDESPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateChallenge<Impl: INDESPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztemplate: super::super::super::Foundation::PWSTR, pwszparams: super::super::super::Foundation::PWSTR, ppwszresponse: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VerifyRequest<Impl: INDESPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctbrequest: *mut CERTTRANSBLOB, pctbsigningcertencoded: *mut CERTTRANSBLOB, pwsztemplate: super::super::super::Foundation::PWSTR, pwsztransactionid: super::super::super::Foundation::PWSTR, pfverified: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notify<Impl: INDESPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszchallenge: super::super::super::Foundation::PWSTR, pwsztransactionid: super::super::super::Foundation::PWSTR, disposition: X509SCEPDisposition, lasthresult: i32, pctbissuedcertencoded: *mut CERTTRANSBLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Uninitialize::<Impl, IMPL_OFFSET>, GenerateChallenge::<Impl, IMPL_OFFSET>, VerifyRequest::<Impl, IMPL_OFFSET>, Notify::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INDESPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOCSPAdminImpl: Sized + IDispatchImpl {
    fn OCSPServiceProperties();
    fn OCSPCAConfigurationCollection();
    fn GetConfiguration();
    fn SetConfiguration();
    fn GetMyRoles();
    fn Ping();
    fn SetSecurity();
    fn GetSecurity();
    fn GetSigningCertificates();
    fn GetHashAlgorithms();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOCSPAdminVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOCSPAdminImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOCSPAdminVtbl {
        unsafe extern "system" fn OCSPServiceProperties<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OCSPCAConfigurationCollection<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConfiguration<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bforce: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConfiguration<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bforce: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMyRoles<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, proles: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ping<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurity<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSecurity<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSigningCertificates<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pcacertvar: *const super::super::super::System::Com::VARIANT, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHashAlgorithms<Impl: IOCSPAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcaid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            OCSPServiceProperties::<Impl, IMPL_OFFSET>,
            OCSPCAConfigurationCollection::<Impl, IMPL_OFFSET>,
            GetConfiguration::<Impl, IMPL_OFFSET>,
            SetConfiguration::<Impl, IMPL_OFFSET>,
            GetMyRoles::<Impl, IMPL_OFFSET>,
            Ping::<Impl, IMPL_OFFSET>,
            SetSecurity::<Impl, IMPL_OFFSET>,
            GetSecurity::<Impl, IMPL_OFFSET>,
            GetSigningCertificates::<Impl, IMPL_OFFSET>,
            GetHashAlgorithms::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOCSPAdmin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOCSPCAConfigurationImpl: Sized + IDispatchImpl {
    fn Identifier();
    fn CACertificate();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn SigningFlags();
    fn SetSigningFlags();
    fn SigningCertificate();
    fn SetSigningCertificate();
    fn ReminderDuration();
    fn SetReminderDuration();
    fn ErrorCode();
    fn CSPName();
    fn KeySpec();
    fn ProviderCLSID();
    fn SetProviderCLSID();
    fn ProviderProperties();
    fn SetProviderProperties();
    fn Modified();
    fn LocalRevocationInformation();
    fn SetLocalRevocationInformation();
    fn SigningCertificateTemplate();
    fn SetSigningCertificateTemplate();
    fn CAConfig();
    fn SetCAConfig();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOCSPCAConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOCSPCAConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOCSPCAConfigurationVtbl {
        unsafe extern "system" fn Identifier<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CACertificate<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SigningFlags<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSigningFlags<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SigningCertificate<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSigningCertificate<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReminderDuration<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReminderDuration<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ErrorCode<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CSPName<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeySpec<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderCLSID<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderCLSID<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderProperties<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderProperties<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Modified<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalRevocationInformation<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalRevocationInformation<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SigningCertificateTemplate<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSigningCertificateTemplate<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CAConfig<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCAConfig<Impl: IOCSPCAConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Identifier::<Impl, IMPL_OFFSET>,
            CACertificate::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            SigningFlags::<Impl, IMPL_OFFSET>,
            SetSigningFlags::<Impl, IMPL_OFFSET>,
            SigningCertificate::<Impl, IMPL_OFFSET>,
            SetSigningCertificate::<Impl, IMPL_OFFSET>,
            ReminderDuration::<Impl, IMPL_OFFSET>,
            SetReminderDuration::<Impl, IMPL_OFFSET>,
            ErrorCode::<Impl, IMPL_OFFSET>,
            CSPName::<Impl, IMPL_OFFSET>,
            KeySpec::<Impl, IMPL_OFFSET>,
            ProviderCLSID::<Impl, IMPL_OFFSET>,
            SetProviderCLSID::<Impl, IMPL_OFFSET>,
            ProviderProperties::<Impl, IMPL_OFFSET>,
            SetProviderProperties::<Impl, IMPL_OFFSET>,
            Modified::<Impl, IMPL_OFFSET>,
            LocalRevocationInformation::<Impl, IMPL_OFFSET>,
            SetLocalRevocationInformation::<Impl, IMPL_OFFSET>,
            SigningCertificateTemplate::<Impl, IMPL_OFFSET>,
            SetSigningCertificateTemplate::<Impl, IMPL_OFFSET>,
            CAConfig::<Impl, IMPL_OFFSET>,
            SetCAConfig::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOCSPCAConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOCSPCAConfigurationCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn ItemByName();
    fn CreateCAConfiguration();
    fn DeleteCAConfiguration();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOCSPCAConfigurationCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOCSPCAConfigurationCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOCSPCAConfigurationCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IOCSPCAConfigurationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IOCSPCAConfigurationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IOCSPCAConfigurationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemByName<Impl: IOCSPCAConfigurationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstridentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCAConfiguration<Impl: IOCSPCAConfigurationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstridentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varcacert: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteCAConfiguration<Impl: IOCSPCAConfigurationCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstridentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            _NewEnum::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            ItemByName::<Impl, IMPL_OFFSET>,
            CreateCAConfiguration::<Impl, IMPL_OFFSET>,
            DeleteCAConfiguration::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOCSPCAConfigurationCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOCSPPropertyImpl: Sized + IDispatchImpl {
    fn Name();
    fn Value();
    fn SetValue();
    fn Modified();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOCSPPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOCSPPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOCSPPropertyVtbl {
        unsafe extern "system" fn Name<Impl: IOCSPPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: IOCSPPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: IOCSPPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Modified<Impl: IOCSPPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>, Modified::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOCSPProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOCSPPropertyCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn ItemByName();
    fn CreateProperty();
    fn DeleteProperty();
    fn InitializeFromProperties();
    fn GetAllProperties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOCSPPropertyCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOCSPPropertyCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOCSPPropertyCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemByName<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateProperty<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvarpropvalue: *const super::super::super::System::Com::VARIANT, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteProperty<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromProperties<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarproperties: *const super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllProperties<Impl: IOCSPPropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarproperties: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            _NewEnum::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            ItemByName::<Impl, IMPL_OFFSET>,
            CreateProperty::<Impl, IMPL_OFFSET>,
            DeleteProperty::<Impl, IMPL_OFFSET>,
            InitializeFromProperties::<Impl, IMPL_OFFSET>,
            GetAllProperties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOCSPPropertyCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IObjectIdImpl: Sized + IDispatchImpl {
    fn InitializeFromName();
    fn InitializeFromValue();
    fn InitializeFromAlgorithmName();
    fn Name();
    fn FriendlyName();
    fn SetFriendlyName();
    fn Value();
    fn GetAlgorithmName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IObjectIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectIdVtbl {
        unsafe extern "system" fn InitializeFromName<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: CERTENROLL_OBJECTID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromValue<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromAlgorithmName<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags, algflags: AlgorithmFlags, stralgorithmname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut CERTENROLL_OBJECTID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FriendlyName<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFriendlyName<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAlgorithmName<Impl: IObjectIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags, pstralgorithmname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            InitializeFromName::<Impl, IMPL_OFFSET>,
            InitializeFromValue::<Impl, IMPL_OFFSET>,
            InitializeFromAlgorithmName::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            FriendlyName::<Impl, IMPL_OFFSET>,
            SetFriendlyName::<Impl, IMPL_OFFSET>,
            Value::<Impl, IMPL_OFFSET>,
            GetAlgorithmName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IObjectIdsImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn AddRange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IObjectIdsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectIdsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectIdsVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRange<Impl: IObjectIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            ItemByIndex::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            AddRange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectIds as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPolicyQualifierImpl: Sized + IDispatchImpl {
    fn InitializeEncode();
    fn ObjectId();
    fn Qualifier();
    fn Type();
    fn RawData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPolicyQualifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolicyQualifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolicyQualifierVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IPolicyQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqualifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: PolicyQualifierType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ObjectId<Impl: IPolicyQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Qualifier<Impl: IPolicyQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IPolicyQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut PolicyQualifierType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RawData<Impl: IPolicyQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, InitializeEncode::<Impl, IMPL_OFFSET>, ObjectId::<Impl, IMPL_OFFSET>, Qualifier::<Impl, IMPL_OFFSET>, Type::<Impl, IMPL_OFFSET>, RawData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolicyQualifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPolicyQualifiersImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPolicyQualifiersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPolicyQualifiersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPolicyQualifiersVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IPolicyQualifiersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IPolicyQualifiersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IPolicyQualifiersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IPolicyQualifiersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IPolicyQualifiersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IPolicyQualifiersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, ItemByIndex::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPolicyQualifiers as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISignerCertificateImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Certificate();
    fn PrivateKey();
    fn Silent();
    fn SetSilent();
    fn ParentWindow();
    fn SetParentWindow();
    fn UIContextMessage();
    fn SetUIContextMessage();
    fn SetPin();
    fn SignatureInformation();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISignerCertificateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISignerCertificateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISignerCertificateVtbl {
        unsafe extern "system" fn Initialize<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16, verifytype: X509PrivateKeyVerify, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Certificate<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivateKey<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Silent<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSilent<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ParentWindow<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParentWindow<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UIContextMessage<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUIContextMessage<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPin<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SignatureInformation<Impl: ISignerCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Certificate::<Impl, IMPL_OFFSET>,
            PrivateKey::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SetPin::<Impl, IMPL_OFFSET>,
            SignatureInformation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISignerCertificate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISignerCertificatesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn Find();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISignerCertificatesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISignerCertificatesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISignerCertificatesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Find<Impl: ISignerCertificatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignercert: ::windows::core::RawPtr, pisignercert: *mut i32) -> ::windows::core::HRESULT {
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
            ItemByIndex::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            Find::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISignerCertificates as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISmimeCapabilitiesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn AddFromCsp();
    fn AddAvailableSmimeCapabilities();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISmimeCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmimeCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmimeCapabilitiesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFromCsp<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAvailableSmimeCapabilities<Impl: ISmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinecontext: i16) -> ::windows::core::HRESULT {
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
            ItemByIndex::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            AddFromCsp::<Impl, IMPL_OFFSET>,
            AddAvailableSmimeCapabilities::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmimeCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISmimeCapabilityImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn ObjectId();
    fn BitCount();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISmimeCapabilityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmimeCapabilityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmimeCapabilityVtbl {
        unsafe extern "system" fn Initialize<Impl: ISmimeCapabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, bitcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ObjectId<Impl: ISmimeCapabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BitCount<Impl: ISmimeCapabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, ObjectId::<Impl, IMPL_OFFSET>, BitCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmimeCapability as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX500DistinguishedNameImpl: Sized + IDispatchImpl {
    fn Decode();
    fn Encode();
    fn Name();
    fn EncodedName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX500DistinguishedNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX500DistinguishedNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX500DistinguishedNameVtbl {
        unsafe extern "system" fn Decode<Impl: IX500DistinguishedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodedname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, nameflags: X500NameFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Encode<Impl: IX500DistinguishedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nameflags: X500NameFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IX500DistinguishedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncodedName<Impl: IX500DistinguishedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Decode::<Impl, IMPL_OFFSET>, Encode::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, EncodedName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX500DistinguishedName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn ObjectId();
    fn RawData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509AttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ObjectId<Impl: IX509AttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RawData<Impl: IX509AttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, ObjectId::<Impl, IMPL_OFFSET>, RawData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509Attribute as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeArchiveKeyImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn EncryptedKeyBlob();
    fn EncryptionAlgorithm();
    fn EncryptionStrength();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeArchiveKeyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeArchiveKeyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeArchiveKeyVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509AttributeArchiveKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: ::windows::core::RawPtr, encoding: EncodingType, strcaxcert: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, palgorithm: ::windows::core::RawPtr, encryptionstrength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeArchiveKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptedKeyBlob<Impl: IX509AttributeArchiveKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptionAlgorithm<Impl: IX509AttributeArchiveKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptionStrength<Impl: IX509AttributeArchiveKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            EncryptedKeyBlob::<Impl, IMPL_OFFSET>,
            EncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptionStrength::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeArchiveKey as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeArchiveKeyHashImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncodeFromEncryptedKeyBlob();
    fn InitializeDecode();
    fn EncryptedKeyHashBlob();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeArchiveKeyHashVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeArchiveKeyHashImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeArchiveKeyHashVtbl {
        unsafe extern "system" fn InitializeEncodeFromEncryptedKeyBlob<Impl: IX509AttributeArchiveKeyHashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencryptedkeyblob: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeArchiveKeyHashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptedKeyHashBlob<Impl: IX509AttributeArchiveKeyHashImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeEncodeFromEncryptedKeyBlob::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            EncryptedKeyHashBlob::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeArchiveKeyHash as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeClientIdImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn ClientId();
    fn MachineDnsName();
    fn UserSamName();
    fn ProcessName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeClientIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeClientIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeClientIdVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509AttributeClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientid: RequestClientInfoClientId, strmachinednsname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strusersamname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strprocessname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientId<Impl: IX509AttributeClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut RequestClientInfoClientId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MachineDnsName<Impl: IX509AttributeClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserSamName<Impl: IX509AttributeClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessName<Impl: IX509AttributeClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            MachineDnsName::<Impl, IMPL_OFFSET>,
            UserSamName::<Impl, IMPL_OFFSET>,
            ProcessName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeClientId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeCspProviderImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn KeySpec();
    fn ProviderName();
    fn Signature();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeCspProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeCspProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeCspProviderVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509AttributeCspProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keyspec: X509KeySpec, strprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, strsignature: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeCspProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeySpec<Impl: IX509AttributeCspProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509KeySpec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderName<Impl: IX509AttributeCspProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Signature<Impl: IX509AttributeCspProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            KeySpec::<Impl, IMPL_OFFSET>,
            ProviderName::<Impl, IMPL_OFFSET>,
            Signature::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeCspProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeExtensionsImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn X509Extensions();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeExtensionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeExtensionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeExtensionsVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509AttributeExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextensions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn X509Extensions<Impl: IX509AttributeExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            X509Extensions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeExtensions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeOSVersionImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn OSVersion();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeOSVersionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeOSVersionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeOSVersionVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509AttributeOSVersionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strosversion: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeOSVersionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OSVersion<Impl: IX509AttributeOSVersionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            OSVersion::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeOSVersion as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributeRenewalCertificateImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn RenewalCertificate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributeRenewalCertificateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributeRenewalCertificateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributeRenewalCertificateVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509AttributeRenewalCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strcert: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509AttributeRenewalCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RenewalCertificate<Impl: IX509AttributeRenewalCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509AttributeRenewalCertificate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509AttributesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509AttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509AttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509AttributesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IX509AttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IX509AttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IX509AttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IX509AttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IX509AttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IX509AttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, ItemByIndex::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509Attributes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Encode();
    fn ResetForEncode();
    fn GetInnerRequest();
    fn Type();
    fn EnrollmentContext();
    fn Silent();
    fn SetSilent();
    fn ParentWindow();
    fn SetParentWindow();
    fn UIContextMessage();
    fn SetUIContextMessage();
    fn SuppressDefaults();
    fn SetSuppressDefaults();
    fn RenewalCertificate();
    fn SetRenewalCertificate();
    fn ClientId();
    fn SetClientId();
    fn CspInformations();
    fn SetCspInformations();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn AlternateSignatureAlgorithm();
    fn SetAlternateSignatureAlgorithm();
    fn RawData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Encode<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetForEncode<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInnerRequest<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: InnerRequestLevel, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509RequestType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnrollmentContext<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Silent<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSilent<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ParentWindow<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParentWindow<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UIContextMessage<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUIContextMessage<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SuppressDefaults<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSuppressDefaults<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RenewalCertificate<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRenewalCertificate<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientId<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut RequestClientInfoClientId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientId<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: RequestClientInfoClientId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CspInformations<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCspInformations<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AlternateSignatureAlgorithm<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlternateSignatureAlgorithm<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RawData<Impl: IX509CertificateRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode::<Impl, IMPL_OFFSET>,
            GetInnerRequest::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            EnrollmentContext::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SuppressDefaults::<Impl, IMPL_OFFSET>,
            SetSuppressDefaults::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            SetClientId::<Impl, IMPL_OFFSET>,
            CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestCertificateImpl: Sized + IX509CertificateRequestPkcs10Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn CheckPublicKeySignature();
    fn Issuer();
    fn SetIssuer();
    fn NotBefore();
    fn SetNotBefore();
    fn NotAfter();
    fn SetNotAfter();
    fn SerialNumber();
    fn SetSerialNumber();
    fn SignerCertificate();
    fn SetSignerCertificate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestCertificateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestCertificateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestCertificateVtbl {
        unsafe extern "system" fn CheckPublicKeySignature<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppublickey: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Issuer<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIssuer<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotBefore<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNotBefore<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotAfter<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNotAfter<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SerialNumber<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSerialNumber<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SignerCertificate<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignerCertificate<Impl: IX509CertificateRequestCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode::<Impl, IMPL_OFFSET>,
            GetInnerRequest::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            EnrollmentContext::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SuppressDefaults::<Impl, IMPL_OFFSET>,
            SetSuppressDefaults::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            SetClientId::<Impl, IMPL_OFFSET>,
            CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromPrivateKey::<Impl, IMPL_OFFSET>,
            InitializeFromPublicKey::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            CheckSignature::<Impl, IMPL_OFFSET>,
            IsSmartCard::<Impl, IMPL_OFFSET>,
            TemplateObjectId::<Impl, IMPL_OFFSET>,
            PublicKey::<Impl, IMPL_OFFSET>,
            PrivateKey::<Impl, IMPL_OFFSET>,
            NullSigned::<Impl, IMPL_OFFSET>,
            ReuseKey::<Impl, IMPL_OFFSET>,
            OldCertificate::<Impl, IMPL_OFFSET>,
            Subject::<Impl, IMPL_OFFSET>,
            SetSubject::<Impl, IMPL_OFFSET>,
            CspStatuses::<Impl, IMPL_OFFSET>,
            SmimeCapabilities::<Impl, IMPL_OFFSET>,
            SetSmimeCapabilities::<Impl, IMPL_OFFSET>,
            SignatureInformation::<Impl, IMPL_OFFSET>,
            KeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            SetKeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            CryptAttributes::<Impl, IMPL_OFFSET>,
            X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions::<Impl, IMPL_OFFSET>,
            SuppressOids::<Impl, IMPL_OFFSET>,
            RawDataToBeSigned::<Impl, IMPL_OFFSET>,
            Signature::<Impl, IMPL_OFFSET>,
            GetCspStatuses::<Impl, IMPL_OFFSET>,
            CheckPublicKeySignature::<Impl, IMPL_OFFSET>,
            Issuer::<Impl, IMPL_OFFSET>,
            SetIssuer::<Impl, IMPL_OFFSET>,
            NotBefore::<Impl, IMPL_OFFSET>,
            SetNotBefore::<Impl, IMPL_OFFSET>,
            NotAfter::<Impl, IMPL_OFFSET>,
            SetNotAfter::<Impl, IMPL_OFFSET>,
            SerialNumber::<Impl, IMPL_OFFSET>,
            SetSerialNumber::<Impl, IMPL_OFFSET>,
            SignerCertificate::<Impl, IMPL_OFFSET>,
            SetSignerCertificate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestCertificate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestCertificate2Impl: Sized + IX509CertificateRequestCertificateImpl + IX509CertificateRequestPkcs10Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromTemplate();
    fn InitializeFromPrivateKeyTemplate();
    fn PolicyServer();
    fn Template();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestCertificate2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestCertificate2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestCertificate2Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Impl: IX509CertificateRequestCertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromPrivateKeyTemplate<Impl: IX509CertificateRequestCertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, pprivatekey: ::windows::core::RawPtr, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PolicyServer<Impl: IX509CertificateRequestCertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppolicyserver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Template<Impl: IX509CertificateRequestCertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode::<Impl, IMPL_OFFSET>,
            GetInnerRequest::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            EnrollmentContext::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SuppressDefaults::<Impl, IMPL_OFFSET>,
            SetSuppressDefaults::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            SetClientId::<Impl, IMPL_OFFSET>,
            CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromPrivateKey::<Impl, IMPL_OFFSET>,
            InitializeFromPublicKey::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            CheckSignature::<Impl, IMPL_OFFSET>,
            IsSmartCard::<Impl, IMPL_OFFSET>,
            TemplateObjectId::<Impl, IMPL_OFFSET>,
            PublicKey::<Impl, IMPL_OFFSET>,
            PrivateKey::<Impl, IMPL_OFFSET>,
            NullSigned::<Impl, IMPL_OFFSET>,
            ReuseKey::<Impl, IMPL_OFFSET>,
            OldCertificate::<Impl, IMPL_OFFSET>,
            Subject::<Impl, IMPL_OFFSET>,
            SetSubject::<Impl, IMPL_OFFSET>,
            CspStatuses::<Impl, IMPL_OFFSET>,
            SmimeCapabilities::<Impl, IMPL_OFFSET>,
            SetSmimeCapabilities::<Impl, IMPL_OFFSET>,
            SignatureInformation::<Impl, IMPL_OFFSET>,
            KeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            SetKeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            CryptAttributes::<Impl, IMPL_OFFSET>,
            X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions::<Impl, IMPL_OFFSET>,
            SuppressOids::<Impl, IMPL_OFFSET>,
            RawDataToBeSigned::<Impl, IMPL_OFFSET>,
            Signature::<Impl, IMPL_OFFSET>,
            GetCspStatuses::<Impl, IMPL_OFFSET>,
            CheckPublicKeySignature::<Impl, IMPL_OFFSET>,
            Issuer::<Impl, IMPL_OFFSET>,
            SetIssuer::<Impl, IMPL_OFFSET>,
            NotBefore::<Impl, IMPL_OFFSET>,
            SetNotBefore::<Impl, IMPL_OFFSET>,
            NotAfter::<Impl, IMPL_OFFSET>,
            SetNotAfter::<Impl, IMPL_OFFSET>,
            SerialNumber::<Impl, IMPL_OFFSET>,
            SetSerialNumber::<Impl, IMPL_OFFSET>,
            SignerCertificate::<Impl, IMPL_OFFSET>,
            SetSignerCertificate::<Impl, IMPL_OFFSET>,
            InitializeFromTemplate::<Impl, IMPL_OFFSET>,
            InitializeFromPrivateKeyTemplate::<Impl, IMPL_OFFSET>,
            PolicyServer::<Impl, IMPL_OFFSET>,
            Template::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestCertificate2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestCmcImpl: Sized + IX509CertificateRequestPkcs7Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromInnerRequestTemplateName();
    fn TemplateObjectId();
    fn NullSigned();
    fn CryptAttributes();
    fn NameValuePairs();
    fn X509Extensions();
    fn CriticalExtensions();
    fn SuppressOids();
    fn TransactionId();
    fn SetTransactionId();
    fn SenderNonce();
    fn SetSenderNonce();
    fn SignatureInformation();
    fn ArchivePrivateKey();
    fn SetArchivePrivateKey();
    fn KeyArchivalCertificate();
    fn SetKeyArchivalCertificate();
    fn EncryptionAlgorithm();
    fn SetEncryptionAlgorithm();
    fn EncryptionStrength();
    fn SetEncryptionStrength();
    fn EncryptedKeyHash();
    fn SignerCertificates();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestCmcVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestCmcImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestCmcVtbl {
        unsafe extern "system" fn InitializeFromInnerRequestTemplateName<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinnerrequest: ::windows::core::RawPtr, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TemplateObjectId<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NullSigned<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CryptAttributes<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NameValuePairs<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn X509Extensions<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CriticalExtensions<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SuppressOids<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionId<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransactionId<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderNonce<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSenderNonce<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SignatureInformation<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ArchivePrivateKey<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetArchivePrivateKey<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeyArchivalCertificate<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeyArchivalCertificate<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptionAlgorithm<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEncryptionAlgorithm<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptionStrength<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEncryptionStrength<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptedKeyHash<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SignerCertificates<Impl: IX509CertificateRequestCmcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode::<Impl, IMPL_OFFSET>,
            GetInnerRequest::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            EnrollmentContext::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SuppressDefaults::<Impl, IMPL_OFFSET>,
            SetSuppressDefaults::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            SetClientId::<Impl, IMPL_OFFSET>,
            CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeFromInnerRequest::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            RequesterName::<Impl, IMPL_OFFSET>,
            SetRequesterName::<Impl, IMPL_OFFSET>,
            SignerCertificate::<Impl, IMPL_OFFSET>,
            SetSignerCertificate::<Impl, IMPL_OFFSET>,
            InitializeFromInnerRequestTemplateName::<Impl, IMPL_OFFSET>,
            TemplateObjectId::<Impl, IMPL_OFFSET>,
            NullSigned::<Impl, IMPL_OFFSET>,
            CryptAttributes::<Impl, IMPL_OFFSET>,
            NameValuePairs::<Impl, IMPL_OFFSET>,
            X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions::<Impl, IMPL_OFFSET>,
            SuppressOids::<Impl, IMPL_OFFSET>,
            TransactionId::<Impl, IMPL_OFFSET>,
            SetTransactionId::<Impl, IMPL_OFFSET>,
            SenderNonce::<Impl, IMPL_OFFSET>,
            SetSenderNonce::<Impl, IMPL_OFFSET>,
            SignatureInformation::<Impl, IMPL_OFFSET>,
            ArchivePrivateKey::<Impl, IMPL_OFFSET>,
            SetArchivePrivateKey::<Impl, IMPL_OFFSET>,
            KeyArchivalCertificate::<Impl, IMPL_OFFSET>,
            SetKeyArchivalCertificate::<Impl, IMPL_OFFSET>,
            EncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptionStrength::<Impl, IMPL_OFFSET>,
            SetEncryptionStrength::<Impl, IMPL_OFFSET>,
            EncryptedKeyHash::<Impl, IMPL_OFFSET>,
            SignerCertificates::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestCmc as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestCmc2Impl: Sized + IX509CertificateRequestCmcImpl + IX509CertificateRequestPkcs7Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromTemplate();
    fn InitializeFromInnerRequestTemplate();
    fn PolicyServer();
    fn Template();
    fn CheckSignature();
    fn CheckCertificateSignature();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestCmc2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestCmc2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestCmc2Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Impl: IX509CertificateRequestCmc2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromInnerRequestTemplate<Impl: IX509CertificateRequestCmc2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinnerrequest: ::windows::core::RawPtr, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PolicyServer<Impl: IX509CertificateRequestCmc2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppolicyserver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Template<Impl: IX509CertificateRequestCmc2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckSignature<Impl: IX509CertificateRequestCmc2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckCertificateSignature<Impl: IX509CertificateRequestCmc2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignercertificate: ::windows::core::RawPtr, validatecertificatechain: i16) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode::<Impl, IMPL_OFFSET>,
            GetInnerRequest::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            EnrollmentContext::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SuppressDefaults::<Impl, IMPL_OFFSET>,
            SetSuppressDefaults::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            SetClientId::<Impl, IMPL_OFFSET>,
            CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeFromInnerRequest::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            RequesterName::<Impl, IMPL_OFFSET>,
            SetRequesterName::<Impl, IMPL_OFFSET>,
            SignerCertificate::<Impl, IMPL_OFFSET>,
            SetSignerCertificate::<Impl, IMPL_OFFSET>,
            InitializeFromInnerRequestTemplateName::<Impl, IMPL_OFFSET>,
            TemplateObjectId::<Impl, IMPL_OFFSET>,
            NullSigned::<Impl, IMPL_OFFSET>,
            CryptAttributes::<Impl, IMPL_OFFSET>,
            NameValuePairs::<Impl, IMPL_OFFSET>,
            X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions::<Impl, IMPL_OFFSET>,
            SuppressOids::<Impl, IMPL_OFFSET>,
            TransactionId::<Impl, IMPL_OFFSET>,
            SetTransactionId::<Impl, IMPL_OFFSET>,
            SenderNonce::<Impl, IMPL_OFFSET>,
            SetSenderNonce::<Impl, IMPL_OFFSET>,
            SignatureInformation::<Impl, IMPL_OFFSET>,
            ArchivePrivateKey::<Impl, IMPL_OFFSET>,
            SetArchivePrivateKey::<Impl, IMPL_OFFSET>,
            KeyArchivalCertificate::<Impl, IMPL_OFFSET>,
            SetKeyArchivalCertificate::<Impl, IMPL_OFFSET>,
            EncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptionStrength::<Impl, IMPL_OFFSET>,
            SetEncryptionStrength::<Impl, IMPL_OFFSET>,
            EncryptedKeyHash::<Impl, IMPL_OFFSET>,
            SignerCertificates::<Impl, IMPL_OFFSET>,
            InitializeFromTemplate::<Impl, IMPL_OFFSET>,
            InitializeFromInnerRequestTemplate::<Impl, IMPL_OFFSET>,
            PolicyServer::<Impl, IMPL_OFFSET>,
            Template::<Impl, IMPL_OFFSET>,
            CheckSignature::<Impl, IMPL_OFFSET>,
            CheckCertificateSignature::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestCmc2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestPkcs10Impl: Sized + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromTemplateName();
    fn InitializeFromPrivateKey();
    fn InitializeFromPublicKey();
    fn InitializeFromCertificate();
    fn InitializeDecode();
    fn CheckSignature();
    fn IsSmartCard();
    fn TemplateObjectId();
    fn PublicKey();
    fn PrivateKey();
    fn NullSigned();
    fn ReuseKey();
    fn OldCertificate();
    fn Subject();
    fn SetSubject();
    fn CspStatuses();
    fn SmimeCapabilities();
    fn SetSmimeCapabilities();
    fn SignatureInformation();
    fn KeyContainerNamePrefix();
    fn SetKeyContainerNamePrefix();
    fn CryptAttributes();
    fn X509Extensions();
    fn CriticalExtensions();
    fn SuppressOids();
    fn RawDataToBeSigned();
    fn Signature();
    fn GetCspStatuses();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestPkcs10Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestPkcs10Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestPkcs10Vtbl {
        unsafe extern "system" fn InitializeFromTemplateName<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromPrivateKey<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, pprivatekey: ::windows::core::RawPtr, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromPublicKey<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppublickey: ::windows::core::RawPtr, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromCertificate<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckSignature<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSmartCard<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TemplateObjectId<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PublicKey<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivateKey<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NullSigned<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReuseKey<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OldCertificate<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Subject<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSubject<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CspStatuses<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SmimeCapabilities<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSmimeCapabilities<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SignatureInformation<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeyContainerNamePrefix<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeyContainerNamePrefix<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CryptAttributes<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn X509Extensions<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CriticalExtensions<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SuppressOids<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RawDataToBeSigned<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Signature<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCspStatuses<Impl: IX509CertificateRequestPkcs10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keyspec: X509KeySpec, ppcspstatuses: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode::<Impl, IMPL_OFFSET>,
            GetInnerRequest::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            EnrollmentContext::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SuppressDefaults::<Impl, IMPL_OFFSET>,
            SetSuppressDefaults::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            SetClientId::<Impl, IMPL_OFFSET>,
            CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromPrivateKey::<Impl, IMPL_OFFSET>,
            InitializeFromPublicKey::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            CheckSignature::<Impl, IMPL_OFFSET>,
            IsSmartCard::<Impl, IMPL_OFFSET>,
            TemplateObjectId::<Impl, IMPL_OFFSET>,
            PublicKey::<Impl, IMPL_OFFSET>,
            PrivateKey::<Impl, IMPL_OFFSET>,
            NullSigned::<Impl, IMPL_OFFSET>,
            ReuseKey::<Impl, IMPL_OFFSET>,
            OldCertificate::<Impl, IMPL_OFFSET>,
            Subject::<Impl, IMPL_OFFSET>,
            SetSubject::<Impl, IMPL_OFFSET>,
            CspStatuses::<Impl, IMPL_OFFSET>,
            SmimeCapabilities::<Impl, IMPL_OFFSET>,
            SetSmimeCapabilities::<Impl, IMPL_OFFSET>,
            SignatureInformation::<Impl, IMPL_OFFSET>,
            KeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            SetKeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            CryptAttributes::<Impl, IMPL_OFFSET>,
            X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions::<Impl, IMPL_OFFSET>,
            SuppressOids::<Impl, IMPL_OFFSET>,
            RawDataToBeSigned::<Impl, IMPL_OFFSET>,
            Signature::<Impl, IMPL_OFFSET>,
            GetCspStatuses::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs10 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestPkcs10V2Impl: Sized + IX509CertificateRequestPkcs10Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromTemplate();
    fn InitializeFromPrivateKeyTemplate();
    fn InitializeFromPublicKeyTemplate();
    fn PolicyServer();
    fn Template();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestPkcs10V2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestPkcs10V2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestPkcs10V2Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Impl: IX509CertificateRequestPkcs10V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromPrivateKeyTemplate<Impl: IX509CertificateRequestPkcs10V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, pprivatekey: ::windows::core::RawPtr, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromPublicKeyTemplate<Impl: IX509CertificateRequestPkcs10V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppublickey: ::windows::core::RawPtr, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PolicyServer<Impl: IX509CertificateRequestPkcs10V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppolicyserver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Template<Impl: IX509CertificateRequestPkcs10V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode::<Impl, IMPL_OFFSET>,
            GetInnerRequest::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            EnrollmentContext::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SuppressDefaults::<Impl, IMPL_OFFSET>,
            SetSuppressDefaults::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            SetClientId::<Impl, IMPL_OFFSET>,
            CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromPrivateKey::<Impl, IMPL_OFFSET>,
            InitializeFromPublicKey::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            CheckSignature::<Impl, IMPL_OFFSET>,
            IsSmartCard::<Impl, IMPL_OFFSET>,
            TemplateObjectId::<Impl, IMPL_OFFSET>,
            PublicKey::<Impl, IMPL_OFFSET>,
            PrivateKey::<Impl, IMPL_OFFSET>,
            NullSigned::<Impl, IMPL_OFFSET>,
            ReuseKey::<Impl, IMPL_OFFSET>,
            OldCertificate::<Impl, IMPL_OFFSET>,
            Subject::<Impl, IMPL_OFFSET>,
            SetSubject::<Impl, IMPL_OFFSET>,
            CspStatuses::<Impl, IMPL_OFFSET>,
            SmimeCapabilities::<Impl, IMPL_OFFSET>,
            SetSmimeCapabilities::<Impl, IMPL_OFFSET>,
            SignatureInformation::<Impl, IMPL_OFFSET>,
            KeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            SetKeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            CryptAttributes::<Impl, IMPL_OFFSET>,
            X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions::<Impl, IMPL_OFFSET>,
            SuppressOids::<Impl, IMPL_OFFSET>,
            RawDataToBeSigned::<Impl, IMPL_OFFSET>,
            Signature::<Impl, IMPL_OFFSET>,
            GetCspStatuses::<Impl, IMPL_OFFSET>,
            InitializeFromTemplate::<Impl, IMPL_OFFSET>,
            InitializeFromPrivateKeyTemplate::<Impl, IMPL_OFFSET>,
            InitializeFromPublicKeyTemplate::<Impl, IMPL_OFFSET>,
            PolicyServer::<Impl, IMPL_OFFSET>,
            Template::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs10V2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestPkcs10V3Impl: Sized + IX509CertificateRequestPkcs10V2Impl + IX509CertificateRequestPkcs10Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn AttestPrivateKey();
    fn SetAttestPrivateKey();
    fn AttestationEncryptionCertificate();
    fn SetAttestationEncryptionCertificate();
    fn EncryptionAlgorithm();
    fn SetEncryptionAlgorithm();
    fn EncryptionStrength();
    fn SetEncryptionStrength();
    fn ChallengePassword();
    fn SetChallengePassword();
    fn NameValuePairs();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestPkcs10V3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestPkcs10V3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestPkcs10V3Vtbl {
        unsafe extern "system" fn AttestPrivateKey<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttestPrivateKey<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttestationEncryptionCertificate<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttestationEncryptionCertificate<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptionAlgorithm<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEncryptionAlgorithm<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptionStrength<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEncryptionStrength<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChallengePassword<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetChallengePassword<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NameValuePairs<Impl: IX509CertificateRequestPkcs10V3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode::<Impl, IMPL_OFFSET>,
            GetInnerRequest::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            EnrollmentContext::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SuppressDefaults::<Impl, IMPL_OFFSET>,
            SetSuppressDefaults::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            SetClientId::<Impl, IMPL_OFFSET>,
            CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromPrivateKey::<Impl, IMPL_OFFSET>,
            InitializeFromPublicKey::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            CheckSignature::<Impl, IMPL_OFFSET>,
            IsSmartCard::<Impl, IMPL_OFFSET>,
            TemplateObjectId::<Impl, IMPL_OFFSET>,
            PublicKey::<Impl, IMPL_OFFSET>,
            PrivateKey::<Impl, IMPL_OFFSET>,
            NullSigned::<Impl, IMPL_OFFSET>,
            ReuseKey::<Impl, IMPL_OFFSET>,
            OldCertificate::<Impl, IMPL_OFFSET>,
            Subject::<Impl, IMPL_OFFSET>,
            SetSubject::<Impl, IMPL_OFFSET>,
            CspStatuses::<Impl, IMPL_OFFSET>,
            SmimeCapabilities::<Impl, IMPL_OFFSET>,
            SetSmimeCapabilities::<Impl, IMPL_OFFSET>,
            SignatureInformation::<Impl, IMPL_OFFSET>,
            KeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            SetKeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            CryptAttributes::<Impl, IMPL_OFFSET>,
            X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions::<Impl, IMPL_OFFSET>,
            SuppressOids::<Impl, IMPL_OFFSET>,
            RawDataToBeSigned::<Impl, IMPL_OFFSET>,
            Signature::<Impl, IMPL_OFFSET>,
            GetCspStatuses::<Impl, IMPL_OFFSET>,
            InitializeFromTemplate::<Impl, IMPL_OFFSET>,
            InitializeFromPrivateKeyTemplate::<Impl, IMPL_OFFSET>,
            InitializeFromPublicKeyTemplate::<Impl, IMPL_OFFSET>,
            PolicyServer::<Impl, IMPL_OFFSET>,
            Template::<Impl, IMPL_OFFSET>,
            AttestPrivateKey::<Impl, IMPL_OFFSET>,
            SetAttestPrivateKey::<Impl, IMPL_OFFSET>,
            AttestationEncryptionCertificate::<Impl, IMPL_OFFSET>,
            SetAttestationEncryptionCertificate::<Impl, IMPL_OFFSET>,
            EncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptionStrength::<Impl, IMPL_OFFSET>,
            SetEncryptionStrength::<Impl, IMPL_OFFSET>,
            ChallengePassword::<Impl, IMPL_OFFSET>,
            SetChallengePassword::<Impl, IMPL_OFFSET>,
            NameValuePairs::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs10V3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestPkcs10V4Impl: Sized + IX509CertificateRequestPkcs10V3Impl + IX509CertificateRequestPkcs10V2Impl + IX509CertificateRequestPkcs10Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn ClaimType();
    fn SetClaimType();
    fn AttestPrivateKeyPreferred();
    fn SetAttestPrivateKeyPreferred();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestPkcs10V4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestPkcs10V4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestPkcs10V4Vtbl {
        unsafe extern "system" fn ClaimType<Impl: IX509CertificateRequestPkcs10V4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut KeyAttestationClaimType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClaimType<Impl: IX509CertificateRequestPkcs10V4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: KeyAttestationClaimType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttestPrivateKeyPreferred<Impl: IX509CertificateRequestPkcs10V4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttestPrivateKeyPreferred<Impl: IX509CertificateRequestPkcs10V4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode::<Impl, IMPL_OFFSET>,
            GetInnerRequest::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            EnrollmentContext::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SuppressDefaults::<Impl, IMPL_OFFSET>,
            SetSuppressDefaults::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            SetClientId::<Impl, IMPL_OFFSET>,
            CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromPrivateKey::<Impl, IMPL_OFFSET>,
            InitializeFromPublicKey::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            CheckSignature::<Impl, IMPL_OFFSET>,
            IsSmartCard::<Impl, IMPL_OFFSET>,
            TemplateObjectId::<Impl, IMPL_OFFSET>,
            PublicKey::<Impl, IMPL_OFFSET>,
            PrivateKey::<Impl, IMPL_OFFSET>,
            NullSigned::<Impl, IMPL_OFFSET>,
            ReuseKey::<Impl, IMPL_OFFSET>,
            OldCertificate::<Impl, IMPL_OFFSET>,
            Subject::<Impl, IMPL_OFFSET>,
            SetSubject::<Impl, IMPL_OFFSET>,
            CspStatuses::<Impl, IMPL_OFFSET>,
            SmimeCapabilities::<Impl, IMPL_OFFSET>,
            SetSmimeCapabilities::<Impl, IMPL_OFFSET>,
            SignatureInformation::<Impl, IMPL_OFFSET>,
            KeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            SetKeyContainerNamePrefix::<Impl, IMPL_OFFSET>,
            CryptAttributes::<Impl, IMPL_OFFSET>,
            X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions::<Impl, IMPL_OFFSET>,
            SuppressOids::<Impl, IMPL_OFFSET>,
            RawDataToBeSigned::<Impl, IMPL_OFFSET>,
            Signature::<Impl, IMPL_OFFSET>,
            GetCspStatuses::<Impl, IMPL_OFFSET>,
            InitializeFromTemplate::<Impl, IMPL_OFFSET>,
            InitializeFromPrivateKeyTemplate::<Impl, IMPL_OFFSET>,
            InitializeFromPublicKeyTemplate::<Impl, IMPL_OFFSET>,
            PolicyServer::<Impl, IMPL_OFFSET>,
            Template::<Impl, IMPL_OFFSET>,
            AttestPrivateKey::<Impl, IMPL_OFFSET>,
            SetAttestPrivateKey::<Impl, IMPL_OFFSET>,
            AttestationEncryptionCertificate::<Impl, IMPL_OFFSET>,
            SetAttestationEncryptionCertificate::<Impl, IMPL_OFFSET>,
            EncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptionAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptionStrength::<Impl, IMPL_OFFSET>,
            SetEncryptionStrength::<Impl, IMPL_OFFSET>,
            ChallengePassword::<Impl, IMPL_OFFSET>,
            SetChallengePassword::<Impl, IMPL_OFFSET>,
            NameValuePairs::<Impl, IMPL_OFFSET>,
            ClaimType::<Impl, IMPL_OFFSET>,
            SetClaimType::<Impl, IMPL_OFFSET>,
            AttestPrivateKeyPreferred::<Impl, IMPL_OFFSET>,
            SetAttestPrivateKeyPreferred::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs10V4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestPkcs7Impl: Sized + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromTemplateName();
    fn InitializeFromCertificate();
    fn InitializeFromInnerRequest();
    fn InitializeDecode();
    fn RequesterName();
    fn SetRequesterName();
    fn SignerCertificate();
    fn SetSignerCertificate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestPkcs7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestPkcs7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestPkcs7Vtbl {
        unsafe extern "system" fn InitializeFromTemplateName<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromCertificate<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, renewalrequest: i16, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromInnerRequest<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinnerrequest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequesterName<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRequesterName<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SignerCertificate<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignerCertificate<Impl: IX509CertificateRequestPkcs7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode::<Impl, IMPL_OFFSET>,
            GetInnerRequest::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            EnrollmentContext::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SuppressDefaults::<Impl, IMPL_OFFSET>,
            SetSuppressDefaults::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            SetClientId::<Impl, IMPL_OFFSET>,
            CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeFromInnerRequest::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            RequesterName::<Impl, IMPL_OFFSET>,
            SetRequesterName::<Impl, IMPL_OFFSET>,
            SignerCertificate::<Impl, IMPL_OFFSET>,
            SetSignerCertificate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRequestPkcs7V2Impl: Sized + IX509CertificateRequestPkcs7Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromTemplate();
    fn PolicyServer();
    fn Template();
    fn CheckCertificateSignature();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRequestPkcs7V2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRequestPkcs7V2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRequestPkcs7V2Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Impl: IX509CertificateRequestPkcs7V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PolicyServer<Impl: IX509CertificateRequestPkcs7V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppolicyserver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Template<Impl: IX509CertificateRequestPkcs7V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckCertificateSignature<Impl: IX509CertificateRequestPkcs7V2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, validatecertificatechain: i16) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode::<Impl, IMPL_OFFSET>,
            GetInnerRequest::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            EnrollmentContext::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SuppressDefaults::<Impl, IMPL_OFFSET>,
            SetSuppressDefaults::<Impl, IMPL_OFFSET>,
            RenewalCertificate::<Impl, IMPL_OFFSET>,
            SetRenewalCertificate::<Impl, IMPL_OFFSET>,
            ClientId::<Impl, IMPL_OFFSET>,
            SetClientId::<Impl, IMPL_OFFSET>,
            CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromCertificate::<Impl, IMPL_OFFSET>,
            InitializeFromInnerRequest::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            RequesterName::<Impl, IMPL_OFFSET>,
            SetRequesterName::<Impl, IMPL_OFFSET>,
            SignerCertificate::<Impl, IMPL_OFFSET>,
            SetSignerCertificate::<Impl, IMPL_OFFSET>,
            InitializeFromTemplate::<Impl, IMPL_OFFSET>,
            PolicyServer::<Impl, IMPL_OFFSET>,
            Template::<Impl, IMPL_OFFSET>,
            CheckCertificateSignature::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs7V2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRevocationListImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn InitializeDecode();
    fn Encode();
    fn ResetForEncode();
    fn CheckPublicKeySignature();
    fn CheckSignature();
    fn Issuer();
    fn SetIssuer();
    fn ThisUpdate();
    fn SetThisUpdate();
    fn NextUpdate();
    fn SetNextUpdate();
    fn X509CRLEntries();
    fn X509Extensions();
    fn CriticalExtensions();
    fn SignerCertificate();
    fn SetSignerCertificate();
    fn CRLNumber();
    fn SetCRLNumber();
    fn CAVersion();
    fn SetCAVersion();
    fn BaseCRL();
    fn NullSigned();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn AlternateSignatureAlgorithm();
    fn SetAlternateSignatureAlgorithm();
    fn SignatureInformation();
    fn RawData();
    fn RawDataToBeSigned();
    fn Signature();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRevocationListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRevocationListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRevocationListVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Encode<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetForEncode<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckPublicKeySignature<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppublickey: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckSignature<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Issuer<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIssuer<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ThisUpdate<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetThisUpdate<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NextUpdate<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNextUpdate<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn X509CRLEntries<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn X509Extensions<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CriticalExtensions<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SignerCertificate<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignerCertificate<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CRLNumber<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCRLNumber<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CAVersion<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCAVersion<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BaseCRL<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NullSigned<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AlternateSignatureAlgorithm<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlternateSignatureAlgorithm<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SignatureInformation<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RawData<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RawDataToBeSigned<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Signature<Impl: IX509CertificateRevocationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            Encode::<Impl, IMPL_OFFSET>,
            ResetForEncode::<Impl, IMPL_OFFSET>,
            CheckPublicKeySignature::<Impl, IMPL_OFFSET>,
            CheckSignature::<Impl, IMPL_OFFSET>,
            Issuer::<Impl, IMPL_OFFSET>,
            SetIssuer::<Impl, IMPL_OFFSET>,
            ThisUpdate::<Impl, IMPL_OFFSET>,
            SetThisUpdate::<Impl, IMPL_OFFSET>,
            NextUpdate::<Impl, IMPL_OFFSET>,
            SetNextUpdate::<Impl, IMPL_OFFSET>,
            X509CRLEntries::<Impl, IMPL_OFFSET>,
            X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions::<Impl, IMPL_OFFSET>,
            SignerCertificate::<Impl, IMPL_OFFSET>,
            SetSignerCertificate::<Impl, IMPL_OFFSET>,
            CRLNumber::<Impl, IMPL_OFFSET>,
            SetCRLNumber::<Impl, IMPL_OFFSET>,
            CAVersion::<Impl, IMPL_OFFSET>,
            SetCAVersion::<Impl, IMPL_OFFSET>,
            BaseCRL::<Impl, IMPL_OFFSET>,
            NullSigned::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SignatureInformation::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            RawDataToBeSigned::<Impl, IMPL_OFFSET>,
            Signature::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRevocationList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRevocationListEntriesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn IndexBySerialNumber();
    fn AddRange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRevocationListEntriesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRevocationListEntriesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRevocationListEntriesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IndexBySerialNumber<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, serialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRange<Impl: IX509CertificateRevocationListEntriesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            ItemByIndex::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            IndexBySerialNumber::<Impl, IMPL_OFFSET>,
            AddRange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRevocationListEntries as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateRevocationListEntryImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn SerialNumber();
    fn RevocationDate();
    fn RevocationReason();
    fn SetRevocationReason();
    fn X509Extensions();
    fn CriticalExtensions();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateRevocationListEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateRevocationListEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateRevocationListEntryVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, serialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, revocationdate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SerialNumber<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RevocationDate<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RevocationReason<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut CRLRevocationReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRevocationReason<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CRLRevocationReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn X509Extensions<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CriticalExtensions<Impl: IX509CertificateRevocationListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            SerialNumber::<Impl, IMPL_OFFSET>,
            RevocationDate::<Impl, IMPL_OFFSET>,
            RevocationReason::<Impl, IMPL_OFFSET>,
            SetRevocationReason::<Impl, IMPL_OFFSET>,
            X509Extensions::<Impl, IMPL_OFFSET>,
            CriticalExtensions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateRevocationListEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateTemplateImpl: Sized + IDispatchImpl {
    fn Property();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateTemplateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateTemplateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateTemplateVtbl {
        unsafe extern "system" fn Property<Impl: IX509CertificateTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: EnrollmentTemplateProperty, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Property::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateTemplateWritableImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Commit();
    fn Property();
    fn SetProperty();
    fn Template();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateTemplateWritableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateTemplateWritableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateTemplateWritableVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509CertificateTemplateWritableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: IX509CertificateTemplateWritableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commitflags: CommitTemplateFlags, strservercontext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Property<Impl: IX509CertificateTemplateWritableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: EnrollmentTemplateProperty, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IX509CertificateTemplateWritableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: EnrollmentTemplateProperty, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Template<Impl: IX509CertificateTemplateWritableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Commit::<Impl, IMPL_OFFSET>, Property::<Impl, IMPL_OFFSET>, SetProperty::<Impl, IMPL_OFFSET>, Template::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateTemplateWritable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509CertificateTemplatesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn ItemByName();
    fn ItemByOid();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509CertificateTemplatesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509CertificateTemplatesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509CertificateTemplatesVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemByName<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemByOid<Impl: IX509CertificateTemplatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poid: ::windows::core::RawPtr, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            ItemByIndex::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            ItemByName::<Impl, IMPL_OFFSET>,
            ItemByOid::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509CertificateTemplates as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509EndorsementKeyImpl: Sized + IDispatchImpl {
    fn ProviderName();
    fn SetProviderName();
    fn Length();
    fn Opened();
    fn AddCertificate();
    fn RemoveCertificate();
    fn GetCertificateByIndex();
    fn GetCertificateCount();
    fn ExportPublicKey();
    fn Open();
    fn Close();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509EndorsementKeyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509EndorsementKeyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509EndorsementKeyVtbl {
        unsafe extern "system" fn ProviderName<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderName<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Length<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Opened<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddCertificate<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveCertificate<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateByIndex<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manufactureronly: i16, dwindex: i32, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateCount<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manufactureronly: i16, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExportPublicKey<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppublickey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Open<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IX509EndorsementKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Opened::<Impl, IMPL_OFFSET>,
            AddCertificate::<Impl, IMPL_OFFSET>,
            RemoveCertificate::<Impl, IMPL_OFFSET>,
            GetCertificateByIndex::<Impl, IMPL_OFFSET>,
            GetCertificateCount::<Impl, IMPL_OFFSET>,
            ExportPublicKey::<Impl, IMPL_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509EndorsementKey as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509EnrollmentImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn InitializeFromTemplateName();
    fn InitializeFromRequest();
    fn CreateRequest();
    fn Enroll();
    fn InstallResponse();
    fn CreatePFX();
    fn Request();
    fn Silent();
    fn SetSilent();
    fn ParentWindow();
    fn SetParentWindow();
    fn NameValuePairs();
    fn EnrollmentContext();
    fn Status();
    fn Certificate();
    fn Response();
    fn CertificateFriendlyName();
    fn SetCertificateFriendlyName();
    fn CertificateDescription();
    fn SetCertificateDescription();
    fn RequestId();
    fn CAConfigString();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509EnrollmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509EnrollmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509EnrollmentVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromTemplateName<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromRequest<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRequest<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enroll<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallResponse<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictions: InstallResponseRestrictionFlags, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePFX<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, exportoptions: PFXExportOptions, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Request<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Silent<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSilent<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ParentWindow<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParentWindow<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NameValuePairs<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnrollmentContext<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Status<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Certificate<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Response<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CertificateFriendlyName<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCertificateFriendlyName<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CertificateDescription<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCertificateDescription<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestId<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CAConfigString<Impl: IX509EnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromRequest::<Impl, IMPL_OFFSET>,
            CreateRequest::<Impl, IMPL_OFFSET>,
            Enroll::<Impl, IMPL_OFFSET>,
            InstallResponse::<Impl, IMPL_OFFSET>,
            CreatePFX::<Impl, IMPL_OFFSET>,
            Request::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            NameValuePairs::<Impl, IMPL_OFFSET>,
            EnrollmentContext::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            Certificate::<Impl, IMPL_OFFSET>,
            Response::<Impl, IMPL_OFFSET>,
            CertificateFriendlyName::<Impl, IMPL_OFFSET>,
            SetCertificateFriendlyName::<Impl, IMPL_OFFSET>,
            CertificateDescription::<Impl, IMPL_OFFSET>,
            SetCertificateDescription::<Impl, IMPL_OFFSET>,
            RequestId::<Impl, IMPL_OFFSET>,
            CAConfigString::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509Enrollment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509Enrollment2Impl: Sized + IX509EnrollmentImpl + IDispatchImpl {
    fn InitializeFromTemplate();
    fn InstallResponse2();
    fn PolicyServer();
    fn Template();
    fn RequestIdString();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509Enrollment2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509Enrollment2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509Enrollment2Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Impl: IX509Enrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallResponse2<Impl: IX509Enrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictions: InstallResponseRestrictionFlags, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strenrollmentpolicyserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strenrollmentpolicyserverid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PolicyServer<Impl: IX509Enrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppolicyserver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Template<Impl: IX509Enrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestIdString<Impl: IX509Enrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            InitializeFromTemplateName::<Impl, IMPL_OFFSET>,
            InitializeFromRequest::<Impl, IMPL_OFFSET>,
            CreateRequest::<Impl, IMPL_OFFSET>,
            Enroll::<Impl, IMPL_OFFSET>,
            InstallResponse::<Impl, IMPL_OFFSET>,
            CreatePFX::<Impl, IMPL_OFFSET>,
            Request::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            NameValuePairs::<Impl, IMPL_OFFSET>,
            EnrollmentContext::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            Certificate::<Impl, IMPL_OFFSET>,
            Response::<Impl, IMPL_OFFSET>,
            CertificateFriendlyName::<Impl, IMPL_OFFSET>,
            SetCertificateFriendlyName::<Impl, IMPL_OFFSET>,
            CertificateDescription::<Impl, IMPL_OFFSET>,
            SetCertificateDescription::<Impl, IMPL_OFFSET>,
            RequestId::<Impl, IMPL_OFFSET>,
            CAConfigString::<Impl, IMPL_OFFSET>,
            InitializeFromTemplate::<Impl, IMPL_OFFSET>,
            InstallResponse2::<Impl, IMPL_OFFSET>,
            PolicyServer::<Impl, IMPL_OFFSET>,
            Template::<Impl, IMPL_OFFSET>,
            RequestIdString::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509Enrollment2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509EnrollmentHelperImpl: Sized + IDispatchImpl {
    fn AddPolicyServer();
    fn AddEnrollmentServer();
    fn Enroll();
    fn Initialize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509EnrollmentHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509EnrollmentHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509EnrollmentHelperVtbl {
        unsafe extern "system" fn AddPolicyServer<Impl: IX509EnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strenrollmentpolicyserveruri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strenrollmentpolicyid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags, strcredential: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEnrollmentServer<Impl: IX509EnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strenrollmentserveruri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, authflags: X509EnrollmentAuthFlags, strcredential: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enroll<Impl: IX509EnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strenrollmentpolicyserveruri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, enrollflags: WebEnrollmentFlags, pstrcertificate: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IX509EnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, AddPolicyServer::<Impl, IMPL_OFFSET>, AddEnrollmentServer::<Impl, IMPL_OFFSET>, Enroll::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509EnrollmentHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509EnrollmentPolicyServerImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn LoadPolicy();
    fn GetTemplates();
    fn GetCAsForTemplate();
    fn GetCAs();
    fn Validate();
    fn GetCustomOids();
    fn GetNextUpdateTime();
    fn GetLastUpdateTime();
    fn GetPolicyServerUrl();
    fn GetPolicyServerId();
    fn GetFriendlyName();
    fn GetIsDefaultCEP();
    fn GetUseClientId();
    fn GetAllowUnTrustedCA();
    fn GetCachePath();
    fn GetCacheDir();
    fn GetAuthFlags();
    fn SetCredential();
    fn QueryChanges();
    fn InitializeImport();
    fn Export();
    fn Cost();
    fn SetCost();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509EnrollmentPolicyServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509EnrollmentPolicyServerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509EnrollmentPolicyServerVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpolicyserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrpolicyserverid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, authflags: X509EnrollmentAuthFlags, fisuntrusted: i16, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadPolicy<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: X509EnrollmentPolicyLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTemplates<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCAsForTemplate<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptemplate: ::windows::core::RawPtr, ppcas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCAs<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Validate<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCustomOids<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjectids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextUpdateTime<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastUpdateTime<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPolicyServerUrl<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPolicyServerId<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFriendlyName<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIsDefaultCEP<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUseClientId<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllowUnTrustedCA<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachePath<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCacheDir<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAuthFlags<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509EnrollmentAuthFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCredential<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: i32, flag: X509EnrollmentAuthFlags, strcredential: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryChanges<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeImport<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Export<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, exportflags: X509EnrollmentPolicyExportFlags, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cost<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCost<Impl: IX509EnrollmentPolicyServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            LoadPolicy::<Impl, IMPL_OFFSET>,
            GetTemplates::<Impl, IMPL_OFFSET>,
            GetCAsForTemplate::<Impl, IMPL_OFFSET>,
            GetCAs::<Impl, IMPL_OFFSET>,
            Validate::<Impl, IMPL_OFFSET>,
            GetCustomOids::<Impl, IMPL_OFFSET>,
            GetNextUpdateTime::<Impl, IMPL_OFFSET>,
            GetLastUpdateTime::<Impl, IMPL_OFFSET>,
            GetPolicyServerUrl::<Impl, IMPL_OFFSET>,
            GetPolicyServerId::<Impl, IMPL_OFFSET>,
            GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetIsDefaultCEP::<Impl, IMPL_OFFSET>,
            GetUseClientId::<Impl, IMPL_OFFSET>,
            GetAllowUnTrustedCA::<Impl, IMPL_OFFSET>,
            GetCachePath::<Impl, IMPL_OFFSET>,
            GetCacheDir::<Impl, IMPL_OFFSET>,
            GetAuthFlags::<Impl, IMPL_OFFSET>,
            SetCredential::<Impl, IMPL_OFFSET>,
            QueryChanges::<Impl, IMPL_OFFSET>,
            InitializeImport::<Impl, IMPL_OFFSET>,
            Export::<Impl, IMPL_OFFSET>,
            Cost::<Impl, IMPL_OFFSET>,
            SetCost::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509EnrollmentPolicyServer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509EnrollmentStatusImpl: Sized + IDispatchImpl {
    fn AppendText();
    fn Text();
    fn SetText();
    fn Selected();
    fn SetSelected();
    fn Display();
    fn SetDisplay();
    fn Status();
    fn SetStatus();
    fn Error();
    fn SetError();
    fn ErrorText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509EnrollmentStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509EnrollmentStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509EnrollmentStatusVtbl {
        unsafe extern "system" fn AppendText<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strtext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Text<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetText<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Selected<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut EnrollmentSelectionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelected<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EnrollmentSelectionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Display<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut EnrollmentDisplayStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplay<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EnrollmentDisplayStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Status<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut EnrollmentEnrollStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatus<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EnrollmentEnrollStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Error<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetError<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ErrorText<Impl: IX509EnrollmentStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            AppendText::<Impl, IMPL_OFFSET>,
            Text::<Impl, IMPL_OFFSET>,
            SetText::<Impl, IMPL_OFFSET>,
            Selected::<Impl, IMPL_OFFSET>,
            SetSelected::<Impl, IMPL_OFFSET>,
            Display::<Impl, IMPL_OFFSET>,
            SetDisplay::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            SetStatus::<Impl, IMPL_OFFSET>,
            Error::<Impl, IMPL_OFFSET>,
            SetError::<Impl, IMPL_OFFSET>,
            ErrorText::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509EnrollmentStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509EnrollmentWebClassFactoryImpl: Sized + IDispatchImpl {
    fn CreateObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509EnrollmentWebClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509EnrollmentWebClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509EnrollmentWebClassFactoryVtbl {
        unsafe extern "system" fn CreateObject<Impl: IX509EnrollmentWebClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprogid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppiunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, CreateObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509EnrollmentWebClassFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn ObjectId();
    fn RawData();
    fn Critical();
    fn SetCritical();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ObjectId<Impl: IX509ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RawData<Impl: IX509ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Critical<Impl: IX509ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCritical<Impl: IX509ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, ObjectId::<Impl, IMPL_OFFSET>, RawData::<Impl, IMPL_OFFSET>, Critical::<Impl, IMPL_OFFSET>, SetCritical::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509Extension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionAlternativeNamesImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn AlternativeNames();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionAlternativeNamesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionAlternativeNamesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionAlternativeNamesVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AlternativeNames<Impl: IX509ExtensionAlternativeNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            Critical::<Impl, IMPL_OFFSET>,
            SetCritical::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            AlternativeNames::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionAlternativeNames as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionAuthorityKeyIdentifierImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn AuthorityKeyIdentifier();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionAuthorityKeyIdentifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionAuthorityKeyIdentifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionAuthorityKeyIdentifierVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionAuthorityKeyIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strkeyidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionAuthorityKeyIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthorityKeyIdentifier<Impl: IX509ExtensionAuthorityKeyIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            Critical::<Impl, IMPL_OFFSET>,
            SetCritical::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            AuthorityKeyIdentifier::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionAuthorityKeyIdentifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionBasicConstraintsImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn IsCA();
    fn PathLenConstraint();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionBasicConstraintsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionBasicConstraintsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionBasicConstraintsVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionBasicConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isca: i16, pathlenconstraint: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionBasicConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCA<Impl: IX509ExtensionBasicConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PathLenConstraint<Impl: IX509ExtensionBasicConstraintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            Critical::<Impl, IMPL_OFFSET>,
            SetCritical::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            IsCA::<Impl, IMPL_OFFSET>,
            PathLenConstraint::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionBasicConstraints as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionCertificatePoliciesImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn Policies();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionCertificatePoliciesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionCertificatePoliciesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionCertificatePoliciesVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionCertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionCertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Policies<Impl: IX509ExtensionCertificatePoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            Critical::<Impl, IMPL_OFFSET>,
            SetCritical::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            Policies::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionCertificatePolicies as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionEnhancedKeyUsageImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn EnhancedKeyUsage();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionEnhancedKeyUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionEnhancedKeyUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionEnhancedKeyUsageVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionEnhancedKeyUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionEnhancedKeyUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnhancedKeyUsage<Impl: IX509ExtensionEnhancedKeyUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            Critical::<Impl, IMPL_OFFSET>,
            SetCritical::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            EnhancedKeyUsage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionEnhancedKeyUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionKeyUsageImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn KeyUsage();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionKeyUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionKeyUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionKeyUsageVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionKeyUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usageflags: X509KeyUsageFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionKeyUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeyUsage<Impl: IX509ExtensionKeyUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509KeyUsageFlags) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            Critical::<Impl, IMPL_OFFSET>,
            SetCritical::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            KeyUsage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionKeyUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionMSApplicationPoliciesImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn Policies();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionMSApplicationPoliciesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionMSApplicationPoliciesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionMSApplicationPoliciesVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionMSApplicationPoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionMSApplicationPoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Policies<Impl: IX509ExtensionMSApplicationPoliciesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            Critical::<Impl, IMPL_OFFSET>,
            SetCritical::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            Policies::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionMSApplicationPolicies as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionSmimeCapabilitiesImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn SmimeCapabilities();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionSmimeCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionSmimeCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionSmimeCapabilitiesVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionSmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionSmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SmimeCapabilities<Impl: IX509ExtensionSmimeCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            Critical::<Impl, IMPL_OFFSET>,
            SetCritical::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            SmimeCapabilities::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionSmimeCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionSubjectKeyIdentifierImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn SubjectKeyIdentifier();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionSubjectKeyIdentifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionSubjectKeyIdentifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionSubjectKeyIdentifierVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionSubjectKeyIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strkeyidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionSubjectKeyIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubjectKeyIdentifier<Impl: IX509ExtensionSubjectKeyIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            Critical::<Impl, IMPL_OFFSET>,
            SetCritical::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            SubjectKeyIdentifier::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionSubjectKeyIdentifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionTemplateImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn TemplateOid();
    fn MajorVersion();
    fn MinorVersion();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionTemplateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionTemplateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionTemplateVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptemplateoid: ::windows::core::RawPtr, majorversion: i32, minorversion: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TemplateOid<Impl: IX509ExtensionTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MajorVersion<Impl: IX509ExtensionTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MinorVersion<Impl: IX509ExtensionTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            Critical::<Impl, IMPL_OFFSET>,
            SetCritical::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            TemplateOid::<Impl, IMPL_OFFSET>,
            MajorVersion::<Impl, IMPL_OFFSET>,
            MinorVersion::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionTemplateNameImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn TemplateName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionTemplateNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionTemplateNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionTemplateNameVtbl {
        unsafe extern "system" fn InitializeEncode<Impl: IX509ExtensionTemplateNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDecode<Impl: IX509ExtensionTemplateNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TemplateName<Impl: IX509ExtensionTemplateNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            ObjectId::<Impl, IMPL_OFFSET>,
            RawData::<Impl, IMPL_OFFSET>,
            Critical::<Impl, IMPL_OFFSET>,
            SetCritical::<Impl, IMPL_OFFSET>,
            InitializeEncode::<Impl, IMPL_OFFSET>,
            InitializeDecode::<Impl, IMPL_OFFSET>,
            TemplateName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509ExtensionTemplateName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509ExtensionsImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn IndexByObjectId();
    fn AddRange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509ExtensionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509ExtensionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509ExtensionsVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IndexByObjectId<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRange<Impl: IX509ExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            ItemByIndex::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            IndexByObjectId::<Impl, IMPL_OFFSET>,
            AddRange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509Extensions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509MachineEnrollmentFactoryImpl: Sized + IDispatchImpl {
    fn CreateObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509MachineEnrollmentFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509MachineEnrollmentFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509MachineEnrollmentFactoryVtbl {
        unsafe extern "system" fn CreateObject<Impl: IX509MachineEnrollmentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprogid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppihelper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, CreateObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509MachineEnrollmentFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509NameValuePairImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Value();
    fn Name();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509NameValuePairVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509NameValuePairImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509NameValuePairVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509NameValuePairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: IX509NameValuePairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IX509NameValuePairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509NameValuePair as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509NameValuePairsImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509NameValuePairsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509NameValuePairsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509NameValuePairsVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IX509NameValuePairsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IX509NameValuePairsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IX509NameValuePairsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IX509NameValuePairsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IX509NameValuePairsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IX509NameValuePairsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, ItemByIndex::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509NameValuePairs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509PolicyServerListManagerImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn Initialize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509PolicyServerListManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509PolicyServerListManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509PolicyServerListManagerVtbl {
        unsafe extern "system" fn ItemByIndex<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IX509PolicyServerListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, flags: PolicyServerUrlFlags) -> ::windows::core::HRESULT {
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
            ItemByIndex::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509PolicyServerListManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509PolicyServerUrlImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Url();
    fn SetUrl();
    fn Default();
    fn SetDefault();
    fn Flags();
    fn SetFlags();
    fn AuthFlags();
    fn SetAuthFlags();
    fn Cost();
    fn SetCost();
    fn GetStringProperty();
    fn SetStringProperty();
    fn UpdateRegistry();
    fn RemoveFromRegistry();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509PolicyServerUrlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509PolicyServerUrlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509PolicyServerUrlVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Url<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUrl<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Default<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefault<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flags<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut PolicyServerUrlFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFlags<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: PolicyServerUrlFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthFlags<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509EnrollmentAuthFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthFlags<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: X509EnrollmentAuthFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cost<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCost<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStringProperty<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: PolicyServerUrlPropertyID, ppvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStringProperty<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: PolicyServerUrlPropertyID, pvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateRegistry<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveFromRegistry<Impl: IX509PolicyServerUrlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Url::<Impl, IMPL_OFFSET>,
            SetUrl::<Impl, IMPL_OFFSET>,
            Default::<Impl, IMPL_OFFSET>,
            SetDefault::<Impl, IMPL_OFFSET>,
            Flags::<Impl, IMPL_OFFSET>,
            SetFlags::<Impl, IMPL_OFFSET>,
            AuthFlags::<Impl, IMPL_OFFSET>,
            SetAuthFlags::<Impl, IMPL_OFFSET>,
            Cost::<Impl, IMPL_OFFSET>,
            SetCost::<Impl, IMPL_OFFSET>,
            GetStringProperty::<Impl, IMPL_OFFSET>,
            SetStringProperty::<Impl, IMPL_OFFSET>,
            UpdateRegistry::<Impl, IMPL_OFFSET>,
            RemoveFromRegistry::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509PolicyServerUrl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509PrivateKeyImpl: Sized + IDispatchImpl {
    fn Open();
    fn Create();
    fn Close();
    fn Delete();
    fn Verify();
    fn Import();
    fn Export();
    fn ExportPublicKey();
    fn ContainerName();
    fn SetContainerName();
    fn ContainerNamePrefix();
    fn SetContainerNamePrefix();
    fn ReaderName();
    fn SetReaderName();
    fn CspInformations();
    fn SetCspInformations();
    fn CspStatus();
    fn SetCspStatus();
    fn ProviderName();
    fn SetProviderName();
    fn ProviderType();
    fn SetProviderType();
    fn LegacyCsp();
    fn SetLegacyCsp();
    fn Algorithm();
    fn SetAlgorithm();
    fn KeySpec();
    fn SetKeySpec();
    fn Length();
    fn SetLength();
    fn ExportPolicy();
    fn SetExportPolicy();
    fn KeyUsage();
    fn SetKeyUsage();
    fn KeyProtection();
    fn SetKeyProtection();
    fn MachineContext();
    fn SetMachineContext();
    fn SecurityDescriptor();
    fn SetSecurityDescriptor();
    fn Certificate();
    fn SetCertificate();
    fn UniqueContainerName();
    fn Opened();
    fn DefaultContainer();
    fn Existing();
    fn SetExisting();
    fn Silent();
    fn SetSilent();
    fn ParentWindow();
    fn SetParentWindow();
    fn UIContextMessage();
    fn SetUIContextMessage();
    fn SetPin();
    fn FriendlyName();
    fn SetFriendlyName();
    fn Description();
    fn SetDescription();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509PrivateKeyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509PrivateKeyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509PrivateKeyVtbl {
        unsafe extern "system" fn Open<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Verify<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, verifytype: X509PrivateKeyVerify) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Import<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strexporttype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strencodedkey: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Export<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strexporttype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, pstrencodedkey: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExportPublicKey<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppublickey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContainerName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContainerName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContainerNamePrefix<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContainerNamePrefix<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReaderName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReaderName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CspInformations<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCspInformations<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CspStatus<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCspStatus<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderType<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509ProviderType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderType<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509ProviderType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LegacyCsp<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLegacyCsp<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Algorithm<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlgorithm<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeySpec<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509KeySpec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeySpec<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509KeySpec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Length<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLength<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExportPolicy<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509PrivateKeyExportFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExportPolicy<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509PrivateKeyExportFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeyUsage<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509PrivateKeyUsageFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeyUsage<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509PrivateKeyUsageFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeyProtection<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509PrivateKeyProtection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeyProtection<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509PrivateKeyProtection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MachineContext<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMachineContext<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SecurityDescriptor<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Certificate<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCertificate<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UniqueContainerName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Opened<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultContainer<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Existing<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExisting<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Silent<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSilent<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ParentWindow<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParentWindow<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UIContextMessage<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUIContextMessage<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPin<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FriendlyName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFriendlyName<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IX509PrivateKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Open::<Impl, IMPL_OFFSET>,
            Create::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Verify::<Impl, IMPL_OFFSET>,
            Import::<Impl, IMPL_OFFSET>,
            Export::<Impl, IMPL_OFFSET>,
            ExportPublicKey::<Impl, IMPL_OFFSET>,
            ContainerName::<Impl, IMPL_OFFSET>,
            SetContainerName::<Impl, IMPL_OFFSET>,
            ContainerNamePrefix::<Impl, IMPL_OFFSET>,
            SetContainerNamePrefix::<Impl, IMPL_OFFSET>,
            ReaderName::<Impl, IMPL_OFFSET>,
            SetReaderName::<Impl, IMPL_OFFSET>,
            CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations::<Impl, IMPL_OFFSET>,
            CspStatus::<Impl, IMPL_OFFSET>,
            SetCspStatus::<Impl, IMPL_OFFSET>,
            ProviderName::<Impl, IMPL_OFFSET>,
            SetProviderName::<Impl, IMPL_OFFSET>,
            ProviderType::<Impl, IMPL_OFFSET>,
            SetProviderType::<Impl, IMPL_OFFSET>,
            LegacyCsp::<Impl, IMPL_OFFSET>,
            SetLegacyCsp::<Impl, IMPL_OFFSET>,
            Algorithm::<Impl, IMPL_OFFSET>,
            SetAlgorithm::<Impl, IMPL_OFFSET>,
            KeySpec::<Impl, IMPL_OFFSET>,
            SetKeySpec::<Impl, IMPL_OFFSET>,
            Length::<Impl, IMPL_OFFSET>,
            SetLength::<Impl, IMPL_OFFSET>,
            ExportPolicy::<Impl, IMPL_OFFSET>,
            SetExportPolicy::<Impl, IMPL_OFFSET>,
            KeyUsage::<Impl, IMPL_OFFSET>,
            SetKeyUsage::<Impl, IMPL_OFFSET>,
            KeyProtection::<Impl, IMPL_OFFSET>,
            SetKeyProtection::<Impl, IMPL_OFFSET>,
            MachineContext::<Impl, IMPL_OFFSET>,
            SetMachineContext::<Impl, IMPL_OFFSET>,
            SecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            Certificate::<Impl, IMPL_OFFSET>,
            SetCertificate::<Impl, IMPL_OFFSET>,
            UniqueContainerName::<Impl, IMPL_OFFSET>,
            Opened::<Impl, IMPL_OFFSET>,
            DefaultContainer::<Impl, IMPL_OFFSET>,
            Existing::<Impl, IMPL_OFFSET>,
            SetExisting::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SetPin::<Impl, IMPL_OFFSET>,
            FriendlyName::<Impl, IMPL_OFFSET>,
            SetFriendlyName::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509PrivateKey as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509PrivateKey2Impl: Sized + IX509PrivateKeyImpl + IDispatchImpl {
    fn HardwareKeyUsage();
    fn SetHardwareKeyUsage();
    fn AlternateStorageLocation();
    fn SetAlternateStorageLocation();
    fn AlgorithmName();
    fn SetAlgorithmName();
    fn AlgorithmParameters();
    fn SetAlgorithmParameters();
    fn ParametersExportType();
    fn SetParametersExportType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509PrivateKey2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509PrivateKey2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509PrivateKey2Vtbl {
        unsafe extern "system" fn HardwareKeyUsage<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509HardwareKeyUsageFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHardwareKeyUsage<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509HardwareKeyUsageFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AlternateStorageLocation<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlternateStorageLocation<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AlgorithmName<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlgorithmName<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AlgorithmParameters<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlgorithmParameters<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ParametersExportType<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509KeyParametersExportType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParametersExportType<Impl: IX509PrivateKey2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: X509KeyParametersExportType) -> ::windows::core::HRESULT {
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
            Open::<Impl, IMPL_OFFSET>,
            Create::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Verify::<Impl, IMPL_OFFSET>,
            Import::<Impl, IMPL_OFFSET>,
            Export::<Impl, IMPL_OFFSET>,
            ExportPublicKey::<Impl, IMPL_OFFSET>,
            ContainerName::<Impl, IMPL_OFFSET>,
            SetContainerName::<Impl, IMPL_OFFSET>,
            ContainerNamePrefix::<Impl, IMPL_OFFSET>,
            SetContainerNamePrefix::<Impl, IMPL_OFFSET>,
            ReaderName::<Impl, IMPL_OFFSET>,
            SetReaderName::<Impl, IMPL_OFFSET>,
            CspInformations::<Impl, IMPL_OFFSET>,
            SetCspInformations::<Impl, IMPL_OFFSET>,
            CspStatus::<Impl, IMPL_OFFSET>,
            SetCspStatus::<Impl, IMPL_OFFSET>,
            ProviderName::<Impl, IMPL_OFFSET>,
            SetProviderName::<Impl, IMPL_OFFSET>,
            ProviderType::<Impl, IMPL_OFFSET>,
            SetProviderType::<Impl, IMPL_OFFSET>,
            LegacyCsp::<Impl, IMPL_OFFSET>,
            SetLegacyCsp::<Impl, IMPL_OFFSET>,
            Algorithm::<Impl, IMPL_OFFSET>,
            SetAlgorithm::<Impl, IMPL_OFFSET>,
            KeySpec::<Impl, IMPL_OFFSET>,
            SetKeySpec::<Impl, IMPL_OFFSET>,
            Length::<Impl, IMPL_OFFSET>,
            SetLength::<Impl, IMPL_OFFSET>,
            ExportPolicy::<Impl, IMPL_OFFSET>,
            SetExportPolicy::<Impl, IMPL_OFFSET>,
            KeyUsage::<Impl, IMPL_OFFSET>,
            SetKeyUsage::<Impl, IMPL_OFFSET>,
            KeyProtection::<Impl, IMPL_OFFSET>,
            SetKeyProtection::<Impl, IMPL_OFFSET>,
            MachineContext::<Impl, IMPL_OFFSET>,
            SetMachineContext::<Impl, IMPL_OFFSET>,
            SecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            Certificate::<Impl, IMPL_OFFSET>,
            SetCertificate::<Impl, IMPL_OFFSET>,
            UniqueContainerName::<Impl, IMPL_OFFSET>,
            Opened::<Impl, IMPL_OFFSET>,
            DefaultContainer::<Impl, IMPL_OFFSET>,
            Existing::<Impl, IMPL_OFFSET>,
            SetExisting::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            ParentWindow::<Impl, IMPL_OFFSET>,
            SetParentWindow::<Impl, IMPL_OFFSET>,
            UIContextMessage::<Impl, IMPL_OFFSET>,
            SetUIContextMessage::<Impl, IMPL_OFFSET>,
            SetPin::<Impl, IMPL_OFFSET>,
            FriendlyName::<Impl, IMPL_OFFSET>,
            SetFriendlyName::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            HardwareKeyUsage::<Impl, IMPL_OFFSET>,
            SetHardwareKeyUsage::<Impl, IMPL_OFFSET>,
            AlternateStorageLocation::<Impl, IMPL_OFFSET>,
            SetAlternateStorageLocation::<Impl, IMPL_OFFSET>,
            AlgorithmName::<Impl, IMPL_OFFSET>,
            SetAlgorithmName::<Impl, IMPL_OFFSET>,
            AlgorithmParameters::<Impl, IMPL_OFFSET>,
            SetAlgorithmParameters::<Impl, IMPL_OFFSET>,
            ParametersExportType::<Impl, IMPL_OFFSET>,
            SetParametersExportType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509PrivateKey2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509PublicKeyImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn InitializeFromEncodedPublicKeyInfo();
    fn Algorithm();
    fn Length();
    fn EncodedKey();
    fn EncodedParameters();
    fn ComputeKeyIdentifier();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509PublicKeyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509PublicKeyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509PublicKeyVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows::core::RawPtr, strencodedkey: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strencodedparameters: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeFromEncodedPublicKeyInfo<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencodedpublickeyinfo: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Algorithm<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Length<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncodedKey<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncodedParameters<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputeKeyIdentifier<Impl: IX509PublicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, algorithm: KeyIdentifierHashAlgorithm, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            InitializeFromEncodedPublicKeyInfo::<Impl, IMPL_OFFSET>,
            Algorithm::<Impl, IMPL_OFFSET>,
            Length::<Impl, IMPL_OFFSET>,
            EncodedKey::<Impl, IMPL_OFFSET>,
            EncodedParameters::<Impl, IMPL_OFFSET>,
            ComputeKeyIdentifier::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509PublicKey as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509SCEPEnrollmentImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn InitializeForPending();
    fn CreateRequestMessage();
    fn CreateRetrievePendingMessage();
    fn CreateRetrieveCertificateMessage();
    fn ProcessResponseMessage();
    fn SetServerCapabilities();
    fn FailInfo();
    fn SignerCertificate();
    fn SetSignerCertificate();
    fn OldCertificate();
    fn SetOldCertificate();
    fn TransactionId();
    fn SetTransactionId();
    fn Request();
    fn CertificateFriendlyName();
    fn SetCertificateFriendlyName();
    fn Status();
    fn Certificate();
    fn Silent();
    fn SetSilent();
    fn DeleteRequest();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509SCEPEnrollmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509SCEPEnrollmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509SCEPEnrollmentVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: ::windows::core::RawPtr, strthumbprint: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, thumprintencoding: EncodingType, strservercertificates: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeForPending<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRequestMessage<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRetrievePendingMessage<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRetrieveCertificateMessage<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: X509CertificateEnrollmentContext, strissuer: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, issuerencoding: EncodingType, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, serialnumberencoding: EncodingType, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessResponseMessage<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, pdisposition: *mut X509SCEPDisposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServerCapabilities<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FailInfo<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut X509SCEPFailInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SignerCertificate<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignerCertificate<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OldCertificate<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOldCertificate<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionId<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransactionId<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Request<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CertificateFriendlyName<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCertificateFriendlyName<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Status<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Certificate<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Silent<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSilent<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteRequest<Impl: IX509SCEPEnrollmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            InitializeForPending::<Impl, IMPL_OFFSET>,
            CreateRequestMessage::<Impl, IMPL_OFFSET>,
            CreateRetrievePendingMessage::<Impl, IMPL_OFFSET>,
            CreateRetrieveCertificateMessage::<Impl, IMPL_OFFSET>,
            ProcessResponseMessage::<Impl, IMPL_OFFSET>,
            SetServerCapabilities::<Impl, IMPL_OFFSET>,
            FailInfo::<Impl, IMPL_OFFSET>,
            SignerCertificate::<Impl, IMPL_OFFSET>,
            SetSignerCertificate::<Impl, IMPL_OFFSET>,
            OldCertificate::<Impl, IMPL_OFFSET>,
            SetOldCertificate::<Impl, IMPL_OFFSET>,
            TransactionId::<Impl, IMPL_OFFSET>,
            SetTransactionId::<Impl, IMPL_OFFSET>,
            Request::<Impl, IMPL_OFFSET>,
            CertificateFriendlyName::<Impl, IMPL_OFFSET>,
            SetCertificateFriendlyName::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            Certificate::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            DeleteRequest::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509SCEPEnrollment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509SCEPEnrollment2Impl: Sized + IX509SCEPEnrollmentImpl + IDispatchImpl {
    fn CreateChallengeAnswerMessage();
    fn ProcessResponseMessage2();
    fn ResultMessageText();
    fn DelayRetry();
    fn ActivityId();
    fn SetActivityId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509SCEPEnrollment2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509SCEPEnrollment2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509SCEPEnrollment2Vtbl {
        unsafe extern "system" fn CreateChallengeAnswerMessage<Impl: IX509SCEPEnrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessResponseMessage2<Impl: IX509SCEPEnrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: X509SCEPProcessMessageFlags, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, pdisposition: *mut X509SCEPDisposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResultMessageText<Impl: IX509SCEPEnrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DelayRetry<Impl: IX509SCEPEnrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut DelayRetryAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ActivityId<Impl: IX509SCEPEnrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActivityId<Impl: IX509SCEPEnrollment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            InitializeForPending::<Impl, IMPL_OFFSET>,
            CreateRequestMessage::<Impl, IMPL_OFFSET>,
            CreateRetrievePendingMessage::<Impl, IMPL_OFFSET>,
            CreateRetrieveCertificateMessage::<Impl, IMPL_OFFSET>,
            ProcessResponseMessage::<Impl, IMPL_OFFSET>,
            SetServerCapabilities::<Impl, IMPL_OFFSET>,
            FailInfo::<Impl, IMPL_OFFSET>,
            SignerCertificate::<Impl, IMPL_OFFSET>,
            SetSignerCertificate::<Impl, IMPL_OFFSET>,
            OldCertificate::<Impl, IMPL_OFFSET>,
            SetOldCertificate::<Impl, IMPL_OFFSET>,
            TransactionId::<Impl, IMPL_OFFSET>,
            SetTransactionId::<Impl, IMPL_OFFSET>,
            Request::<Impl, IMPL_OFFSET>,
            CertificateFriendlyName::<Impl, IMPL_OFFSET>,
            SetCertificateFriendlyName::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            Certificate::<Impl, IMPL_OFFSET>,
            Silent::<Impl, IMPL_OFFSET>,
            SetSilent::<Impl, IMPL_OFFSET>,
            DeleteRequest::<Impl, IMPL_OFFSET>,
            CreateChallengeAnswerMessage::<Impl, IMPL_OFFSET>,
            ProcessResponseMessage2::<Impl, IMPL_OFFSET>,
            ResultMessageText::<Impl, IMPL_OFFSET>,
            DelayRetry::<Impl, IMPL_OFFSET>,
            ActivityId::<Impl, IMPL_OFFSET>,
            SetActivityId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509SCEPEnrollment2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509SCEPEnrollmentHelperImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn InitializeForPending();
    fn Enroll();
    fn FetchPending();
    fn X509SCEPEnrollment();
    fn ResultMessageText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509SCEPEnrollmentHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509SCEPEnrollmentHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509SCEPEnrollmentHelperVtbl {
        unsafe extern "system" fn Initialize<Impl: IX509SCEPEnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strrequestheaders: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, prequest: ::windows::core::RawPtr, strcacertificatethumbprint: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeForPending<Impl: IX509SCEPEnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strrequestheaders: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, context: X509CertificateEnrollmentContext, strtransactionid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enroll<Impl: IX509SCEPEnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processflags: X509SCEPProcessMessageFlags, pdisposition: *mut X509SCEPDisposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FetchPending<Impl: IX509SCEPEnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processflags: X509SCEPProcessMessageFlags, pdisposition: *mut X509SCEPDisposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn X509SCEPEnrollment<Impl: IX509SCEPEnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResultMessageText<Impl: IX509SCEPEnrollmentHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            InitializeForPending::<Impl, IMPL_OFFSET>,
            Enroll::<Impl, IMPL_OFFSET>,
            FetchPending::<Impl, IMPL_OFFSET>,
            X509SCEPEnrollment::<Impl, IMPL_OFFSET>,
            ResultMessageText::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509SCEPEnrollmentHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IX509SignatureInformationImpl: Sized + IDispatchImpl {
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn PublicKeyAlgorithm();
    fn SetPublicKeyAlgorithm();
    fn Parameters();
    fn SetParameters();
    fn AlternateSignatureAlgorithm();
    fn SetAlternateSignatureAlgorithm();
    fn AlternateSignatureAlgorithmSet();
    fn NullSigned();
    fn SetNullSigned();
    fn GetSignatureAlgorithm();
    fn SetDefaultValues();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IX509SignatureInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IX509SignatureInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IX509SignatureInformationVtbl {
        unsafe extern "system" fn HashAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PublicKeyAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPublicKeyAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parameters<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParameters<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AlternateSignatureAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlternateSignatureAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AlternateSignatureAlgorithmSet<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NullSigned<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNullSigned<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureAlgorithm<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkcs7signature: i16, signaturekey: i16, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultValues<Impl: IX509SignatureInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            PublicKeyAlgorithm::<Impl, IMPL_OFFSET>,
            SetPublicKeyAlgorithm::<Impl, IMPL_OFFSET>,
            Parameters::<Impl, IMPL_OFFSET>,
            SetParameters::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetAlternateSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            AlternateSignatureAlgorithmSet::<Impl, IMPL_OFFSET>,
            NullSigned::<Impl, IMPL_OFFSET>,
            SetNullSigned::<Impl, IMPL_OFFSET>,
            GetSignatureAlgorithm::<Impl, IMPL_OFFSET>,
            SetDefaultValues::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IX509SignatureInformation as ::windows::core::Interface>::IID
    }
}
