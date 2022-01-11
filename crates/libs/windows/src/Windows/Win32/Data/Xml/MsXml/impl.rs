#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXAttributesImpl: Sized + IDispatchImpl {
    fn addAttribute();
    fn addAttributeFromIndex();
    fn clear();
    fn removeAttribute();
    fn setAttribute();
    fn setAttributes();
    fn setLocalName();
    fn setQName();
    fn setType();
    fn setURI();
    fn setValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXAttributesVtbl {
        unsafe extern "system" fn addAttribute<Impl: IMXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addAttributeFromIndex<Impl: IMXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varatts: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, nindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn clear<Impl: IMXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeAttribute<Impl: IMXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setAttribute<Impl: IMXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setAttributes<Impl: IMXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varatts: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setLocalName<Impl: IMXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setQName<Impl: IMXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setType<Impl: IMXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setURI<Impl: IMXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setValue<Impl: IMXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            addAttribute::<Impl, IMPL_OFFSET>,
            addAttributeFromIndex::<Impl, IMPL_OFFSET>,
            clear::<Impl, IMPL_OFFSET>,
            removeAttribute::<Impl, IMPL_OFFSET>,
            setAttribute::<Impl, IMPL_OFFSET>,
            setAttributes::<Impl, IMPL_OFFSET>,
            setLocalName::<Impl, IMPL_OFFSET>,
            setQName::<Impl, IMPL_OFFSET>,
            setType::<Impl, IMPL_OFFSET>,
            setURI::<Impl, IMPL_OFFSET>,
            setValue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXAttributes as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMXNamespaceManagerImpl: Sized {
    fn putAllowOverride();
    fn getAllowOverride();
    fn reset();
    fn pushContext();
    fn pushNodeContext();
    fn popContext();
    fn declarePrefix();
    fn getDeclaredPrefix();
    fn getPrefix();
    fn getURI();
}
#[cfg(feature = "Win32_Foundation")]
impl IMXNamespaceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespaceManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXNamespaceManagerVtbl {
        unsafe extern "system" fn putAllowOverride<Impl: IMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAllowOverride<Impl: IMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn reset<Impl: IMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn pushContext<Impl: IMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn pushNodeContext<Impl: IMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextnode: ::windows::core::RawPtr, fdeep: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn popContext<Impl: IMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn declarePrefix<Impl: IMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: super::super::super::Foundation::PWSTR, namespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getDeclaredPrefix<Impl: IMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pwchprefix: super::super::super::Foundation::PWSTR, pcchprefix: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getPrefix<Impl: IMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznamespaceuri: super::super::super::Foundation::PWSTR, nindex: i32, pwchprefix: super::super::super::Foundation::PWSTR, pcchprefix: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getURI<Impl: IMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: super::super::super::Foundation::PWSTR, pcontextnode: ::windows::core::RawPtr, pwchuri: super::super::super::Foundation::PWSTR, pcchuri: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            putAllowOverride::<Impl, IMPL_OFFSET>,
            getAllowOverride::<Impl, IMPL_OFFSET>,
            reset::<Impl, IMPL_OFFSET>,
            pushContext::<Impl, IMPL_OFFSET>,
            pushNodeContext::<Impl, IMPL_OFFSET>,
            popContext::<Impl, IMPL_OFFSET>,
            declarePrefix::<Impl, IMPL_OFFSET>,
            getDeclaredPrefix::<Impl, IMPL_OFFSET>,
            getPrefix::<Impl, IMPL_OFFSET>,
            getURI::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXNamespaceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXNamespacePrefixesImpl: Sized + IDispatchImpl {
    fn item();
    fn length();
    fn _newEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXNamespacePrefixesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespacePrefixesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXNamespacePrefixesVtbl {
        unsafe extern "system" fn item<Impl: IMXNamespacePrefixesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, prefix: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn length<Impl: IMXNamespacePrefixesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _newEnum<Impl: IMXNamespacePrefixesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, item::<Impl, IMPL_OFFSET>, length::<Impl, IMPL_OFFSET>, _newEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXNamespacePrefixes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXReaderControlImpl: Sized + IDispatchImpl {
    fn abort();
    fn resume();
    fn suspend();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXReaderControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXReaderControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXReaderControlVtbl {
        unsafe extern "system" fn abort<Impl: IMXReaderControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn resume<Impl: IMXReaderControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn suspend<Impl: IMXReaderControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, abort::<Impl, IMPL_OFFSET>, resume::<Impl, IMPL_OFFSET>, suspend::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXReaderControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXSchemaDeclHandlerImpl: Sized + IDispatchImpl {
    fn schemaElementDecl();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXSchemaDeclHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXSchemaDeclHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXSchemaDeclHandlerVtbl {
        unsafe extern "system" fn schemaElementDecl<Impl: IMXSchemaDeclHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oschemaelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, schemaElementDecl::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXSchemaDeclHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXWriterImpl: Sized + IDispatchImpl {
    fn Setoutput();
    fn output();
    fn Setencoding();
    fn encoding();
    fn SetbyteOrderMark();
    fn byteOrderMark();
    fn Setindent();
    fn indent();
    fn Setstandalone();
    fn standalone();
    fn SetomitXMLDeclaration();
    fn omitXMLDeclaration();
    fn Setversion();
    fn version();
    fn SetdisableOutputEscaping();
    fn disableOutputEscaping();
    fn flush();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXWriterVtbl {
        unsafe extern "system" fn Setoutput<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestination: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn output<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestination: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setencoding<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencoding: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn encoding<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencoding: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetbyteOrderMark<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn byteOrderMark<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setindent<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findentmode: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn indent<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findentmode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setstandalone<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn standalone<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetomitXMLDeclaration<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn omitXMLDeclaration<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setversion<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strversion: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn version<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strversion: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetdisableOutputEscaping<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn disableOutputEscaping<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn flush<Impl: IMXWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Setoutput::<Impl, IMPL_OFFSET>,
            output::<Impl, IMPL_OFFSET>,
            Setencoding::<Impl, IMPL_OFFSET>,
            encoding::<Impl, IMPL_OFFSET>,
            SetbyteOrderMark::<Impl, IMPL_OFFSET>,
            byteOrderMark::<Impl, IMPL_OFFSET>,
            Setindent::<Impl, IMPL_OFFSET>,
            indent::<Impl, IMPL_OFFSET>,
            Setstandalone::<Impl, IMPL_OFFSET>,
            standalone::<Impl, IMPL_OFFSET>,
            SetomitXMLDeclaration::<Impl, IMPL_OFFSET>,
            omitXMLDeclaration::<Impl, IMPL_OFFSET>,
            Setversion::<Impl, IMPL_OFFSET>,
            version::<Impl, IMPL_OFFSET>,
            SetdisableOutputEscaping::<Impl, IMPL_OFFSET>,
            disableOutputEscaping::<Impl, IMPL_OFFSET>,
            flush::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXXMLFilterImpl: Sized + IDispatchImpl {
    fn getFeature();
    fn putFeature();
    fn getProperty();
    fn putProperty();
    fn entityResolver();
    fn putref_entityResolver();
    fn contentHandler();
    fn putref_contentHandler();
    fn dtdHandler();
    fn putref_dtdHandler();
    fn errorHandler();
    fn putref_errorHandler();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXXMLFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXXMLFilterVtbl {
        unsafe extern "system" fn getFeature<Impl: IMXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putFeature<Impl: IMXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getProperty<Impl: IMXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putProperty<Impl: IMXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn entityResolver<Impl: IMXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_entityResolver<Impl: IMXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn contentHandler<Impl: IMXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_contentHandler<Impl: IMXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn dtdHandler<Impl: IMXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_dtdHandler<Impl: IMXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn errorHandler<Impl: IMXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_errorHandler<Impl: IMXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            getFeature::<Impl, IMPL_OFFSET>,
            putFeature::<Impl, IMPL_OFFSET>,
            getProperty::<Impl, IMPL_OFFSET>,
            putProperty::<Impl, IMPL_OFFSET>,
            entityResolver::<Impl, IMPL_OFFSET>,
            putref_entityResolver::<Impl, IMPL_OFFSET>,
            contentHandler::<Impl, IMPL_OFFSET>,
            putref_contentHandler::<Impl, IMPL_OFFSET>,
            dtdHandler::<Impl, IMPL_OFFSET>,
            putref_dtdHandler::<Impl, IMPL_OFFSET>,
            errorHandler::<Impl, IMPL_OFFSET>,
            putref_errorHandler::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXXMLFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISAXAttributesImpl: Sized {
    fn getLength();
    fn getURI();
    fn getLocalName();
    fn getQName();
    fn getName();
    fn getIndexFromName();
    fn getIndexFromQName();
    fn getType();
    fn getTypeFromName();
    fn getTypeFromQName();
    fn getValue();
    fn getValueFromName();
    fn getValueFromQName();
}
#[cfg(feature = "Win32_Foundation")]
impl ISAXAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXAttributesVtbl {
        unsafe extern "system" fn getLength<Impl: ISAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getURI<Impl: ISAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getLocalName<Impl: ISAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getQName<Impl: ISAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getName<Impl: ISAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getIndexFromName<Impl: ISAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, pnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getIndexFromQName<Impl: ISAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, pnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getType<Impl: ISAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getTypeFromName<Impl: ISAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getTypeFromQName<Impl: ISAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getValue<Impl: ISAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getValueFromName<Impl: ISAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getValueFromQName<Impl: ISAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            getLength::<Impl, IMPL_OFFSET>,
            getURI::<Impl, IMPL_OFFSET>,
            getLocalName::<Impl, IMPL_OFFSET>,
            getQName::<Impl, IMPL_OFFSET>,
            getName::<Impl, IMPL_OFFSET>,
            getIndexFromName::<Impl, IMPL_OFFSET>,
            getIndexFromQName::<Impl, IMPL_OFFSET>,
            getType::<Impl, IMPL_OFFSET>,
            getTypeFromName::<Impl, IMPL_OFFSET>,
            getTypeFromQName::<Impl, IMPL_OFFSET>,
            getValue::<Impl, IMPL_OFFSET>,
            getValueFromName::<Impl, IMPL_OFFSET>,
            getValueFromQName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXAttributes as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISAXContentHandlerImpl: Sized {
    fn putDocumentLocator();
    fn startDocument();
    fn endDocument();
    fn startPrefixMapping();
    fn endPrefixMapping();
    fn startElement();
    fn endElement();
    fn characters();
    fn ignorableWhitespace();
    fn processingInstruction();
    fn skippedEntity();
}
#[cfg(feature = "Win32_Foundation")]
impl ISAXContentHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXContentHandlerVtbl {
        unsafe extern "system" fn putDocumentLocator<Impl: ISAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startDocument<Impl: ISAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn endDocument<Impl: ISAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startPrefixMapping<Impl: ISAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: super::super::super::Foundation::PWSTR, cchprefix: i32, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn endPrefixMapping<Impl: ISAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: super::super::super::Foundation::PWSTR, cchprefix: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startElement<Impl: ISAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: super::super::super::Foundation::PWSTR, cchnamespaceuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn endElement<Impl: ISAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: super::super::super::Foundation::PWSTR, cchnamespaceuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn characters<Impl: ISAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: super::super::super::Foundation::PWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ignorableWhitespace<Impl: ISAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: super::super::super::Foundation::PWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn processingInstruction<Impl: ISAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchtarget: super::super::super::Foundation::PWSTR, cchtarget: i32, pwchdata: super::super::super::Foundation::PWSTR, cchdata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn skippedEntity<Impl: ISAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            putDocumentLocator::<Impl, IMPL_OFFSET>,
            startDocument::<Impl, IMPL_OFFSET>,
            endDocument::<Impl, IMPL_OFFSET>,
            startPrefixMapping::<Impl, IMPL_OFFSET>,
            endPrefixMapping::<Impl, IMPL_OFFSET>,
            startElement::<Impl, IMPL_OFFSET>,
            endElement::<Impl, IMPL_OFFSET>,
            characters::<Impl, IMPL_OFFSET>,
            ignorableWhitespace::<Impl, IMPL_OFFSET>,
            processingInstruction::<Impl, IMPL_OFFSET>,
            skippedEntity::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXContentHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISAXDTDHandlerImpl: Sized {
    fn notationDecl();
    fn unparsedEntityDecl();
}
#[cfg(feature = "Win32_Foundation")]
impl ISAXDTDHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXDTDHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXDTDHandlerVtbl {
        unsafe extern "system" fn notationDecl<Impl: ISAXDTDHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn unparsedEntityDecl<Impl: ISAXDTDHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32, pwchnotationname: super::super::super::Foundation::PWSTR, cchnotationname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, notationDecl::<Impl, IMPL_OFFSET>, unparsedEntityDecl::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXDTDHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISAXDeclHandlerImpl: Sized {
    fn elementDecl();
    fn attributeDecl();
    fn internalEntityDecl();
    fn externalEntityDecl();
}
#[cfg(feature = "Win32_Foundation")]
impl ISAXDeclHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXDeclHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXDeclHandlerVtbl {
        unsafe extern "system" fn elementDecl<Impl: ISAXDeclHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchmodel: super::super::super::Foundation::PWSTR, cchmodel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn attributeDecl<Impl: ISAXDeclHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchelementname: super::super::super::Foundation::PWSTR, cchelementname: i32, pwchattributename: super::super::super::Foundation::PWSTR, cchattributename: i32, pwchtype: super::super::super::Foundation::PWSTR, cchtype: i32, pwchvaluedefault: super::super::super::Foundation::PWSTR, cchvaluedefault: i32, pwchvalue: super::super::super::Foundation::PWSTR, cchvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn internalEntityDecl<Impl: ISAXDeclHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchvalue: super::super::super::Foundation::PWSTR, cchvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn externalEntityDecl<Impl: ISAXDeclHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, elementDecl::<Impl, IMPL_OFFSET>, attributeDecl::<Impl, IMPL_OFFSET>, internalEntityDecl::<Impl, IMPL_OFFSET>, externalEntityDecl::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXDeclHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISAXEntityResolverImpl: Sized {
    fn resolveEntity();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISAXEntityResolverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXEntityResolverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXEntityResolverVtbl {
        unsafe extern "system" fn resolveEntity<Impl: ISAXEntityResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchpublicid: super::super::super::Foundation::PWSTR, pwchsystemid: super::super::super::Foundation::PWSTR, pvarinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, resolveEntity::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXEntityResolver as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISAXErrorHandlerImpl: Sized {
    fn error();
    fn fatalError();
    fn ignorableWarning();
}
#[cfg(feature = "Win32_Foundation")]
impl ISAXErrorHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXErrorHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXErrorHandlerVtbl {
        unsafe extern "system" fn error<Impl: ISAXErrorHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr, pwcherrormessage: super::super::super::Foundation::PWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fatalError<Impl: ISAXErrorHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr, pwcherrormessage: super::super::super::Foundation::PWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ignorableWarning<Impl: ISAXErrorHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr, pwcherrormessage: super::super::super::Foundation::PWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, error::<Impl, IMPL_OFFSET>, fatalError::<Impl, IMPL_OFFSET>, ignorableWarning::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXErrorHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISAXLexicalHandlerImpl: Sized {
    fn startDTD();
    fn endDTD();
    fn startEntity();
    fn endEntity();
    fn startCDATA();
    fn endCDATA();
    fn comment();
}
#[cfg(feature = "Win32_Foundation")]
impl ISAXLexicalHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLexicalHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXLexicalHandlerVtbl {
        unsafe extern "system" fn startDTD<Impl: ISAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn endDTD<Impl: ISAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startEntity<Impl: ISAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn endEntity<Impl: ISAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startCDATA<Impl: ISAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn endCDATA<Impl: ISAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn comment<Impl: ISAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: super::super::super::Foundation::PWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, startDTD::<Impl, IMPL_OFFSET>, endDTD::<Impl, IMPL_OFFSET>, startEntity::<Impl, IMPL_OFFSET>, endEntity::<Impl, IMPL_OFFSET>, startCDATA::<Impl, IMPL_OFFSET>, endCDATA::<Impl, IMPL_OFFSET>, comment::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXLexicalHandler as ::windows::core::Interface>::IID
    }
}
pub trait ISAXLocatorImpl: Sized {
    fn getColumnNumber();
    fn getLineNumber();
    fn getPublicId();
    fn getSystemId();
}
impl ISAXLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXLocatorVtbl {
        unsafe extern "system" fn getColumnNumber<Impl: ISAXLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncolumn: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getLineNumber<Impl: ISAXLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnline: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getPublicId<Impl: ISAXLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchpublicid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getSystemId<Impl: ISAXLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchsystemid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, getColumnNumber::<Impl, IMPL_OFFSET>, getLineNumber::<Impl, IMPL_OFFSET>, getPublicId::<Impl, IMPL_OFFSET>, getSystemId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISAXXMLFilterImpl: Sized + ISAXXMLReaderImpl {
    fn getParent();
    fn putParent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISAXXMLFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXXMLFilterVtbl {
        unsafe extern "system" fn getParent<Impl: ISAXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putParent<Impl: ISAXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            getFeature::<Impl, IMPL_OFFSET>,
            putFeature::<Impl, IMPL_OFFSET>,
            getProperty::<Impl, IMPL_OFFSET>,
            putProperty::<Impl, IMPL_OFFSET>,
            getEntityResolver::<Impl, IMPL_OFFSET>,
            putEntityResolver::<Impl, IMPL_OFFSET>,
            getContentHandler::<Impl, IMPL_OFFSET>,
            putContentHandler::<Impl, IMPL_OFFSET>,
            getDTDHandler::<Impl, IMPL_OFFSET>,
            putDTDHandler::<Impl, IMPL_OFFSET>,
            getErrorHandler::<Impl, IMPL_OFFSET>,
            putErrorHandler::<Impl, IMPL_OFFSET>,
            getBaseURL::<Impl, IMPL_OFFSET>,
            putBaseURL::<Impl, IMPL_OFFSET>,
            getSecureBaseURL::<Impl, IMPL_OFFSET>,
            putSecureBaseURL::<Impl, IMPL_OFFSET>,
            parse::<Impl, IMPL_OFFSET>,
            parseURL::<Impl, IMPL_OFFSET>,
            getParent::<Impl, IMPL_OFFSET>,
            putParent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXXMLFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISAXXMLReaderImpl: Sized {
    fn getFeature();
    fn putFeature();
    fn getProperty();
    fn putProperty();
    fn getEntityResolver();
    fn putEntityResolver();
    fn getContentHandler();
    fn putContentHandler();
    fn getDTDHandler();
    fn putDTDHandler();
    fn getErrorHandler();
    fn putErrorHandler();
    fn getBaseURL();
    fn putBaseURL();
    fn getSecureBaseURL();
    fn putSecureBaseURL();
    fn parse();
    fn parseURL();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISAXXMLReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXXMLReaderVtbl {
        unsafe extern "system" fn getFeature<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, pvfvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putFeature<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, vfvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getProperty<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putProperty<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, varvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getEntityResolver<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresolver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putEntityResolver<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getContentHandler<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putContentHandler<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getDTDHandler<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putDTDHandler<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getErrorHandler<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putErrorHandler<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getBaseURL<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchbaseurl: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putBaseURL<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchbaseurl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getSecureBaseURL<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchsecurebaseurl: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putSecureBaseURL<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchsecurebaseurl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn parse<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinput: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn parseURL<Impl: ISAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchurl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            getFeature::<Impl, IMPL_OFFSET>,
            putFeature::<Impl, IMPL_OFFSET>,
            getProperty::<Impl, IMPL_OFFSET>,
            putProperty::<Impl, IMPL_OFFSET>,
            getEntityResolver::<Impl, IMPL_OFFSET>,
            putEntityResolver::<Impl, IMPL_OFFSET>,
            getContentHandler::<Impl, IMPL_OFFSET>,
            putContentHandler::<Impl, IMPL_OFFSET>,
            getDTDHandler::<Impl, IMPL_OFFSET>,
            putDTDHandler::<Impl, IMPL_OFFSET>,
            getErrorHandler::<Impl, IMPL_OFFSET>,
            putErrorHandler::<Impl, IMPL_OFFSET>,
            getBaseURL::<Impl, IMPL_OFFSET>,
            putBaseURL::<Impl, IMPL_OFFSET>,
            getSecureBaseURL::<Impl, IMPL_OFFSET>,
            putSecureBaseURL::<Impl, IMPL_OFFSET>,
            parse::<Impl, IMPL_OFFSET>,
            parseURL::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXXMLReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn targetNamespace();
    fn version();
    fn types();
    fn elements();
    fn attributes();
    fn attributeGroups();
    fn modelGroups();
    fn notations();
    fn schemaLocations();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaVtbl {
        unsafe extern "system" fn targetNamespace<Impl: ISchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetnamespace: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn version<Impl: ISchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn types<Impl: ISchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, types: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn elements<Impl: ISchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elements: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn attributes<Impl: ISchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn attributeGroups<Impl: ISchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn modelGroups<Impl: ISchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn notations<Impl: ISchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn schemaLocations<Impl: ISchemaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, schemalocations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            name::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            schema::<Impl, IMPL_OFFSET>,
            id::<Impl, IMPL_OFFSET>,
            itemType::<Impl, IMPL_OFFSET>,
            unhandledAttributes::<Impl, IMPL_OFFSET>,
            writeAnnotation::<Impl, IMPL_OFFSET>,
            targetNamespace::<Impl, IMPL_OFFSET>,
            version::<Impl, IMPL_OFFSET>,
            types::<Impl, IMPL_OFFSET>,
            elements::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            attributeGroups::<Impl, IMPL_OFFSET>,
            modelGroups::<Impl, IMPL_OFFSET>,
            notations::<Impl, IMPL_OFFSET>,
            schemaLocations::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchema as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaAnyImpl: Sized + ISchemaParticleImpl + ISchemaItemImpl + IDispatchImpl {
    fn namespaces();
    fn processContents();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaAnyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAnyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaAnyVtbl {
        unsafe extern "system" fn namespaces<Impl: ISchemaAnyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn processContents<Impl: ISchemaAnyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processcontents: *mut SCHEMAPROCESSCONTENTS) -> ::windows::core::HRESULT {
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
            name::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            schema::<Impl, IMPL_OFFSET>,
            id::<Impl, IMPL_OFFSET>,
            itemType::<Impl, IMPL_OFFSET>,
            unhandledAttributes::<Impl, IMPL_OFFSET>,
            writeAnnotation::<Impl, IMPL_OFFSET>,
            minOccurs::<Impl, IMPL_OFFSET>,
            maxOccurs::<Impl, IMPL_OFFSET>,
            namespaces::<Impl, IMPL_OFFSET>,
            processContents::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaAny as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaAttributeImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn r#type();
    fn scope();
    fn defaultValue();
    fn fixedValue();
    fn r#use();
    fn isReference();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaAttributeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttributeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaAttributeVtbl {
        unsafe extern "system" fn r#type<Impl: ISchemaAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn scope<Impl: ISchemaAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn defaultValue<Impl: ISchemaAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fixedValue<Impl: ISchemaAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn r#use<Impl: ISchemaAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#use: *mut SCHEMAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isReference<Impl: ISchemaAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut i16) -> ::windows::core::HRESULT {
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
            name::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            schema::<Impl, IMPL_OFFSET>,
            id::<Impl, IMPL_OFFSET>,
            itemType::<Impl, IMPL_OFFSET>,
            unhandledAttributes::<Impl, IMPL_OFFSET>,
            writeAnnotation::<Impl, IMPL_OFFSET>,
            r#type::<Impl, IMPL_OFFSET>,
            scope::<Impl, IMPL_OFFSET>,
            defaultValue::<Impl, IMPL_OFFSET>,
            fixedValue::<Impl, IMPL_OFFSET>,
            r#use::<Impl, IMPL_OFFSET>,
            isReference::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaAttribute as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaAttributeGroupImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn anyAttribute();
    fn attributes();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaAttributeGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttributeGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaAttributeGroupVtbl {
        unsafe extern "system" fn anyAttribute<Impl: ISchemaAttributeGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anyattribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn attributes<Impl: ISchemaAttributeGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            name::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            schema::<Impl, IMPL_OFFSET>,
            id::<Impl, IMPL_OFFSET>,
            itemType::<Impl, IMPL_OFFSET>,
            unhandledAttributes::<Impl, IMPL_OFFSET>,
            writeAnnotation::<Impl, IMPL_OFFSET>,
            anyAttribute::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaAttributeGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaComplexTypeImpl: Sized + ISchemaTypeImpl + ISchemaItemImpl + IDispatchImpl {
    fn isAbstract();
    fn anyAttribute();
    fn attributes();
    fn contentType();
    fn contentModel();
    fn prohibitedSubstitutions();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaComplexTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaComplexTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaComplexTypeVtbl {
        unsafe extern "system" fn isAbstract<Impl: ISchemaComplexTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#abstract: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn anyAttribute<Impl: ISchemaComplexTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anyattribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn attributes<Impl: ISchemaComplexTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn contentType<Impl: ISchemaComplexTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut SCHEMACONTENTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn contentModel<Impl: ISchemaComplexTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentmodel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn prohibitedSubstitutions<Impl: ISchemaComplexTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibited: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
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
            name::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            schema::<Impl, IMPL_OFFSET>,
            id::<Impl, IMPL_OFFSET>,
            itemType::<Impl, IMPL_OFFSET>,
            unhandledAttributes::<Impl, IMPL_OFFSET>,
            writeAnnotation::<Impl, IMPL_OFFSET>,
            baseTypes::<Impl, IMPL_OFFSET>,
            r#final::<Impl, IMPL_OFFSET>,
            variety::<Impl, IMPL_OFFSET>,
            derivedBy::<Impl, IMPL_OFFSET>,
            isValid::<Impl, IMPL_OFFSET>,
            minExclusive::<Impl, IMPL_OFFSET>,
            minInclusive::<Impl, IMPL_OFFSET>,
            maxExclusive::<Impl, IMPL_OFFSET>,
            maxInclusive::<Impl, IMPL_OFFSET>,
            totalDigits::<Impl, IMPL_OFFSET>,
            fractionDigits::<Impl, IMPL_OFFSET>,
            length::<Impl, IMPL_OFFSET>,
            minLength::<Impl, IMPL_OFFSET>,
            maxLength::<Impl, IMPL_OFFSET>,
            enumeration::<Impl, IMPL_OFFSET>,
            whitespace::<Impl, IMPL_OFFSET>,
            patterns::<Impl, IMPL_OFFSET>,
            isAbstract::<Impl, IMPL_OFFSET>,
            anyAttribute::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            contentType::<Impl, IMPL_OFFSET>,
            contentModel::<Impl, IMPL_OFFSET>,
            prohibitedSubstitutions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaComplexType as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaElementImpl: Sized + ISchemaParticleImpl + ISchemaItemImpl + IDispatchImpl {
    fn r#type();
    fn scope();
    fn defaultValue();
    fn fixedValue();
    fn isNillable();
    fn identityConstraints();
    fn substitutionGroup();
    fn substitutionGroupExclusions();
    fn disallowedSubstitutions();
    fn isAbstract();
    fn isReference();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaElementVtbl {
        unsafe extern "system" fn r#type<Impl: ISchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn scope<Impl: ISchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn defaultValue<Impl: ISchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fixedValue<Impl: ISchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isNillable<Impl: ISchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nillable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn identityConstraints<Impl: ISchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, constraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn substitutionGroup<Impl: ISchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn substitutionGroupExclusions<Impl: ISchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, exclusions: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn disallowedSubstitutions<Impl: ISchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowed: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isAbstract<Impl: ISchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#abstract: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isReference<Impl: ISchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut i16) -> ::windows::core::HRESULT {
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
            name::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            schema::<Impl, IMPL_OFFSET>,
            id::<Impl, IMPL_OFFSET>,
            itemType::<Impl, IMPL_OFFSET>,
            unhandledAttributes::<Impl, IMPL_OFFSET>,
            writeAnnotation::<Impl, IMPL_OFFSET>,
            minOccurs::<Impl, IMPL_OFFSET>,
            maxOccurs::<Impl, IMPL_OFFSET>,
            r#type::<Impl, IMPL_OFFSET>,
            scope::<Impl, IMPL_OFFSET>,
            defaultValue::<Impl, IMPL_OFFSET>,
            fixedValue::<Impl, IMPL_OFFSET>,
            isNillable::<Impl, IMPL_OFFSET>,
            identityConstraints::<Impl, IMPL_OFFSET>,
            substitutionGroup::<Impl, IMPL_OFFSET>,
            substitutionGroupExclusions::<Impl, IMPL_OFFSET>,
            disallowedSubstitutions::<Impl, IMPL_OFFSET>,
            isAbstract::<Impl, IMPL_OFFSET>,
            isReference::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaIdentityConstraintImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn selector();
    fn fields();
    fn referencedKey();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaIdentityConstraintVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaIdentityConstraintImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaIdentityConstraintVtbl {
        unsafe extern "system" fn selector<Impl: ISchemaIdentityConstraintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selector: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fields<Impl: ISchemaIdentityConstraintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fields: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn referencedKey<Impl: ISchemaIdentityConstraintImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            name::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            schema::<Impl, IMPL_OFFSET>,
            id::<Impl, IMPL_OFFSET>,
            itemType::<Impl, IMPL_OFFSET>,
            unhandledAttributes::<Impl, IMPL_OFFSET>,
            writeAnnotation::<Impl, IMPL_OFFSET>,
            selector::<Impl, IMPL_OFFSET>,
            fields::<Impl, IMPL_OFFSET>,
            referencedKey::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaIdentityConstraint as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaItemImpl: Sized + IDispatchImpl {
    fn name();
    fn namespaceURI();
    fn schema();
    fn id();
    fn itemType();
    fn unhandledAttributes();
    fn writeAnnotation();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaItemVtbl {
        unsafe extern "system" fn name<Impl: ISchemaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn namespaceURI<Impl: ISchemaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn schema<Impl: ISchemaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, schema: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn id<Impl: ISchemaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn itemType<Impl: ISchemaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemtype: *mut SOMITEMTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn unhandledAttributes<Impl: ISchemaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn writeAnnotation<Impl: ISchemaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationsink: *mut ::core::ffi::c_void, iswritten: *mut i16) -> ::windows::core::HRESULT {
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
            name::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            schema::<Impl, IMPL_OFFSET>,
            id::<Impl, IMPL_OFFSET>,
            itemType::<Impl, IMPL_OFFSET>,
            unhandledAttributes::<Impl, IMPL_OFFSET>,
            writeAnnotation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaItemCollectionImpl: Sized + IDispatchImpl {
    fn item();
    fn itemByName();
    fn itemByQName();
    fn length();
    fn _newEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaItemCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItemCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaItemCollectionVtbl {
        unsafe extern "system" fn item<Impl: ISchemaItemCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn itemByName<Impl: ISchemaItemCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn itemByQName<Impl: ISchemaItemCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn length<Impl: ISchemaItemCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _newEnum<Impl: ISchemaItemCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, item::<Impl, IMPL_OFFSET>, itemByName::<Impl, IMPL_OFFSET>, itemByQName::<Impl, IMPL_OFFSET>, length::<Impl, IMPL_OFFSET>, _newEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaItemCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaModelGroupImpl: Sized + ISchemaParticleImpl + ISchemaItemImpl + IDispatchImpl {
    fn particles();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaModelGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaModelGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaModelGroupVtbl {
        unsafe extern "system" fn particles<Impl: ISchemaModelGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, particles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            name::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            schema::<Impl, IMPL_OFFSET>,
            id::<Impl, IMPL_OFFSET>,
            itemType::<Impl, IMPL_OFFSET>,
            unhandledAttributes::<Impl, IMPL_OFFSET>,
            writeAnnotation::<Impl, IMPL_OFFSET>,
            minOccurs::<Impl, IMPL_OFFSET>,
            maxOccurs::<Impl, IMPL_OFFSET>,
            particles::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaModelGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaNotationImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn systemIdentifier();
    fn publicIdentifier();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaNotationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaNotationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaNotationVtbl {
        unsafe extern "system" fn systemIdentifier<Impl: ISchemaNotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn publicIdentifier<Impl: ISchemaNotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            name::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            schema::<Impl, IMPL_OFFSET>,
            id::<Impl, IMPL_OFFSET>,
            itemType::<Impl, IMPL_OFFSET>,
            unhandledAttributes::<Impl, IMPL_OFFSET>,
            writeAnnotation::<Impl, IMPL_OFFSET>,
            systemIdentifier::<Impl, IMPL_OFFSET>,
            publicIdentifier::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaNotation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaParticleImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn minOccurs();
    fn maxOccurs();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaParticleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaParticleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaParticleVtbl {
        unsafe extern "system" fn minOccurs<Impl: ISchemaParticleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minoccurs: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn maxOccurs<Impl: ISchemaParticleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxoccurs: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            name::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            schema::<Impl, IMPL_OFFSET>,
            id::<Impl, IMPL_OFFSET>,
            itemType::<Impl, IMPL_OFFSET>,
            unhandledAttributes::<Impl, IMPL_OFFSET>,
            writeAnnotation::<Impl, IMPL_OFFSET>,
            minOccurs::<Impl, IMPL_OFFSET>,
            maxOccurs::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaParticle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaStringCollectionImpl: Sized + IDispatchImpl {
    fn item();
    fn length();
    fn _newEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaStringCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaStringCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaStringCollectionVtbl {
        unsafe extern "system" fn item<Impl: ISchemaStringCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, bstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn length<Impl: ISchemaStringCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _newEnum<Impl: ISchemaStringCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, item::<Impl, IMPL_OFFSET>, length::<Impl, IMPL_OFFSET>, _newEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaStringCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaTypeImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn baseTypes();
    fn r#final();
    fn variety();
    fn derivedBy();
    fn isValid();
    fn minExclusive();
    fn minInclusive();
    fn maxExclusive();
    fn maxInclusive();
    fn totalDigits();
    fn fractionDigits();
    fn length();
    fn minLength();
    fn maxLength();
    fn enumeration();
    fn whitespace();
    fn patterns();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaTypeVtbl {
        unsafe extern "system" fn baseTypes<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basetypes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn r#final<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#final: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn variety<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variety: *mut SCHEMATYPEVARIETY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn derivedBy<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, derivedby: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn isValid<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, valid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn minExclusive<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minexclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn minInclusive<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mininclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn maxExclusive<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxexclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn maxInclusive<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxinclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn totalDigits<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, totaldigits: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fractionDigits<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fractiondigits: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn length<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn minLength<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minlength: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn maxLength<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn enumeration<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumeration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn whitespace<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitespace: *mut SCHEMAWHITESPACE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn patterns<Impl: ISchemaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patterns: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            name::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            schema::<Impl, IMPL_OFFSET>,
            id::<Impl, IMPL_OFFSET>,
            itemType::<Impl, IMPL_OFFSET>,
            unhandledAttributes::<Impl, IMPL_OFFSET>,
            writeAnnotation::<Impl, IMPL_OFFSET>,
            baseTypes::<Impl, IMPL_OFFSET>,
            r#final::<Impl, IMPL_OFFSET>,
            variety::<Impl, IMPL_OFFSET>,
            derivedBy::<Impl, IMPL_OFFSET>,
            isValid::<Impl, IMPL_OFFSET>,
            minExclusive::<Impl, IMPL_OFFSET>,
            minInclusive::<Impl, IMPL_OFFSET>,
            maxExclusive::<Impl, IMPL_OFFSET>,
            maxInclusive::<Impl, IMPL_OFFSET>,
            totalDigits::<Impl, IMPL_OFFSET>,
            fractionDigits::<Impl, IMPL_OFFSET>,
            length::<Impl, IMPL_OFFSET>,
            minLength::<Impl, IMPL_OFFSET>,
            maxLength::<Impl, IMPL_OFFSET>,
            enumeration::<Impl, IMPL_OFFSET>,
            whitespace::<Impl, IMPL_OFFSET>,
            patterns::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaType as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IServerXMLHTTPRequestImpl: Sized + IXMLHTTPRequestImpl + IDispatchImpl {
    fn setTimeouts();
    fn waitForResponse();
    fn getOption();
    fn setOption();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IServerXMLHTTPRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerXMLHTTPRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerXMLHTTPRequestVtbl {
        unsafe extern "system" fn setTimeouts<Impl: IServerXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn waitForResponse<Impl: IServerXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeoutinseconds: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, issuccessful: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getOption<Impl: IServerXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setOption<Impl: IServerXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
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
            open::<Impl, IMPL_OFFSET>,
            setRequestHeader::<Impl, IMPL_OFFSET>,
            getResponseHeader::<Impl, IMPL_OFFSET>,
            getAllResponseHeaders::<Impl, IMPL_OFFSET>,
            send::<Impl, IMPL_OFFSET>,
            abort::<Impl, IMPL_OFFSET>,
            status::<Impl, IMPL_OFFSET>,
            statusText::<Impl, IMPL_OFFSET>,
            responseXML::<Impl, IMPL_OFFSET>,
            responseText::<Impl, IMPL_OFFSET>,
            responseBody::<Impl, IMPL_OFFSET>,
            responseStream::<Impl, IMPL_OFFSET>,
            readyState::<Impl, IMPL_OFFSET>,
            Setonreadystatechange::<Impl, IMPL_OFFSET>,
            setTimeouts::<Impl, IMPL_OFFSET>,
            waitForResponse::<Impl, IMPL_OFFSET>,
            getOption::<Impl, IMPL_OFFSET>,
            setOption::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IServerXMLHTTPRequest2Impl: Sized + IServerXMLHTTPRequestImpl + IXMLHTTPRequestImpl + IDispatchImpl {
    fn setProxy();
    fn setProxyCredentials();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IServerXMLHTTPRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerXMLHTTPRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerXMLHTTPRequest2Vtbl {
        unsafe extern "system" fn setProxy<Impl: IServerXMLHTTPRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proxysetting: SXH_PROXY_SETTING, varproxyserver: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, varbypasslist: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setProxyCredentials<Impl: IServerXMLHTTPRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            open::<Impl, IMPL_OFFSET>,
            setRequestHeader::<Impl, IMPL_OFFSET>,
            getResponseHeader::<Impl, IMPL_OFFSET>,
            getAllResponseHeaders::<Impl, IMPL_OFFSET>,
            send::<Impl, IMPL_OFFSET>,
            abort::<Impl, IMPL_OFFSET>,
            status::<Impl, IMPL_OFFSET>,
            statusText::<Impl, IMPL_OFFSET>,
            responseXML::<Impl, IMPL_OFFSET>,
            responseText::<Impl, IMPL_OFFSET>,
            responseBody::<Impl, IMPL_OFFSET>,
            responseStream::<Impl, IMPL_OFFSET>,
            readyState::<Impl, IMPL_OFFSET>,
            Setonreadystatechange::<Impl, IMPL_OFFSET>,
            setTimeouts::<Impl, IMPL_OFFSET>,
            waitForResponse::<Impl, IMPL_OFFSET>,
            getOption::<Impl, IMPL_OFFSET>,
            setOption::<Impl, IMPL_OFFSET>,
            setProxy::<Impl, IMPL_OFFSET>,
            setProxyCredentials::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBMXNamespaceManagerImpl: Sized + IDispatchImpl {
    fn SetallowOverride();
    fn allowOverride();
    fn reset();
    fn pushContext();
    fn pushNodeContext();
    fn popContext();
    fn declarePrefix();
    fn getDeclaredPrefixes();
    fn getPrefixes();
    fn getURI();
    fn getURIFromNode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBMXNamespaceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBMXNamespaceManagerVtbl {
        unsafe extern "system" fn SetallowOverride<Impl: IVBMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn allowOverride<Impl: IVBMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn reset<Impl: IVBMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn pushContext<Impl: IVBMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn pushNodeContext<Impl: IVBMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextnode: ::windows::core::RawPtr, fdeep: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn popContext<Impl: IVBMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn declarePrefix<Impl: IVBMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getDeclaredPrefixes<Impl: IVBMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefixes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getPrefixes<Impl: IVBMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, prefixes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getURI<Impl: IVBMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, uri: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getURIFromNode<Impl: IVBMXNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, contextnode: ::windows::core::RawPtr, uri: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            SetallowOverride::<Impl, IMPL_OFFSET>,
            allowOverride::<Impl, IMPL_OFFSET>,
            reset::<Impl, IMPL_OFFSET>,
            pushContext::<Impl, IMPL_OFFSET>,
            pushNodeContext::<Impl, IMPL_OFFSET>,
            popContext::<Impl, IMPL_OFFSET>,
            declarePrefix::<Impl, IMPL_OFFSET>,
            getDeclaredPrefixes::<Impl, IMPL_OFFSET>,
            getPrefixes::<Impl, IMPL_OFFSET>,
            getURI::<Impl, IMPL_OFFSET>,
            getURIFromNode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBMXNamespaceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXAttributesImpl: Sized + IDispatchImpl {
    fn length();
    fn getURI();
    fn getLocalName();
    fn getQName();
    fn getIndexFromName();
    fn getIndexFromQName();
    fn getType();
    fn getTypeFromName();
    fn getTypeFromQName();
    fn getValue();
    fn getValueFromName();
    fn getValueFromQName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXAttributesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXAttributesVtbl {
        unsafe extern "system" fn length<Impl: IVBSAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getURI<Impl: IVBSAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getLocalName<Impl: IVBSAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getQName<Impl: IVBSAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getIndexFromName<Impl: IVBSAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getIndexFromQName<Impl: IVBSAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getType<Impl: IVBSAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getTypeFromName<Impl: IVBSAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getTypeFromQName<Impl: IVBSAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getValue<Impl: IVBSAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getValueFromName<Impl: IVBSAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getValueFromQName<Impl: IVBSAXAttributesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            length::<Impl, IMPL_OFFSET>,
            getURI::<Impl, IMPL_OFFSET>,
            getLocalName::<Impl, IMPL_OFFSET>,
            getQName::<Impl, IMPL_OFFSET>,
            getIndexFromName::<Impl, IMPL_OFFSET>,
            getIndexFromQName::<Impl, IMPL_OFFSET>,
            getType::<Impl, IMPL_OFFSET>,
            getTypeFromName::<Impl, IMPL_OFFSET>,
            getTypeFromQName::<Impl, IMPL_OFFSET>,
            getValue::<Impl, IMPL_OFFSET>,
            getValueFromName::<Impl, IMPL_OFFSET>,
            getValueFromQName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXAttributes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXContentHandlerImpl: Sized + IDispatchImpl {
    fn putref_documentLocator();
    fn startDocument();
    fn endDocument();
    fn startPrefixMapping();
    fn endPrefixMapping();
    fn startElement();
    fn endElement();
    fn characters();
    fn ignorableWhitespace();
    fn processingInstruction();
    fn skippedEntity();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXContentHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXContentHandlerVtbl {
        unsafe extern "system" fn putref_documentLocator<Impl: IVBSAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startDocument<Impl: IVBSAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn endDocument<Impl: IVBSAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startPrefixMapping<Impl: IVBSAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: *mut super::super::super::Foundation::BSTR, struri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn endPrefixMapping<Impl: IVBSAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startElement<Impl: IVBSAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut super::super::super::Foundation::BSTR, strlocalname: *mut super::super::super::Foundation::BSTR, strqname: *mut super::super::super::Foundation::BSTR, oattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn endElement<Impl: IVBSAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut super::super::super::Foundation::BSTR, strlocalname: *mut super::super::super::Foundation::BSTR, strqname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn characters<Impl: IVBSAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ignorableWhitespace<Impl: IVBSAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn processingInstruction<Impl: IVBSAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strtarget: *mut super::super::super::Foundation::BSTR, strdata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn skippedEntity<Impl: IVBSAXContentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            putref_documentLocator::<Impl, IMPL_OFFSET>,
            startDocument::<Impl, IMPL_OFFSET>,
            endDocument::<Impl, IMPL_OFFSET>,
            startPrefixMapping::<Impl, IMPL_OFFSET>,
            endPrefixMapping::<Impl, IMPL_OFFSET>,
            startElement::<Impl, IMPL_OFFSET>,
            endElement::<Impl, IMPL_OFFSET>,
            characters::<Impl, IMPL_OFFSET>,
            ignorableWhitespace::<Impl, IMPL_OFFSET>,
            processingInstruction::<Impl, IMPL_OFFSET>,
            skippedEntity::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXContentHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXDTDHandlerImpl: Sized + IDispatchImpl {
    fn notationDecl();
    fn unparsedEntityDecl();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXDTDHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXDTDHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXDTDHandlerVtbl {
        unsafe extern "system" fn notationDecl<Impl: IVBSAXDTDHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn unparsedEntityDecl<Impl: IVBSAXDTDHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR, strnotationname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, notationDecl::<Impl, IMPL_OFFSET>, unparsedEntityDecl::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXDTDHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXDeclHandlerImpl: Sized + IDispatchImpl {
    fn elementDecl();
    fn attributeDecl();
    fn internalEntityDecl();
    fn externalEntityDecl();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXDeclHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXDeclHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXDeclHandlerVtbl {
        unsafe extern "system" fn elementDecl<Impl: IVBSAXDeclHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strmodel: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn attributeDecl<Impl: IVBSAXDeclHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strelementname: *mut super::super::super::Foundation::BSTR, strattributename: *mut super::super::super::Foundation::BSTR, strtype: *mut super::super::super::Foundation::BSTR, strvaluedefault: *mut super::super::super::Foundation::BSTR, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn internalEntityDecl<Impl: IVBSAXDeclHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn externalEntityDecl<Impl: IVBSAXDeclHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, elementDecl::<Impl, IMPL_OFFSET>, attributeDecl::<Impl, IMPL_OFFSET>, internalEntityDecl::<Impl, IMPL_OFFSET>, externalEntityDecl::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXDeclHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXEntityResolverImpl: Sized + IDispatchImpl {
    fn resolveEntity();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXEntityResolverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXEntityResolverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXEntityResolverVtbl {
        unsafe extern "system" fn resolveEntity<Impl: IVBSAXEntityResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR, varinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, resolveEntity::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXEntityResolver as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXErrorHandlerImpl: Sized + IDispatchImpl {
    fn error();
    fn fatalError();
    fn ignorableWarning();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXErrorHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXErrorHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXErrorHandlerVtbl {
        unsafe extern "system" fn error<Impl: IVBSAXErrorHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fatalError<Impl: IVBSAXErrorHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ignorableWarning<Impl: IVBSAXErrorHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, error::<Impl, IMPL_OFFSET>, fatalError::<Impl, IMPL_OFFSET>, ignorableWarning::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXErrorHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXLexicalHandlerImpl: Sized + IDispatchImpl {
    fn startDTD();
    fn endDTD();
    fn startEntity();
    fn endEntity();
    fn startCDATA();
    fn endCDATA();
    fn comment();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXLexicalHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLexicalHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXLexicalHandlerVtbl {
        unsafe extern "system" fn startDTD<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn endDTD<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startEntity<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn endEntity<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startCDATA<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn endCDATA<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn comment<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            startDTD::<Impl, IMPL_OFFSET>,
            endDTD::<Impl, IMPL_OFFSET>,
            startEntity::<Impl, IMPL_OFFSET>,
            endEntity::<Impl, IMPL_OFFSET>,
            startCDATA::<Impl, IMPL_OFFSET>,
            endCDATA::<Impl, IMPL_OFFSET>,
            comment::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXLexicalHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXLocatorImpl: Sized + IDispatchImpl {
    fn columnNumber();
    fn lineNumber();
    fn publicId();
    fn systemId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXLocatorVtbl {
        unsafe extern "system" fn columnNumber<Impl: IVBSAXLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn lineNumber<Impl: IVBSAXLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nline: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn publicId<Impl: IVBSAXLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpublicid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn systemId<Impl: IVBSAXLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, columnNumber::<Impl, IMPL_OFFSET>, lineNumber::<Impl, IMPL_OFFSET>, publicId::<Impl, IMPL_OFFSET>, systemId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXXMLFilterImpl: Sized + IDispatchImpl {
    fn parent();
    fn putref_parent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXXMLFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXXMLFilterVtbl {
        unsafe extern "system" fn parent<Impl: IVBSAXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_parent<Impl: IVBSAXXMLFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oreader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, parent::<Impl, IMPL_OFFSET>, putref_parent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXXMLFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXXMLReaderImpl: Sized + IDispatchImpl {
    fn getFeature();
    fn putFeature();
    fn getProperty();
    fn putProperty();
    fn entityResolver();
    fn putref_entityResolver();
    fn contentHandler();
    fn putref_contentHandler();
    fn dtdHandler();
    fn putref_dtdHandler();
    fn errorHandler();
    fn putref_errorHandler();
    fn baseURL();
    fn SetbaseURL();
    fn secureBaseURL();
    fn SetsecureBaseURL();
    fn parse();
    fn parseURL();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXXMLReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXXMLReaderVtbl {
        unsafe extern "system" fn getFeature<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putFeature<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getProperty<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putProperty<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn entityResolver<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_entityResolver<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn contentHandler<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_contentHandler<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn dtdHandler<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_dtdHandler<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn errorHandler<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_errorHandler<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn baseURL<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbaseurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetbaseURL<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbaseurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn secureBaseURL<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetsecureBaseURL<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn parse<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinput: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn parseURL<Impl: IVBSAXXMLReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            getFeature::<Impl, IMPL_OFFSET>,
            putFeature::<Impl, IMPL_OFFSET>,
            getProperty::<Impl, IMPL_OFFSET>,
            putProperty::<Impl, IMPL_OFFSET>,
            entityResolver::<Impl, IMPL_OFFSET>,
            putref_entityResolver::<Impl, IMPL_OFFSET>,
            contentHandler::<Impl, IMPL_OFFSET>,
            putref_contentHandler::<Impl, IMPL_OFFSET>,
            dtdHandler::<Impl, IMPL_OFFSET>,
            putref_dtdHandler::<Impl, IMPL_OFFSET>,
            errorHandler::<Impl, IMPL_OFFSET>,
            putref_errorHandler::<Impl, IMPL_OFFSET>,
            baseURL::<Impl, IMPL_OFFSET>,
            SetbaseURL::<Impl, IMPL_OFFSET>,
            secureBaseURL::<Impl, IMPL_OFFSET>,
            SetsecureBaseURL::<Impl, IMPL_OFFSET>,
            parse::<Impl, IMPL_OFFSET>,
            parseURL::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXXMLReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLAttributeImpl: Sized + IDispatchImpl {
    fn name();
    fn value();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLAttributeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLAttributeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLAttributeVtbl {
        unsafe extern "system" fn name<Impl: IXMLAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, n: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn value<Impl: IXMLAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, name::<Impl, IMPL_OFFSET>, value::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLAttribute as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMAttributeImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn name();
    fn value();
    fn Setvalue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMAttributeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMAttributeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMAttributeVtbl {
        unsafe extern "system" fn name<Impl: IXMLDOMAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn value<Impl: IXMLDOMAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributevalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setvalue<Impl: IXMLDOMAttributeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributevalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
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
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            name::<Impl, IMPL_OFFSET>,
            value::<Impl, IMPL_OFFSET>,
            Setvalue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMAttribute as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMCDATASectionImpl: Sized + IXMLDOMTextImpl + IXMLDOMCharacterDataImpl + IXMLDOMNodeImpl + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMCDATASectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCDATASectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMCDATASectionVtbl {
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            data::<Impl, IMPL_OFFSET>,
            Setdata::<Impl, IMPL_OFFSET>,
            length::<Impl, IMPL_OFFSET>,
            substringData::<Impl, IMPL_OFFSET>,
            appendData::<Impl, IMPL_OFFSET>,
            insertData::<Impl, IMPL_OFFSET>,
            deleteData::<Impl, IMPL_OFFSET>,
            replaceData::<Impl, IMPL_OFFSET>,
            splitText::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMCDATASection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMCharacterDataImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn data();
    fn Setdata();
    fn length();
    fn substringData();
    fn appendData();
    fn insertData();
    fn deleteData();
    fn replaceData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMCharacterDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCharacterDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMCharacterDataVtbl {
        unsafe extern "system" fn data<Impl: IXMLDOMCharacterDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setdata<Impl: IXMLDOMCharacterDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn length<Impl: IXMLDOMCharacterDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datalength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn substringData<Impl: IXMLDOMCharacterDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn appendData<Impl: IXMLDOMCharacterDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn insertData<Impl: IXMLDOMCharacterDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn deleteData<Impl: IXMLDOMCharacterDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn replaceData<Impl: IXMLDOMCharacterDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            data::<Impl, IMPL_OFFSET>,
            Setdata::<Impl, IMPL_OFFSET>,
            length::<Impl, IMPL_OFFSET>,
            substringData::<Impl, IMPL_OFFSET>,
            appendData::<Impl, IMPL_OFFSET>,
            insertData::<Impl, IMPL_OFFSET>,
            deleteData::<Impl, IMPL_OFFSET>,
            replaceData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMCharacterData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMCommentImpl: Sized + IXMLDOMCharacterDataImpl + IXMLDOMNodeImpl + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMCommentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCommentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMCommentVtbl {
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            data::<Impl, IMPL_OFFSET>,
            Setdata::<Impl, IMPL_OFFSET>,
            length::<Impl, IMPL_OFFSET>,
            substringData::<Impl, IMPL_OFFSET>,
            appendData::<Impl, IMPL_OFFSET>,
            insertData::<Impl, IMPL_OFFSET>,
            deleteData::<Impl, IMPL_OFFSET>,
            replaceData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMComment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocumentImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn doctype();
    fn implementation();
    fn documentElement();
    fn putref_documentElement();
    fn createElement();
    fn createDocumentFragment();
    fn createTextNode();
    fn createComment();
    fn createCDATASection();
    fn createProcessingInstruction();
    fn createAttribute();
    fn createEntityReference();
    fn getElementsByTagName();
    fn createNode();
    fn nodeFromID();
    fn load();
    fn readyState();
    fn parseError();
    fn url();
    fn r#async();
    fn Setasync();
    fn abort();
    fn loadXML();
    fn save();
    fn validateOnParse();
    fn SetvalidateOnParse();
    fn resolveExternals();
    fn SetresolveExternals();
    fn preserveWhiteSpace();
    fn SetpreserveWhiteSpace();
    fn Setonreadystatechange();
    fn Setondataavailable();
    fn Setontransformnode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMDocumentVtbl {
        unsafe extern "system" fn doctype<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn implementation<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#impl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn documentElement<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_documentElement<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createElement<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createDocumentFragment<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, docfrag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createTextNode<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, text: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createComment<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, comment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createCDATASection<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, cdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createProcessingInstruction<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createAttribute<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, attribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createEntityReference<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, entityref: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getElementsByTagName<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createNode<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn nodeFromID<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn load<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlsource: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, issuccessful: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn readyState<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn parseError<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn url<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn r#async<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isasync: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setasync<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isasync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn abort<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn loadXML<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, issuccessful: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn save<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn validateOnParse<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalidating: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetvalidateOnParse<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalidating: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn resolveExternals<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isresolving: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetresolveExternals<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isresolving: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn preserveWhiteSpace<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispreserving: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetpreserveWhiteSpace<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispreserving: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setonreadystatechange<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readystatechangesink: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setondataavailable<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ondataavailablesink: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setontransformnode<Impl: IXMLDOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ontransformnodesink: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
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
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            doctype::<Impl, IMPL_OFFSET>,
            implementation::<Impl, IMPL_OFFSET>,
            documentElement::<Impl, IMPL_OFFSET>,
            putref_documentElement::<Impl, IMPL_OFFSET>,
            createElement::<Impl, IMPL_OFFSET>,
            createDocumentFragment::<Impl, IMPL_OFFSET>,
            createTextNode::<Impl, IMPL_OFFSET>,
            createComment::<Impl, IMPL_OFFSET>,
            createCDATASection::<Impl, IMPL_OFFSET>,
            createProcessingInstruction::<Impl, IMPL_OFFSET>,
            createAttribute::<Impl, IMPL_OFFSET>,
            createEntityReference::<Impl, IMPL_OFFSET>,
            getElementsByTagName::<Impl, IMPL_OFFSET>,
            createNode::<Impl, IMPL_OFFSET>,
            nodeFromID::<Impl, IMPL_OFFSET>,
            load::<Impl, IMPL_OFFSET>,
            readyState::<Impl, IMPL_OFFSET>,
            parseError::<Impl, IMPL_OFFSET>,
            url::<Impl, IMPL_OFFSET>,
            r#async::<Impl, IMPL_OFFSET>,
            Setasync::<Impl, IMPL_OFFSET>,
            abort::<Impl, IMPL_OFFSET>,
            loadXML::<Impl, IMPL_OFFSET>,
            save::<Impl, IMPL_OFFSET>,
            validateOnParse::<Impl, IMPL_OFFSET>,
            SetvalidateOnParse::<Impl, IMPL_OFFSET>,
            resolveExternals::<Impl, IMPL_OFFSET>,
            SetresolveExternals::<Impl, IMPL_OFFSET>,
            preserveWhiteSpace::<Impl, IMPL_OFFSET>,
            SetpreserveWhiteSpace::<Impl, IMPL_OFFSET>,
            Setonreadystatechange::<Impl, IMPL_OFFSET>,
            Setondataavailable::<Impl, IMPL_OFFSET>,
            Setontransformnode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocument2Impl: Sized + IXMLDOMDocumentImpl + IXMLDOMNodeImpl + IDispatchImpl {
    fn namespaces();
    fn schemas();
    fn putref_schemas();
    fn validate();
    fn setProperty();
    fn getProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocument2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMDocument2Vtbl {
        unsafe extern "system" fn namespaces<Impl: IXMLDOMDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespacecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn schemas<Impl: IXMLDOMDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_schemas<Impl: IXMLDOMDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn validate<Impl: IXMLDOMDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setProperty<Impl: IXMLDOMDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getProperty<Impl: IXMLDOMDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            doctype::<Impl, IMPL_OFFSET>,
            implementation::<Impl, IMPL_OFFSET>,
            documentElement::<Impl, IMPL_OFFSET>,
            putref_documentElement::<Impl, IMPL_OFFSET>,
            createElement::<Impl, IMPL_OFFSET>,
            createDocumentFragment::<Impl, IMPL_OFFSET>,
            createTextNode::<Impl, IMPL_OFFSET>,
            createComment::<Impl, IMPL_OFFSET>,
            createCDATASection::<Impl, IMPL_OFFSET>,
            createProcessingInstruction::<Impl, IMPL_OFFSET>,
            createAttribute::<Impl, IMPL_OFFSET>,
            createEntityReference::<Impl, IMPL_OFFSET>,
            getElementsByTagName::<Impl, IMPL_OFFSET>,
            createNode::<Impl, IMPL_OFFSET>,
            nodeFromID::<Impl, IMPL_OFFSET>,
            load::<Impl, IMPL_OFFSET>,
            readyState::<Impl, IMPL_OFFSET>,
            parseError::<Impl, IMPL_OFFSET>,
            url::<Impl, IMPL_OFFSET>,
            r#async::<Impl, IMPL_OFFSET>,
            Setasync::<Impl, IMPL_OFFSET>,
            abort::<Impl, IMPL_OFFSET>,
            loadXML::<Impl, IMPL_OFFSET>,
            save::<Impl, IMPL_OFFSET>,
            validateOnParse::<Impl, IMPL_OFFSET>,
            SetvalidateOnParse::<Impl, IMPL_OFFSET>,
            resolveExternals::<Impl, IMPL_OFFSET>,
            SetresolveExternals::<Impl, IMPL_OFFSET>,
            preserveWhiteSpace::<Impl, IMPL_OFFSET>,
            SetpreserveWhiteSpace::<Impl, IMPL_OFFSET>,
            Setonreadystatechange::<Impl, IMPL_OFFSET>,
            Setondataavailable::<Impl, IMPL_OFFSET>,
            Setontransformnode::<Impl, IMPL_OFFSET>,
            namespaces::<Impl, IMPL_OFFSET>,
            schemas::<Impl, IMPL_OFFSET>,
            putref_schemas::<Impl, IMPL_OFFSET>,
            validate::<Impl, IMPL_OFFSET>,
            setProperty::<Impl, IMPL_OFFSET>,
            getProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocument2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocument3Impl: Sized + IXMLDOMDocument2Impl + IXMLDOMDocumentImpl + IXMLDOMNodeImpl + IDispatchImpl {
    fn validateNode();
    fn importNode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocument3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMDocument3Vtbl {
        unsafe extern "system" fn validateNode<Impl: IXMLDOMDocument3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, errorobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn importNode<Impl: IXMLDOMDocument3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, deep: i16, clone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            doctype::<Impl, IMPL_OFFSET>,
            implementation::<Impl, IMPL_OFFSET>,
            documentElement::<Impl, IMPL_OFFSET>,
            putref_documentElement::<Impl, IMPL_OFFSET>,
            createElement::<Impl, IMPL_OFFSET>,
            createDocumentFragment::<Impl, IMPL_OFFSET>,
            createTextNode::<Impl, IMPL_OFFSET>,
            createComment::<Impl, IMPL_OFFSET>,
            createCDATASection::<Impl, IMPL_OFFSET>,
            createProcessingInstruction::<Impl, IMPL_OFFSET>,
            createAttribute::<Impl, IMPL_OFFSET>,
            createEntityReference::<Impl, IMPL_OFFSET>,
            getElementsByTagName::<Impl, IMPL_OFFSET>,
            createNode::<Impl, IMPL_OFFSET>,
            nodeFromID::<Impl, IMPL_OFFSET>,
            load::<Impl, IMPL_OFFSET>,
            readyState::<Impl, IMPL_OFFSET>,
            parseError::<Impl, IMPL_OFFSET>,
            url::<Impl, IMPL_OFFSET>,
            r#async::<Impl, IMPL_OFFSET>,
            Setasync::<Impl, IMPL_OFFSET>,
            abort::<Impl, IMPL_OFFSET>,
            loadXML::<Impl, IMPL_OFFSET>,
            save::<Impl, IMPL_OFFSET>,
            validateOnParse::<Impl, IMPL_OFFSET>,
            SetvalidateOnParse::<Impl, IMPL_OFFSET>,
            resolveExternals::<Impl, IMPL_OFFSET>,
            SetresolveExternals::<Impl, IMPL_OFFSET>,
            preserveWhiteSpace::<Impl, IMPL_OFFSET>,
            SetpreserveWhiteSpace::<Impl, IMPL_OFFSET>,
            Setonreadystatechange::<Impl, IMPL_OFFSET>,
            Setondataavailable::<Impl, IMPL_OFFSET>,
            Setontransformnode::<Impl, IMPL_OFFSET>,
            namespaces::<Impl, IMPL_OFFSET>,
            schemas::<Impl, IMPL_OFFSET>,
            putref_schemas::<Impl, IMPL_OFFSET>,
            validate::<Impl, IMPL_OFFSET>,
            setProperty::<Impl, IMPL_OFFSET>,
            getProperty::<Impl, IMPL_OFFSET>,
            validateNode::<Impl, IMPL_OFFSET>,
            importNode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocument3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocumentFragmentImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocumentFragmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocumentFragmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMDocumentFragmentVtbl {
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocumentFragment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocumentTypeImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn name();
    fn entities();
    fn notations();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocumentTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocumentTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMDocumentTypeVtbl {
        unsafe extern "system" fn name<Impl: IXMLDOMDocumentTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn entities<Impl: IXMLDOMDocumentTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entitymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn notations<Impl: IXMLDOMDocumentTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notationmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            name::<Impl, IMPL_OFFSET>,
            entities::<Impl, IMPL_OFFSET>,
            notations::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocumentType as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMElementImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn tagName();
    fn getAttribute();
    fn setAttribute();
    fn removeAttribute();
    fn getAttributeNode();
    fn setAttributeNode();
    fn removeAttributeNode();
    fn getElementsByTagName();
    fn normalize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMElementVtbl {
        unsafe extern "system" fn tagName<Impl: IXMLDOMElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAttribute<Impl: IXMLDOMElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setAttribute<Impl: IXMLDOMElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeAttribute<Impl: IXMLDOMElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAttributeNode<Impl: IXMLDOMElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, attributenode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setAttributeNode<Impl: IXMLDOMElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domattribute: ::windows::core::RawPtr, attributenode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeAttributeNode<Impl: IXMLDOMElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domattribute: ::windows::core::RawPtr, attributenode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getElementsByTagName<Impl: IXMLDOMElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn normalize<Impl: IXMLDOMElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            tagName::<Impl, IMPL_OFFSET>,
            getAttribute::<Impl, IMPL_OFFSET>,
            setAttribute::<Impl, IMPL_OFFSET>,
            removeAttribute::<Impl, IMPL_OFFSET>,
            getAttributeNode::<Impl, IMPL_OFFSET>,
            setAttributeNode::<Impl, IMPL_OFFSET>,
            removeAttributeNode::<Impl, IMPL_OFFSET>,
            getElementsByTagName::<Impl, IMPL_OFFSET>,
            normalize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMEntityImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn publicId();
    fn systemId();
    fn notationName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMEntityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMEntityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMEntityVtbl {
        unsafe extern "system" fn publicId<Impl: IXMLDOMEntityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn systemId<Impl: IXMLDOMEntityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn notationName<Impl: IXMLDOMEntityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            publicId::<Impl, IMPL_OFFSET>,
            systemId::<Impl, IMPL_OFFSET>,
            notationName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMEntity as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMEntityReferenceImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMEntityReferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMEntityReferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMEntityReferenceVtbl {
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMEntityReference as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMImplementationImpl: Sized + IDispatchImpl {
    fn hasFeature();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMImplementationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMImplementationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMImplementationVtbl {
        unsafe extern "system" fn hasFeature<Impl: IXMLDOMImplementationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, version: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, hasfeature: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, hasFeature::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMImplementation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNamedNodeMapImpl: Sized + IDispatchImpl {
    fn getNamedItem();
    fn setNamedItem();
    fn removeNamedItem();
    fn item();
    fn length();
    fn getQualifiedItem();
    fn removeQualifiedItem();
    fn nextNode();
    fn reset();
    fn _newEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNamedNodeMapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNamedNodeMapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMNamedNodeMapVtbl {
        unsafe extern "system" fn getNamedItem<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nameditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setNamedItem<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newitem: ::windows::core::RawPtr, nameitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeNamedItem<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nameditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn item<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn length<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getQualifiedItem<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, qualifieditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeQualifiedItem<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, qualifieditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn nextNode<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn reset<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            getNamedItem::<Impl, IMPL_OFFSET>,
            setNamedItem::<Impl, IMPL_OFFSET>,
            removeNamedItem::<Impl, IMPL_OFFSET>,
            item::<Impl, IMPL_OFFSET>,
            length::<Impl, IMPL_OFFSET>,
            getQualifiedItem::<Impl, IMPL_OFFSET>,
            removeQualifiedItem::<Impl, IMPL_OFFSET>,
            nextNode::<Impl, IMPL_OFFSET>,
            reset::<Impl, IMPL_OFFSET>,
            _newEnum::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNamedNodeMap as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNodeImpl: Sized + IDispatchImpl {
    fn nodeName();
    fn nodeValue();
    fn SetnodeValue();
    fn nodeType();
    fn parentNode();
    fn childNodes();
    fn firstChild();
    fn lastChild();
    fn previousSibling();
    fn nextSibling();
    fn attributes();
    fn insertBefore();
    fn replaceChild();
    fn removeChild();
    fn appendChild();
    fn hasChildNodes();
    fn ownerDocument();
    fn cloneNode();
    fn nodeTypeString();
    fn text();
    fn Settext();
    fn specified();
    fn definition();
    fn nodeTypedValue();
    fn SetnodeTypedValue();
    fn dataType();
    fn SetdataType();
    fn xml();
    fn transformNode();
    fn selectNodes();
    fn selectSingleNode();
    fn parsed();
    fn namespaceURI();
    fn prefix();
    fn baseName();
    fn transformNodeToObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMNodeVtbl {
        unsafe extern "system" fn nodeName<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn nodeValue<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetnodeValue<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn nodeType<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut DOMNodeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn parentNode<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn childNodes<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn firstChild<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn lastChild<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn previousSibling<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previoussibling: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn nextSibling<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextsibling: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn attributes<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributemap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn insertBefore<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, refchild: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, outnewchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn replaceChild<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, oldchild: ::windows::core::RawPtr, outoldchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeChild<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childnode: ::windows::core::RawPtr, oldchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn appendChild<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, outnewchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn hasChildNodes<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haschild: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ownerDocument<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmldomdocument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn cloneNode<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deep: i16, cloneroot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn nodeTypeString<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodetype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn text<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Settext<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn specified<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn definition<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, definitionnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn nodeTypedValue<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetnodeTypedValue<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typedvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn dataType<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetdataType<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatypename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn xml<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn transformNode<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: ::windows::core::RawPtr, xmlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn selectNodes<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querystring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn selectSingleNode<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querystring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn parsed<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isparsed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn namespaceURI<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn prefix<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefixstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn baseName<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namestring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn transformNodeToObject<Impl: IXMLDOMNodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: ::windows::core::RawPtr, outputobject: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
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
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNodeListImpl: Sized + IDispatchImpl {
    fn item();
    fn length();
    fn nextNode();
    fn reset();
    fn _newEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNodeListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNodeListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMNodeListVtbl {
        unsafe extern "system" fn item<Impl: IXMLDOMNodeListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn length<Impl: IXMLDOMNodeListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn nextNode<Impl: IXMLDOMNodeListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn reset<Impl: IXMLDOMNodeListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLDOMNodeListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, item::<Impl, IMPL_OFFSET>, length::<Impl, IMPL_OFFSET>, nextNode::<Impl, IMPL_OFFSET>, reset::<Impl, IMPL_OFFSET>, _newEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNodeList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNotationImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn publicId();
    fn systemId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNotationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNotationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMNotationVtbl {
        unsafe extern "system" fn publicId<Impl: IXMLDOMNotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn systemId<Impl: IXMLDOMNotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            publicId::<Impl, IMPL_OFFSET>,
            systemId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNotation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMParseErrorImpl: Sized + IDispatchImpl {
    fn errorCode();
    fn url();
    fn reason();
    fn srcText();
    fn line();
    fn linepos();
    fn filepos();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMParseErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMParseErrorVtbl {
        unsafe extern "system" fn errorCode<Impl: IXMLDOMParseErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn url<Impl: IXMLDOMParseErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn reason<Impl: IXMLDOMParseErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reasonstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn srcText<Impl: IXMLDOMParseErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn line<Impl: IXMLDOMParseErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn linepos<Impl: IXMLDOMParseErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn filepos<Impl: IXMLDOMParseErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileposition: *mut i32) -> ::windows::core::HRESULT {
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
            errorCode::<Impl, IMPL_OFFSET>,
            url::<Impl, IMPL_OFFSET>,
            reason::<Impl, IMPL_OFFSET>,
            srcText::<Impl, IMPL_OFFSET>,
            line::<Impl, IMPL_OFFSET>,
            linepos::<Impl, IMPL_OFFSET>,
            filepos::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMParseError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMParseError2Impl: Sized + IXMLDOMParseErrorImpl + IDispatchImpl {
    fn errorXPath();
    fn allErrors();
    fn errorParameters();
    fn errorParametersCount();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMParseError2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMParseError2Vtbl {
        unsafe extern "system" fn errorXPath<Impl: IXMLDOMParseError2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpathexpr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn allErrors<Impl: IXMLDOMParseError2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allerrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn errorParameters<Impl: IXMLDOMParseError2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, param1: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn errorParametersCount<Impl: IXMLDOMParseError2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
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
            errorCode::<Impl, IMPL_OFFSET>,
            url::<Impl, IMPL_OFFSET>,
            reason::<Impl, IMPL_OFFSET>,
            srcText::<Impl, IMPL_OFFSET>,
            line::<Impl, IMPL_OFFSET>,
            linepos::<Impl, IMPL_OFFSET>,
            filepos::<Impl, IMPL_OFFSET>,
            errorXPath::<Impl, IMPL_OFFSET>,
            allErrors::<Impl, IMPL_OFFSET>,
            errorParameters::<Impl, IMPL_OFFSET>,
            errorParametersCount::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMParseError2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMParseErrorCollectionImpl: Sized + IDispatchImpl {
    fn item();
    fn length();
    fn next();
    fn reset();
    fn _newEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMParseErrorCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseErrorCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMParseErrorCollectionVtbl {
        unsafe extern "system" fn item<Impl: IXMLDOMParseErrorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, error: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn length<Impl: IXMLDOMParseErrorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn next<Impl: IXMLDOMParseErrorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn reset<Impl: IXMLDOMParseErrorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLDOMParseErrorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, item::<Impl, IMPL_OFFSET>, length::<Impl, IMPL_OFFSET>, next::<Impl, IMPL_OFFSET>, reset::<Impl, IMPL_OFFSET>, _newEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMParseErrorCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMProcessingInstructionImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn target();
    fn data();
    fn Setdata();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMProcessingInstructionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMProcessingInstructionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMProcessingInstructionVtbl {
        unsafe extern "system" fn target<Impl: IXMLDOMProcessingInstructionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn data<Impl: IXMLDOMProcessingInstructionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setdata<Impl: IXMLDOMProcessingInstructionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            target::<Impl, IMPL_OFFSET>,
            data::<Impl, IMPL_OFFSET>,
            Setdata::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMProcessingInstruction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMSchemaCollectionImpl: Sized + IDispatchImpl {
    fn add();
    fn get();
    fn remove();
    fn length();
    fn namespaceURI();
    fn addCollection();
    fn _newEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMSchemaCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMSchemaCollectionVtbl {
        unsafe extern "system" fn add<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, var: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn get<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, schemanode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn remove<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn length<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn namespaceURI<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, length: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addCollection<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            add::<Impl, IMPL_OFFSET>,
            get::<Impl, IMPL_OFFSET>,
            remove::<Impl, IMPL_OFFSET>,
            length::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            addCollection::<Impl, IMPL_OFFSET>,
            _newEnum::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMSchemaCollection2Impl: Sized + IXMLDOMSchemaCollectionImpl + IDispatchImpl {
    fn validate();
    fn SetvalidateOnLoad();
    fn validateOnLoad();
    fn getSchema();
    fn getDeclaration();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMSchemaCollection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMSchemaCollection2Vtbl {
        unsafe extern "system" fn validate<Impl: IXMLDOMSchemaCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetvalidateOnLoad<Impl: IXMLDOMSchemaCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, validateonload: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn validateOnLoad<Impl: IXMLDOMSchemaCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, validateonload: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getSchema<Impl: IXMLDOMSchemaCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, schema: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getDeclaration<Impl: IXMLDOMSchemaCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            add::<Impl, IMPL_OFFSET>,
            get::<Impl, IMPL_OFFSET>,
            remove::<Impl, IMPL_OFFSET>,
            length::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            addCollection::<Impl, IMPL_OFFSET>,
            _newEnum::<Impl, IMPL_OFFSET>,
            validate::<Impl, IMPL_OFFSET>,
            SetvalidateOnLoad::<Impl, IMPL_OFFSET>,
            validateOnLoad::<Impl, IMPL_OFFSET>,
            getSchema::<Impl, IMPL_OFFSET>,
            getDeclaration::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMSelectionImpl: Sized + IXMLDOMNodeListImpl + IDispatchImpl {
    fn expr();
    fn Setexpr();
    fn context();
    fn putref_context();
    fn peekNode();
    fn matches();
    fn removeNext();
    fn removeAll();
    fn clone();
    fn getProperty();
    fn setProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMSelectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMSelectionVtbl {
        unsafe extern "system" fn expr<Impl: IXMLDOMSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expression: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setexpr<Impl: IXMLDOMSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expression: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn context<Impl: IXMLDOMSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_context<Impl: IXMLDOMSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn peekNode<Impl: IXMLDOMSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn matches<Impl: IXMLDOMSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeNext<Impl: IXMLDOMSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeAll<Impl: IXMLDOMSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn clone<Impl: IXMLDOMSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getProperty<Impl: IXMLDOMSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setProperty<Impl: IXMLDOMSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
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
            item::<Impl, IMPL_OFFSET>,
            length::<Impl, IMPL_OFFSET>,
            nextNode::<Impl, IMPL_OFFSET>,
            reset::<Impl, IMPL_OFFSET>,
            _newEnum::<Impl, IMPL_OFFSET>,
            expr::<Impl, IMPL_OFFSET>,
            Setexpr::<Impl, IMPL_OFFSET>,
            context::<Impl, IMPL_OFFSET>,
            putref_context::<Impl, IMPL_OFFSET>,
            peekNode::<Impl, IMPL_OFFSET>,
            matches::<Impl, IMPL_OFFSET>,
            removeNext::<Impl, IMPL_OFFSET>,
            removeAll::<Impl, IMPL_OFFSET>,
            clone::<Impl, IMPL_OFFSET>,
            getProperty::<Impl, IMPL_OFFSET>,
            setProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMSelection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMTextImpl: Sized + IXMLDOMCharacterDataImpl + IXMLDOMNodeImpl + IDispatchImpl {
    fn splitText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMTextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMTextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMTextVtbl {
        unsafe extern "system" fn splitText<Impl: IXMLDOMTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, righthandtextnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            data::<Impl, IMPL_OFFSET>,
            Setdata::<Impl, IMPL_OFFSET>,
            length::<Impl, IMPL_OFFSET>,
            substringData::<Impl, IMPL_OFFSET>,
            appendData::<Impl, IMPL_OFFSET>,
            insertData::<Impl, IMPL_OFFSET>,
            deleteData::<Impl, IMPL_OFFSET>,
            replaceData::<Impl, IMPL_OFFSET>,
            splitText::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMText as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDSOControlImpl: Sized + IDispatchImpl {
    fn XMLDocument();
    fn SetXMLDocument();
    fn JavaDSOCompatible();
    fn SetJavaDSOCompatible();
    fn readyState();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDSOControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDSOControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDSOControlVtbl {
        unsafe extern "system" fn XMLDocument<Impl: IXMLDSOControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetXMLDocument<Impl: IXMLDSOControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdoc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn JavaDSOCompatible<Impl: IXMLDSOControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJavaDSOCompatible<Impl: IXMLDSOControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn readyState<Impl: IXMLDSOControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut i32) -> ::windows::core::HRESULT {
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
            XMLDocument::<Impl, IMPL_OFFSET>,
            SetXMLDocument::<Impl, IMPL_OFFSET>,
            JavaDSOCompatible::<Impl, IMPL_OFFSET>,
            SetJavaDSOCompatible::<Impl, IMPL_OFFSET>,
            readyState::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDSOControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDocumentImpl: Sized + IDispatchImpl {
    fn root();
    fn fileSize();
    fn fileModifiedDate();
    fn fileUpdatedDate();
    fn URL();
    fn SetURL();
    fn mimeType();
    fn readyState();
    fn charset();
    fn Setcharset();
    fn version();
    fn doctype();
    fn dtdURL();
    fn createElement();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDocumentVtbl {
        unsafe extern "system" fn root<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fileSize<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fileModifiedDate<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fileUpdatedDate<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn URL<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetURL<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn mimeType<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn readyState<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn charset<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setcharset<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn version<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn doctype<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn dtdURL<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createElement<Impl: IXMLDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtype: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var1: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppelem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            root::<Impl, IMPL_OFFSET>,
            fileSize::<Impl, IMPL_OFFSET>,
            fileModifiedDate::<Impl, IMPL_OFFSET>,
            fileUpdatedDate::<Impl, IMPL_OFFSET>,
            URL::<Impl, IMPL_OFFSET>,
            SetURL::<Impl, IMPL_OFFSET>,
            mimeType::<Impl, IMPL_OFFSET>,
            readyState::<Impl, IMPL_OFFSET>,
            charset::<Impl, IMPL_OFFSET>,
            Setcharset::<Impl, IMPL_OFFSET>,
            version::<Impl, IMPL_OFFSET>,
            doctype::<Impl, IMPL_OFFSET>,
            dtdURL::<Impl, IMPL_OFFSET>,
            createElement::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDocument2Impl: Sized + IDispatchImpl {
    fn root();
    fn fileSize();
    fn fileModifiedDate();
    fn fileUpdatedDate();
    fn URL();
    fn SetURL();
    fn mimeType();
    fn readyState();
    fn charset();
    fn Setcharset();
    fn version();
    fn doctype();
    fn dtdURL();
    fn createElement();
    fn r#async();
    fn Setasync();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDocument2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDocument2Vtbl {
        unsafe extern "system" fn root<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fileSize<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fileModifiedDate<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn fileUpdatedDate<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn URL<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetURL<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn mimeType<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn readyState<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn charset<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setcharset<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn version<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn doctype<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn dtdURL<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createElement<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtype: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var1: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppelem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn r#async<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pf: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setasync<Impl: IXMLDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, f: i16) -> ::windows::core::HRESULT {
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
            root::<Impl, IMPL_OFFSET>,
            fileSize::<Impl, IMPL_OFFSET>,
            fileModifiedDate::<Impl, IMPL_OFFSET>,
            fileUpdatedDate::<Impl, IMPL_OFFSET>,
            URL::<Impl, IMPL_OFFSET>,
            SetURL::<Impl, IMPL_OFFSET>,
            mimeType::<Impl, IMPL_OFFSET>,
            readyState::<Impl, IMPL_OFFSET>,
            charset::<Impl, IMPL_OFFSET>,
            Setcharset::<Impl, IMPL_OFFSET>,
            version::<Impl, IMPL_OFFSET>,
            doctype::<Impl, IMPL_OFFSET>,
            dtdURL::<Impl, IMPL_OFFSET>,
            createElement::<Impl, IMPL_OFFSET>,
            r#async::<Impl, IMPL_OFFSET>,
            Setasync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDocument2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLElementImpl: Sized + IDispatchImpl {
    fn tagName();
    fn SettagName();
    fn parent();
    fn setAttribute();
    fn getAttribute();
    fn removeAttribute();
    fn children();
    fn r#type();
    fn text();
    fn Settext();
    fn addChild();
    fn removeChild();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLElementVtbl {
        unsafe extern "system" fn tagName<Impl: IXMLElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SettagName<Impl: IXMLElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn parent<Impl: IXMLElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setAttribute<Impl: IXMLElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAttribute<Impl: IXMLElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeAttribute<Impl: IXMLElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn children<Impl: IXMLElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn r#type<Impl: IXMLElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn text<Impl: IXMLElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Settext<Impl: IXMLElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addChild<Impl: IXMLElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr, lindex: i32, lreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeChild<Impl: IXMLElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            tagName::<Impl, IMPL_OFFSET>,
            SettagName::<Impl, IMPL_OFFSET>,
            parent::<Impl, IMPL_OFFSET>,
            setAttribute::<Impl, IMPL_OFFSET>,
            getAttribute::<Impl, IMPL_OFFSET>,
            removeAttribute::<Impl, IMPL_OFFSET>,
            children::<Impl, IMPL_OFFSET>,
            r#type::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            addChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLElement2Impl: Sized + IDispatchImpl {
    fn tagName();
    fn SettagName();
    fn parent();
    fn setAttribute();
    fn getAttribute();
    fn removeAttribute();
    fn children();
    fn r#type();
    fn text();
    fn Settext();
    fn addChild();
    fn removeChild();
    fn attributes();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLElement2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLElement2Vtbl {
        unsafe extern "system" fn tagName<Impl: IXMLElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SettagName<Impl: IXMLElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn parent<Impl: IXMLElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setAttribute<Impl: IXMLElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAttribute<Impl: IXMLElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeAttribute<Impl: IXMLElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn children<Impl: IXMLElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn r#type<Impl: IXMLElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn text<Impl: IXMLElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Settext<Impl: IXMLElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addChild<Impl: IXMLElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr, lindex: i32, lreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn removeChild<Impl: IXMLElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn attributes<Impl: IXMLElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            tagName::<Impl, IMPL_OFFSET>,
            SettagName::<Impl, IMPL_OFFSET>,
            parent::<Impl, IMPL_OFFSET>,
            setAttribute::<Impl, IMPL_OFFSET>,
            getAttribute::<Impl, IMPL_OFFSET>,
            removeAttribute::<Impl, IMPL_OFFSET>,
            children::<Impl, IMPL_OFFSET>,
            r#type::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            addChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLElement2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLElementCollectionImpl: Sized + IDispatchImpl {
    fn Setlength();
    fn length();
    fn _newEnum();
    fn item();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLElementCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElementCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLElementCollectionVtbl {
        unsafe extern "system" fn Setlength<Impl: IXMLElementCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn length<Impl: IXMLElementCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLElementCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn item<Impl: IXMLElementCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var1: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var2: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppdisp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Setlength::<Impl, IMPL_OFFSET>, length::<Impl, IMPL_OFFSET>, _newEnum::<Impl, IMPL_OFFSET>, item::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLElementCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXMLErrorImpl: Sized {
    fn GetErrorInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IXMLErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLErrorVtbl {
        unsafe extern "system" fn GetErrorInfo<Impl: IXMLErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorreturn: *mut XML_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetErrorInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLHTTPRequestImpl: Sized + IDispatchImpl {
    fn open();
    fn setRequestHeader();
    fn getResponseHeader();
    fn getAllResponseHeaders();
    fn send();
    fn abort();
    fn status();
    fn statusText();
    fn responseXML();
    fn responseText();
    fn responseBody();
    fn responseStream();
    fn readyState();
    fn Setonreadystatechange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLHTTPRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLHTTPRequestVtbl {
        unsafe extern "system" fn open<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethod: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varasync: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstruser: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrpassword: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setRequestHeader<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getResponseHeader<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAllResponseHeaders<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrheaders: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn send<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn abort<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn status<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn statusText<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn responseXML<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbody: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn responseText<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn responseBody<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn responseStream<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn readyState<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setonreadystatechange<Impl: IXMLHTTPRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadystatesink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            open::<Impl, IMPL_OFFSET>,
            setRequestHeader::<Impl, IMPL_OFFSET>,
            getResponseHeader::<Impl, IMPL_OFFSET>,
            getAllResponseHeaders::<Impl, IMPL_OFFSET>,
            send::<Impl, IMPL_OFFSET>,
            abort::<Impl, IMPL_OFFSET>,
            status::<Impl, IMPL_OFFSET>,
            statusText::<Impl, IMPL_OFFSET>,
            responseXML::<Impl, IMPL_OFFSET>,
            responseText::<Impl, IMPL_OFFSET>,
            responseBody::<Impl, IMPL_OFFSET>,
            responseStream::<Impl, IMPL_OFFSET>,
            readyState::<Impl, IMPL_OFFSET>,
            Setonreadystatechange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest2Impl: Sized {
    fn Open();
    fn Send();
    fn Abort();
    fn SetCookie();
    fn SetCustomResponseStream();
    fn SetProperty();
    fn SetRequestHeader();
    fn GetAllResponseHeaders();
    fn GetCookie();
    fn GetResponseHeader();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLHTTPRequest2Vtbl {
        unsafe extern "system" fn Open<Impl: IXMLHTTPRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmethod: super::super::super::Foundation::PWSTR, pwszurl: super::super::super::Foundation::PWSTR, pstatuscallback: ::windows::core::RawPtr, pwszusername: super::super::super::Foundation::PWSTR, pwszpassword: super::super::super::Foundation::PWSTR, pwszproxyusername: super::super::super::Foundation::PWSTR, pwszproxypassword: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Send<Impl: IXMLHTTPRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: ::windows::core::RawPtr, cbbody: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Abort<Impl: IXMLHTTPRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCookie<Impl: IXMLHTTPRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcookie: *const XHR_COOKIE, pdwcookiestate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCustomResponseStream<Impl: IXMLHTTPRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psequentialstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IXMLHTTPRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRequestHeader<Impl: IXMLHTTPRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszheader: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllResponseHeaders<Impl: IXMLHTTPRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszheaders: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCookie<Impl: IXMLHTTPRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, pwszname: super::super::super::Foundation::PWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResponseHeader<Impl: IXMLHTTPRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszheader: super::super::super::Foundation::PWSTR, ppwszvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Send::<Impl, IMPL_OFFSET>,
            Abort::<Impl, IMPL_OFFSET>,
            SetCookie::<Impl, IMPL_OFFSET>,
            SetCustomResponseStream::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            SetRequestHeader::<Impl, IMPL_OFFSET>,
            GetAllResponseHeaders::<Impl, IMPL_OFFSET>,
            GetCookie::<Impl, IMPL_OFFSET>,
            GetResponseHeader::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest2CallbackImpl: Sized {
    fn OnRedirect();
    fn OnHeadersAvailable();
    fn OnDataAvailable();
    fn OnResponseReceived();
    fn OnError();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest2CallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2CallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLHTTPRequest2CallbackVtbl {
        unsafe extern "system" fn OnRedirect<Impl: IXMLHTTPRequest2CallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, pwszredirecturl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnHeadersAvailable<Impl: IXMLHTTPRequest2CallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, dwstatus: u32, pwszstatus: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDataAvailable<Impl: IXMLHTTPRequest2CallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, presponsestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnResponseReceived<Impl: IXMLHTTPRequest2CallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, presponsestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnError<Impl: IXMLHTTPRequest2CallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnRedirect::<Impl, IMPL_OFFSET>, OnHeadersAvailable::<Impl, IMPL_OFFSET>, OnDataAvailable::<Impl, IMPL_OFFSET>, OnResponseReceived::<Impl, IMPL_OFFSET>, OnError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest2Callback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest3Impl: Sized + IXMLHTTPRequest2Impl {
    fn SetClientCertificate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLHTTPRequest3Vtbl {
        unsafe extern "system" fn SetClientCertificate<Impl: IXMLHTTPRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Send::<Impl, IMPL_OFFSET>,
            Abort::<Impl, IMPL_OFFSET>,
            SetCookie::<Impl, IMPL_OFFSET>,
            SetCustomResponseStream::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            SetRequestHeader::<Impl, IMPL_OFFSET>,
            GetAllResponseHeaders::<Impl, IMPL_OFFSET>,
            GetCookie::<Impl, IMPL_OFFSET>,
            GetResponseHeader::<Impl, IMPL_OFFSET>,
            SetClientCertificate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest3CallbackImpl: Sized + IXMLHTTPRequest2CallbackImpl {
    fn OnServerCertificateReceived();
    fn OnClientCertificateRequested();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest3CallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest3CallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLHTTPRequest3CallbackVtbl {
        unsafe extern "system" fn OnServerCertificateReceived<Impl: IXMLHTTPRequest3CallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnClientCertificateRequested<Impl: IXMLHTTPRequest3CallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnRedirect::<Impl, IMPL_OFFSET>, OnHeadersAvailable::<Impl, IMPL_OFFSET>, OnDataAvailable::<Impl, IMPL_OFFSET>, OnResponseReceived::<Impl, IMPL_OFFSET>, OnError::<Impl, IMPL_OFFSET>, OnServerCertificateReceived::<Impl, IMPL_OFFSET>, OnClientCertificateRequested::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3Callback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLHttpRequestImpl: Sized + IDispatchImpl {
    fn open();
    fn setRequestHeader();
    fn getResponseHeader();
    fn getAllResponseHeaders();
    fn send();
    fn abort();
    fn status();
    fn statusText();
    fn responseXML();
    fn responseText();
    fn responseBody();
    fn responseStream();
    fn readyState();
    fn Setonreadystatechange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLHttpRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLHttpRequestVtbl {
        unsafe extern "system" fn open<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethod: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varasync: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstruser: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrpassword: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setRequestHeader<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getResponseHeader<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn getAllResponseHeaders<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrheaders: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn send<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn abort<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn status<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn statusText<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn responseXML<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbody: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn responseText<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn responseBody<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn responseStream<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn readyState<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setonreadystatechange<Impl: IXMLHttpRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadystatesink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            open::<Impl, IMPL_OFFSET>,
            setRequestHeader::<Impl, IMPL_OFFSET>,
            getResponseHeader::<Impl, IMPL_OFFSET>,
            getAllResponseHeaders::<Impl, IMPL_OFFSET>,
            send::<Impl, IMPL_OFFSET>,
            abort::<Impl, IMPL_OFFSET>,
            status::<Impl, IMPL_OFFSET>,
            statusText::<Impl, IMPL_OFFSET>,
            responseXML::<Impl, IMPL_OFFSET>,
            responseText::<Impl, IMPL_OFFSET>,
            responseBody::<Impl, IMPL_OFFSET>,
            responseStream::<Impl, IMPL_OFFSET>,
            readyState::<Impl, IMPL_OFFSET>,
            Setonreadystatechange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHttpRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXSLProcessorImpl: Sized + IDispatchImpl {
    fn Setinput();
    fn input();
    fn ownerTemplate();
    fn setStartMode();
    fn startMode();
    fn startModeURI();
    fn Setoutput();
    fn output();
    fn transform();
    fn reset();
    fn readyState();
    fn addParameter();
    fn addObject();
    fn stylesheet();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXSLProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXSLProcessorVtbl {
        unsafe extern "system" fn Setinput<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn input<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ownerTemplate<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn setStartMode<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startMode<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn startModeURI<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Setoutput<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, output: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn output<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn transform<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdone: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn reset<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn readyState<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadystate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addParameter<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, parameter: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn addObject<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, obj: ::windows::core::RawPtr, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn stylesheet<Impl: IXSLProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Setinput::<Impl, IMPL_OFFSET>,
            input::<Impl, IMPL_OFFSET>,
            ownerTemplate::<Impl, IMPL_OFFSET>,
            setStartMode::<Impl, IMPL_OFFSET>,
            startMode::<Impl, IMPL_OFFSET>,
            startModeURI::<Impl, IMPL_OFFSET>,
            Setoutput::<Impl, IMPL_OFFSET>,
            output::<Impl, IMPL_OFFSET>,
            transform::<Impl, IMPL_OFFSET>,
            reset::<Impl, IMPL_OFFSET>,
            readyState::<Impl, IMPL_OFFSET>,
            addParameter::<Impl, IMPL_OFFSET>,
            addObject::<Impl, IMPL_OFFSET>,
            stylesheet::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXSLProcessor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXSLTemplateImpl: Sized + IDispatchImpl {
    fn putref_stylesheet();
    fn stylesheet();
    fn createProcessor();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXSLTemplateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXSLTemplateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXSLTemplateVtbl {
        unsafe extern "system" fn putref_stylesheet<Impl: IXSLTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn stylesheet<Impl: IXSLTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn createProcessor<Impl: IXSLTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprocessor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, putref_stylesheet::<Impl, IMPL_OFFSET>, stylesheet::<Impl, IMPL_OFFSET>, createProcessor::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXSLTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXTLRuntimeImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn uniqueID();
    fn depth();
    fn childNumber();
    fn ancestorChildNumber();
    fn absoluteChildNumber();
    fn formatIndex();
    fn formatNumber();
    fn formatDate();
    fn formatTime();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXTLRuntimeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXTLRuntimeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXTLRuntimeVtbl {
        unsafe extern "system" fn uniqueID<Impl: IXTLRuntimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn depth<Impl: IXTLRuntimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pdepth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn childNumber<Impl: IXTLRuntimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ancestorChildNumber<Impl: IXTLRuntimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnodename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pnode: ::windows::core::RawPtr, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn absoluteChildNumber<Impl: IXTLRuntimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn formatIndex<Impl: IXTLRuntimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn formatNumber<Impl: IXTLRuntimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dblnumber: f64, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn formatDate<Impl: IXTLRuntimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardate: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vardestlocale: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn formatTime<Impl: IXTLRuntimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartime: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vardestlocale: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            nodeName::<Impl, IMPL_OFFSET>,
            nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType::<Impl, IMPL_OFFSET>,
            parentNode::<Impl, IMPL_OFFSET>,
            childNodes::<Impl, IMPL_OFFSET>,
            firstChild::<Impl, IMPL_OFFSET>,
            lastChild::<Impl, IMPL_OFFSET>,
            previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling::<Impl, IMPL_OFFSET>,
            attributes::<Impl, IMPL_OFFSET>,
            insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild::<Impl, IMPL_OFFSET>,
            removeChild::<Impl, IMPL_OFFSET>,
            appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString::<Impl, IMPL_OFFSET>,
            text::<Impl, IMPL_OFFSET>,
            Settext::<Impl, IMPL_OFFSET>,
            specified::<Impl, IMPL_OFFSET>,
            definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType::<Impl, IMPL_OFFSET>,
            SetdataType::<Impl, IMPL_OFFSET>,
            xml::<Impl, IMPL_OFFSET>,
            transformNode::<Impl, IMPL_OFFSET>,
            selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed::<Impl, IMPL_OFFSET>,
            namespaceURI::<Impl, IMPL_OFFSET>,
            prefix::<Impl, IMPL_OFFSET>,
            baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject::<Impl, IMPL_OFFSET>,
            uniqueID::<Impl, IMPL_OFFSET>,
            depth::<Impl, IMPL_OFFSET>,
            childNumber::<Impl, IMPL_OFFSET>,
            ancestorChildNumber::<Impl, IMPL_OFFSET>,
            absoluteChildNumber::<Impl, IMPL_OFFSET>,
            formatIndex::<Impl, IMPL_OFFSET>,
            formatNumber::<Impl, IMPL_OFFSET>,
            formatDate::<Impl, IMPL_OFFSET>,
            formatTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXTLRuntime as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait XMLDOMDocumentEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl XMLDOMDocumentEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: XMLDOMDocumentEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> XMLDOMDocumentEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<XMLDOMDocumentEvents as ::windows::core::Interface>::IID
    }
}
