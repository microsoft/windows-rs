#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXAttributes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn addAttribute(&mut self, struri: &super::super::super::Foundation::BSTR, strlocalname: &super::super::super::Foundation::BSTR, strqname: &super::super::super::Foundation::BSTR, strtype: &super::super::super::Foundation::BSTR, strvalue: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn addAttributeFromIndex(&mut self, varatts: &super::super::super::System::Com::VARIANT, nindex: i32) -> ::windows::core::Result<()>;
    fn clear(&mut self) -> ::windows::core::Result<()>;
    fn removeAttribute(&mut self, nindex: i32) -> ::windows::core::Result<()>;
    fn setAttribute(&mut self, nindex: i32, struri: &super::super::super::Foundation::BSTR, strlocalname: &super::super::super::Foundation::BSTR, strqname: &super::super::super::Foundation::BSTR, strtype: &super::super::super::Foundation::BSTR, strvalue: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setAttributes(&mut self, varatts: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setLocalName(&mut self, nindex: i32, strlocalname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setQName(&mut self, nindex: i32, strqname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setType(&mut self, nindex: i32, strtype: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setURI(&mut self, nindex: i32, struri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setValue(&mut self, nindex: i32, strvalue: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXAttributes_Vtbl {
        unsafe extern "system" fn addAttribute<Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addAttribute(::core::mem::transmute_copy(&struri), ::core::mem::transmute_copy(&strlocalname), ::core::mem::transmute_copy(&strqname), ::core::mem::transmute_copy(&strtype), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn addAttributeFromIndex<Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varatts: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, nindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addAttributeFromIndex(::core::mem::transmute_copy(&varatts), ::core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn clear<Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).clear().into()
        }
        unsafe extern "system" fn removeAttribute<Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).removeAttribute(::core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn setAttribute<Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setAttribute(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&struri), ::core::mem::transmute_copy(&strlocalname), ::core::mem::transmute_copy(&strqname), ::core::mem::transmute_copy(&strtype), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn setAttributes<Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varatts: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setAttributes(::core::mem::transmute_copy(&varatts)).into()
        }
        unsafe extern "system" fn setLocalName<Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setLocalName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&strlocalname)).into()
        }
        unsafe extern "system" fn setQName<Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setQName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&strqname)).into()
        }
        unsafe extern "system" fn setType<Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setType(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&strtype)).into()
        }
        unsafe extern "system" fn setURI<Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setURI(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&struri)).into()
        }
        unsafe extern "system" fn setValue<Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setValue(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&strvalue)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            addAttribute: addAttribute::<Impl, IMPL_OFFSET>,
            addAttributeFromIndex: addAttributeFromIndex::<Impl, IMPL_OFFSET>,
            clear: clear::<Impl, IMPL_OFFSET>,
            removeAttribute: removeAttribute::<Impl, IMPL_OFFSET>,
            setAttribute: setAttribute::<Impl, IMPL_OFFSET>,
            setAttributes: setAttributes::<Impl, IMPL_OFFSET>,
            setLocalName: setLocalName::<Impl, IMPL_OFFSET>,
            setQName: setQName::<Impl, IMPL_OFFSET>,
            setType: setType::<Impl, IMPL_OFFSET>,
            setURI: setURI::<Impl, IMPL_OFFSET>,
            setValue: setValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXAttributes as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMXNamespaceManager_Impl: Sized {
    fn putAllowOverride(&mut self, foverride: i16) -> ::windows::core::Result<()>;
    fn getAllowOverride(&mut self) -> ::windows::core::Result<i16>;
    fn reset(&mut self) -> ::windows::core::Result<()>;
    fn pushContext(&mut self) -> ::windows::core::Result<()>;
    fn pushNodeContext(&mut self, contextnode: &::core::option::Option<IXMLDOMNode>, fdeep: i16) -> ::windows::core::Result<()>;
    fn popContext(&mut self) -> ::windows::core::Result<()>;
    fn declarePrefix(&mut self, prefix: super::super::super::Foundation::PWSTR, namespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn getDeclaredPrefix(&mut self, nindex: i32, pwchprefix: super::super::super::Foundation::PWSTR, pcchprefix: *mut i32) -> ::windows::core::Result<()>;
    fn getPrefix(&mut self, pwsznamespaceuri: super::super::super::Foundation::PWSTR, nindex: i32, pwchprefix: super::super::super::Foundation::PWSTR, pcchprefix: *mut i32) -> ::windows::core::Result<()>;
    fn getURI(&mut self, pwchprefix: super::super::super::Foundation::PWSTR, pcontextnode: &::core::option::Option<IXMLDOMNode>, pwchuri: super::super::super::Foundation::PWSTR, pcchuri: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMXNamespaceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespaceManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXNamespaceManager_Vtbl {
        unsafe extern "system" fn putAllowOverride<Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putAllowOverride(::core::mem::transmute_copy(&foverride)).into()
        }
        unsafe extern "system" fn getAllowOverride<Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAllowOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *foverride = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).reset().into()
        }
        unsafe extern "system" fn pushContext<Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).pushContext().into()
        }
        unsafe extern "system" fn pushNodeContext<Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextnode: ::windows::core::RawPtr, fdeep: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).pushNodeContext(::core::mem::transmute(&contextnode), ::core::mem::transmute_copy(&fdeep)).into()
        }
        unsafe extern "system" fn popContext<Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).popContext().into()
        }
        unsafe extern "system" fn declarePrefix<Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: super::super::super::Foundation::PWSTR, namespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).declarePrefix(::core::mem::transmute_copy(&prefix), ::core::mem::transmute_copy(&namespaceuri)).into()
        }
        unsafe extern "system" fn getDeclaredPrefix<Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pwchprefix: super::super::super::Foundation::PWSTR, pcchprefix: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getDeclaredPrefix(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pwchprefix), ::core::mem::transmute_copy(&pcchprefix)).into()
        }
        unsafe extern "system" fn getPrefix<Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznamespaceuri: super::super::super::Foundation::PWSTR, nindex: i32, pwchprefix: super::super::super::Foundation::PWSTR, pcchprefix: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getPrefix(::core::mem::transmute_copy(&pwsznamespaceuri), ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pwchprefix), ::core::mem::transmute_copy(&pcchprefix)).into()
        }
        unsafe extern "system" fn getURI<Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: super::super::super::Foundation::PWSTR, pcontextnode: ::windows::core::RawPtr, pwchuri: super::super::super::Foundation::PWSTR, pcchuri: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getURI(::core::mem::transmute_copy(&pwchprefix), ::core::mem::transmute(&pcontextnode), ::core::mem::transmute_copy(&pwchuri), ::core::mem::transmute_copy(&pcchuri)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            putAllowOverride: putAllowOverride::<Impl, IMPL_OFFSET>,
            getAllowOverride: getAllowOverride::<Impl, IMPL_OFFSET>,
            reset: reset::<Impl, IMPL_OFFSET>,
            pushContext: pushContext::<Impl, IMPL_OFFSET>,
            pushNodeContext: pushNodeContext::<Impl, IMPL_OFFSET>,
            popContext: popContext::<Impl, IMPL_OFFSET>,
            declarePrefix: declarePrefix::<Impl, IMPL_OFFSET>,
            getDeclaredPrefix: getDeclaredPrefix::<Impl, IMPL_OFFSET>,
            getPrefix: getPrefix::<Impl, IMPL_OFFSET>,
            getURI: getURI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXNamespaceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXNamespacePrefixes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn item(&mut self, index: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn length(&mut self) -> ::windows::core::Result<i32>;
    fn _newEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXNamespacePrefixes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespacePrefixes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXNamespacePrefixes_Vtbl {
        unsafe extern "system" fn item<Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, prefix: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *prefix = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            item: item::<Impl, IMPL_OFFSET>,
            length: length::<Impl, IMPL_OFFSET>,
            _newEnum: _newEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXNamespacePrefixes as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXReaderControl_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn abort(&mut self) -> ::windows::core::Result<()>;
    fn resume(&mut self) -> ::windows::core::Result<()>;
    fn suspend(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXReaderControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXReaderControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXReaderControl_Vtbl {
        unsafe extern "system" fn abort<Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).abort().into()
        }
        unsafe extern "system" fn resume<Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).resume().into()
        }
        unsafe extern "system" fn suspend<Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).suspend().into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            abort: abort::<Impl, IMPL_OFFSET>,
            resume: resume::<Impl, IMPL_OFFSET>,
            suspend: suspend::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXReaderControl as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXSchemaDeclHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn schemaElementDecl(&mut self, oschemaelement: &::core::option::Option<ISchemaElement>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXSchemaDeclHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXSchemaDeclHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXSchemaDeclHandler_Vtbl {
        unsafe extern "system" fn schemaElementDecl<Impl: IMXSchemaDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oschemaelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).schemaElementDecl(::core::mem::transmute(&oschemaelement)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            schemaElementDecl: schemaElementDecl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXSchemaDeclHandler as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXWriter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setoutput(&mut self, vardestination: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn output(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn Setencoding(&mut self, strencoding: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn encoding(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetbyteOrderMark(&mut self, fwritebyteordermark: i16) -> ::windows::core::Result<()>;
    fn byteOrderMark(&mut self) -> ::windows::core::Result<i16>;
    fn Setindent(&mut self, findentmode: i16) -> ::windows::core::Result<()>;
    fn indent(&mut self) -> ::windows::core::Result<i16>;
    fn Setstandalone(&mut self, fvalue: i16) -> ::windows::core::Result<()>;
    fn standalone(&mut self) -> ::windows::core::Result<i16>;
    fn SetomitXMLDeclaration(&mut self, fvalue: i16) -> ::windows::core::Result<()>;
    fn omitXMLDeclaration(&mut self) -> ::windows::core::Result<i16>;
    fn Setversion(&mut self, strversion: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn version(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetdisableOutputEscaping(&mut self, fvalue: i16) -> ::windows::core::Result<()>;
    fn disableOutputEscaping(&mut self) -> ::windows::core::Result<i16>;
    fn flush(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXWriter_Vtbl {
        unsafe extern "system" fn Setoutput<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestination: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setoutput(::core::mem::transmute_copy(&vardestination)).into()
        }
        unsafe extern "system" fn output<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestination: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).output() {
                ::core::result::Result::Ok(ok__) => {
                    *vardestination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setencoding<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencoding: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setencoding(::core::mem::transmute_copy(&strencoding)).into()
        }
        unsafe extern "system" fn encoding<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencoding: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).encoding() {
                ::core::result::Result::Ok(ok__) => {
                    *strencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbyteOrderMark<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetbyteOrderMark(::core::mem::transmute_copy(&fwritebyteordermark)).into()
        }
        unsafe extern "system" fn byteOrderMark<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).byteOrderMark() {
                ::core::result::Result::Ok(ok__) => {
                    *fwritebyteordermark = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setindent<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findentmode: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setindent(::core::mem::transmute_copy(&findentmode)).into()
        }
        unsafe extern "system" fn indent<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findentmode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).indent() {
                ::core::result::Result::Ok(ok__) => {
                    *findentmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setstandalone<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setstandalone(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn standalone<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).standalone() {
                ::core::result::Result::Ok(ok__) => {
                    *fvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetomitXMLDeclaration<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetomitXMLDeclaration(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn omitXMLDeclaration<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).omitXMLDeclaration() {
                ::core::result::Result::Ok(ok__) => {
                    *fvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setversion<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strversion: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setversion(::core::mem::transmute_copy(&strversion)).into()
        }
        unsafe extern "system" fn version<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strversion: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).version() {
                ::core::result::Result::Ok(ok__) => {
                    *strversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdisableOutputEscaping<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetdisableOutputEscaping(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn disableOutputEscaping<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).disableOutputEscaping() {
                ::core::result::Result::Ok(ok__) => {
                    *fvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn flush<Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).flush().into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Setoutput: Setoutput::<Impl, IMPL_OFFSET>,
            output: output::<Impl, IMPL_OFFSET>,
            Setencoding: Setencoding::<Impl, IMPL_OFFSET>,
            encoding: encoding::<Impl, IMPL_OFFSET>,
            SetbyteOrderMark: SetbyteOrderMark::<Impl, IMPL_OFFSET>,
            byteOrderMark: byteOrderMark::<Impl, IMPL_OFFSET>,
            Setindent: Setindent::<Impl, IMPL_OFFSET>,
            indent: indent::<Impl, IMPL_OFFSET>,
            Setstandalone: Setstandalone::<Impl, IMPL_OFFSET>,
            standalone: standalone::<Impl, IMPL_OFFSET>,
            SetomitXMLDeclaration: SetomitXMLDeclaration::<Impl, IMPL_OFFSET>,
            omitXMLDeclaration: omitXMLDeclaration::<Impl, IMPL_OFFSET>,
            Setversion: Setversion::<Impl, IMPL_OFFSET>,
            version: version::<Impl, IMPL_OFFSET>,
            SetdisableOutputEscaping: SetdisableOutputEscaping::<Impl, IMPL_OFFSET>,
            disableOutputEscaping: disableOutputEscaping::<Impl, IMPL_OFFSET>,
            flush: flush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXWriter as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXXMLFilter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getFeature(&mut self, strname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn putFeature(&mut self, strname: &super::super::super::Foundation::BSTR, fvalue: i16) -> ::windows::core::Result<()>;
    fn getProperty(&mut self, strname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn putProperty(&mut self, strname: &super::super::super::Foundation::BSTR, varvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn entityResolver(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_entityResolver(&mut self, oresolver: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn contentHandler(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_contentHandler(&mut self, ohandler: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn dtdHandler(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_dtdHandler(&mut self, ohandler: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn errorHandler(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_errorHandler(&mut self, ohandler: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXXMLFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMXXMLFilter_Vtbl {
        unsafe extern "system" fn getFeature<Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getFeature(::core::mem::transmute_copy(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *fvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putFeature(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn getProperty<Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getProperty(::core::mem::transmute_copy(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *varvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putProperty(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn entityResolver<Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).entityResolver() {
                ::core::result::Result::Ok(ok__) => {
                    *oresolver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_entityResolver<Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_entityResolver(::core::mem::transmute(&oresolver)).into()
        }
        unsafe extern "system" fn contentHandler<Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).contentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ohandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_contentHandler<Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_contentHandler(::core::mem::transmute(&ohandler)).into()
        }
        unsafe extern "system" fn dtdHandler<Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).dtdHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ohandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_dtdHandler(::core::mem::transmute(&ohandler)).into()
        }
        unsafe extern "system" fn errorHandler<Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).errorHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ohandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_errorHandler<Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_errorHandler(::core::mem::transmute(&ohandler)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            getFeature: getFeature::<Impl, IMPL_OFFSET>,
            putFeature: putFeature::<Impl, IMPL_OFFSET>,
            getProperty: getProperty::<Impl, IMPL_OFFSET>,
            putProperty: putProperty::<Impl, IMPL_OFFSET>,
            entityResolver: entityResolver::<Impl, IMPL_OFFSET>,
            putref_entityResolver: putref_entityResolver::<Impl, IMPL_OFFSET>,
            contentHandler: contentHandler::<Impl, IMPL_OFFSET>,
            putref_contentHandler: putref_contentHandler::<Impl, IMPL_OFFSET>,
            dtdHandler: dtdHandler::<Impl, IMPL_OFFSET>,
            putref_dtdHandler: putref_dtdHandler::<Impl, IMPL_OFFSET>,
            errorHandler: errorHandler::<Impl, IMPL_OFFSET>,
            putref_errorHandler: putref_errorHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXXMLFilter as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISAXAttributes_Impl: Sized {
    fn getLength(&mut self) -> ::windows::core::Result<i32>;
    fn getURI(&mut self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> ::windows::core::Result<()>;
    fn getLocalName(&mut self, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> ::windows::core::Result<()>;
    fn getQName(&mut self, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::Result<()>;
    fn getName(&mut self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::Result<()>;
    fn getIndexFromName(&mut self, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32) -> ::windows::core::Result<i32>;
    fn getIndexFromQName(&mut self, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32) -> ::windows::core::Result<i32>;
    fn getType(&mut self, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::Result<()>;
    fn getTypeFromName(&mut self, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::Result<()>;
    fn getTypeFromQName(&mut self, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::Result<()>;
    fn getValue(&mut self, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::Result<()>;
    fn getValueFromName(&mut self, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::Result<()>;
    fn getValueFromQName(&mut self, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISAXAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXAttributes_Vtbl {
        unsafe extern "system" fn getLength<Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pnlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getURI(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchuri), ::core::mem::transmute_copy(&pcchuri)).into()
        }
        unsafe extern "system" fn getLocalName<Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getLocalName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchlocalname), ::core::mem::transmute_copy(&pcchlocalname)).into()
        }
        unsafe extern "system" fn getQName<Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getQName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchqname), ::core::mem::transmute_copy(&pcchqname)).into()
        }
        unsafe extern "system" fn getName<Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchuri), ::core::mem::transmute_copy(&pcchuri), ::core::mem::transmute_copy(&ppwchlocalname), ::core::mem::transmute_copy(&pcchlocalname), ::core::mem::transmute_copy(&ppwchqname), ::core::mem::transmute_copy(&pcchqname)).into()
        }
        unsafe extern "system" fn getIndexFromName<Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, pnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getIndexFromName(::core::mem::transmute_copy(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute_copy(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromQName<Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, pnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getIndexFromQName(::core::mem::transmute_copy(&pwchqname), ::core::mem::transmute_copy(&cchqname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getType<Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getType(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getTypeFromName<Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getTypeFromName(::core::mem::transmute_copy(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute_copy(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getTypeFromQName<Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getTypeFromQName(::core::mem::transmute_copy(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getValue<Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getValue(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into()
        }
        unsafe extern "system" fn getValueFromName<Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getValueFromName(::core::mem::transmute_copy(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute_copy(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into()
        }
        unsafe extern "system" fn getValueFromQName<Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getValueFromQName(::core::mem::transmute_copy(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            getLength: getLength::<Impl, IMPL_OFFSET>,
            getURI: getURI::<Impl, IMPL_OFFSET>,
            getLocalName: getLocalName::<Impl, IMPL_OFFSET>,
            getQName: getQName::<Impl, IMPL_OFFSET>,
            getName: getName::<Impl, IMPL_OFFSET>,
            getIndexFromName: getIndexFromName::<Impl, IMPL_OFFSET>,
            getIndexFromQName: getIndexFromQName::<Impl, IMPL_OFFSET>,
            getType: getType::<Impl, IMPL_OFFSET>,
            getTypeFromName: getTypeFromName::<Impl, IMPL_OFFSET>,
            getTypeFromQName: getTypeFromQName::<Impl, IMPL_OFFSET>,
            getValue: getValue::<Impl, IMPL_OFFSET>,
            getValueFromName: getValueFromName::<Impl, IMPL_OFFSET>,
            getValueFromQName: getValueFromQName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXAttributes as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISAXContentHandler_Impl: Sized {
    fn putDocumentLocator(&mut self, plocator: &::core::option::Option<ISAXLocator>) -> ::windows::core::Result<()>;
    fn startDocument(&mut self) -> ::windows::core::Result<()>;
    fn endDocument(&mut self) -> ::windows::core::Result<()>;
    fn startPrefixMapping(&mut self, pwchprefix: super::super::super::Foundation::PWSTR, cchprefix: i32, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32) -> ::windows::core::Result<()>;
    fn endPrefixMapping(&mut self, pwchprefix: super::super::super::Foundation::PWSTR, cchprefix: i32) -> ::windows::core::Result<()>;
    fn startElement(&mut self, pwchnamespaceuri: super::super::super::Foundation::PWSTR, cchnamespaceuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, pattributes: &::core::option::Option<ISAXAttributes>) -> ::windows::core::Result<()>;
    fn endElement(&mut self, pwchnamespaceuri: super::super::super::Foundation::PWSTR, cchnamespaceuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32) -> ::windows::core::Result<()>;
    fn characters(&mut self, pwchchars: super::super::super::Foundation::PWSTR, cchchars: i32) -> ::windows::core::Result<()>;
    fn ignorableWhitespace(&mut self, pwchchars: super::super::super::Foundation::PWSTR, cchchars: i32) -> ::windows::core::Result<()>;
    fn processingInstruction(&mut self, pwchtarget: super::super::super::Foundation::PWSTR, cchtarget: i32, pwchdata: super::super::super::Foundation::PWSTR, cchdata: i32) -> ::windows::core::Result<()>;
    fn skippedEntity(&mut self, pwchname: super::super::super::Foundation::PWSTR, cchname: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISAXContentHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXContentHandler_Vtbl {
        unsafe extern "system" fn putDocumentLocator<Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putDocumentLocator(::core::mem::transmute(&plocator)).into()
        }
        unsafe extern "system" fn startDocument<Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startDocument().into()
        }
        unsafe extern "system" fn endDocument<Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).endDocument().into()
        }
        unsafe extern "system" fn startPrefixMapping<Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: super::super::super::Foundation::PWSTR, cchprefix: i32, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startPrefixMapping(::core::mem::transmute_copy(&pwchprefix), ::core::mem::transmute_copy(&cchprefix), ::core::mem::transmute_copy(&pwchuri), ::core::mem::transmute_copy(&cchuri)).into()
        }
        unsafe extern "system" fn endPrefixMapping<Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: super::super::super::Foundation::PWSTR, cchprefix: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).endPrefixMapping(::core::mem::transmute_copy(&pwchprefix), ::core::mem::transmute_copy(&cchprefix)).into()
        }
        unsafe extern "system" fn startElement<Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: super::super::super::Foundation::PWSTR, cchnamespaceuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startElement(::core::mem::transmute_copy(&pwchnamespaceuri), ::core::mem::transmute_copy(&cchnamespaceuri), ::core::mem::transmute_copy(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute_copy(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::core::mem::transmute(&pattributes)).into()
        }
        unsafe extern "system" fn endElement<Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: super::super::super::Foundation::PWSTR, cchnamespaceuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).endElement(::core::mem::transmute_copy(&pwchnamespaceuri), ::core::mem::transmute_copy(&cchnamespaceuri), ::core::mem::transmute_copy(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute_copy(&pwchqname), ::core::mem::transmute_copy(&cchqname)).into()
        }
        unsafe extern "system" fn characters<Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: super::super::super::Foundation::PWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).characters(::core::mem::transmute_copy(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into()
        }
        unsafe extern "system" fn ignorableWhitespace<Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: super::super::super::Foundation::PWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ignorableWhitespace(::core::mem::transmute_copy(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into()
        }
        unsafe extern "system" fn processingInstruction<Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchtarget: super::super::super::Foundation::PWSTR, cchtarget: i32, pwchdata: super::super::super::Foundation::PWSTR, cchdata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).processingInstruction(::core::mem::transmute_copy(&pwchtarget), ::core::mem::transmute_copy(&cchtarget), ::core::mem::transmute_copy(&pwchdata), ::core::mem::transmute_copy(&cchdata)).into()
        }
        unsafe extern "system" fn skippedEntity<Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).skippedEntity(::core::mem::transmute_copy(&pwchname), ::core::mem::transmute_copy(&cchname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            putDocumentLocator: putDocumentLocator::<Impl, IMPL_OFFSET>,
            startDocument: startDocument::<Impl, IMPL_OFFSET>,
            endDocument: endDocument::<Impl, IMPL_OFFSET>,
            startPrefixMapping: startPrefixMapping::<Impl, IMPL_OFFSET>,
            endPrefixMapping: endPrefixMapping::<Impl, IMPL_OFFSET>,
            startElement: startElement::<Impl, IMPL_OFFSET>,
            endElement: endElement::<Impl, IMPL_OFFSET>,
            characters: characters::<Impl, IMPL_OFFSET>,
            ignorableWhitespace: ignorableWhitespace::<Impl, IMPL_OFFSET>,
            processingInstruction: processingInstruction::<Impl, IMPL_OFFSET>,
            skippedEntity: skippedEntity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXContentHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISAXDTDHandler_Impl: Sized {
    fn notationDecl(&mut self, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32) -> ::windows::core::Result<()>;
    fn unparsedEntityDecl(&mut self, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32, pwchnotationname: super::super::super::Foundation::PWSTR, cchnotationname: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISAXDTDHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXDTDHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXDTDHandler_Vtbl {
        unsafe extern "system" fn notationDecl<Impl: ISAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).notationDecl(::core::mem::transmute_copy(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute_copy(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into()
        }
        unsafe extern "system" fn unparsedEntityDecl<Impl: ISAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32, pwchnotationname: super::super::super::Foundation::PWSTR, cchnotationname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).unparsedEntityDecl(::core::mem::transmute_copy(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute_copy(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid), ::core::mem::transmute_copy(&pwchnotationname), ::core::mem::transmute_copy(&cchnotationname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            notationDecl: notationDecl::<Impl, IMPL_OFFSET>,
            unparsedEntityDecl: unparsedEntityDecl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXDTDHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISAXDeclHandler_Impl: Sized {
    fn elementDecl(&mut self, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchmodel: super::super::super::Foundation::PWSTR, cchmodel: i32) -> ::windows::core::Result<()>;
    fn attributeDecl(&mut self, pwchelementname: super::super::super::Foundation::PWSTR, cchelementname: i32, pwchattributename: super::super::super::Foundation::PWSTR, cchattributename: i32, pwchtype: super::super::super::Foundation::PWSTR, cchtype: i32, pwchvaluedefault: super::super::super::Foundation::PWSTR, cchvaluedefault: i32, pwchvalue: super::super::super::Foundation::PWSTR, cchvalue: i32) -> ::windows::core::Result<()>;
    fn internalEntityDecl(&mut self, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchvalue: super::super::super::Foundation::PWSTR, cchvalue: i32) -> ::windows::core::Result<()>;
    fn externalEntityDecl(&mut self, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISAXDeclHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXDeclHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXDeclHandler_Vtbl {
        unsafe extern "system" fn elementDecl<Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchmodel: super::super::super::Foundation::PWSTR, cchmodel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).elementDecl(::core::mem::transmute_copy(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pwchmodel), ::core::mem::transmute_copy(&cchmodel)).into()
        }
        unsafe extern "system" fn attributeDecl<Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchelementname: super::super::super::Foundation::PWSTR, cchelementname: i32, pwchattributename: super::super::super::Foundation::PWSTR, cchattributename: i32, pwchtype: super::super::super::Foundation::PWSTR, cchtype: i32, pwchvaluedefault: super::super::super::Foundation::PWSTR, cchvaluedefault: i32, pwchvalue: super::super::super::Foundation::PWSTR, cchvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).attributeDecl(::core::mem::transmute_copy(&pwchelementname), ::core::mem::transmute_copy(&cchelementname), ::core::mem::transmute_copy(&pwchattributename), ::core::mem::transmute_copy(&cchattributename), ::core::mem::transmute_copy(&pwchtype), ::core::mem::transmute_copy(&cchtype), ::core::mem::transmute_copy(&pwchvaluedefault), ::core::mem::transmute_copy(&cchvaluedefault), ::core::mem::transmute_copy(&pwchvalue), ::core::mem::transmute_copy(&cchvalue)).into()
        }
        unsafe extern "system" fn internalEntityDecl<Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchvalue: super::super::super::Foundation::PWSTR, cchvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).internalEntityDecl(::core::mem::transmute_copy(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pwchvalue), ::core::mem::transmute_copy(&cchvalue)).into()
        }
        unsafe extern "system" fn externalEntityDecl<Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).externalEntityDecl(::core::mem::transmute_copy(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute_copy(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            elementDecl: elementDecl::<Impl, IMPL_OFFSET>,
            attributeDecl: attributeDecl::<Impl, IMPL_OFFSET>,
            internalEntityDecl: internalEntityDecl::<Impl, IMPL_OFFSET>,
            externalEntityDecl: externalEntityDecl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXDeclHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISAXEntityResolver_Impl: Sized {
    fn resolveEntity(&mut self, pwchpublicid: super::super::super::Foundation::PWSTR, pwchsystemid: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISAXEntityResolver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXEntityResolver_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXEntityResolver_Vtbl {
        unsafe extern "system" fn resolveEntity<Impl: ISAXEntityResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchpublicid: super::super::super::Foundation::PWSTR, pwchsystemid: super::super::super::Foundation::PWSTR, pvarinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).resolveEntity(::core::mem::transmute_copy(&pwchpublicid), ::core::mem::transmute_copy(&pwchsystemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarinput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), resolveEntity: resolveEntity::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXEntityResolver as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISAXErrorHandler_Impl: Sized {
    fn error(&mut self, plocator: &::core::option::Option<ISAXLocator>, pwcherrormessage: super::super::super::Foundation::PWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn fatalError(&mut self, plocator: &::core::option::Option<ISAXLocator>, pwcherrormessage: super::super::super::Foundation::PWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn ignorableWarning(&mut self, plocator: &::core::option::Option<ISAXLocator>, pwcherrormessage: super::super::super::Foundation::PWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISAXErrorHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXErrorHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXErrorHandler_Vtbl {
        unsafe extern "system" fn error<Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr, pwcherrormessage: super::super::super::Foundation::PWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).error(::core::mem::transmute(&plocator), ::core::mem::transmute_copy(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into()
        }
        unsafe extern "system" fn fatalError<Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr, pwcherrormessage: super::super::super::Foundation::PWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).fatalError(::core::mem::transmute(&plocator), ::core::mem::transmute_copy(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into()
        }
        unsafe extern "system" fn ignorableWarning<Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr, pwcherrormessage: super::super::super::Foundation::PWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ignorableWarning(::core::mem::transmute(&plocator), ::core::mem::transmute_copy(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            error: error::<Impl, IMPL_OFFSET>,
            fatalError: fatalError::<Impl, IMPL_OFFSET>,
            ignorableWarning: ignorableWarning::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXErrorHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISAXLexicalHandler_Impl: Sized {
    fn startDTD(&mut self, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32) -> ::windows::core::Result<()>;
    fn endDTD(&mut self) -> ::windows::core::Result<()>;
    fn startEntity(&mut self, pwchname: super::super::super::Foundation::PWSTR, cchname: i32) -> ::windows::core::Result<()>;
    fn endEntity(&mut self, pwchname: super::super::super::Foundation::PWSTR, cchname: i32) -> ::windows::core::Result<()>;
    fn startCDATA(&mut self) -> ::windows::core::Result<()>;
    fn endCDATA(&mut self) -> ::windows::core::Result<()>;
    fn comment(&mut self, pwchchars: super::super::super::Foundation::PWSTR, cchchars: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISAXLexicalHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLexicalHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXLexicalHandler_Vtbl {
        unsafe extern "system" fn startDTD<Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startDTD(::core::mem::transmute_copy(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute_copy(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into()
        }
        unsafe extern "system" fn endDTD<Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).endDTD().into()
        }
        unsafe extern "system" fn startEntity<Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startEntity(::core::mem::transmute_copy(&pwchname), ::core::mem::transmute_copy(&cchname)).into()
        }
        unsafe extern "system" fn endEntity<Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).endEntity(::core::mem::transmute_copy(&pwchname), ::core::mem::transmute_copy(&cchname)).into()
        }
        unsafe extern "system" fn startCDATA<Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startCDATA().into()
        }
        unsafe extern "system" fn endCDATA<Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).endCDATA().into()
        }
        unsafe extern "system" fn comment<Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: super::super::super::Foundation::PWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).comment(::core::mem::transmute_copy(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            startDTD: startDTD::<Impl, IMPL_OFFSET>,
            endDTD: endDTD::<Impl, IMPL_OFFSET>,
            startEntity: startEntity::<Impl, IMPL_OFFSET>,
            endEntity: endEntity::<Impl, IMPL_OFFSET>,
            startCDATA: startCDATA::<Impl, IMPL_OFFSET>,
            endCDATA: endCDATA::<Impl, IMPL_OFFSET>,
            comment: comment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXLexicalHandler as ::windows::core::Interface>::IID
    }
}
pub trait ISAXLocator_Impl: Sized {
    fn getColumnNumber(&mut self) -> ::windows::core::Result<i32>;
    fn getLineNumber(&mut self) -> ::windows::core::Result<i32>;
    fn getPublicId(&mut self) -> ::windows::core::Result<*mut u16>;
    fn getSystemId(&mut self) -> ::windows::core::Result<*mut u16>;
}
impl ISAXLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLocator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXLocator_Vtbl {
        unsafe extern "system" fn getColumnNumber<Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncolumn: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getColumnNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pncolumn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLineNumber<Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnline: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getLineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pnline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPublicId<Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchpublicid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getPublicId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwchpublicid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSystemId<Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchsystemid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getSystemId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwchsystemid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            getColumnNumber: getColumnNumber::<Impl, IMPL_OFFSET>,
            getLineNumber: getLineNumber::<Impl, IMPL_OFFSET>,
            getPublicId: getPublicId::<Impl, IMPL_OFFSET>,
            getSystemId: getSystemId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISAXXMLFilter_Impl: Sized + ISAXXMLReader_Impl {
    fn getParent(&mut self) -> ::windows::core::Result<ISAXXMLReader>;
    fn putParent(&mut self, preader: &::core::option::Option<ISAXXMLReader>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISAXXMLFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLFilter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXXMLFilter_Vtbl {
        unsafe extern "system" fn getParent<Impl: ISAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getParent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putParent<Impl: ISAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putParent(::core::mem::transmute(&preader)).into()
        }
        Self {
            base: ISAXXMLReader_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            getParent: getParent::<Impl, IMPL_OFFSET>,
            putParent: putParent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXXMLFilter as ::windows::core::Interface>::IID || iid == &<ISAXXMLReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISAXXMLReader_Impl: Sized {
    fn getFeature(&mut self, pwchname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<i16>;
    fn putFeature(&mut self, pwchname: super::super::super::Foundation::PWSTR, vfvalue: i16) -> ::windows::core::Result<()>;
    fn getProperty(&mut self, pwchname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn putProperty(&mut self, pwchname: super::super::super::Foundation::PWSTR, varvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getEntityResolver(&mut self) -> ::windows::core::Result<ISAXEntityResolver>;
    fn putEntityResolver(&mut self, presolver: &::core::option::Option<ISAXEntityResolver>) -> ::windows::core::Result<()>;
    fn getContentHandler(&mut self) -> ::windows::core::Result<ISAXContentHandler>;
    fn putContentHandler(&mut self, phandler: &::core::option::Option<ISAXContentHandler>) -> ::windows::core::Result<()>;
    fn getDTDHandler(&mut self) -> ::windows::core::Result<ISAXDTDHandler>;
    fn putDTDHandler(&mut self, phandler: &::core::option::Option<ISAXDTDHandler>) -> ::windows::core::Result<()>;
    fn getErrorHandler(&mut self) -> ::windows::core::Result<ISAXErrorHandler>;
    fn putErrorHandler(&mut self, phandler: &::core::option::Option<ISAXErrorHandler>) -> ::windows::core::Result<()>;
    fn getBaseURL(&mut self) -> ::windows::core::Result<*mut u16>;
    fn putBaseURL(&mut self, pwchbaseurl: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn getSecureBaseURL(&mut self) -> ::windows::core::Result<*mut u16>;
    fn putSecureBaseURL(&mut self, pwchsecurebaseurl: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn parse(&mut self, varinput: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn parseURL(&mut self, pwchurl: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISAXXMLReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISAXXMLReader_Vtbl {
        unsafe extern "system" fn getFeature<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, pvfvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getFeature(::core::mem::transmute_copy(&pwchname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvfvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, vfvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putFeature(::core::mem::transmute_copy(&pwchname), ::core::mem::transmute_copy(&vfvalue)).into()
        }
        unsafe extern "system" fn getProperty<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getProperty(::core::mem::transmute_copy(&pwchname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, varvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putProperty(::core::mem::transmute_copy(&pwchname), ::core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn getEntityResolver<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresolver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getEntityResolver() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresolver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putEntityResolver<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putEntityResolver(::core::mem::transmute(&presolver)).into()
        }
        unsafe extern "system" fn getContentHandler<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getContentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *pphandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putContentHandler<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putContentHandler(::core::mem::transmute(&phandler)).into()
        }
        unsafe extern "system" fn getDTDHandler<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getDTDHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *pphandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putDTDHandler<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putDTDHandler(::core::mem::transmute(&phandler)).into()
        }
        unsafe extern "system" fn getErrorHandler<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getErrorHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *pphandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putErrorHandler<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putErrorHandler(::core::mem::transmute(&phandler)).into()
        }
        unsafe extern "system" fn getBaseURL<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchbaseurl: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getBaseURL() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwchbaseurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putBaseURL<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchbaseurl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putBaseURL(::core::mem::transmute_copy(&pwchbaseurl)).into()
        }
        unsafe extern "system" fn getSecureBaseURL<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchsecurebaseurl: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getSecureBaseURL() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwchsecurebaseurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putSecureBaseURL<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchsecurebaseurl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putSecureBaseURL(::core::mem::transmute_copy(&pwchsecurebaseurl)).into()
        }
        unsafe extern "system" fn parse<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinput: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).parse(::core::mem::transmute_copy(&varinput)).into()
        }
        unsafe extern "system" fn parseURL<Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchurl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).parseURL(::core::mem::transmute_copy(&pwchurl)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            getFeature: getFeature::<Impl, IMPL_OFFSET>,
            putFeature: putFeature::<Impl, IMPL_OFFSET>,
            getProperty: getProperty::<Impl, IMPL_OFFSET>,
            putProperty: putProperty::<Impl, IMPL_OFFSET>,
            getEntityResolver: getEntityResolver::<Impl, IMPL_OFFSET>,
            putEntityResolver: putEntityResolver::<Impl, IMPL_OFFSET>,
            getContentHandler: getContentHandler::<Impl, IMPL_OFFSET>,
            putContentHandler: putContentHandler::<Impl, IMPL_OFFSET>,
            getDTDHandler: getDTDHandler::<Impl, IMPL_OFFSET>,
            putDTDHandler: putDTDHandler::<Impl, IMPL_OFFSET>,
            getErrorHandler: getErrorHandler::<Impl, IMPL_OFFSET>,
            putErrorHandler: putErrorHandler::<Impl, IMPL_OFFSET>,
            getBaseURL: getBaseURL::<Impl, IMPL_OFFSET>,
            putBaseURL: putBaseURL::<Impl, IMPL_OFFSET>,
            getSecureBaseURL: getSecureBaseURL::<Impl, IMPL_OFFSET>,
            putSecureBaseURL: putSecureBaseURL::<Impl, IMPL_OFFSET>,
            parse: parse::<Impl, IMPL_OFFSET>,
            parseURL: parseURL::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXXMLReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchema_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn targetNamespace(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn version(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn types(&mut self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn elements(&mut self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn attributes(&mut self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn attributeGroups(&mut self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn modelGroups(&mut self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn notations(&mut self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn schemaLocations(&mut self) -> ::windows::core::Result<ISchemaStringCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchema_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchema_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchema_Vtbl {
        unsafe extern "system" fn targetNamespace<Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetnamespace: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).targetNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    *targetnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn version<Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).version() {
                ::core::result::Result::Ok(ok__) => {
                    *version = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn types<Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, types: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).types() {
                ::core::result::Result::Ok(ok__) => {
                    *types = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn elements<Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elements: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).elements() {
                ::core::result::Result::Ok(ok__) => {
                    *elements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributeGroups<Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).attributeGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *attributegroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn modelGroups<Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).modelGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *modelgroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notations<Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).notations() {
                ::core::result::Result::Ok(ok__) => {
                    *notations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schemaLocations<Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, schemalocations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).schemaLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *schemalocations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            targetNamespace: targetNamespace::<Impl, IMPL_OFFSET>,
            version: version::<Impl, IMPL_OFFSET>,
            types: types::<Impl, IMPL_OFFSET>,
            elements: elements::<Impl, IMPL_OFFSET>,
            attributes: attributes::<Impl, IMPL_OFFSET>,
            attributeGroups: attributeGroups::<Impl, IMPL_OFFSET>,
            modelGroups: modelGroups::<Impl, IMPL_OFFSET>,
            notations: notations::<Impl, IMPL_OFFSET>,
            schemaLocations: schemaLocations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchema as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaAny_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl + ISchemaParticle_Impl {
    fn namespaces(&mut self) -> ::windows::core::Result<ISchemaStringCollection>;
    fn processContents(&mut self) -> ::windows::core::Result<SCHEMAPROCESSCONTENTS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaAny_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAny_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaAny_Vtbl {
        unsafe extern "system" fn namespaces<Impl: ISchemaAny_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).namespaces() {
                ::core::result::Result::Ok(ok__) => {
                    *namespaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn processContents<Impl: ISchemaAny_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processcontents: *mut SCHEMAPROCESSCONTENTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).processContents() {
                ::core::result::Result::Ok(ok__) => {
                    *processcontents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaParticle_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            namespaces: namespaces::<Impl, IMPL_OFFSET>,
            processContents: processContents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaAny as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID || iid == &<ISchemaParticle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaAttribute_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn r#type(&mut self) -> ::windows::core::Result<ISchemaType>;
    fn scope(&mut self) -> ::windows::core::Result<ISchemaComplexType>;
    fn defaultValue(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fixedValue(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn r#use(&mut self) -> ::windows::core::Result<SCHEMAUSE>;
    fn isReference(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaAttribute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttribute_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaAttribute_Vtbl {
        unsafe extern "system" fn r#type<Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).r#type() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scope<Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).scope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn defaultValue<Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).defaultValue() {
                ::core::result::Result::Ok(ok__) => {
                    *defaultvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fixedValue<Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fixedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *fixedvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#use<Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#use: *mut SCHEMAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).r#use() {
                ::core::result::Result::Ok(ok__) => {
                    *r#use = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isReference<Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isReference() {
                ::core::result::Result::Ok(ok__) => {
                    *reference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            r#type: r#type::<Impl, IMPL_OFFSET>,
            scope: scope::<Impl, IMPL_OFFSET>,
            defaultValue: defaultValue::<Impl, IMPL_OFFSET>,
            fixedValue: fixedValue::<Impl, IMPL_OFFSET>,
            r#use: r#use::<Impl, IMPL_OFFSET>,
            isReference: isReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaAttribute as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaAttributeGroup_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn anyAttribute(&mut self) -> ::windows::core::Result<ISchemaAny>;
    fn attributes(&mut self) -> ::windows::core::Result<ISchemaItemCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaAttributeGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttributeGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaAttributeGroup_Vtbl {
        unsafe extern "system" fn anyAttribute<Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anyattribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).anyAttribute() {
                ::core::result::Result::Ok(ok__) => {
                    *anyattribute = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            anyAttribute: anyAttribute::<Impl, IMPL_OFFSET>,
            attributes: attributes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaAttributeGroup as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaComplexType_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl + ISchemaType_Impl {
    fn isAbstract(&mut self) -> ::windows::core::Result<i16>;
    fn anyAttribute(&mut self) -> ::windows::core::Result<ISchemaAny>;
    fn attributes(&mut self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn contentType(&mut self) -> ::windows::core::Result<SCHEMACONTENTTYPE>;
    fn contentModel(&mut self) -> ::windows::core::Result<ISchemaModelGroup>;
    fn prohibitedSubstitutions(&mut self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaComplexType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaComplexType_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaComplexType_Vtbl {
        unsafe extern "system" fn isAbstract<Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#abstract: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isAbstract() {
                ::core::result::Result::Ok(ok__) => {
                    *r#abstract = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn anyAttribute<Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anyattribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).anyAttribute() {
                ::core::result::Result::Ok(ok__) => {
                    *anyattribute = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentType<Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut SCHEMACONTENTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).contentType() {
                ::core::result::Result::Ok(ok__) => {
                    *contenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentModel<Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentmodel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).contentModel() {
                ::core::result::Result::Ok(ok__) => {
                    *contentmodel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn prohibitedSubstitutions<Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibited: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).prohibitedSubstitutions() {
                ::core::result::Result::Ok(ok__) => {
                    *prohibited = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaType_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            isAbstract: isAbstract::<Impl, IMPL_OFFSET>,
            anyAttribute: anyAttribute::<Impl, IMPL_OFFSET>,
            attributes: attributes::<Impl, IMPL_OFFSET>,
            contentType: contentType::<Impl, IMPL_OFFSET>,
            contentModel: contentModel::<Impl, IMPL_OFFSET>,
            prohibitedSubstitutions: prohibitedSubstitutions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaComplexType as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID || iid == &<ISchemaType as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaElement_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl + ISchemaParticle_Impl {
    fn r#type(&mut self) -> ::windows::core::Result<ISchemaType>;
    fn scope(&mut self) -> ::windows::core::Result<ISchemaComplexType>;
    fn defaultValue(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fixedValue(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn isNillable(&mut self) -> ::windows::core::Result<i16>;
    fn identityConstraints(&mut self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn substitutionGroup(&mut self) -> ::windows::core::Result<ISchemaElement>;
    fn substitutionGroupExclusions(&mut self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
    fn disallowedSubstitutions(&mut self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
    fn isAbstract(&mut self) -> ::windows::core::Result<i16>;
    fn isReference(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaElement_Vtbl {
        unsafe extern "system" fn r#type<Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).r#type() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scope<Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).scope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn defaultValue<Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).defaultValue() {
                ::core::result::Result::Ok(ok__) => {
                    *defaultvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fixedValue<Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fixedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *fixedvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isNillable<Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nillable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isNillable() {
                ::core::result::Result::Ok(ok__) => {
                    *nillable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn identityConstraints<Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, constraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).identityConstraints() {
                ::core::result::Result::Ok(ok__) => {
                    *constraints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substitutionGroup<Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).substitutionGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substitutionGroupExclusions<Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, exclusions: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).substitutionGroupExclusions() {
                ::core::result::Result::Ok(ok__) => {
                    *exclusions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn disallowedSubstitutions<Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowed: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).disallowedSubstitutions() {
                ::core::result::Result::Ok(ok__) => {
                    *disallowed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isAbstract<Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#abstract: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isAbstract() {
                ::core::result::Result::Ok(ok__) => {
                    *r#abstract = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isReference<Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isReference() {
                ::core::result::Result::Ok(ok__) => {
                    *reference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaParticle_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            r#type: r#type::<Impl, IMPL_OFFSET>,
            scope: scope::<Impl, IMPL_OFFSET>,
            defaultValue: defaultValue::<Impl, IMPL_OFFSET>,
            fixedValue: fixedValue::<Impl, IMPL_OFFSET>,
            isNillable: isNillable::<Impl, IMPL_OFFSET>,
            identityConstraints: identityConstraints::<Impl, IMPL_OFFSET>,
            substitutionGroup: substitutionGroup::<Impl, IMPL_OFFSET>,
            substitutionGroupExclusions: substitutionGroupExclusions::<Impl, IMPL_OFFSET>,
            disallowedSubstitutions: disallowedSubstitutions::<Impl, IMPL_OFFSET>,
            isAbstract: isAbstract::<Impl, IMPL_OFFSET>,
            isReference: isReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaElement as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID || iid == &<ISchemaParticle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaIdentityConstraint_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn selector(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fields(&mut self) -> ::windows::core::Result<ISchemaStringCollection>;
    fn referencedKey(&mut self) -> ::windows::core::Result<ISchemaIdentityConstraint>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaIdentityConstraint_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaIdentityConstraint_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaIdentityConstraint_Vtbl {
        unsafe extern "system" fn selector<Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selector: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).selector() {
                ::core::result::Result::Ok(ok__) => {
                    *selector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fields<Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fields: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fields() {
                ::core::result::Result::Ok(ok__) => {
                    *fields = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn referencedKey<Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).referencedKey() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            selector: selector::<Impl, IMPL_OFFSET>,
            fields: fields::<Impl, IMPL_OFFSET>,
            referencedKey: referencedKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaIdentityConstraint as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaItem_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn name(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn namespaceURI(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn schema(&mut self) -> ::windows::core::Result<ISchema>;
    fn id(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn itemType(&mut self) -> ::windows::core::Result<SOMITEMTYPE>;
    fn unhandledAttributes(&mut self) -> ::windows::core::Result<IVBSAXAttributes>;
    fn writeAnnotation(&mut self, annotationsink: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaItem_Vtbl {
        unsafe extern "system" fn name<Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn namespaceURI<Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).namespaceURI() {
                ::core::result::Result::Ok(ok__) => {
                    *namespaceuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schema<Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, schema: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).schema() {
                ::core::result::Result::Ok(ok__) => {
                    *schema = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn id<Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemType<Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemtype: *mut SOMITEMTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).itemType() {
                ::core::result::Result::Ok(ok__) => {
                    *itemtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn unhandledAttributes<Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).unhandledAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn writeAnnotation<Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationsink: *mut ::core::ffi::c_void, iswritten: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).writeAnnotation(::core::mem::transmute(&annotationsink)) {
                ::core::result::Result::Ok(ok__) => {
                    *iswritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            name: name::<Impl, IMPL_OFFSET>,
            namespaceURI: namespaceURI::<Impl, IMPL_OFFSET>,
            schema: schema::<Impl, IMPL_OFFSET>,
            id: id::<Impl, IMPL_OFFSET>,
            itemType: itemType::<Impl, IMPL_OFFSET>,
            unhandledAttributes: unhandledAttributes::<Impl, IMPL_OFFSET>,
            writeAnnotation: writeAnnotation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaItem as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaItemCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn item(&mut self, index: i32) -> ::windows::core::Result<ISchemaItem>;
    fn itemByName(&mut self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<ISchemaItem>;
    fn itemByQName(&mut self, name: &super::super::super::Foundation::BSTR, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<ISchemaItem>;
    fn length(&mut self) -> ::windows::core::Result<i32>;
    fn _newEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaItemCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItemCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaItemCollection_Vtbl {
        unsafe extern "system" fn item<Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemByName<Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).itemByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemByQName<Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).itemByQName(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            item: item::<Impl, IMPL_OFFSET>,
            itemByName: itemByName::<Impl, IMPL_OFFSET>,
            itemByQName: itemByQName::<Impl, IMPL_OFFSET>,
            length: length::<Impl, IMPL_OFFSET>,
            _newEnum: _newEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaItemCollection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaModelGroup_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl + ISchemaParticle_Impl {
    fn particles(&mut self) -> ::windows::core::Result<ISchemaItemCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaModelGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaModelGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaModelGroup_Vtbl {
        unsafe extern "system" fn particles<Impl: ISchemaModelGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, particles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).particles() {
                ::core::result::Result::Ok(ok__) => {
                    *particles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ISchemaParticle_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), particles: particles::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaModelGroup as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID || iid == &<ISchemaParticle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaNotation_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn systemIdentifier(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn publicIdentifier(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaNotation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaNotation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaNotation_Vtbl {
        unsafe extern "system" fn systemIdentifier<Impl: ISchemaNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).systemIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *uri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn publicIdentifier<Impl: ISchemaNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).publicIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *uri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            systemIdentifier: systemIdentifier::<Impl, IMPL_OFFSET>,
            publicIdentifier: publicIdentifier::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaNotation as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaParticle_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn minOccurs(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn maxOccurs(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaParticle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaParticle_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaParticle_Vtbl {
        unsafe extern "system" fn minOccurs<Impl: ISchemaParticle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minoccurs: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).minOccurs() {
                ::core::result::Result::Ok(ok__) => {
                    *minoccurs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxOccurs<Impl: ISchemaParticle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxoccurs: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).maxOccurs() {
                ::core::result::Result::Ok(ok__) => {
                    *maxoccurs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            minOccurs: minOccurs::<Impl, IMPL_OFFSET>,
            maxOccurs: maxOccurs::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaParticle as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaStringCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn item(&mut self, index: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn length(&mut self) -> ::windows::core::Result<i32>;
    fn _newEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaStringCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaStringCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaStringCollection_Vtbl {
        unsafe extern "system" fn item<Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, bstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *bstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            item: item::<Impl, IMPL_OFFSET>,
            length: length::<Impl, IMPL_OFFSET>,
            _newEnum: _newEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaStringCollection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaType_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn baseTypes(&mut self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn r#final(&mut self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
    fn variety(&mut self) -> ::windows::core::Result<SCHEMATYPEVARIETY>;
    fn derivedBy(&mut self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
    fn isValid(&mut self, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn minExclusive(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn minInclusive(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn maxExclusive(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn maxInclusive(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn totalDigits(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn fractionDigits(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn length(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn minLength(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn maxLength(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn enumeration(&mut self) -> ::windows::core::Result<ISchemaStringCollection>;
    fn whitespace(&mut self) -> ::windows::core::Result<SCHEMAWHITESPACE>;
    fn patterns(&mut self) -> ::windows::core::Result<ISchemaStringCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchemaType_Vtbl {
        unsafe extern "system" fn baseTypes<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basetypes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).baseTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *basetypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#final<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#final: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).r#final() {
                ::core::result::Result::Ok(ok__) => {
                    *r#final = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn variety<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variety: *mut SCHEMATYPEVARIETY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).variety() {
                ::core::result::Result::Ok(ok__) => {
                    *variety = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn derivedBy<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, derivedby: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).derivedBy() {
                ::core::result::Result::Ok(ok__) => {
                    *derivedby = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isValid<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, valid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isValid(::core::mem::transmute_copy(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *valid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minExclusive<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minexclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).minExclusive() {
                ::core::result::Result::Ok(ok__) => {
                    *minexclusive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minInclusive<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mininclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).minInclusive() {
                ::core::result::Result::Ok(ok__) => {
                    *mininclusive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxExclusive<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxexclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).maxExclusive() {
                ::core::result::Result::Ok(ok__) => {
                    *maxexclusive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxInclusive<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxinclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).maxInclusive() {
                ::core::result::Result::Ok(ok__) => {
                    *maxinclusive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn totalDigits<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, totaldigits: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).totalDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *totaldigits = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fractionDigits<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fractiondigits: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fractionDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *fractiondigits = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minLength<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minlength: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).minLength() {
                ::core::result::Result::Ok(ok__) => {
                    *minlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxLength<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).maxLength() {
                ::core::result::Result::Ok(ok__) => {
                    *maxlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enumeration<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumeration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).enumeration() {
                ::core::result::Result::Ok(ok__) => {
                    *enumeration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn whitespace<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitespace: *mut SCHEMAWHITESPACE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).whitespace() {
                ::core::result::Result::Ok(ok__) => {
                    *whitespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn patterns<Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patterns: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).patterns() {
                ::core::result::Result::Ok(ok__) => {
                    *patterns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            baseTypes: baseTypes::<Impl, IMPL_OFFSET>,
            r#final: r#final::<Impl, IMPL_OFFSET>,
            variety: variety::<Impl, IMPL_OFFSET>,
            derivedBy: derivedBy::<Impl, IMPL_OFFSET>,
            isValid: isValid::<Impl, IMPL_OFFSET>,
            minExclusive: minExclusive::<Impl, IMPL_OFFSET>,
            minInclusive: minInclusive::<Impl, IMPL_OFFSET>,
            maxExclusive: maxExclusive::<Impl, IMPL_OFFSET>,
            maxInclusive: maxInclusive::<Impl, IMPL_OFFSET>,
            totalDigits: totalDigits::<Impl, IMPL_OFFSET>,
            fractionDigits: fractionDigits::<Impl, IMPL_OFFSET>,
            length: length::<Impl, IMPL_OFFSET>,
            minLength: minLength::<Impl, IMPL_OFFSET>,
            maxLength: maxLength::<Impl, IMPL_OFFSET>,
            enumeration: enumeration::<Impl, IMPL_OFFSET>,
            whitespace: whitespace::<Impl, IMPL_OFFSET>,
            patterns: patterns::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaType as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IServerXMLHTTPRequest_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLHTTPRequest_Impl {
    fn setTimeouts(&mut self, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows::core::Result<()>;
    fn waitForResponse(&mut self, timeoutinseconds: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn getOption(&mut self, option: SERVERXMLHTTP_OPTION) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn setOption(&mut self, option: SERVERXMLHTTP_OPTION, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IServerXMLHTTPRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerXMLHTTPRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerXMLHTTPRequest_Vtbl {
        unsafe extern "system" fn setTimeouts<Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setTimeouts(::core::mem::transmute_copy(&resolvetimeout), ::core::mem::transmute_copy(&connecttimeout), ::core::mem::transmute_copy(&sendtimeout), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn waitForResponse<Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeoutinseconds: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, issuccessful: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).waitForResponse(::core::mem::transmute_copy(&timeoutinseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *issuccessful = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getOption<Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getOption(::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setOption<Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setOption(::core::mem::transmute_copy(&option), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IXMLHTTPRequest_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            setTimeouts: setTimeouts::<Impl, IMPL_OFFSET>,
            waitForResponse: waitForResponse::<Impl, IMPL_OFFSET>,
            getOption: getOption::<Impl, IMPL_OFFSET>,
            setOption: setOption::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLHTTPRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IServerXMLHTTPRequest2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLHTTPRequest_Impl + IServerXMLHTTPRequest_Impl {
    fn setProxy(&mut self, proxysetting: SXH_PROXY_SETTING, varproxyserver: &super::super::super::System::Com::VARIANT, varbypasslist: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setProxyCredentials(&mut self, bstrusername: &super::super::super::Foundation::BSTR, bstrpassword: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IServerXMLHTTPRequest2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerXMLHTTPRequest2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerXMLHTTPRequest2_Vtbl {
        unsafe extern "system" fn setProxy<Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proxysetting: SXH_PROXY_SETTING, varproxyserver: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, varbypasslist: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setProxy(::core::mem::transmute_copy(&proxysetting), ::core::mem::transmute_copy(&varproxyserver), ::core::mem::transmute_copy(&varbypasslist)).into()
        }
        unsafe extern "system" fn setProxyCredentials<Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setProxyCredentials(::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrpassword)).into()
        }
        Self {
            base: IServerXMLHTTPRequest_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            setProxy: setProxy::<Impl, IMPL_OFFSET>,
            setProxyCredentials: setProxyCredentials::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLHTTPRequest as ::windows::core::Interface>::IID || iid == &<IServerXMLHTTPRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBMXNamespaceManager_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn SetallowOverride(&mut self, foverride: i16) -> ::windows::core::Result<()>;
    fn allowOverride(&mut self) -> ::windows::core::Result<i16>;
    fn reset(&mut self) -> ::windows::core::Result<()>;
    fn pushContext(&mut self) -> ::windows::core::Result<()>;
    fn pushNodeContext(&mut self, contextnode: &::core::option::Option<IXMLDOMNode>, fdeep: i16) -> ::windows::core::Result<()>;
    fn popContext(&mut self) -> ::windows::core::Result<()>;
    fn declarePrefix(&mut self, prefix: &super::super::super::Foundation::BSTR, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getDeclaredPrefixes(&mut self) -> ::windows::core::Result<IMXNamespacePrefixes>;
    fn getPrefixes(&mut self, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IMXNamespacePrefixes>;
    fn getURI(&mut self, prefix: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn getURIFromNode(&mut self, strprefix: &super::super::super::Foundation::BSTR, contextnode: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBMXNamespaceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBMXNamespaceManager_Vtbl {
        unsafe extern "system" fn SetallowOverride<Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetallowOverride(::core::mem::transmute_copy(&foverride)).into()
        }
        unsafe extern "system" fn allowOverride<Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).allowOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *foverride = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).reset().into()
        }
        unsafe extern "system" fn pushContext<Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).pushContext().into()
        }
        unsafe extern "system" fn pushNodeContext<Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextnode: ::windows::core::RawPtr, fdeep: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).pushNodeContext(::core::mem::transmute(&contextnode), ::core::mem::transmute_copy(&fdeep)).into()
        }
        unsafe extern "system" fn popContext<Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).popContext().into()
        }
        unsafe extern "system" fn declarePrefix<Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).declarePrefix(::core::mem::transmute_copy(&prefix), ::core::mem::transmute_copy(&namespaceuri)).into()
        }
        unsafe extern "system" fn getDeclaredPrefixes<Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefixes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getDeclaredPrefixes() {
                ::core::result::Result::Ok(ok__) => {
                    *prefixes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPrefixes<Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, prefixes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getPrefixes(::core::mem::transmute_copy(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *prefixes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, uri: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getURI(::core::mem::transmute_copy(&prefix)) {
                ::core::result::Result::Ok(ok__) => {
                    *uri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURIFromNode<Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, contextnode: ::windows::core::RawPtr, uri: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getURIFromNode(::core::mem::transmute_copy(&strprefix), ::core::mem::transmute(&contextnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *uri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetallowOverride: SetallowOverride::<Impl, IMPL_OFFSET>,
            allowOverride: allowOverride::<Impl, IMPL_OFFSET>,
            reset: reset::<Impl, IMPL_OFFSET>,
            pushContext: pushContext::<Impl, IMPL_OFFSET>,
            pushNodeContext: pushNodeContext::<Impl, IMPL_OFFSET>,
            popContext: popContext::<Impl, IMPL_OFFSET>,
            declarePrefix: declarePrefix::<Impl, IMPL_OFFSET>,
            getDeclaredPrefixes: getDeclaredPrefixes::<Impl, IMPL_OFFSET>,
            getPrefixes: getPrefixes::<Impl, IMPL_OFFSET>,
            getURI: getURI::<Impl, IMPL_OFFSET>,
            getURIFromNode: getURIFromNode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBMXNamespaceManager as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXAttributes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn length(&mut self) -> ::windows::core::Result<i32>;
    fn getURI(&mut self, nindex: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getLocalName(&mut self, nindex: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getQName(&mut self, nindex: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getIndexFromName(&mut self, struri: &super::super::super::Foundation::BSTR, strlocalname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn getIndexFromQName(&mut self, strqname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn getType(&mut self, nindex: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getTypeFromName(&mut self, struri: &super::super::super::Foundation::BSTR, strlocalname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getTypeFromQName(&mut self, strqname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getValue(&mut self, nindex: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getValueFromName(&mut self, struri: &super::super::super::Foundation::BSTR, strlocalname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getValueFromQName(&mut self, strqname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXAttributes_Vtbl {
        unsafe extern "system" fn length<Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *nlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getURI(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *struri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLocalName<Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getLocalName(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *strlocalname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getQName<Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getQName(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *strqname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromName<Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getIndexFromName(::core::mem::transmute_copy(&struri), ::core::mem::transmute_copy(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    *nindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromQName<Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getIndexFromQName(::core::mem::transmute_copy(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    *nindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getType<Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getType(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *strtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromName<Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getTypeFromName(::core::mem::transmute_copy(&struri), ::core::mem::transmute_copy(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    *strtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromQName<Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getTypeFromQName(::core::mem::transmute_copy(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    *strtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValue<Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getValue(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *strvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromName<Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getValueFromName(::core::mem::transmute_copy(&struri), ::core::mem::transmute_copy(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    *strvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromQName<Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getValueFromQName(::core::mem::transmute_copy(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    *strvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            length: length::<Impl, IMPL_OFFSET>,
            getURI: getURI::<Impl, IMPL_OFFSET>,
            getLocalName: getLocalName::<Impl, IMPL_OFFSET>,
            getQName: getQName::<Impl, IMPL_OFFSET>,
            getIndexFromName: getIndexFromName::<Impl, IMPL_OFFSET>,
            getIndexFromQName: getIndexFromQName::<Impl, IMPL_OFFSET>,
            getType: getType::<Impl, IMPL_OFFSET>,
            getTypeFromName: getTypeFromName::<Impl, IMPL_OFFSET>,
            getTypeFromQName: getTypeFromQName::<Impl, IMPL_OFFSET>,
            getValue: getValue::<Impl, IMPL_OFFSET>,
            getValueFromName: getValueFromName::<Impl, IMPL_OFFSET>,
            getValueFromQName: getValueFromQName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXAttributes as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXContentHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn putref_documentLocator(&mut self, olocator: &::core::option::Option<IVBSAXLocator>) -> ::windows::core::Result<()>;
    fn startDocument(&mut self) -> ::windows::core::Result<()>;
    fn endDocument(&mut self) -> ::windows::core::Result<()>;
    fn startPrefixMapping(&mut self, strprefix: *mut super::super::super::Foundation::BSTR, struri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn endPrefixMapping(&mut self, strprefix: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn startElement(&mut self, strnamespaceuri: *mut super::super::super::Foundation::BSTR, strlocalname: *mut super::super::super::Foundation::BSTR, strqname: *mut super::super::super::Foundation::BSTR, oattributes: &::core::option::Option<IVBSAXAttributes>) -> ::windows::core::Result<()>;
    fn endElement(&mut self, strnamespaceuri: *mut super::super::super::Foundation::BSTR, strlocalname: *mut super::super::super::Foundation::BSTR, strqname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn characters(&mut self, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ignorableWhitespace(&mut self, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn processingInstruction(&mut self, strtarget: *mut super::super::super::Foundation::BSTR, strdata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn skippedEntity(&mut self, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXContentHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXContentHandler_Vtbl {
        unsafe extern "system" fn putref_documentLocator<Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_documentLocator(::core::mem::transmute(&olocator)).into()
        }
        unsafe extern "system" fn startDocument<Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startDocument().into()
        }
        unsafe extern "system" fn endDocument<Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).endDocument().into()
        }
        unsafe extern "system" fn startPrefixMapping<Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: *mut super::super::super::Foundation::BSTR, struri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startPrefixMapping(::core::mem::transmute_copy(&strprefix), ::core::mem::transmute_copy(&struri)).into()
        }
        unsafe extern "system" fn endPrefixMapping<Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).endPrefixMapping(::core::mem::transmute_copy(&strprefix)).into()
        }
        unsafe extern "system" fn startElement<Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut super::super::super::Foundation::BSTR, strlocalname: *mut super::super::super::Foundation::BSTR, strqname: *mut super::super::super::Foundation::BSTR, oattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startElement(::core::mem::transmute_copy(&strnamespaceuri), ::core::mem::transmute_copy(&strlocalname), ::core::mem::transmute_copy(&strqname), ::core::mem::transmute(&oattributes)).into()
        }
        unsafe extern "system" fn endElement<Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut super::super::super::Foundation::BSTR, strlocalname: *mut super::super::super::Foundation::BSTR, strqname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).endElement(::core::mem::transmute_copy(&strnamespaceuri), ::core::mem::transmute_copy(&strlocalname), ::core::mem::transmute_copy(&strqname)).into()
        }
        unsafe extern "system" fn characters<Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).characters(::core::mem::transmute_copy(&strchars)).into()
        }
        unsafe extern "system" fn ignorableWhitespace<Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ignorableWhitespace(::core::mem::transmute_copy(&strchars)).into()
        }
        unsafe extern "system" fn processingInstruction<Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strtarget: *mut super::super::super::Foundation::BSTR, strdata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).processingInstruction(::core::mem::transmute_copy(&strtarget), ::core::mem::transmute_copy(&strdata)).into()
        }
        unsafe extern "system" fn skippedEntity<Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).skippedEntity(::core::mem::transmute_copy(&strname)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            putref_documentLocator: putref_documentLocator::<Impl, IMPL_OFFSET>,
            startDocument: startDocument::<Impl, IMPL_OFFSET>,
            endDocument: endDocument::<Impl, IMPL_OFFSET>,
            startPrefixMapping: startPrefixMapping::<Impl, IMPL_OFFSET>,
            endPrefixMapping: endPrefixMapping::<Impl, IMPL_OFFSET>,
            startElement: startElement::<Impl, IMPL_OFFSET>,
            endElement: endElement::<Impl, IMPL_OFFSET>,
            characters: characters::<Impl, IMPL_OFFSET>,
            ignorableWhitespace: ignorableWhitespace::<Impl, IMPL_OFFSET>,
            processingInstruction: processingInstruction::<Impl, IMPL_OFFSET>,
            skippedEntity: skippedEntity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXContentHandler as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXDTDHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn notationDecl(&mut self, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn unparsedEntityDecl(&mut self, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR, strnotationname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXDTDHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXDTDHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXDTDHandler_Vtbl {
        unsafe extern "system" fn notationDecl<Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).notationDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into()
        }
        unsafe extern "system" fn unparsedEntityDecl<Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR, strnotationname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).unparsedEntityDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid), ::core::mem::transmute_copy(&strnotationname)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            notationDecl: notationDecl::<Impl, IMPL_OFFSET>,
            unparsedEntityDecl: unparsedEntityDecl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXDTDHandler as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXDeclHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn elementDecl(&mut self, strname: *mut super::super::super::Foundation::BSTR, strmodel: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn attributeDecl(&mut self, strelementname: *mut super::super::super::Foundation::BSTR, strattributename: *mut super::super::super::Foundation::BSTR, strtype: *mut super::super::super::Foundation::BSTR, strvaluedefault: *mut super::super::super::Foundation::BSTR, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn internalEntityDecl(&mut self, strname: *mut super::super::super::Foundation::BSTR, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn externalEntityDecl(&mut self, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXDeclHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXDeclHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXDeclHandler_Vtbl {
        unsafe extern "system" fn elementDecl<Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strmodel: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).elementDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strmodel)).into()
        }
        unsafe extern "system" fn attributeDecl<Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strelementname: *mut super::super::super::Foundation::BSTR, strattributename: *mut super::super::super::Foundation::BSTR, strtype: *mut super::super::super::Foundation::BSTR, strvaluedefault: *mut super::super::super::Foundation::BSTR, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).attributeDecl(::core::mem::transmute_copy(&strelementname), ::core::mem::transmute_copy(&strattributename), ::core::mem::transmute_copy(&strtype), ::core::mem::transmute_copy(&strvaluedefault), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn internalEntityDecl<Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).internalEntityDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn externalEntityDecl<Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).externalEntityDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            elementDecl: elementDecl::<Impl, IMPL_OFFSET>,
            attributeDecl: attributeDecl::<Impl, IMPL_OFFSET>,
            internalEntityDecl: internalEntityDecl::<Impl, IMPL_OFFSET>,
            externalEntityDecl: externalEntityDecl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXDeclHandler as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXEntityResolver_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn resolveEntity(&mut self, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR, varinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXEntityResolver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXEntityResolver_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXEntityResolver_Vtbl {
        unsafe extern "system" fn resolveEntity<Impl: IVBSAXEntityResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR, varinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).resolveEntity(::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid), ::core::mem::transmute_copy(&varinput)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            resolveEntity: resolveEntity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXEntityResolver as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXErrorHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn error(&mut self, olocator: &::core::option::Option<IVBSAXLocator>, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::Result<()>;
    fn fatalError(&mut self, olocator: &::core::option::Option<IVBSAXLocator>, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::Result<()>;
    fn ignorableWarning(&mut self, olocator: &::core::option::Option<IVBSAXLocator>, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXErrorHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXErrorHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXErrorHandler_Vtbl {
        unsafe extern "system" fn error<Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).error(::core::mem::transmute(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into()
        }
        unsafe extern "system" fn fatalError<Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).fatalError(::core::mem::transmute(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into()
        }
        unsafe extern "system" fn ignorableWarning<Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ignorableWarning(::core::mem::transmute(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            error: error::<Impl, IMPL_OFFSET>,
            fatalError: fatalError::<Impl, IMPL_OFFSET>,
            ignorableWarning: ignorableWarning::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXErrorHandler as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXLexicalHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn startDTD(&mut self, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn endDTD(&mut self) -> ::windows::core::Result<()>;
    fn startEntity(&mut self, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn endEntity(&mut self, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn startCDATA(&mut self) -> ::windows::core::Result<()>;
    fn endCDATA(&mut self) -> ::windows::core::Result<()>;
    fn comment(&mut self, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXLexicalHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLexicalHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXLexicalHandler_Vtbl {
        unsafe extern "system" fn startDTD<Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startDTD(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into()
        }
        unsafe extern "system" fn endDTD<Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).endDTD().into()
        }
        unsafe extern "system" fn startEntity<Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startEntity(::core::mem::transmute_copy(&strname)).into()
        }
        unsafe extern "system" fn endEntity<Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).endEntity(::core::mem::transmute_copy(&strname)).into()
        }
        unsafe extern "system" fn startCDATA<Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startCDATA().into()
        }
        unsafe extern "system" fn endCDATA<Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).endCDATA().into()
        }
        unsafe extern "system" fn comment<Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).comment(::core::mem::transmute_copy(&strchars)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            startDTD: startDTD::<Impl, IMPL_OFFSET>,
            endDTD: endDTD::<Impl, IMPL_OFFSET>,
            startEntity: startEntity::<Impl, IMPL_OFFSET>,
            endEntity: endEntity::<Impl, IMPL_OFFSET>,
            startCDATA: startCDATA::<Impl, IMPL_OFFSET>,
            endCDATA: endCDATA::<Impl, IMPL_OFFSET>,
            comment: comment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXLexicalHandler as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXLocator_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn columnNumber(&mut self) -> ::windows::core::Result<i32>;
    fn lineNumber(&mut self) -> ::windows::core::Result<i32>;
    fn publicId(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn systemId(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLocator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXLocator_Vtbl {
        unsafe extern "system" fn columnNumber<Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).columnNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *ncolumn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lineNumber<Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nline: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).lineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *nline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn publicId<Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpublicid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).publicId() {
                ::core::result::Result::Ok(ok__) => {
                    *strpublicid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemId<Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).systemId() {
                ::core::result::Result::Ok(ok__) => {
                    *strsystemid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            columnNumber: columnNumber::<Impl, IMPL_OFFSET>,
            lineNumber: lineNumber::<Impl, IMPL_OFFSET>,
            publicId: publicId::<Impl, IMPL_OFFSET>,
            systemId: systemId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXLocator as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXXMLFilter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn parent(&mut self) -> ::windows::core::Result<IVBSAXXMLReader>;
    fn putref_parent(&mut self, oreader: &::core::option::Option<IVBSAXXMLReader>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXXMLFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLFilter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXXMLFilter_Vtbl {
        unsafe extern "system" fn parent<Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).parent() {
                ::core::result::Result::Ok(ok__) => {
                    *oreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_parent<Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oreader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_parent(::core::mem::transmute(&oreader)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            parent: parent::<Impl, IMPL_OFFSET>,
            putref_parent: putref_parent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXXMLFilter as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXXMLReader_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getFeature(&mut self, strname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn putFeature(&mut self, strname: &super::super::super::Foundation::BSTR, fvalue: i16) -> ::windows::core::Result<()>;
    fn getProperty(&mut self, strname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn putProperty(&mut self, strname: &super::super::super::Foundation::BSTR, varvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn entityResolver(&mut self) -> ::windows::core::Result<IVBSAXEntityResolver>;
    fn putref_entityResolver(&mut self, oresolver: &::core::option::Option<IVBSAXEntityResolver>) -> ::windows::core::Result<()>;
    fn contentHandler(&mut self) -> ::windows::core::Result<IVBSAXContentHandler>;
    fn putref_contentHandler(&mut self, ohandler: &::core::option::Option<IVBSAXContentHandler>) -> ::windows::core::Result<()>;
    fn dtdHandler(&mut self) -> ::windows::core::Result<IVBSAXDTDHandler>;
    fn putref_dtdHandler(&mut self, ohandler: &::core::option::Option<IVBSAXDTDHandler>) -> ::windows::core::Result<()>;
    fn errorHandler(&mut self) -> ::windows::core::Result<IVBSAXErrorHandler>;
    fn putref_errorHandler(&mut self, ohandler: &::core::option::Option<IVBSAXErrorHandler>) -> ::windows::core::Result<()>;
    fn baseURL(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetbaseURL(&mut self, strbaseurl: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn secureBaseURL(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetsecureBaseURL(&mut self, strsecurebaseurl: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn parse(&mut self, varinput: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn parseURL(&mut self, strurl: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXXMLReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVBSAXXMLReader_Vtbl {
        unsafe extern "system" fn getFeature<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getFeature(::core::mem::transmute_copy(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *fvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putFeature(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn getProperty<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getProperty(::core::mem::transmute_copy(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *varvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putProperty(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn entityResolver<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).entityResolver() {
                ::core::result::Result::Ok(ok__) => {
                    *oresolver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_entityResolver<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_entityResolver(::core::mem::transmute(&oresolver)).into()
        }
        unsafe extern "system" fn contentHandler<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).contentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ohandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_contentHandler<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_contentHandler(::core::mem::transmute(&ohandler)).into()
        }
        unsafe extern "system" fn dtdHandler<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).dtdHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ohandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_dtdHandler(::core::mem::transmute(&ohandler)).into()
        }
        unsafe extern "system" fn errorHandler<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).errorHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ohandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_errorHandler<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_errorHandler(::core::mem::transmute(&ohandler)).into()
        }
        unsafe extern "system" fn baseURL<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbaseurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).baseURL() {
                ::core::result::Result::Ok(ok__) => {
                    *strbaseurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbaseURL<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbaseurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetbaseURL(::core::mem::transmute_copy(&strbaseurl)).into()
        }
        unsafe extern "system" fn secureBaseURL<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).secureBaseURL() {
                ::core::result::Result::Ok(ok__) => {
                    *strsecurebaseurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetsecureBaseURL<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetsecureBaseURL(::core::mem::transmute_copy(&strsecurebaseurl)).into()
        }
        unsafe extern "system" fn parse<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinput: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).parse(::core::mem::transmute_copy(&varinput)).into()
        }
        unsafe extern "system" fn parseURL<Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).parseURL(::core::mem::transmute_copy(&strurl)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            getFeature: getFeature::<Impl, IMPL_OFFSET>,
            putFeature: putFeature::<Impl, IMPL_OFFSET>,
            getProperty: getProperty::<Impl, IMPL_OFFSET>,
            putProperty: putProperty::<Impl, IMPL_OFFSET>,
            entityResolver: entityResolver::<Impl, IMPL_OFFSET>,
            putref_entityResolver: putref_entityResolver::<Impl, IMPL_OFFSET>,
            contentHandler: contentHandler::<Impl, IMPL_OFFSET>,
            putref_contentHandler: putref_contentHandler::<Impl, IMPL_OFFSET>,
            dtdHandler: dtdHandler::<Impl, IMPL_OFFSET>,
            putref_dtdHandler: putref_dtdHandler::<Impl, IMPL_OFFSET>,
            errorHandler: errorHandler::<Impl, IMPL_OFFSET>,
            putref_errorHandler: putref_errorHandler::<Impl, IMPL_OFFSET>,
            baseURL: baseURL::<Impl, IMPL_OFFSET>,
            SetbaseURL: SetbaseURL::<Impl, IMPL_OFFSET>,
            secureBaseURL: secureBaseURL::<Impl, IMPL_OFFSET>,
            SetsecureBaseURL: SetsecureBaseURL::<Impl, IMPL_OFFSET>,
            parse: parse::<Impl, IMPL_OFFSET>,
            parseURL: parseURL::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXXMLReader as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLAttribute_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn name(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn value(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLAttribute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLAttribute_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLAttribute_Vtbl {
        unsafe extern "system" fn name<Impl: IXMLAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, n: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).name() {
                ::core::result::Result::Ok(ok__) => {
                    *n = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn value<Impl: IXMLAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).value() {
                ::core::result::Result::Ok(ok__) => {
                    *v = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            name: name::<Impl, IMPL_OFFSET>,
            value: value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLAttribute as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMAttribute_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn name(&mut self, attributename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn value(&mut self, attributevalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Setvalue(&mut self, attributevalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMAttribute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMAttribute_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMAttribute_Vtbl {
        unsafe extern "system" fn name<Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).name(::core::mem::transmute_copy(&attributename)).into()
        }
        unsafe extern "system" fn value<Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributevalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).value(::core::mem::transmute_copy(&attributevalue)).into()
        }
        unsafe extern "system" fn Setvalue<Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributevalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setvalue(::core::mem::transmute_copy(&attributevalue)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            name: name::<Impl, IMPL_OFFSET>,
            value: value::<Impl, IMPL_OFFSET>,
            Setvalue: Setvalue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMAttribute as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMCDATASection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl + IXMLDOMCharacterData_Impl + IXMLDOMText_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMCDATASection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCDATASection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMCDATASection_Vtbl {
        Self { base: IXMLDOMText_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMCDATASection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID || iid == &<IXMLDOMCharacterData as ::windows::core::Interface>::IID || iid == &<IXMLDOMText as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMCharacterData_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn data(&mut self, data: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Setdata(&mut self, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn length(&mut self, datalength: *mut i32) -> ::windows::core::Result<()>;
    fn substringData(&mut self, offset: i32, count: i32, data: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn appendData(&mut self, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn insertData(&mut self, offset: i32, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn deleteData(&mut self, offset: i32, count: i32) -> ::windows::core::Result<()>;
    fn replaceData(&mut self, offset: i32, count: i32, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMCharacterData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCharacterData_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMCharacterData_Vtbl {
        unsafe extern "system" fn data<Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).data(::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn Setdata<Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setdata(::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn length<Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datalength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).length(::core::mem::transmute_copy(&datalength)).into()
        }
        unsafe extern "system" fn substringData<Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).substringData(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn appendData<Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).appendData(::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn insertData<Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).insertData(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn deleteData<Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).deleteData(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn replaceData<Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).replaceData(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&data)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            data: data::<Impl, IMPL_OFFSET>,
            Setdata: Setdata::<Impl, IMPL_OFFSET>,
            length: length::<Impl, IMPL_OFFSET>,
            substringData: substringData::<Impl, IMPL_OFFSET>,
            appendData: appendData::<Impl, IMPL_OFFSET>,
            insertData: insertData::<Impl, IMPL_OFFSET>,
            deleteData: deleteData::<Impl, IMPL_OFFSET>,
            replaceData: replaceData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMCharacterData as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMComment_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl + IXMLDOMCharacterData_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMComment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMComment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMComment_Vtbl {
        Self { base: IXMLDOMCharacterData_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMComment as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID || iid == &<IXMLDOMCharacterData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocument_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn doctype(&mut self) -> ::windows::core::Result<IXMLDOMDocumentType>;
    fn implementation(&mut self) -> ::windows::core::Result<IXMLDOMImplementation>;
    fn documentElement(&mut self) -> ::windows::core::Result<IXMLDOMElement>;
    fn putref_documentElement(&mut self, domelement: &::core::option::Option<IXMLDOMElement>) -> ::windows::core::Result<()>;
    fn createElement(&mut self, tagname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMElement>;
    fn createDocumentFragment(&mut self) -> ::windows::core::Result<IXMLDOMDocumentFragment>;
    fn createTextNode(&mut self, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMText>;
    fn createComment(&mut self, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMComment>;
    fn createCDATASection(&mut self, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMCDATASection>;
    fn createProcessingInstruction(&mut self, target: &super::super::super::Foundation::BSTR, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMProcessingInstruction>;
    fn createAttribute(&mut self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMAttribute>;
    fn createEntityReference(&mut self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMEntityReference>;
    fn getElementsByTagName(&mut self, tagname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNodeList>;
    fn createNode(&mut self, r#type: &super::super::super::System::Com::VARIANT, name: &super::super::super::Foundation::BSTR, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn nodeFromID(&mut self, idstring: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn load(&mut self, xmlsource: &super::super::super::System::Com::VARIANT, issuccessful: *mut i16) -> ::windows::core::Result<()>;
    fn readyState(&mut self, value: *mut i32) -> ::windows::core::Result<()>;
    fn parseError(&mut self) -> ::windows::core::Result<IXMLDOMParseError>;
    fn url(&mut self, urlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn r#async(&mut self, isasync: *mut i16) -> ::windows::core::Result<()>;
    fn Setasync(&mut self, isasync: i16) -> ::windows::core::Result<()>;
    fn abort(&mut self) -> ::windows::core::Result<()>;
    fn loadXML(&mut self, bstrxml: &super::super::super::Foundation::BSTR, issuccessful: *mut i16) -> ::windows::core::Result<()>;
    fn save(&mut self, destination: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn validateOnParse(&mut self, isvalidating: *mut i16) -> ::windows::core::Result<()>;
    fn SetvalidateOnParse(&mut self, isvalidating: i16) -> ::windows::core::Result<()>;
    fn resolveExternals(&mut self, isresolving: *mut i16) -> ::windows::core::Result<()>;
    fn SetresolveExternals(&mut self, isresolving: i16) -> ::windows::core::Result<()>;
    fn preserveWhiteSpace(&mut self, ispreserving: *mut i16) -> ::windows::core::Result<()>;
    fn SetpreserveWhiteSpace(&mut self, ispreserving: i16) -> ::windows::core::Result<()>;
    fn Setonreadystatechange(&mut self, readystatechangesink: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Setondataavailable(&mut self, ondataavailablesink: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Setontransformnode(&mut self, ontransformnodesink: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMDocument_Vtbl {
        unsafe extern "system" fn doctype<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).doctype() {
                ::core::result::Result::Ok(ok__) => {
                    *documenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn implementation<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#impl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).implementation() {
                ::core::result::Result::Ok(ok__) => {
                    *r#impl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn documentElement<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).documentElement() {
                ::core::result::Result::Ok(ok__) => {
                    *domelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_documentElement<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_documentElement(::core::mem::transmute(&domelement)).into()
        }
        unsafe extern "system" fn createElement<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createElement(::core::mem::transmute_copy(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createDocumentFragment<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, docfrag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createDocumentFragment() {
                ::core::result::Result::Ok(ok__) => {
                    *docfrag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createTextNode<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, text: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createTextNode(::core::mem::transmute_copy(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createComment<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, comment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createComment(::core::mem::transmute_copy(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *comment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createCDATASection<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, cdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createCDATASection(::core::mem::transmute_copy(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *cdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createProcessingInstruction<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createProcessingInstruction(::core::mem::transmute_copy(&target), ::core::mem::transmute_copy(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *pi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createAttribute<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, attribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createAttribute(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *attribute = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createEntityReference<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, entityref: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createEntityReference(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *entityref = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getElementsByTagName<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getElementsByTagName(::core::mem::transmute_copy(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    *resultlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createNode<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createNode(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *node = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeFromID<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).nodeFromID(::core::mem::transmute_copy(&idstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *node = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn load<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlsource: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, issuccessful: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).load(::core::mem::transmute_copy(&xmlsource), ::core::mem::transmute_copy(&issuccessful)).into()
        }
        unsafe extern "system" fn readyState<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).readyState(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn parseError<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).parseError() {
                ::core::result::Result::Ok(ok__) => {
                    *errorobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn url<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).url(::core::mem::transmute_copy(&urlstring)).into()
        }
        unsafe extern "system" fn r#async<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isasync: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).r#async(::core::mem::transmute_copy(&isasync)).into()
        }
        unsafe extern "system" fn Setasync<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isasync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setasync(::core::mem::transmute_copy(&isasync)).into()
        }
        unsafe extern "system" fn abort<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).abort().into()
        }
        unsafe extern "system" fn loadXML<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, issuccessful: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).loadXML(::core::mem::transmute_copy(&bstrxml), ::core::mem::transmute_copy(&issuccessful)).into()
        }
        unsafe extern "system" fn save<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).save(::core::mem::transmute_copy(&destination)).into()
        }
        unsafe extern "system" fn validateOnParse<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalidating: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).validateOnParse(::core::mem::transmute_copy(&isvalidating)).into()
        }
        unsafe extern "system" fn SetvalidateOnParse<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalidating: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetvalidateOnParse(::core::mem::transmute_copy(&isvalidating)).into()
        }
        unsafe extern "system" fn resolveExternals<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isresolving: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).resolveExternals(::core::mem::transmute_copy(&isresolving)).into()
        }
        unsafe extern "system" fn SetresolveExternals<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isresolving: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetresolveExternals(::core::mem::transmute_copy(&isresolving)).into()
        }
        unsafe extern "system" fn preserveWhiteSpace<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispreserving: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).preserveWhiteSpace(::core::mem::transmute_copy(&ispreserving)).into()
        }
        unsafe extern "system" fn SetpreserveWhiteSpace<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispreserving: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetpreserveWhiteSpace(::core::mem::transmute_copy(&ispreserving)).into()
        }
        unsafe extern "system" fn Setonreadystatechange<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readystatechangesink: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setonreadystatechange(::core::mem::transmute_copy(&readystatechangesink)).into()
        }
        unsafe extern "system" fn Setondataavailable<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ondataavailablesink: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setondataavailable(::core::mem::transmute_copy(&ondataavailablesink)).into()
        }
        unsafe extern "system" fn Setontransformnode<Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ontransformnodesink: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setontransformnode(::core::mem::transmute_copy(&ontransformnodesink)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            doctype: doctype::<Impl, IMPL_OFFSET>,
            implementation: implementation::<Impl, IMPL_OFFSET>,
            documentElement: documentElement::<Impl, IMPL_OFFSET>,
            putref_documentElement: putref_documentElement::<Impl, IMPL_OFFSET>,
            createElement: createElement::<Impl, IMPL_OFFSET>,
            createDocumentFragment: createDocumentFragment::<Impl, IMPL_OFFSET>,
            createTextNode: createTextNode::<Impl, IMPL_OFFSET>,
            createComment: createComment::<Impl, IMPL_OFFSET>,
            createCDATASection: createCDATASection::<Impl, IMPL_OFFSET>,
            createProcessingInstruction: createProcessingInstruction::<Impl, IMPL_OFFSET>,
            createAttribute: createAttribute::<Impl, IMPL_OFFSET>,
            createEntityReference: createEntityReference::<Impl, IMPL_OFFSET>,
            getElementsByTagName: getElementsByTagName::<Impl, IMPL_OFFSET>,
            createNode: createNode::<Impl, IMPL_OFFSET>,
            nodeFromID: nodeFromID::<Impl, IMPL_OFFSET>,
            load: load::<Impl, IMPL_OFFSET>,
            readyState: readyState::<Impl, IMPL_OFFSET>,
            parseError: parseError::<Impl, IMPL_OFFSET>,
            url: url::<Impl, IMPL_OFFSET>,
            r#async: r#async::<Impl, IMPL_OFFSET>,
            Setasync: Setasync::<Impl, IMPL_OFFSET>,
            abort: abort::<Impl, IMPL_OFFSET>,
            loadXML: loadXML::<Impl, IMPL_OFFSET>,
            save: save::<Impl, IMPL_OFFSET>,
            validateOnParse: validateOnParse::<Impl, IMPL_OFFSET>,
            SetvalidateOnParse: SetvalidateOnParse::<Impl, IMPL_OFFSET>,
            resolveExternals: resolveExternals::<Impl, IMPL_OFFSET>,
            SetresolveExternals: SetresolveExternals::<Impl, IMPL_OFFSET>,
            preserveWhiteSpace: preserveWhiteSpace::<Impl, IMPL_OFFSET>,
            SetpreserveWhiteSpace: SetpreserveWhiteSpace::<Impl, IMPL_OFFSET>,
            Setonreadystatechange: Setonreadystatechange::<Impl, IMPL_OFFSET>,
            Setondataavailable: Setondataavailable::<Impl, IMPL_OFFSET>,
            Setontransformnode: Setontransformnode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocument as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocument2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl + IXMLDOMDocument_Impl {
    fn namespaces(&mut self) -> ::windows::core::Result<IXMLDOMSchemaCollection>;
    fn schemas(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn putref_schemas(&mut self, othercollection: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn validate(&mut self) -> ::windows::core::Result<IXMLDOMParseError>;
    fn setProperty(&mut self, name: &super::super::super::Foundation::BSTR, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getProperty(&mut self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocument2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMDocument2_Vtbl {
        unsafe extern "system" fn namespaces<Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespacecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).namespaces() {
                ::core::result::Result::Ok(ok__) => {
                    *namespacecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schemas<Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).schemas() {
                ::core::result::Result::Ok(ok__) => {
                    *othercollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_schemas<Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_schemas(::core::mem::transmute_copy(&othercollection)).into()
        }
        unsafe extern "system" fn validate<Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).validate() {
                ::core::result::Result::Ok(ok__) => {
                    *errorobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProperty<Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setProperty(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn getProperty<Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getProperty(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXMLDOMDocument_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            namespaces: namespaces::<Impl, IMPL_OFFSET>,
            schemas: schemas::<Impl, IMPL_OFFSET>,
            putref_schemas: putref_schemas::<Impl, IMPL_OFFSET>,
            validate: validate::<Impl, IMPL_OFFSET>,
            setProperty: setProperty::<Impl, IMPL_OFFSET>,
            getProperty: getProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocument2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID || iid == &<IXMLDOMDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocument3_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl + IXMLDOMDocument_Impl + IXMLDOMDocument2_Impl {
    fn validateNode(&mut self, node: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMParseError>;
    fn importNode(&mut self, node: &::core::option::Option<IXMLDOMNode>, deep: i16) -> ::windows::core::Result<IXMLDOMNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocument3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMDocument3_Vtbl {
        unsafe extern "system" fn validateNode<Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, errorobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).validateNode(::core::mem::transmute(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    *errorobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn importNode<Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, deep: i16, clone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).importNode(::core::mem::transmute(&node), ::core::mem::transmute_copy(&deep)) {
                ::core::result::Result::Ok(ok__) => {
                    *clone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXMLDOMDocument2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            validateNode: validateNode::<Impl, IMPL_OFFSET>,
            importNode: importNode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocument3 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID || iid == &<IXMLDOMDocument as ::windows::core::Interface>::IID || iid == &<IXMLDOMDocument2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocumentFragment_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocumentFragment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocumentFragment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMDocumentFragment_Vtbl {
        Self { base: IXMLDOMNode_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocumentFragment as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocumentType_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn name(&mut self, rootname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn entities(&mut self) -> ::windows::core::Result<IXMLDOMNamedNodeMap>;
    fn notations(&mut self) -> ::windows::core::Result<IXMLDOMNamedNodeMap>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocumentType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocumentType_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMDocumentType_Vtbl {
        unsafe extern "system" fn name<Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).name(::core::mem::transmute_copy(&rootname)).into()
        }
        unsafe extern "system" fn entities<Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entitymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).entities() {
                ::core::result::Result::Ok(ok__) => {
                    *entitymap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notations<Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notationmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).notations() {
                ::core::result::Result::Ok(ok__) => {
                    *notationmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            name: name::<Impl, IMPL_OFFSET>,
            entities: entities::<Impl, IMPL_OFFSET>,
            notations: notations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocumentType as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMElement_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn tagName(&mut self, tagname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getAttribute(&mut self, name: &super::super::super::Foundation::BSTR, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setAttribute(&mut self, name: &super::super::super::Foundation::BSTR, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn removeAttribute(&mut self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getAttributeNode(&mut self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMAttribute>;
    fn setAttributeNode(&mut self, domattribute: &::core::option::Option<IXMLDOMAttribute>) -> ::windows::core::Result<IXMLDOMAttribute>;
    fn removeAttributeNode(&mut self, domattribute: &::core::option::Option<IXMLDOMAttribute>) -> ::windows::core::Result<IXMLDOMAttribute>;
    fn getElementsByTagName(&mut self, tagname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNodeList>;
    fn normalize(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMElement_Vtbl {
        unsafe extern "system" fn tagName<Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).tagName(::core::mem::transmute_copy(&tagname)).into()
        }
        unsafe extern "system" fn getAttribute<Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getAttribute(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn setAttribute<Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setAttribute(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn removeAttribute<Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).removeAttribute(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn getAttributeNode<Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, attributenode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAttributeNode(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *attributenode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttributeNode<Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domattribute: ::windows::core::RawPtr, attributenode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setAttributeNode(::core::mem::transmute(&domattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    *attributenode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttributeNode<Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domattribute: ::windows::core::RawPtr, attributenode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).removeAttributeNode(::core::mem::transmute(&domattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    *attributenode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getElementsByTagName<Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getElementsByTagName(::core::mem::transmute_copy(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    *resultlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn normalize<Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).normalize().into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            tagName: tagName::<Impl, IMPL_OFFSET>,
            getAttribute: getAttribute::<Impl, IMPL_OFFSET>,
            setAttribute: setAttribute::<Impl, IMPL_OFFSET>,
            removeAttribute: removeAttribute::<Impl, IMPL_OFFSET>,
            getAttributeNode: getAttributeNode::<Impl, IMPL_OFFSET>,
            setAttributeNode: setAttributeNode::<Impl, IMPL_OFFSET>,
            removeAttributeNode: removeAttributeNode::<Impl, IMPL_OFFSET>,
            getElementsByTagName: getElementsByTagName::<Impl, IMPL_OFFSET>,
            normalize: normalize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMElement as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMEntity_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn publicId(&mut self, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn systemId(&mut self, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn notationName(&mut self, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMEntity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMEntity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMEntity_Vtbl {
        unsafe extern "system" fn publicId<Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).publicId(::core::mem::transmute_copy(&publicid)).into()
        }
        unsafe extern "system" fn systemId<Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).systemId(::core::mem::transmute_copy(&systemid)).into()
        }
        unsafe extern "system" fn notationName<Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).notationName(::core::mem::transmute_copy(&name)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            publicId: publicId::<Impl, IMPL_OFFSET>,
            systemId: systemId::<Impl, IMPL_OFFSET>,
            notationName: notationName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMEntity as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMEntityReference_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMEntityReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMEntityReference_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMEntityReference_Vtbl {
        Self { base: IXMLDOMNode_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMEntityReference as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMImplementation_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn hasFeature(&mut self, feature: &super::super::super::Foundation::BSTR, version: &super::super::super::Foundation::BSTR, hasfeature: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMImplementation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMImplementation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMImplementation_Vtbl {
        unsafe extern "system" fn hasFeature<Impl: IXMLDOMImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, version: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, hasfeature: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).hasFeature(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&version), ::core::mem::transmute_copy(&hasfeature)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            hasFeature: hasFeature::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMImplementation as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNamedNodeMap_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getNamedItem(&mut self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn setNamedItem(&mut self, newitem: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeNamedItem(&mut self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn item(&mut self, index: i32) -> ::windows::core::Result<IXMLDOMNode>;
    fn length(&mut self, listlength: *mut i32) -> ::windows::core::Result<()>;
    fn getQualifiedItem(&mut self, basename: &super::super::super::Foundation::BSTR, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeQualifiedItem(&mut self, basename: &super::super::super::Foundation::BSTR, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn nextNode(&mut self) -> ::windows::core::Result<IXMLDOMNode>;
    fn reset(&mut self) -> ::windows::core::Result<()>;
    fn _newEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNamedNodeMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNamedNodeMap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMNamedNodeMap_Vtbl {
        unsafe extern "system" fn getNamedItem<Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nameditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getNamedItem(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *nameditem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setNamedItem<Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newitem: ::windows::core::RawPtr, nameitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setNamedItem(::core::mem::transmute(&newitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *nameitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeNamedItem<Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nameditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).removeNamedItem(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *nameditem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *listitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).length(::core::mem::transmute_copy(&listlength)).into()
        }
        unsafe extern "system" fn getQualifiedItem<Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, qualifieditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getQualifiedItem(::core::mem::transmute_copy(&basename), ::core::mem::transmute_copy(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *qualifieditem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeQualifiedItem<Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, qualifieditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).removeQualifiedItem(::core::mem::transmute_copy(&basename), ::core::mem::transmute_copy(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *qualifieditem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextNode<Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).nextNode() {
                ::core::result::Result::Ok(ok__) => {
                    *nextitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).reset().into()
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            getNamedItem: getNamedItem::<Impl, IMPL_OFFSET>,
            setNamedItem: setNamedItem::<Impl, IMPL_OFFSET>,
            removeNamedItem: removeNamedItem::<Impl, IMPL_OFFSET>,
            item: item::<Impl, IMPL_OFFSET>,
            length: length::<Impl, IMPL_OFFSET>,
            getQualifiedItem: getQualifiedItem::<Impl, IMPL_OFFSET>,
            removeQualifiedItem: removeQualifiedItem::<Impl, IMPL_OFFSET>,
            nextNode: nextNode::<Impl, IMPL_OFFSET>,
            reset: reset::<Impl, IMPL_OFFSET>,
            _newEnum: _newEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNamedNodeMap as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNode_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn nodeName(&mut self, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn nodeValue(&mut self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetnodeValue(&mut self, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn nodeType(&mut self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()>;
    fn parentNode(&mut self) -> ::windows::core::Result<IXMLDOMNode>;
    fn childNodes(&mut self) -> ::windows::core::Result<IXMLDOMNodeList>;
    fn firstChild(&mut self) -> ::windows::core::Result<IXMLDOMNode>;
    fn lastChild(&mut self) -> ::windows::core::Result<IXMLDOMNode>;
    fn previousSibling(&mut self) -> ::windows::core::Result<IXMLDOMNode>;
    fn nextSibling(&mut self) -> ::windows::core::Result<IXMLDOMNode>;
    fn attributes(&mut self) -> ::windows::core::Result<IXMLDOMNamedNodeMap>;
    fn insertBefore(&mut self, newchild: &::core::option::Option<IXMLDOMNode>, refchild: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>;
    fn replaceChild(&mut self, newchild: &::core::option::Option<IXMLDOMNode>, oldchild: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeChild(&mut self, childnode: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn appendChild(&mut self, newchild: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn hasChildNodes(&mut self, haschild: *mut i16) -> ::windows::core::Result<()>;
    fn ownerDocument(&mut self) -> ::windows::core::Result<IXMLDOMDocument>;
    fn cloneNode(&mut self, deep: i16) -> ::windows::core::Result<IXMLDOMNode>;
    fn nodeTypeString(&mut self, nodetype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn text(&mut self, text: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Settext(&mut self, text: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn specified(&mut self, isspecified: *mut i16) -> ::windows::core::Result<()>;
    fn definition(&mut self) -> ::windows::core::Result<IXMLDOMNode>;
    fn nodeTypedValue(&mut self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetnodeTypedValue(&mut self, typedvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn dataType(&mut self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetdataType(&mut self, datatypename: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn xml(&mut self, xmlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn transformNode(&mut self, stylesheet: &::core::option::Option<IXMLDOMNode>, xmlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn selectNodes(&mut self, querystring: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNodeList>;
    fn selectSingleNode(&mut self, querystring: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn parsed(&mut self, isparsed: *mut i16) -> ::windows::core::Result<()>;
    fn namespaceURI(&mut self, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn prefix(&mut self, prefixstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn baseName(&mut self, namestring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn transformNodeToObject(&mut self, stylesheet: &::core::option::Option<IXMLDOMNode>, outputobject: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMNode_Vtbl {
        unsafe extern "system" fn nodeName<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).nodeName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn nodeValue<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).nodeValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetnodeValue<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetnodeValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn nodeType<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut DOMNodeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).nodeType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn parentNode<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).parentNode() {
                ::core::result::Result::Ok(ok__) => {
                    *parent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn childNodes<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).childNodes() {
                ::core::result::Result::Ok(ok__) => {
                    *childlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn firstChild<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).firstChild() {
                ::core::result::Result::Ok(ok__) => {
                    *firstchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lastChild<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).lastChild() {
                ::core::result::Result::Ok(ok__) => {
                    *lastchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn previousSibling<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previoussibling: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).previousSibling() {
                ::core::result::Result::Ok(ok__) => {
                    *previoussibling = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextSibling<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextsibling: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).nextSibling() {
                ::core::result::Result::Ok(ok__) => {
                    *nextsibling = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributemap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributemap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn insertBefore<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, refchild: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, outnewchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).insertBefore(::core::mem::transmute(&newchild), ::core::mem::transmute_copy(&refchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *outnewchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn replaceChild<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, oldchild: ::windows::core::RawPtr, outoldchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).replaceChild(::core::mem::transmute(&newchild), ::core::mem::transmute(&oldchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *outoldchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeChild<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childnode: ::windows::core::RawPtr, oldchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).removeChild(::core::mem::transmute(&childnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *oldchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn appendChild<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, outnewchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).appendChild(::core::mem::transmute(&newchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *outnewchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasChildNodes<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haschild: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).hasChildNodes(::core::mem::transmute_copy(&haschild)).into()
        }
        unsafe extern "system" fn ownerDocument<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmldomdocument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ownerDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *xmldomdocument = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn cloneNode<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deep: i16, cloneroot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).cloneNode(::core::mem::transmute_copy(&deep)) {
                ::core::result::Result::Ok(ok__) => {
                    *cloneroot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeTypeString<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodetype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).nodeTypeString(::core::mem::transmute_copy(&nodetype)).into()
        }
        unsafe extern "system" fn text<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).text(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn Settext<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Settext(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn specified<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).specified(::core::mem::transmute_copy(&isspecified)).into()
        }
        unsafe extern "system" fn definition<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, definitionnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).definition() {
                ::core::result::Result::Ok(ok__) => {
                    *definitionnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeTypedValue<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).nodeTypedValue(::core::mem::transmute_copy(&typedvalue)).into()
        }
        unsafe extern "system" fn SetnodeTypedValue<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typedvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetnodeTypedValue(::core::mem::transmute_copy(&typedvalue)).into()
        }
        unsafe extern "system" fn dataType<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).dataType(::core::mem::transmute_copy(&datatypename)).into()
        }
        unsafe extern "system" fn SetdataType<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatypename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetdataType(::core::mem::transmute_copy(&datatypename)).into()
        }
        unsafe extern "system" fn xml<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).xml(::core::mem::transmute_copy(&xmlstring)).into()
        }
        unsafe extern "system" fn transformNode<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: ::windows::core::RawPtr, xmlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).transformNode(::core::mem::transmute(&stylesheet), ::core::mem::transmute_copy(&xmlstring)).into()
        }
        unsafe extern "system" fn selectNodes<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querystring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).selectNodes(::core::mem::transmute_copy(&querystring)) {
                ::core::result::Result::Ok(ok__) => {
                    *resultlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn selectSingleNode<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querystring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).selectSingleNode(::core::mem::transmute_copy(&querystring)) {
                ::core::result::Result::Ok(ok__) => {
                    *resultnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parsed<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isparsed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).parsed(::core::mem::transmute_copy(&isparsed)).into()
        }
        unsafe extern "system" fn namespaceURI<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).namespaceURI(::core::mem::transmute_copy(&namespaceuri)).into()
        }
        unsafe extern "system" fn prefix<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefixstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).prefix(::core::mem::transmute_copy(&prefixstring)).into()
        }
        unsafe extern "system" fn baseName<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namestring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).baseName(::core::mem::transmute_copy(&namestring)).into()
        }
        unsafe extern "system" fn transformNodeToObject<Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: ::windows::core::RawPtr, outputobject: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).transformNodeToObject(::core::mem::transmute(&stylesheet), ::core::mem::transmute_copy(&outputobject)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            nodeName: nodeName::<Impl, IMPL_OFFSET>,
            nodeValue: nodeValue::<Impl, IMPL_OFFSET>,
            SetnodeValue: SetnodeValue::<Impl, IMPL_OFFSET>,
            nodeType: nodeType::<Impl, IMPL_OFFSET>,
            parentNode: parentNode::<Impl, IMPL_OFFSET>,
            childNodes: childNodes::<Impl, IMPL_OFFSET>,
            firstChild: firstChild::<Impl, IMPL_OFFSET>,
            lastChild: lastChild::<Impl, IMPL_OFFSET>,
            previousSibling: previousSibling::<Impl, IMPL_OFFSET>,
            nextSibling: nextSibling::<Impl, IMPL_OFFSET>,
            attributes: attributes::<Impl, IMPL_OFFSET>,
            insertBefore: insertBefore::<Impl, IMPL_OFFSET>,
            replaceChild: replaceChild::<Impl, IMPL_OFFSET>,
            removeChild: removeChild::<Impl, IMPL_OFFSET>,
            appendChild: appendChild::<Impl, IMPL_OFFSET>,
            hasChildNodes: hasChildNodes::<Impl, IMPL_OFFSET>,
            ownerDocument: ownerDocument::<Impl, IMPL_OFFSET>,
            cloneNode: cloneNode::<Impl, IMPL_OFFSET>,
            nodeTypeString: nodeTypeString::<Impl, IMPL_OFFSET>,
            text: text::<Impl, IMPL_OFFSET>,
            Settext: Settext::<Impl, IMPL_OFFSET>,
            specified: specified::<Impl, IMPL_OFFSET>,
            definition: definition::<Impl, IMPL_OFFSET>,
            nodeTypedValue: nodeTypedValue::<Impl, IMPL_OFFSET>,
            SetnodeTypedValue: SetnodeTypedValue::<Impl, IMPL_OFFSET>,
            dataType: dataType::<Impl, IMPL_OFFSET>,
            SetdataType: SetdataType::<Impl, IMPL_OFFSET>,
            xml: xml::<Impl, IMPL_OFFSET>,
            transformNode: transformNode::<Impl, IMPL_OFFSET>,
            selectNodes: selectNodes::<Impl, IMPL_OFFSET>,
            selectSingleNode: selectSingleNode::<Impl, IMPL_OFFSET>,
            parsed: parsed::<Impl, IMPL_OFFSET>,
            namespaceURI: namespaceURI::<Impl, IMPL_OFFSET>,
            prefix: prefix::<Impl, IMPL_OFFSET>,
            baseName: baseName::<Impl, IMPL_OFFSET>,
            transformNodeToObject: transformNodeToObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNode as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNodeList_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn item(&mut self, index: i32) -> ::windows::core::Result<IXMLDOMNode>;
    fn length(&mut self, listlength: *mut i32) -> ::windows::core::Result<()>;
    fn nextNode(&mut self) -> ::windows::core::Result<IXMLDOMNode>;
    fn reset(&mut self) -> ::windows::core::Result<()>;
    fn _newEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNodeList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNodeList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMNodeList_Vtbl {
        unsafe extern "system" fn item<Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *listitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).length(::core::mem::transmute_copy(&listlength)).into()
        }
        unsafe extern "system" fn nextNode<Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).nextNode() {
                ::core::result::Result::Ok(ok__) => {
                    *nextitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).reset().into()
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            item: item::<Impl, IMPL_OFFSET>,
            length: length::<Impl, IMPL_OFFSET>,
            nextNode: nextNode::<Impl, IMPL_OFFSET>,
            reset: reset::<Impl, IMPL_OFFSET>,
            _newEnum: _newEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNodeList as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNotation_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn publicId(&mut self, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn systemId(&mut self, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNotation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNotation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMNotation_Vtbl {
        unsafe extern "system" fn publicId<Impl: IXMLDOMNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).publicId(::core::mem::transmute_copy(&publicid)).into()
        }
        unsafe extern "system" fn systemId<Impl: IXMLDOMNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).systemId(::core::mem::transmute_copy(&systemid)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            publicId: publicId::<Impl, IMPL_OFFSET>,
            systemId: systemId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNotation as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMParseError_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn errorCode(&mut self, errorcode: *mut i32) -> ::windows::core::Result<()>;
    fn url(&mut self, urlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn reason(&mut self, reasonstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn srcText(&mut self, sourcestring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn line(&mut self, linenumber: *mut i32) -> ::windows::core::Result<()>;
    fn linepos(&mut self, lineposition: *mut i32) -> ::windows::core::Result<()>;
    fn filepos(&mut self, fileposition: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMParseError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMParseError_Vtbl {
        unsafe extern "system" fn errorCode<Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).errorCode(::core::mem::transmute_copy(&errorcode)).into()
        }
        unsafe extern "system" fn url<Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).url(::core::mem::transmute_copy(&urlstring)).into()
        }
        unsafe extern "system" fn reason<Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reasonstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).reason(::core::mem::transmute_copy(&reasonstring)).into()
        }
        unsafe extern "system" fn srcText<Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).srcText(::core::mem::transmute_copy(&sourcestring)).into()
        }
        unsafe extern "system" fn line<Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).line(::core::mem::transmute_copy(&linenumber)).into()
        }
        unsafe extern "system" fn linepos<Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).linepos(::core::mem::transmute_copy(&lineposition)).into()
        }
        unsafe extern "system" fn filepos<Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).filepos(::core::mem::transmute_copy(&fileposition)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            errorCode: errorCode::<Impl, IMPL_OFFSET>,
            url: url::<Impl, IMPL_OFFSET>,
            reason: reason::<Impl, IMPL_OFFSET>,
            srcText: srcText::<Impl, IMPL_OFFSET>,
            line: line::<Impl, IMPL_OFFSET>,
            linepos: linepos::<Impl, IMPL_OFFSET>,
            filepos: filepos::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMParseError as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMParseError2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMParseError_Impl {
    fn errorXPath(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn allErrors(&mut self) -> ::windows::core::Result<IXMLDOMParseErrorCollection>;
    fn errorParameters(&mut self, index: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn errorParametersCount(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMParseError2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMParseError2_Vtbl {
        unsafe extern "system" fn errorXPath<Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpathexpr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).errorXPath() {
                ::core::result::Result::Ok(ok__) => {
                    *xpathexpr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn allErrors<Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allerrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).allErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *allerrors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorParameters<Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, param1: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).errorParameters(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorParametersCount<Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).errorParametersCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXMLDOMParseError_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            errorXPath: errorXPath::<Impl, IMPL_OFFSET>,
            allErrors: allErrors::<Impl, IMPL_OFFSET>,
            errorParameters: errorParameters::<Impl, IMPL_OFFSET>,
            errorParametersCount: errorParametersCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMParseError2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMParseError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMParseErrorCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn item(&mut self, index: i32) -> ::windows::core::Result<IXMLDOMParseError2>;
    fn length(&mut self) -> ::windows::core::Result<i32>;
    fn next(&mut self) -> ::windows::core::Result<IXMLDOMParseError2>;
    fn reset(&mut self) -> ::windows::core::Result<()>;
    fn _newEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMParseErrorCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseErrorCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMParseErrorCollection_Vtbl {
        unsafe extern "system" fn item<Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, error: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *error = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn next<Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).next() {
                ::core::result::Result::Ok(ok__) => {
                    *error = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).reset().into()
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            item: item::<Impl, IMPL_OFFSET>,
            length: length::<Impl, IMPL_OFFSET>,
            next: next::<Impl, IMPL_OFFSET>,
            reset: reset::<Impl, IMPL_OFFSET>,
            _newEnum: _newEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMParseErrorCollection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMProcessingInstruction_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn target(&mut self, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn data(&mut self, value: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Setdata(&mut self, value: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMProcessingInstruction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMProcessingInstruction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMProcessingInstruction_Vtbl {
        unsafe extern "system" fn target<Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).target(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn data<Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).data(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Setdata<Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setdata(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            target: target::<Impl, IMPL_OFFSET>,
            data: data::<Impl, IMPL_OFFSET>,
            Setdata: Setdata::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMProcessingInstruction as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMSchemaCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn add(&mut self, namespaceuri: &super::super::super::Foundation::BSTR, var: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn get(&mut self, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn remove(&mut self, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn length(&mut self) -> ::windows::core::Result<i32>;
    fn namespaceURI(&mut self, index: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn addCollection(&mut self, othercollection: &::core::option::Option<IXMLDOMSchemaCollection>) -> ::windows::core::Result<()>;
    fn _newEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMSchemaCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMSchemaCollection_Vtbl {
        unsafe extern "system" fn add<Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, var: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).add(::core::mem::transmute_copy(&namespaceuri), ::core::mem::transmute_copy(&var)).into()
        }
        unsafe extern "system" fn get<Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, schemanode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).get(::core::mem::transmute_copy(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *schemanode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).remove(::core::mem::transmute_copy(&namespaceuri)).into()
        }
        unsafe extern "system" fn length<Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn namespaceURI<Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, length: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).namespaceURI(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addCollection<Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addCollection(::core::mem::transmute(&othercollection)).into()
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            add: add::<Impl, IMPL_OFFSET>,
            get: get::<Impl, IMPL_OFFSET>,
            remove: remove::<Impl, IMPL_OFFSET>,
            length: length::<Impl, IMPL_OFFSET>,
            namespaceURI: namespaceURI::<Impl, IMPL_OFFSET>,
            addCollection: addCollection::<Impl, IMPL_OFFSET>,
            _newEnum: _newEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMSchemaCollection2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMSchemaCollection_Impl {
    fn validate(&mut self) -> ::windows::core::Result<()>;
    fn SetvalidateOnLoad(&mut self, validateonload: i16) -> ::windows::core::Result<()>;
    fn validateOnLoad(&mut self) -> ::windows::core::Result<i16>;
    fn getSchema(&mut self, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<ISchema>;
    fn getDeclaration(&mut self, node: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<ISchemaItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMSchemaCollection2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMSchemaCollection2_Vtbl {
        unsafe extern "system" fn validate<Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).validate().into()
        }
        unsafe extern "system" fn SetvalidateOnLoad<Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, validateonload: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetvalidateOnLoad(::core::mem::transmute_copy(&validateonload)).into()
        }
        unsafe extern "system" fn validateOnLoad<Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, validateonload: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).validateOnLoad() {
                ::core::result::Result::Ok(ok__) => {
                    *validateonload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSchema<Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, schema: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getSchema(::core::mem::transmute_copy(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *schema = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDeclaration<Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getDeclaration(::core::mem::transmute(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXMLDOMSchemaCollection_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            validate: validate::<Impl, IMPL_OFFSET>,
            SetvalidateOnLoad: SetvalidateOnLoad::<Impl, IMPL_OFFSET>,
            validateOnLoad: validateOnLoad::<Impl, IMPL_OFFSET>,
            getSchema: getSchema::<Impl, IMPL_OFFSET>,
            getDeclaration: getDeclaration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMSchemaCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMSelection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNodeList_Impl {
    fn expr(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Setexpr(&mut self, expression: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn context(&mut self) -> ::windows::core::Result<IXMLDOMNode>;
    fn putref_context(&mut self, pnode: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<()>;
    fn peekNode(&mut self) -> ::windows::core::Result<IXMLDOMNode>;
    fn matches(&mut self, pnode: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeNext(&mut self) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeAll(&mut self) -> ::windows::core::Result<()>;
    fn clone(&mut self) -> ::windows::core::Result<IXMLDOMSelection>;
    fn getProperty(&mut self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn setProperty(&mut self, name: &super::super::super::Foundation::BSTR, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMSelection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMSelection_Vtbl {
        unsafe extern "system" fn expr<Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expression: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).expr() {
                ::core::result::Result::Ok(ok__) => {
                    *expression = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setexpr<Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expression: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setexpr(::core::mem::transmute_copy(&expression)).into()
        }
        unsafe extern "system" fn context<Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).context() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_context<Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_context(::core::mem::transmute(&pnode)).into()
        }
        unsafe extern "system" fn peekNode<Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).peekNode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn matches<Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).matches(::core::mem::transmute(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeNext<Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).removeNext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAll<Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).removeAll().into()
        }
        unsafe extern "system" fn clone<Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProperty<Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getProperty(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProperty<Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setProperty(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IXMLDOMNodeList_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            expr: expr::<Impl, IMPL_OFFSET>,
            Setexpr: Setexpr::<Impl, IMPL_OFFSET>,
            context: context::<Impl, IMPL_OFFSET>,
            putref_context: putref_context::<Impl, IMPL_OFFSET>,
            peekNode: peekNode::<Impl, IMPL_OFFSET>,
            matches: matches::<Impl, IMPL_OFFSET>,
            removeNext: removeNext::<Impl, IMPL_OFFSET>,
            removeAll: removeAll::<Impl, IMPL_OFFSET>,
            clone: clone::<Impl, IMPL_OFFSET>,
            getProperty: getProperty::<Impl, IMPL_OFFSET>,
            setProperty: setProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMSelection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNodeList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMText_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl + IXMLDOMCharacterData_Impl {
    fn splitText(&mut self, offset: i32) -> ::windows::core::Result<IXMLDOMText>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMText_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMText_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDOMText_Vtbl {
        unsafe extern "system" fn splitText<Impl: IXMLDOMText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, righthandtextnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).splitText(::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *righthandtextnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IXMLDOMCharacterData_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), splitText: splitText::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMText as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID || iid == &<IXMLDOMCharacterData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDSOControl_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(&mut self) -> ::windows::core::Result<IXMLDOMDocument>;
    fn SetXMLDocument(&mut self, ppdoc: &::core::option::Option<IXMLDOMDocument>) -> ::windows::core::Result<()>;
    fn JavaDSOCompatible(&mut self, fjavadsocompatible: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetJavaDSOCompatible(&mut self, fjavadsocompatible: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn readyState(&mut self, state: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDSOControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDSOControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDSOControl_Vtbl {
        unsafe extern "system" fn XMLDocument<Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XMLDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdoc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXMLDocument<Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdoc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXMLDocument(::core::mem::transmute(&ppdoc)).into()
        }
        unsafe extern "system" fn JavaDSOCompatible<Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).JavaDSOCompatible(::core::mem::transmute_copy(&fjavadsocompatible)).into()
        }
        unsafe extern "system" fn SetJavaDSOCompatible<Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJavaDSOCompatible(::core::mem::transmute_copy(&fjavadsocompatible)).into()
        }
        unsafe extern "system" fn readyState<Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).readyState(::core::mem::transmute_copy(&state)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            XMLDocument: XMLDocument::<Impl, IMPL_OFFSET>,
            SetXMLDocument: SetXMLDocument::<Impl, IMPL_OFFSET>,
            JavaDSOCompatible: JavaDSOCompatible::<Impl, IMPL_OFFSET>,
            SetJavaDSOCompatible: SetJavaDSOCompatible::<Impl, IMPL_OFFSET>,
            readyState: readyState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDSOControl as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDocument_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn root(&mut self) -> ::windows::core::Result<IXMLElement>;
    fn fileSize(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fileModifiedDate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fileUpdatedDate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn URL(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetURL(&mut self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn mimeType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn readyState(&mut self) -> ::windows::core::Result<i32>;
    fn charset(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Setcharset(&mut self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn version(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn doctype(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn dtdURL(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn createElement(&mut self, vtype: &super::super::super::System::Com::VARIANT, var1: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLElement>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDocument_Vtbl {
        unsafe extern "system" fn root<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).root() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileSize<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileModifiedDate<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fileModifiedDate() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fileUpdatedDate() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).URL() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetURL(::core::mem::transmute_copy(&p)).into()
        }
        unsafe extern "system" fn mimeType<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).mimeType() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).readyState() {
                ::core::result::Result::Ok(ok__) => {
                    *pl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn charset<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).charset() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setcharset<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setcharset(::core::mem::transmute_copy(&p)).into()
        }
        unsafe extern "system" fn version<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).version() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn doctype<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).doctype() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdURL<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).dtdURL() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createElement<Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtype: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var1: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppelem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createElement(::core::mem::transmute_copy(&vtype), ::core::mem::transmute_copy(&var1)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppelem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            root: root::<Impl, IMPL_OFFSET>,
            fileSize: fileSize::<Impl, IMPL_OFFSET>,
            fileModifiedDate: fileModifiedDate::<Impl, IMPL_OFFSET>,
            fileUpdatedDate: fileUpdatedDate::<Impl, IMPL_OFFSET>,
            URL: URL::<Impl, IMPL_OFFSET>,
            SetURL: SetURL::<Impl, IMPL_OFFSET>,
            mimeType: mimeType::<Impl, IMPL_OFFSET>,
            readyState: readyState::<Impl, IMPL_OFFSET>,
            charset: charset::<Impl, IMPL_OFFSET>,
            Setcharset: Setcharset::<Impl, IMPL_OFFSET>,
            version: version::<Impl, IMPL_OFFSET>,
            doctype: doctype::<Impl, IMPL_OFFSET>,
            dtdURL: dtdURL::<Impl, IMPL_OFFSET>,
            createElement: createElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDocument as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDocument2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn root(&mut self) -> ::windows::core::Result<IXMLElement2>;
    fn fileSize(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fileModifiedDate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fileUpdatedDate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn URL(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetURL(&mut self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn mimeType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn readyState(&mut self) -> ::windows::core::Result<i32>;
    fn charset(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Setcharset(&mut self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn version(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn doctype(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn dtdURL(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn createElement(&mut self, vtype: &super::super::super::System::Com::VARIANT, var1: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLElement2>;
    fn r#async(&mut self) -> ::windows::core::Result<i16>;
    fn Setasync(&mut self, f: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDocument2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLDocument2_Vtbl {
        unsafe extern "system" fn root<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).root() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileSize<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileModifiedDate<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fileModifiedDate() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fileUpdatedDate() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).URL() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetURL(::core::mem::transmute_copy(&p)).into()
        }
        unsafe extern "system" fn mimeType<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).mimeType() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).readyState() {
                ::core::result::Result::Ok(ok__) => {
                    *pl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn charset<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).charset() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setcharset<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setcharset(::core::mem::transmute_copy(&p)).into()
        }
        unsafe extern "system" fn version<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).version() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn doctype<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).doctype() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdURL<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).dtdURL() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createElement<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtype: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var1: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppelem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createElement(::core::mem::transmute_copy(&vtype), ::core::mem::transmute_copy(&var1)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppelem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#async<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pf: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).r#async() {
                ::core::result::Result::Ok(ok__) => {
                    *pf = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setasync<Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, f: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setasync(::core::mem::transmute_copy(&f)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            root: root::<Impl, IMPL_OFFSET>,
            fileSize: fileSize::<Impl, IMPL_OFFSET>,
            fileModifiedDate: fileModifiedDate::<Impl, IMPL_OFFSET>,
            fileUpdatedDate: fileUpdatedDate::<Impl, IMPL_OFFSET>,
            URL: URL::<Impl, IMPL_OFFSET>,
            SetURL: SetURL::<Impl, IMPL_OFFSET>,
            mimeType: mimeType::<Impl, IMPL_OFFSET>,
            readyState: readyState::<Impl, IMPL_OFFSET>,
            charset: charset::<Impl, IMPL_OFFSET>,
            Setcharset: Setcharset::<Impl, IMPL_OFFSET>,
            version: version::<Impl, IMPL_OFFSET>,
            doctype: doctype::<Impl, IMPL_OFFSET>,
            dtdURL: dtdURL::<Impl, IMPL_OFFSET>,
            createElement: createElement::<Impl, IMPL_OFFSET>,
            r#async: r#async::<Impl, IMPL_OFFSET>,
            Setasync: Setasync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDocument2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLElement_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn tagName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SettagName(&mut self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn parent(&mut self) -> ::windows::core::Result<IXMLElement>;
    fn setAttribute(&mut self, strpropertyname: &super::super::super::Foundation::BSTR, propertyvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getAttribute(&mut self, strpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn removeAttribute(&mut self, strpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn children(&mut self) -> ::windows::core::Result<IXMLElementCollection>;
    fn r#type(&mut self) -> ::windows::core::Result<i32>;
    fn text(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Settext(&mut self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn addChild(&mut self, pchildelem: &::core::option::Option<IXMLElement>, lindex: i32, lreserved: i32) -> ::windows::core::Result<()>;
    fn removeChild(&mut self, pchildelem: &::core::option::Option<IXMLElement>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLElement_Vtbl {
        unsafe extern "system" fn tagName<Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).tagName() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettagName<Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SettagName(::core::mem::transmute_copy(&p)).into()
        }
        unsafe extern "system" fn parent<Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).parent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setAttribute(::core::mem::transmute_copy(&strpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn getAttribute<Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAttribute(::core::mem::transmute_copy(&strpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).removeAttribute(::core::mem::transmute_copy(&strpropertyname)).into()
        }
        unsafe extern "system" fn children<Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).children() {
                ::core::result::Result::Ok(ok__) => {
                    *pp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).r#type() {
                ::core::result::Result::Ok(ok__) => {
                    *pltype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).text() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Settext(::core::mem::transmute_copy(&p)).into()
        }
        unsafe extern "system" fn addChild<Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr, lindex: i32, lreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addChild(::core::mem::transmute(&pchildelem), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&lreserved)).into()
        }
        unsafe extern "system" fn removeChild<Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).removeChild(::core::mem::transmute(&pchildelem)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            tagName: tagName::<Impl, IMPL_OFFSET>,
            SettagName: SettagName::<Impl, IMPL_OFFSET>,
            parent: parent::<Impl, IMPL_OFFSET>,
            setAttribute: setAttribute::<Impl, IMPL_OFFSET>,
            getAttribute: getAttribute::<Impl, IMPL_OFFSET>,
            removeAttribute: removeAttribute::<Impl, IMPL_OFFSET>,
            children: children::<Impl, IMPL_OFFSET>,
            r#type: r#type::<Impl, IMPL_OFFSET>,
            text: text::<Impl, IMPL_OFFSET>,
            Settext: Settext::<Impl, IMPL_OFFSET>,
            addChild: addChild::<Impl, IMPL_OFFSET>,
            removeChild: removeChild::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLElement as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLElement2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn tagName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SettagName(&mut self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn parent(&mut self) -> ::windows::core::Result<IXMLElement2>;
    fn setAttribute(&mut self, strpropertyname: &super::super::super::Foundation::BSTR, propertyvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getAttribute(&mut self, strpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn removeAttribute(&mut self, strpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn children(&mut self) -> ::windows::core::Result<IXMLElementCollection>;
    fn r#type(&mut self) -> ::windows::core::Result<i32>;
    fn text(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Settext(&mut self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn addChild(&mut self, pchildelem: &::core::option::Option<IXMLElement2>, lindex: i32, lreserved: i32) -> ::windows::core::Result<()>;
    fn removeChild(&mut self, pchildelem: &::core::option::Option<IXMLElement2>) -> ::windows::core::Result<()>;
    fn attributes(&mut self) -> ::windows::core::Result<IXMLElementCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLElement2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLElement2_Vtbl {
        unsafe extern "system" fn tagName<Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).tagName() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettagName<Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SettagName(::core::mem::transmute_copy(&p)).into()
        }
        unsafe extern "system" fn parent<Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).parent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setAttribute(::core::mem::transmute_copy(&strpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn getAttribute<Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAttribute(::core::mem::transmute_copy(&strpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).removeAttribute(::core::mem::transmute_copy(&strpropertyname)).into()
        }
        unsafe extern "system" fn children<Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).children() {
                ::core::result::Result::Ok(ok__) => {
                    *pp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).r#type() {
                ::core::result::Result::Ok(ok__) => {
                    *pltype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).text() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Settext(::core::mem::transmute_copy(&p)).into()
        }
        unsafe extern "system" fn addChild<Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr, lindex: i32, lreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addChild(::core::mem::transmute(&pchildelem), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&lreserved)).into()
        }
        unsafe extern "system" fn removeChild<Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).removeChild(::core::mem::transmute(&pchildelem)).into()
        }
        unsafe extern "system" fn attributes<Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            tagName: tagName::<Impl, IMPL_OFFSET>,
            SettagName: SettagName::<Impl, IMPL_OFFSET>,
            parent: parent::<Impl, IMPL_OFFSET>,
            setAttribute: setAttribute::<Impl, IMPL_OFFSET>,
            getAttribute: getAttribute::<Impl, IMPL_OFFSET>,
            removeAttribute: removeAttribute::<Impl, IMPL_OFFSET>,
            children: children::<Impl, IMPL_OFFSET>,
            r#type: r#type::<Impl, IMPL_OFFSET>,
            text: text::<Impl, IMPL_OFFSET>,
            Settext: Settext::<Impl, IMPL_OFFSET>,
            addChild: addChild::<Impl, IMPL_OFFSET>,
            removeChild: removeChild::<Impl, IMPL_OFFSET>,
            attributes: attributes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLElement2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLElementCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setlength(&mut self, v: i32) -> ::windows::core::Result<()>;
    fn length(&mut self) -> ::windows::core::Result<i32>;
    fn _newEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn item(&mut self, var1: &super::super::super::System::Com::VARIANT, var2: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLElementCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElementCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLElementCollection_Vtbl {
        unsafe extern "system" fn Setlength<Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setlength(::core::mem::transmute_copy(&v)).into()
        }
        unsafe extern "system" fn length<Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var1: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var2: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppdisp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&var1), ::core::mem::transmute_copy(&var2)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Setlength: Setlength::<Impl, IMPL_OFFSET>,
            length: length::<Impl, IMPL_OFFSET>,
            _newEnum: _newEnum::<Impl, IMPL_OFFSET>,
            item: item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLElementCollection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXMLError_Impl: Sized {
    fn GetErrorInfo(&mut self, perrorreturn: *mut XML_ERROR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXMLError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLError_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLError_Vtbl {
        unsafe extern "system" fn GetErrorInfo<Impl: IXMLError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorreturn: *mut XML_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetErrorInfo(::core::mem::transmute_copy(&perrorreturn)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetErrorInfo: GetErrorInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLHTTPRequest_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn open(&mut self, bstrmethod: &super::super::super::Foundation::BSTR, bstrurl: &super::super::super::Foundation::BSTR, varasync: &super::super::super::System::Com::VARIANT, bstruser: &super::super::super::System::Com::VARIANT, bstrpassword: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setRequestHeader(&mut self, bstrheader: &super::super::super::Foundation::BSTR, bstrvalue: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getResponseHeader(&mut self, bstrheader: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getAllResponseHeaders(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn send(&mut self, varbody: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn abort(&mut self) -> ::windows::core::Result<()>;
    fn status(&mut self) -> ::windows::core::Result<i32>;
    fn statusText(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn responseXML(&mut self) -> ::windows::core::Result<super::super::super::System::Com::IDispatch>;
    fn responseText(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn responseBody(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn responseStream(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn readyState(&mut self) -> ::windows::core::Result<i32>;
    fn Setonreadystatechange(&mut self, preadystatesink: &::core::option::Option<super::super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLHTTPRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLHTTPRequest_Vtbl {
        unsafe extern "system" fn open<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethod: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varasync: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstruser: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrpassword: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).open(::core::mem::transmute_copy(&bstrmethod), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&varasync), ::core::mem::transmute_copy(&bstruser), ::core::mem::transmute_copy(&bstrpassword)).into()
        }
        unsafe extern "system" fn setRequestHeader<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setRequestHeader(::core::mem::transmute_copy(&bstrheader), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn getResponseHeader<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getResponseHeader(::core::mem::transmute_copy(&bstrheader)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAllResponseHeaders<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrheaders: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAllResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn send<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).send(::core::mem::transmute_copy(&varbody)).into()
        }
        unsafe extern "system" fn abort<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).abort().into()
        }
        unsafe extern "system" fn status<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).status() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn statusText<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).statusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseXML<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbody: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).responseXML() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseText<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).responseText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseBody<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).responseBody() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseStream<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).responseStream() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).readyState() {
                ::core::result::Result::Ok(ok__) => {
                    *plstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setonreadystatechange<Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadystatesink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setonreadystatechange(::core::mem::transmute(&preadystatesink)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            open: open::<Impl, IMPL_OFFSET>,
            setRequestHeader: setRequestHeader::<Impl, IMPL_OFFSET>,
            getResponseHeader: getResponseHeader::<Impl, IMPL_OFFSET>,
            getAllResponseHeaders: getAllResponseHeaders::<Impl, IMPL_OFFSET>,
            send: send::<Impl, IMPL_OFFSET>,
            abort: abort::<Impl, IMPL_OFFSET>,
            status: status::<Impl, IMPL_OFFSET>,
            statusText: statusText::<Impl, IMPL_OFFSET>,
            responseXML: responseXML::<Impl, IMPL_OFFSET>,
            responseText: responseText::<Impl, IMPL_OFFSET>,
            responseBody: responseBody::<Impl, IMPL_OFFSET>,
            responseStream: responseStream::<Impl, IMPL_OFFSET>,
            readyState: readyState::<Impl, IMPL_OFFSET>,
            Setonreadystatechange: Setonreadystatechange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest2_Impl: Sized {
    fn Open(&mut self, pwszmethod: super::super::super::Foundation::PWSTR, pwszurl: super::super::super::Foundation::PWSTR, pstatuscallback: &::core::option::Option<IXMLHTTPRequest2Callback>, pwszusername: super::super::super::Foundation::PWSTR, pwszpassword: super::super::super::Foundation::PWSTR, pwszproxyusername: super::super::super::Foundation::PWSTR, pwszproxypassword: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Send(&mut self, pbody: &::core::option::Option<super::super::super::System::Com::ISequentialStream>, cbbody: u64) -> ::windows::core::Result<()>;
    fn Abort(&mut self) -> ::windows::core::Result<()>;
    fn SetCookie(&mut self, pcookie: *const XHR_COOKIE) -> ::windows::core::Result<u32>;
    fn SetCustomResponseStream(&mut self, psequentialstream: &::core::option::Option<super::super::super::System::Com::ISequentialStream>) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows::core::Result<()>;
    fn SetRequestHeader(&mut self, pwszheader: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetAllResponseHeaders(&mut self) -> ::windows::core::Result<*mut u16>;
    fn GetCookie(&mut self, pwszurl: super::super::super::Foundation::PWSTR, pwszname: super::super::super::Foundation::PWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows::core::Result<()>;
    fn GetResponseHeader(&mut self, pwszheader: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<*mut u16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLHTTPRequest2_Vtbl {
        unsafe extern "system" fn Open<Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmethod: super::super::super::Foundation::PWSTR, pwszurl: super::super::super::Foundation::PWSTR, pstatuscallback: ::windows::core::RawPtr, pwszusername: super::super::super::Foundation::PWSTR, pwszpassword: super::super::super::Foundation::PWSTR, pwszproxyusername: super::super::super::Foundation::PWSTR, pwszproxypassword: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pwszmethod), ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute(&pstatuscallback), ::core::mem::transmute_copy(&pwszusername), ::core::mem::transmute_copy(&pwszpassword), ::core::mem::transmute_copy(&pwszproxyusername), ::core::mem::transmute_copy(&pwszproxypassword)).into()
        }
        unsafe extern "system" fn Send<Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: ::windows::core::RawPtr, cbbody: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Send(::core::mem::transmute(&pbody), ::core::mem::transmute_copy(&cbbody)).into()
        }
        unsafe extern "system" fn Abort<Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort().into()
        }
        unsafe extern "system" fn SetCookie<Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcookie: *const XHR_COOKIE, pdwcookiestate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCookie(::core::mem::transmute_copy(&pcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookiestate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomResponseStream<Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psequentialstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomResponseStream(::core::mem::transmute(&psequentialstream)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&eproperty), ::core::mem::transmute_copy(&ullvalue)).into()
        }
        unsafe extern "system" fn SetRequestHeader<Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszheader: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestHeader(::core::mem::transmute_copy(&pwszheader), ::core::mem::transmute_copy(&pwszvalue)).into()
        }
        unsafe extern "system" fn GetAllResponseHeaders<Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszheaders: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCookie<Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, pwszname: super::super::super::Foundation::PWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCookie(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pccookies), ::core::mem::transmute_copy(&ppcookies)).into()
        }
        unsafe extern "system" fn GetResponseHeader<Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszheader: super::super::super::Foundation::PWSTR, ppwszvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResponseHeader(::core::mem::transmute_copy(&pwszheader)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Send: Send::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
            SetCookie: SetCookie::<Impl, IMPL_OFFSET>,
            SetCustomResponseStream: SetCustomResponseStream::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            SetRequestHeader: SetRequestHeader::<Impl, IMPL_OFFSET>,
            GetAllResponseHeaders: GetAllResponseHeaders::<Impl, IMPL_OFFSET>,
            GetCookie: GetCookie::<Impl, IMPL_OFFSET>,
            GetResponseHeader: GetResponseHeader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest2Callback_Impl: Sized {
    fn OnRedirect(&mut self, pxhr: &::core::option::Option<IXMLHTTPRequest2>, pwszredirecturl: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn OnHeadersAvailable(&mut self, pxhr: &::core::option::Option<IXMLHTTPRequest2>, dwstatus: u32, pwszstatus: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn OnDataAvailable(&mut self, pxhr: &::core::option::Option<IXMLHTTPRequest2>, presponsestream: &::core::option::Option<super::super::super::System::Com::ISequentialStream>) -> ::windows::core::Result<()>;
    fn OnResponseReceived(&mut self, pxhr: &::core::option::Option<IXMLHTTPRequest2>, presponsestream: &::core::option::Option<super::super::super::System::Com::ISequentialStream>) -> ::windows::core::Result<()>;
    fn OnError(&mut self, pxhr: &::core::option::Option<IXMLHTTPRequest2>, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest2Callback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2Callback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLHTTPRequest2Callback_Vtbl {
        unsafe extern "system" fn OnRedirect<Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, pwszredirecturl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnRedirect(::core::mem::transmute(&pxhr), ::core::mem::transmute_copy(&pwszredirecturl)).into()
        }
        unsafe extern "system" fn OnHeadersAvailable<Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, dwstatus: u32, pwszstatus: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnHeadersAvailable(::core::mem::transmute(&pxhr), ::core::mem::transmute_copy(&dwstatus), ::core::mem::transmute_copy(&pwszstatus)).into()
        }
        unsafe extern "system" fn OnDataAvailable<Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, presponsestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDataAvailable(::core::mem::transmute(&pxhr), ::core::mem::transmute(&presponsestream)).into()
        }
        unsafe extern "system" fn OnResponseReceived<Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, presponsestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnResponseReceived(::core::mem::transmute(&pxhr), ::core::mem::transmute(&presponsestream)).into()
        }
        unsafe extern "system" fn OnError<Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnError(::core::mem::transmute(&pxhr), ::core::mem::transmute_copy(&hrerror)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnRedirect: OnRedirect::<Impl, IMPL_OFFSET>,
            OnHeadersAvailable: OnHeadersAvailable::<Impl, IMPL_OFFSET>,
            OnDataAvailable: OnDataAvailable::<Impl, IMPL_OFFSET>,
            OnResponseReceived: OnResponseReceived::<Impl, IMPL_OFFSET>,
            OnError: OnError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest2Callback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest3_Impl: Sized + IXMLHTTPRequest2_Impl {
    fn SetClientCertificate(&mut self, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLHTTPRequest3_Vtbl {
        unsafe extern "system" fn SetClientCertificate<Impl: IXMLHTTPRequest3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCertificate(::core::mem::transmute_copy(&cbclientcertificatehash), ::core::mem::transmute_copy(&pbclientcertificatehash), ::core::mem::transmute_copy(&pwszpin)).into()
        }
        Self { base: IXMLHTTPRequest2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetClientCertificate: SetClientCertificate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3 as ::windows::core::Interface>::IID || iid == &<IXMLHTTPRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest3Callback_Impl: Sized + IXMLHTTPRequest2Callback_Impl {
    fn OnServerCertificateReceived(&mut self, pxhr: &::core::option::Option<IXMLHTTPRequest3>, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> ::windows::core::Result<()>;
    fn OnClientCertificateRequested(&mut self, pxhr: &::core::option::Option<IXMLHTTPRequest3>, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest3Callback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest3Callback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLHTTPRequest3Callback_Vtbl {
        unsafe extern "system" fn OnServerCertificateReceived<Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnServerCertificateReceived(::core::mem::transmute(&pxhr), ::core::mem::transmute_copy(&dwcertificateerrors), ::core::mem::transmute_copy(&cservercertificatechain), ::core::mem::transmute_copy(&rgservercertificatechain)).into()
        }
        unsafe extern "system" fn OnClientCertificateRequested<Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClientCertificateRequested(::core::mem::transmute(&pxhr), ::core::mem::transmute_copy(&cissuerlist), ::core::mem::transmute_copy(&rgpwszissuerlist)).into()
        }
        Self {
            base: IXMLHTTPRequest2Callback_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnServerCertificateReceived: OnServerCertificateReceived::<Impl, IMPL_OFFSET>,
            OnClientCertificateRequested: OnClientCertificateRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3Callback as ::windows::core::Interface>::IID || iid == &<IXMLHTTPRequest2Callback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLHttpRequest_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn open(&mut self, bstrmethod: &super::super::super::Foundation::BSTR, bstrurl: &super::super::super::Foundation::BSTR, varasync: &super::super::super::System::Com::VARIANT, bstruser: &super::super::super::System::Com::VARIANT, bstrpassword: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setRequestHeader(&mut self, bstrheader: &super::super::super::Foundation::BSTR, bstrvalue: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getResponseHeader(&mut self, bstrheader: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getAllResponseHeaders(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn send(&mut self, varbody: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn abort(&mut self) -> ::windows::core::Result<()>;
    fn status(&mut self) -> ::windows::core::Result<i32>;
    fn statusText(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn responseXML(&mut self) -> ::windows::core::Result<super::super::super::System::Com::IDispatch>;
    fn responseText(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn responseBody(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn responseStream(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn readyState(&mut self) -> ::windows::core::Result<i32>;
    fn Setonreadystatechange(&mut self, preadystatesink: &::core::option::Option<super::super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLHttpRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLHttpRequest_Vtbl {
        unsafe extern "system" fn open<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethod: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varasync: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstruser: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrpassword: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).open(::core::mem::transmute_copy(&bstrmethod), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&varasync), ::core::mem::transmute_copy(&bstruser), ::core::mem::transmute_copy(&bstrpassword)).into()
        }
        unsafe extern "system" fn setRequestHeader<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setRequestHeader(::core::mem::transmute_copy(&bstrheader), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn getResponseHeader<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getResponseHeader(::core::mem::transmute_copy(&bstrheader)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAllResponseHeaders<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrheaders: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAllResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn send<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).send(::core::mem::transmute_copy(&varbody)).into()
        }
        unsafe extern "system" fn abort<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).abort().into()
        }
        unsafe extern "system" fn status<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).status() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn statusText<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).statusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseXML<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbody: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).responseXML() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseText<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).responseText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseBody<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).responseBody() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseStream<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).responseStream() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).readyState() {
                ::core::result::Result::Ok(ok__) => {
                    *plstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setonreadystatechange<Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadystatesink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setonreadystatechange(::core::mem::transmute(&preadystatesink)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            open: open::<Impl, IMPL_OFFSET>,
            setRequestHeader: setRequestHeader::<Impl, IMPL_OFFSET>,
            getResponseHeader: getResponseHeader::<Impl, IMPL_OFFSET>,
            getAllResponseHeaders: getAllResponseHeaders::<Impl, IMPL_OFFSET>,
            send: send::<Impl, IMPL_OFFSET>,
            abort: abort::<Impl, IMPL_OFFSET>,
            status: status::<Impl, IMPL_OFFSET>,
            statusText: statusText::<Impl, IMPL_OFFSET>,
            responseXML: responseXML::<Impl, IMPL_OFFSET>,
            responseText: responseText::<Impl, IMPL_OFFSET>,
            responseBody: responseBody::<Impl, IMPL_OFFSET>,
            responseStream: responseStream::<Impl, IMPL_OFFSET>,
            readyState: readyState::<Impl, IMPL_OFFSET>,
            Setonreadystatechange: Setonreadystatechange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHttpRequest as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXSLProcessor_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setinput(&mut self, var: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn input(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn ownerTemplate(&mut self) -> ::windows::core::Result<IXSLTemplate>;
    fn setStartMode(&mut self, mode: &super::super::super::Foundation::BSTR, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn startMode(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn startModeURI(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Setoutput(&mut self, output: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn output(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn transform(&mut self) -> ::windows::core::Result<i16>;
    fn reset(&mut self) -> ::windows::core::Result<()>;
    fn readyState(&mut self) -> ::windows::core::Result<i32>;
    fn addParameter(&mut self, basename: &super::super::super::Foundation::BSTR, parameter: &super::super::super::System::Com::VARIANT, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn addObject(&mut self, obj: &::core::option::Option<super::super::super::System::Com::IDispatch>, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn stylesheet(&mut self) -> ::windows::core::Result<IXMLDOMNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXSLProcessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXSLProcessor_Vtbl {
        unsafe extern "system" fn Setinput<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setinput(::core::mem::transmute_copy(&var)).into()
        }
        unsafe extern "system" fn input<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).input() {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ownerTemplate<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ownerTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    *pptemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setStartMode<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setStartMode(::core::mem::transmute_copy(&mode), ::core::mem::transmute_copy(&namespaceuri)).into()
        }
        unsafe extern "system" fn startMode<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).startMode() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startModeURI<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).startModeURI() {
                ::core::result::Result::Ok(ok__) => {
                    *namespaceuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setoutput<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, output: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setoutput(::core::mem::transmute_copy(&output)).into()
        }
        unsafe extern "system" fn output<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).output() {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn transform<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdone: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).transform() {
                ::core::result::Result::Ok(ok__) => {
                    *pdone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).reset().into()
        }
        unsafe extern "system" fn readyState<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadystate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).readyState() {
                ::core::result::Result::Ok(ok__) => {
                    *preadystate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addParameter<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, parameter: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addParameter(::core::mem::transmute_copy(&basename), ::core::mem::transmute_copy(&parameter), ::core::mem::transmute_copy(&namespaceuri)).into()
        }
        unsafe extern "system" fn addObject<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, obj: ::windows::core::RawPtr, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addObject(::core::mem::transmute(&obj), ::core::mem::transmute_copy(&namespaceuri)).into()
        }
        unsafe extern "system" fn stylesheet<Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).stylesheet() {
                ::core::result::Result::Ok(ok__) => {
                    *stylesheet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Setinput: Setinput::<Impl, IMPL_OFFSET>,
            input: input::<Impl, IMPL_OFFSET>,
            ownerTemplate: ownerTemplate::<Impl, IMPL_OFFSET>,
            setStartMode: setStartMode::<Impl, IMPL_OFFSET>,
            startMode: startMode::<Impl, IMPL_OFFSET>,
            startModeURI: startModeURI::<Impl, IMPL_OFFSET>,
            Setoutput: Setoutput::<Impl, IMPL_OFFSET>,
            output: output::<Impl, IMPL_OFFSET>,
            transform: transform::<Impl, IMPL_OFFSET>,
            reset: reset::<Impl, IMPL_OFFSET>,
            readyState: readyState::<Impl, IMPL_OFFSET>,
            addParameter: addParameter::<Impl, IMPL_OFFSET>,
            addObject: addObject::<Impl, IMPL_OFFSET>,
            stylesheet: stylesheet::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXSLProcessor as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXSLTemplate_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn putref_stylesheet(&mut self, stylesheet: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<()>;
    fn stylesheet(&mut self) -> ::windows::core::Result<IXMLDOMNode>;
    fn createProcessor(&mut self) -> ::windows::core::Result<IXSLProcessor>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXSLTemplate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXSLTemplate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXSLTemplate_Vtbl {
        unsafe extern "system" fn putref_stylesheet<Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_stylesheet(::core::mem::transmute(&stylesheet)).into()
        }
        unsafe extern "system" fn stylesheet<Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).stylesheet() {
                ::core::result::Result::Ok(ok__) => {
                    *stylesheet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createProcessor<Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprocessor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createProcessor() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprocessor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            putref_stylesheet: putref_stylesheet::<Impl, IMPL_OFFSET>,
            stylesheet: stylesheet::<Impl, IMPL_OFFSET>,
            createProcessor: createProcessor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXSLTemplate as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXTLRuntime_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn uniqueID(&mut self, pnode: &::core::option::Option<IXMLDOMNode>, pid: *mut i32) -> ::windows::core::Result<()>;
    fn depth(&mut self, pnode: &::core::option::Option<IXMLDOMNode>, pdepth: *mut i32) -> ::windows::core::Result<()>;
    fn childNumber(&mut self, pnode: &::core::option::Option<IXMLDOMNode>, pnumber: *mut i32) -> ::windows::core::Result<()>;
    fn ancestorChildNumber(&mut self, bstrnodename: &super::super::super::Foundation::BSTR, pnode: &::core::option::Option<IXMLDOMNode>, pnumber: *mut i32) -> ::windows::core::Result<()>;
    fn absoluteChildNumber(&mut self, pnode: &::core::option::Option<IXMLDOMNode>, pnumber: *mut i32) -> ::windows::core::Result<()>;
    fn formatIndex(&mut self, lindex: i32, bstrformat: &super::super::super::Foundation::BSTR, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn formatNumber(&mut self, dblnumber: f64, bstrformat: &super::super::super::Foundation::BSTR, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn formatDate(&mut self, vardate: &super::super::super::System::Com::VARIANT, bstrformat: &super::super::super::Foundation::BSTR, vardestlocale: &super::super::super::System::Com::VARIANT, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn formatTime(&mut self, vartime: &super::super::super::System::Com::VARIANT, bstrformat: &super::super::super::Foundation::BSTR, vardestlocale: &super::super::super::System::Com::VARIANT, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXTLRuntime_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXTLRuntime_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXTLRuntime_Vtbl {
        unsafe extern "system" fn uniqueID<Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).uniqueID(::core::mem::transmute(&pnode), ::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn depth<Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pdepth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).depth(::core::mem::transmute(&pnode), ::core::mem::transmute_copy(&pdepth)).into()
        }
        unsafe extern "system" fn childNumber<Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).childNumber(::core::mem::transmute(&pnode), ::core::mem::transmute_copy(&pnumber)).into()
        }
        unsafe extern "system" fn ancestorChildNumber<Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnodename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pnode: ::windows::core::RawPtr, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ancestorChildNumber(::core::mem::transmute_copy(&bstrnodename), ::core::mem::transmute(&pnode), ::core::mem::transmute_copy(&pnumber)).into()
        }
        unsafe extern "system" fn absoluteChildNumber<Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).absoluteChildNumber(::core::mem::transmute(&pnode), ::core::mem::transmute_copy(&pnumber)).into()
        }
        unsafe extern "system" fn formatIndex<Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).formatIndex(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&bstrformat), ::core::mem::transmute_copy(&pbstrformattedstring)).into()
        }
        unsafe extern "system" fn formatNumber<Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dblnumber: f64, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).formatNumber(::core::mem::transmute_copy(&dblnumber), ::core::mem::transmute_copy(&bstrformat), ::core::mem::transmute_copy(&pbstrformattedstring)).into()
        }
        unsafe extern "system" fn formatDate<Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardate: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vardestlocale: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).formatDate(::core::mem::transmute_copy(&vardate), ::core::mem::transmute_copy(&bstrformat), ::core::mem::transmute_copy(&vardestlocale), ::core::mem::transmute_copy(&pbstrformattedstring)).into()
        }
        unsafe extern "system" fn formatTime<Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartime: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vardestlocale: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).formatTime(::core::mem::transmute_copy(&vartime), ::core::mem::transmute_copy(&bstrformat), ::core::mem::transmute_copy(&vardestlocale), ::core::mem::transmute_copy(&pbstrformattedstring)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            uniqueID: uniqueID::<Impl, IMPL_OFFSET>,
            depth: depth::<Impl, IMPL_OFFSET>,
            childNumber: childNumber::<Impl, IMPL_OFFSET>,
            ancestorChildNumber: ancestorChildNumber::<Impl, IMPL_OFFSET>,
            absoluteChildNumber: absoluteChildNumber::<Impl, IMPL_OFFSET>,
            formatIndex: formatIndex::<Impl, IMPL_OFFSET>,
            formatNumber: formatNumber::<Impl, IMPL_OFFSET>,
            formatDate: formatDate::<Impl, IMPL_OFFSET>,
            formatTime: formatTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXTLRuntime as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait XMLDOMDocumentEvents_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl XMLDOMDocumentEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: XMLDOMDocumentEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> XMLDOMDocumentEvents_Vtbl {
        Self { base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<XMLDOMDocumentEvents as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
